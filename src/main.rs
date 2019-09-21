use lib_noodle::{d2d, d3d11, dcomp, dxgi, winuser};
use std::ffi::OsStr;
use winapi::um::winuser::*;

struct ExampleWindow;

impl winuser::WindowDelegate for ExampleWindow {
    fn class_name(&self) -> &OsStr {
        OsStr::new("window")
    }
    fn handle_message(&mut self, message: winuser::Message) -> isize {
        use winuser::MessageKind;
        match message.kind {
            MessageKind::Destroy => {
                winuser::post_quit_message(0);
                0
            }
            _ => winuser::default_handle_message(message),
        }
    }
}

fn main() {
    let mut delegate = ExampleWindow;
    let window = winuser::Window::create(
        &mut delegate,
        OsStr::new("Sample"),
        WS_OVERLAPPEDWINDOW | WS_VISIBLE,
        800,
        600,
    )
    .unwrap();

    let d3d11_device = d3d11::create_hardware_device();
    let dxgi_device: dxgi::Device = d3d11_device.query_interface().unwrap();
    let dxgi_factory: dxgi::Factory2 = dxgi::create_factory_2(true);

    let rect = window.client_rect().unwrap();
    let desc = dxgi::SwapChainDesc1 {
        Width: rect.width() as u32,
        Height: rect.height() as u32,
        Format: dxgi::Format::Bgra8.into(),
        BufferUsage: dxgi::Usage::RENDER_TARGET_OUTPUT.into(),
        SwapEffect: dxgi::SwapEffect::FlipSequential.into(),
        BufferCount: 2,
        SampleDesc: dxgi::SampleDesc::NoAntiAliasing.into(),
        AlphaMode: dxgi::AlphaMode::Premultiplied.into(),
        ..Default::default()
    };

    let swap_chain = dxgi_factory.create_swap_chain_for_composition(&dxgi_device, &desc, None);

    // Create a single-threaded Direct2D factory with debugging information
    let options: d2d::FactoryOptions = d2d::DebugLevel::Information.into();
    let d2d_factory: d2d::Factory2 = d2d::create_single_threaded_factory(&options);

    // Create the Direct2D device that links back to the Direct3D device
    let d2d_device: d2d::Device1 = d2d_factory.create_device(&dxgi_device);

    // Create the Direct2D device context that is the actual render target
    // and exposes drawing commands
    let dc: d2d::DeviceContext = d2d_device.create_device_context();

    // Retrieve the swap chain's back buffer
    let surface: dxgi::Surface2 = swap_chain.get_buffer(0);

    // Create a Direct2D bitmap that points to the swap chain surface
    let properties = d2d::BitmapProperties1 {
        pixelFormat: d2d::PixelFormat {
            format: dxgi::Format::Bgra8.into(),
            alphaMode: dxgi::AlphaMode::Premultiplied.into(),
        },
        bitmapOptions: d2d::BITMAP_OPTIONS_TARGET | d2d::BITMAP_OPTIONS_CANNOT_DRAW,
        ..Default::default()
    };

    let bitmap = dc.create_bitmap_from_dxgi_surface(&surface, &properties);

    // Point the device context to the bitmap for rendering
    dc.set_target(&bitmap);

    // Draw something
    dc.begin_draw();
    {
        dc.clear();

        let brush_color = d2d::ColorF {
            r: 0.18,
            g: 0.55,
            b: 0.34,
            a: 0.75,
        };
        let brush = dc.create_solid_color_brush(&brush_color, None);
        let ellipse_center = d2d::Point2F { x: 150.0, y: 150.0 };
        let ellipse = d2d::Ellipse {
            point: ellipse_center,
            radiusX: 100.0,
            radiusY: 100.0,
        };
        dc.fill_ellipse(&ellipse, &brush);
    }
    dc.end_draw();

    // Make the swap chain available to the composition engine
    swap_chain.present(1, 0);

    let dcomp_device: dcomp::Device = dcomp::create_device(&dxgi_device);
    let visual = dcomp_device.create_visual();
    visual.set_content(&swap_chain);

    let target = dcomp_device.create_target_for_hwnd(window, true);
    target.set_root(&visual);

    dcomp_device.commit();

    winuser::run_loop();
}
