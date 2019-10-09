#![warn(clippy::all)]
#![feature(clamp)]

use noodle::{d3d11, dxgi};

fn main() {
    let d3d11_device = d3d11::create_hardware_device();
    let _dxgi_device: dxgi::Device1 = d3d11_device.as_().unwrap();

    let dxgi_factory: dxgi::Factory2 = dxgi::create_factory_2(true);
    for adapter in dxgi_factory.adapters1() {
        println!("desc1: {:?}", adapter.desc1());
        for output in adapter.outputs() {
            println!("output: {:?}", output);
            let output3: Option<dxgi::Output3> = output.as_();
            if let Some(output3) = output3 {
                println!("output3: {:?}", output3);
                println!(" - Supports Overlay: {}", output3.supports_overlays());
                println!(
                    " - Overlay Support: Bgra8 {:?}",
                    output3.check_overlay_support(dxgi::Format::Bgra8, &d3d11_device)
                );
                println!(
                    " - Overlay Support: Nv12 {:?}",
                    output3.check_overlay_support(dxgi::Format::Nv12, &d3d11_device)
                );
            }
        }
    }
}
