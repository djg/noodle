mod device;
mod surface;
mod target;
mod visual;

use crate::{comptr::ComPtr, AsPtr};
use winapi::{
    um::{
        dcomp::{DCompositionCreateDevice, DCompositionCreateDevice2, DCompositionCreateDevice3},
        unknwnbase::IUnknown,
    },
    Interface,
};

pub use device::{DesktopDevice, Device, Device2, DeviceDebug};
pub use surface::{Surface, VirtualSurface};
pub use target::Target;
pub use visual::{Insert, Visual, Visual2, Visual3, VisualDebug};

macro_rules! impl_create_device {
    ($($name:ident, $create_device:ident),*) => {
        $(pub fn $name<I, T>(dxgi_device: &impl AsPtr<IUnknown>) -> T
        where
            T: From<ComPtr<I>>,
            I: Interface,
        {
            unsafe {
                let mut device = ComPtr::<I>::default();
                let hr = $create_device(
                    dxgi_device.as_ptr() as *const _,
                    &I::uuidof(),
                    device.getter_addrefs()
                );
                assert!(hr == 0);
                device.into()
            }
        })*
    }
}

impl_create_device! {
    create_device  , DCompositionCreateDevice,
    create_device_2, DCompositionCreateDevice2,
    create_device_3, DCompositionCreateDevice3
}
