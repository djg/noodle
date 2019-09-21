use super::DeviceContext;
use crate::impl_comptr;
use winapi::um::{
    d2d1_1::{ID2D1Device, D2D1_DEVICE_CONTEXT_OPTIONS_NONE},
    d2d1_2::ID2D1Device1,
};

impl_comptr! {  Device: [ID2D1Device, ] }
impl_comptr! { Device1: [ID2D1Device1, ID2D1Device] }

impl Device1 {
    pub fn create_device_context(&self) -> DeviceContext {
        unsafe {
            let mut native = DeviceContext::default();
            let hr =
                self.CreateDeviceContext(D2D1_DEVICE_CONTEXT_OPTIONS_NONE, native.getter_addrefs());
            assert!(hr == 0);
            native
        }
    }
}
