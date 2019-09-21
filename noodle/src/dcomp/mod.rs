mod device;
mod surface;
mod target;
mod visual;

use crate::{comptr::ComPtr, AsPtr};
use winapi::{shared::dxgi::IDXGIDevice, um::dcomp::DCompositionCreateDevice, Interface};

pub use device::Device;
pub use surface::{Surface, VirtualSurface};
pub use target::Target;
pub use visual::Visual;

pub fn create_device<I>(dxgi_device: &impl AsPtr<IDXGIDevice>) -> ComPtr<I>
where
    I: Interface,
{
    unsafe {
        let mut device = ComPtr::<I>::default();
        let hr =
            DCompositionCreateDevice(dxgi_device.as_ptr(), &I::uuidof(), device.getter_addrefs());
        assert!(hr == 0);
        device
    }
}
