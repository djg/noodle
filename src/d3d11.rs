use crate::{impl_comptr, ComPtr};
use std::ptr;
use winapi::um::{
    d3d11::{D3D11CreateDevice, ID3D11Device, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION},
    d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
    unknwnbase::IUnknown
};

impl_comptr! { Device: [ID3D11Device, IUnknown]}

pub fn create_hardware_device() -> Device {
    unsafe {
        let mut device = ComPtr::<ID3D11Device>::default();
        let hr = D3D11CreateDevice(
            ptr::null_mut(),
            D3D_DRIVER_TYPE_HARDWARE,
            ptr::null_mut(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            ptr::null_mut(),
            0,
            D3D11_SDK_VERSION,
            device.getter_addrefs(),
            ptr::null_mut(),
            ptr::null_mut(),
        );

        assert!(hr == 0);
        device.into()
    }
}
