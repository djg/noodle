#![warn(clippy::all)]
#![feature(clamp)]

use noodle::{d2d, d3d11, dcomp, dxgi, rect, winuser};
use std::ffi::OsStr;
use winapi::um::winuser::*;

fn handle_message(message: &winuser::Message) -> bool {
    use winuser::MessageKind;
    match message.kind {
        MessageKind::Destroy => {
            winuser::post_quit_message(0);
            true
        }
        _ => false,
    }
}

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

/// Convert `a` in range [0..3600] into radians
fn to_radians(a: u32) -> f32 {
    use std::f32::consts::PI;
    PI * (a as f32) / 1800.0
}

fn main() {
    let window = winuser::Window::create(
        OsStr::new("window"),
        OsStr::new("Sample"),
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        800,
        600,
    )
    .unwrap();

    let d3d11_device = d3d11::create_hardware_device(d3d11::CreateDevice::BGRA_SUPPORT);
    let dxgi_device: dxgi::Device = d3d11_device.as_().unwrap();

    // Create a single-threaded Direct2D factory with debugging information
    let options: d2d::FactoryOptions = d2d::DebugLevel::Information.into();
    let d2d_factory: d2d::Factory2 = d2d::create_single_threaded_factory(&options);

    // Create the Direct2D device that links back to the Direct3D device
    let d2d_device: d2d::Device1 = d2d_factory.create_device(&dxgi_device);
    // Create the Direct2D device context that is the actual render target
    // and exposes drawing commands
    let d2d_dc: d2d::DeviceContext1 = d2d_device.create_device_context();

    let dcomp_device: dcomp::DesktopDevice = dcomp::create_device_2(&dxgi_device);
    //let dcomp_device: dcomp::DesktopDevice = dcomp::create_device_2(&d2d_device);
    let dcomp_device_debug: dcomp::DeviceDebug = dcomp_device.as_().unwrap();
    dcomp_device_debug.enable_debug_counters();

    let rect = window.client_rect();
    let surface = dcomp_device.create_surface(
        rect.width() as u32,
        rect.height() as u32,
        dxgi::Format::Bgra8,
        //dxgi::AlphaMode::Ignore,
        dxgi::AlphaMode::Premultiplied,
    );

    let visual = dcomp_device.create_visual();
    visual.set_content(&surface);

    if false {
        let visual_debug: dcomp::VisualDebug = visual.as_().unwrap();
        visual_debug.enable_redraw_regions();
    }

    let target = dcomp_device.create_target_for_hwnd(window, true);
    target.set_root(&visual);

    // Create a Direct2D bitmap that points to the swap chain surface
    let properties = d2d::BitmapProperties1 {
        pixelFormat: d2d::PixelFormat {
            format: dxgi::Format::Bgra8.into(),
            alphaMode: dxgi::AlphaMode::Premultiplied.into(),
        },
        bitmapOptions: d2d::BITMAP_OPTIONS_TARGET | d2d::BITMAP_OPTIONS_CANNOT_DRAW,
        ..Default::default()
    };

    surface.draw(None, |dxgi_surface, _| {
        let bitmap: d2d::Bitmap1 =
            d2d_dc.create_bitmap_from_dxgi_surface(&dxgi_surface, &properties);

        // Point the device context to the bitmap for rendering
        d2d_dc.set_target(&bitmap);

        // Draw something
        d2d_dc.draw(|dc| {
            dc.clear(None);
        });
    });
    dcomp_device.commit();

    let mut angle = 0u32;

    loop {
        if winuser::process_pending_events(handle_message) {
            break;
        }

        surface.draw(&rect!(50, 50, 250, 250,), |dxgi_surface, offset| {
            let bitmap: d2d::Bitmap1 =
                d2d_dc.create_bitmap_from_dxgi_surface(&dxgi_surface, &properties);

            // Point the device context to the bitmap for rendering
            d2d_dc.set_target(&bitmap);

            // Draw something
            d2d_dc.draw(|dc| {
                dc.clear(None);

                let rgb = hls_to_rgb(to_radians(angle), 1.0, 0.5);

                let brush_color = d2d::Color::new(rgb);
                let brush = dc.create_solid_color_brush(&brush_color, None);
                let ellipse_center = d2d::Point2F {
                    x: offset.x as f32 + 100.0,
                    y: offset.y as f32 + 100.0,
                };
                let ellipse = d2d::Ellipse {
                    point: ellipse_center,
                    radiusX: 100.0,
                    radiusY: 100.0,
                };
                dc.fill_ellipse(&ellipse, &brush);
            });
        });

        dcomp_device.commit();
        dcomp_device.wait_for_commit_completion();

        angle = if angle < 3600 { angle + 1 } else { 0 };
    }
}
