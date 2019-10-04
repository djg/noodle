use super::DeviceContext1;
use crate::{impl_comptr, ComPtr};
use winapi::um::{
    d2d1::ID2D1Resource,
    d2d1_1::{ID2D1Device, D2D1_DEVICE_CONTEXT_OPTIONS_NONE},
    d2d1_2::{ID2D1Device1, ID2D1DeviceContext1},
    unknwnbase::IUnknown,
};

impl_comptr! {  Device: [ID2D1Device, ID2D1Resource, IUnknown] }
impl_comptr! { Device1: [ID2D1Device1, ID2D1Device, ID2D1Resource, IUnknown] }

impl Device1 {
    pub fn create_device_context(&self) -> DeviceContext1 {
        unsafe {
            let mut native = ComPtr::<ID2D1DeviceContext1>::default();
            let hr = self
                .0
                .CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE, native.getter_addrefs());
            assert!(hr == 0);
            DeviceContext1(native)
        }
    }
}
