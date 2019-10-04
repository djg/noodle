#![warn(clippy::all)]
#![feature(clamp)]

use itertools::iproduct;
use noodle::{
    d2d, d3d11, dcomp, dwrite, dxgi, offset, point, size, winuser, Point, Rect, Size, ToUtf16,
};
use std::ffi::OsStr;
use winapi::um::{d3d11::D3D11_REQ_TEXTURE2D_U_OR_V_DIMENSION, winuser::*};

fn hls_to_rgb(hue: f32, saturation: f32, lightness: f32) -> [f32; 3] {
    use std::f32::consts::{FRAC_PI_3, PI};

    let saturation = saturation.clamp(0.0, 1.0);
    let lightness = lightness.clamp(0.0, 1.0);

    let hue = hue % (2.0 * PI);
    let c = saturation * (1.0 - (2.0 * lightness - 1.0).abs());
    let x = c * (1.0 - (((hue / FRAC_PI_3) % 2.0) - 1.0).abs());
    let m = lightness - c / 2.0;
    let (red, green, blue) = match hue {
        h if h < 1.0 * FRAC_PI_3 => (c, x, 0.0),
        h if h < 2.0 * FRAC_PI_3 => (x, c, 0.0),
        h if h < 3.0 * FRAC_PI_3 => (0.0, c, x),
        h if h < 4.0 * FRAC_PI_3 => (0.0, x, c),
        h if h < 5.0 * FRAC_PI_3 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    [red + m, green + m, blue + m]
}

#[derive(Clone, Copy, Debug, Default)]
struct Tile {
    rect: Rect,
    row: i32,
    column: i32,
}

impl Tile {
    fn new(row: i32, column: i32, tile_size: i32) -> Self {
        let x = column * tile_size;
        let y = row * tile_size;
        Self {
            row,
            column,
            rect: point!(x, y) + size!(tile_size),
        }
    }

    fn color(&self, surface_size: i32) -> d2d::Color {
        use std::f32::consts::PI;

        let hue = 2.0 * PI * (self.rect.top as f32 + self.rect.left as f32 / surface_size as f32)
            / surface_size as f32;
        d2d::Color::new(hls_to_rgb(hue, 1.0, 0.5))
    }
}

struct TileRenderer {
    text_format: dwrite::TextFormat,
    virtual_surface: dcomp::VirtualSurface,
    visual: dcomp::Visual2,
    surface_size: i32,
}

impl TileRenderer {
    fn new(dcomp_device: &dcomp::DesktopDevice, surface_size: i32) -> Self {
        // Create the Dwrite factory and text format object
        let dwrite_factory: dwrite::Factory = dwrite::create_shared_factory();
        let text_format = dwrite_factory.create_text_format(
            "Segoe UI",
            dwrite::FontWeight::Bold,
            dwrite::FontStyle::Normal,
            dwrite::FontStretch::Normal,
            60.0,
            "en-US",
        );
        text_format.set_paragraph_alignment(dwrite::ParagraphAlignment::Center);
        text_format.set_text_alignment(dwrite::TextAlignment::Center);

        // m_surfaceBrush = CreateVirtualDrawingSurfaceBrush();
        let virtual_surface = dcomp_device.create_virtual_surface(
            surface_size as u32,
            surface_size as u32,
            dxgi::Format::Bgra8,
            dxgi::AlphaMode::Ignore,
        );

        let visual = dcomp_device.create_visual();
        visual.set_content(&virtual_surface);

        #[cfg(feature = "paint-flashing")]
        {
            let visual_debug: dcomp::VisualDebug = visual.as_().unwrap();
            visual_debug.enable_redraw_regions();
        }

        Self {
            text_format,
            virtual_surface,
            visual,
            surface_size,
        }
    }

    fn draw_tile_range(&self, dc: &d2d::DeviceContext1, rect: &Rect, tiles: &[Tile]) {
        let update_size = rect.size() - 5;
        // Making sure the update rect doesn't go past the maximum size of the surface.
        let update_rect = *rect & Rect::new(0, 0, self.surface_size, self.surface_size);
        // Cannot update a surface larger than the max texture size of the hardware. 2048x2048 is the lowest max text suze of relevant hardware.
        const MAX_TEXTURE_SIZE: i32 = D3D11_REQ_TEXTURE2D_U_OR_V_DIMENSION as i32;
        // 3 is the buffer here.
        let constrained_update_size = Size::new(
            std::cmp::min(update_size.width(), MAX_TEXTURE_SIZE - 3),
            std::cmp::min(update_size.height(), MAX_TEXTURE_SIZE - 3),
        );

        // Breaking the BeginDraw/EndDraw calls to update rects that dont exceed the max texture size.
        for y in
            (update_rect.top..update_rect.bottom).step_by(constrained_update_size.height() as usize)
        {
            for x in (update_rect.left..update_rect.right)
                .step_by(constrained_update_size.width() as usize)
            {
                let constrained_update_rect = Rect::new(
                    x,
                    y,
                    std::cmp::min(x + constrained_update_size.width(), update_rect.right),
                    std::cmp::min(y + constrained_update_size.height(), update_rect.bottom),
                );

                let properties = d2d::BitmapProperties1 {
                    pixelFormat: d2d::PixelFormat {
                        format: dxgi::Format::Bgra8.into(),
                        alphaMode: dxgi::AlphaMode::Ignore.into(),
                    },
                    bitmapOptions: d2d::BITMAP_OPTIONS_TARGET | d2d::BITMAP_OPTIONS_CANNOT_DRAW,
                    ..Default::default()
                };

                self.virtual_surface
                    .draw(&constrained_update_rect, |dxgi_surface, offset| {
                        // Create a D2D compatible bitmap from the surface for rendering
                        let bitmap: d2d::Bitmap1 =
                            dc.create_bitmap_from_dxgi_surface(&dxgi_surface, &properties);

                        // Point the device context to the bitmap for rendering
                        dc.set_target(&bitmap);

                        // Draw something
                        dc.draw(|dc| {
                            dc.clear(&d2d::Color::from(d2d::NamedColor::LightGray));

                            // Create a solid color brush for the text. Half alpha to make it more visually pleasing as it blends with the background color.
                            let text_brush = dc.create_solid_color_brush(
                                &(d2d::NamedColor::DimGray, 0.5).into(),
                                None,
                            );

                            // Create a solid color brush for the tiles and which will be set to a different color before rendering.
                            let tile_brush =
                                dc.create_solid_color_brush(&d2d::NamedColor::Green.into(), None);

                            // Get the offset difference that can be applied to every tile before drawing.
                            let difference_offset = offset - (x, y);

                            // Iterate through the tiles and do DrawRectangle and DrawText calls on those.
                            for tile in tiles {
                                // DrawTile(d2dDeviceContext.get(), textBrush.get(), tileBrush.get(), tile, differenceOffset);
                                tile_brush.set_color(&tile.color(self.surface_size));

                                const BORDER_MARGIN: Size = Size::new(5, 5);
                                let tile_rectangle =
                                    (tile.rect + difference_offset - BORDER_MARGIN).into();
                                dc.fill_rectangle(&tile_rectangle, &tile_brush);

                                // DrawTextInTile(tile.row, tile.column, tileRectangle, d2dDeviceContext, textBrush);
                                let text = format!("{},{}", tile.row, tile.column).to_utf16();

                                // Drawing the text in the second third of the rectangle, so it is centered. The centerRect is the new rectangle that is 1/3rd of the height and placed at the center of the Tile.
                                dc.draw_text(
                                    &text,
                                    &self.text_format,
                                    &tile_rectangle,
                                    &text_brush,
                                );
                            }
                        });
                    });
            }
        }
    }

    pub fn trim(&self, trim_rect: &Rect) {
        self.virtual_surface.trim(&[*trim_rect]);
    }
}

#[derive(Default)]
struct TileDrawingManager {
    // Keeps track of the drawn tiles
    drawn_tile_rect: Rect,
    /// Current position.
    current_position: Point,
    /// Size of the viewport.
    view_port_size: Size,
}

impl TileDrawingManager {
    const TILE_SIZE: i32 = 512;
    //const MAX_SURFACE_SIZE: i32 = Self::TILE_SIZE * 10000;
    const MAX_SURFACE_SIZE: i32 = Self::TILE_SIZE * 100;
    // Number of tiles to draw ahead
    const DRAW_AHEAD_TILE_COUNT: i32 = 1;

    /// More unloaded surface is now visible on screen because of some event like manipulations(zoom, pan, etc.). This method, figures
    ///	out the new areas that need to be rendered and fires the draw calls. This is the core of the tile drawing logic
    pub fn update_visible_region(
        &mut self,
        current_position: Point,
        draw_fn: impl Fn(&Rect, &[Tile]),
        trim_fn: impl Fn(&Rect),
    ) {
        self.current_position = current_position;

        let required_tile_rect =
            (self.current_position + self.view_port_size) / Self::TILE_SIZE + Size::new(1, 1);
        let required_tile_rect =
            required_tile_rect.outset(Self::DRAW_AHEAD_TILE_COUNT, Self::DRAW_AHEAD_TILE_COUNT);
        let required_tile_rect = required_tile_rect & Rect::new(0, 0, 10000, 10000);

        // Draws the tiles that are required but not drawn.
        let rects_to_draw = required_tile_rect - self.drawn_tile_rect;
        if rects_to_draw.is_empty() {
            return;
        }

        for rect in &rects_to_draw {
            Self::draw_tile_range(rect, &draw_fn);
        }

        // Trimming the tiles that are not visible on screen
        self.trim(&required_tile_rect, trim_fn);
    }

    /// Updates the Viewport Size of the application.
    pub fn update_viewport_size(
        &mut self,
        new_size: Size,
        draw_fn: impl Fn(&Rect, &[Tile]),
        trim_fn: impl Fn(&Rect),
    ) {
        self.view_port_size = new_size;

        self.update_visible_region(self.current_position, draw_fn, trim_fn)
    }

    /// Converts the tile coordinates into a list of Tile Objects that can be sent to the renderer.
    fn tiles_for_range(tile_rect: &Rect) -> Vec<Tile> {
        // get Tile objects for each tile that needs to be rendered.
        iproduct!(
            tile_rect.left..tile_rect.right,
            tile_rect.top..tile_rect.bottom
        )
        .map(|(i, j)| Tile::new(j, i, Self::TILE_SIZE))
        .collect::<_>()
    }

    fn draw_tile_range(tile_rect: &Rect, f: &dyn Fn(&Rect, &[Tile])) {
        let draw_rect = *tile_rect * Self::TILE_SIZE;
        f(&draw_rect, &Self::tiles_for_range(&tile_rect));
    }

    /// This function combines all the tiles into a single call, so the rendering is faster as opposed to calling BeginDraw on each tile.
    /*    fn draw_visible_tiles_by_range(&mut self,  f: impl Fn(&Rect, &dyn Iterator<Item = Tile>)) {
            // The DRAWAHEADTILECOUNT draws tiles that the configured number of tiles outside the viewport to make sure the user doesnt see a lot
            // of empty areas when scrolling.
            Self::draw_tile_range(
                0,
                0,
                self.horizontal_visible_tile_count + Self::DRAW_AHEAD_TILE_COUNT,
                self.vertical_visible_tile_count + Self::DRAW_AHEAD_TILE_COUNT,
                f,
            );

            // update the tiles that are already drawn, so only the new tiles will have to be rendered when panning.
            self.drawn_right_tile_column =
                self.horizontal_visible_tile_count - 1 + Self::DRAW_AHEAD_TILE_COUNT;
            self.drawn_bottom_tile_row = self.vertical_visible_tile_count - 1 + Self::DRAW_AHEAD_TILE_COUNT;
        }
    */

    /// Trims the tiles that are outside these co-ordinates. So only the contents that are visible are rendered, to save on memory.
    fn trim(&mut self, required_rect: &Rect, trim_fn: impl Fn(&Rect)) {
        let trim_rect = *required_rect * Self::TILE_SIZE;
        trim_fn(&trim_rect);
        self.drawn_tile_rect = *required_rect;
    }
}

fn create_d2d_device_context(dxgi_device: &dxgi::Device) -> d2d::DeviceContext1 {
    // Create a single-threaded Direct2D factory with debugging information
    let options: d2d::FactoryOptions = d2d::DebugLevel::Information.into();
    let d2d_factory: d2d::Factory2 = d2d::create_single_threaded_factory(&options);

    // Create the Direct2D device that links back to the Direct3D device
    let d2d_device: d2d::Device1 = d2d_factory.create_device(&dxgi_device);
    // Create the Direct2D device context that is the actual render target
    // and exposes drawing commands
    d2d_device.create_device_context()
}

fn create_dcomp_device(dxgi_device: &dxgi::Device) -> dcomp::DesktopDevice {
    let dcomp_device: dcomp::DesktopDevice = dcomp::create_device_2(&dxgi_device);
    //let dcomp_device: dcomp::DesktopDevice = dcomp::create_device_2(&d2d_device);
    let dcomp_device_debug: dcomp::DeviceDebug = dcomp_device.as_().unwrap();
    dcomp_device_debug.enable_debug_counters();
    dcomp_device
}

fn main() {
    let window = winuser::Window::create(
        OsStr::new("Window"),
        OsStr::new("Sample"),
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        800,
        600,
    )
    .unwrap();

    let d3d11_device = d3d11::create_hardware_device();
    let dxgi_device: dxgi::Device = d3d11_device.as_().unwrap();

    let d2d_dc = create_d2d_device_context(&dxgi_device);
    let dcomp_device = create_dcomp_device(&dxgi_device);

    let mut tile_drawing_manager = TileDrawingManager::default();
    let tile_renderer = TileRenderer::new(&dcomp_device, TileDrawingManager::MAX_SURFACE_SIZE);

    let target = dcomp_device.create_target_for_hwnd(window, true);
    target.set_root(&tile_renderer.visual);

    let window_size = window.window_rect().size();
    tile_drawing_manager.update_viewport_size(
        window_size,
        |draw_rect, tiles| tile_renderer.draw_tile_range(&d2d_dc, draw_rect, tiles),
        |required_rect| tile_renderer.trim(required_rect),
    );
    dcomp_device.commit();

    let mut current_position = point!(0, 0);
    let mut offset = offset!(5, 10);

    loop {
        let quit = winuser::process_pending_events(|message| {
            use winuser::MessageKind;
            match message.kind {
                MessageKind::Destroy => {
                    winuser::post_quit_message(0);
                    true
                }
                MessageKind::Paint => {
                    // Check the dcomp device is OK
                    false
                }
                MessageKind::Size { .. } => {
                    // Update the view port to the new size of the parent window
                    let window_size = message.window.window_rect().size();
                    //tile_renderer.visual.set_size(&window_size);
                    tile_drawing_manager.update_viewport_size(
                        window_size,
                        |draw_rect, tiles| tile_renderer.draw_tile_range(&d2d_dc, draw_rect, tiles),
                        |required_rect| tile_renderer.trim(required_rect),
                    );
                    true
                }
                _ => false,
            }
        });

        if quit {
            break;
        }

        tile_renderer
            .visual
            .set_offset([-current_position.x as f32, -current_position.y as f32]);

        tile_drawing_manager.update_visible_region(
            current_position,
            |draw_rect, tiles| tile_renderer.draw_tile_range(&d2d_dc, draw_rect, tiles),
            |required_rect| tile_renderer.trim(required_rect),
        );

        current_position += offset;

        current_position.x = match current_position.x {
            x if x < 0 => {
                offset.x = -offset.x;
                0
            }
            x if x
                >= TileDrawingManager::MAX_SURFACE_SIZE
                    - tile_drawing_manager.view_port_size.width() =>
            {
                offset.x = -offset.x;
                TileDrawingManager::MAX_SURFACE_SIZE - tile_drawing_manager.view_port_size.width()
            }
            x => x,
        };

        current_position.y = match current_position.y {
            y if y < 0 => {
                offset.y = -offset.y;
                0
            }
            y if y
                >= TileDrawingManager::MAX_SURFACE_SIZE
                    - tile_drawing_manager.view_port_size.height() =>
            {
                offset.y = -offset.y;
                TileDrawingManager::MAX_SURFACE_SIZE - tile_drawing_manager.view_port_size.height()
            }
            y => y,
        };

        dcomp_device.commit();
        dcomp_device.wait_for_commit_completion();
    }
}
