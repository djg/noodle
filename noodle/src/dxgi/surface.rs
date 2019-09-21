use crate::impl_comptr;
use winapi::shared::{
    dxgi::{IDXGIDeviceSubObject, IDXGIObject, IDXGISurface, IDXGISurface1},
    dxgi1_2::IDXGISurface2,
};

impl_comptr! { Surface: [IDXGISurface, IDXGIDeviceSubObject, IDXGIObject] }
impl_comptr! { Surface1: [IDXGISurface1, IDXGISurface, IDXGIDeviceSubObject, IDXGIObject] }
impl_comptr! { Surface2: [IDXGISurface2, IDXGISurface1, IDXGISurface, IDXGIDeviceSubObject, IDXGIObject] }
