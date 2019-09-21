use crate::impl_comptr;
use winapi::{
    shared::{
        dxgi::{IDXGIDevice, IDXGIDevice1},
        dxgi1_2::IDXGIDevice2,
        dxgi1_3::IDXGIDevice3,
        dxgi1_5::IDXGIDevice4,
    },
    um::unknwnbase::IUnknown,
};

impl_comptr!( Device: [IDXGIDevice, IUnknown]);
impl_comptr!(Device1: [IDXGIDevice1, IDXGIDevice, IUnknown]);
impl_comptr!(Device2: [IDXGIDevice2, IDXGIDevice1, IDXGIDevice, IUnknown]);
impl_comptr!(Device3: [IDXGIDevice3, IDXGIDevice2, IDXGIDevice1, IDXGIDevice, IUnknown]);
impl_comptr!(Device4: [IDXGIDevice4, IDXGIDevice3, IDXGIDevice2, IDXGIDevice1, IDXGIDevice, IUnknown]);
