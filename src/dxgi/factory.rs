use crate::{comptr::ComPtr, impl_comptr};
use derive_newtype::NewType;
use std::{ffi::OsString, os::windows::ffi::OsStringExt};
use winapi::{
    shared::{
        dxgi::{
            CreateDXGIFactory1, IDXGIAdapter, IDXGIAdapter1, IDXGIFactory, IDXGIFactory1,
            IDXGIObject, DXGI_ADAPTER_DESC1,
        },
        dxgi1_2::IDXGIFactory2,
        dxgi1_3::{CreateDXGIFactory2, DXGI_CREATE_FACTORY_DEBUG},
        winerror::DXGI_ERROR_NOT_FOUND,
    },
    um::{unknwnbase::IUnknown, winbase::lstrlenW},
    Interface,
};

impl_comptr! {  Factory: [IDXGIFactory,]}
impl_comptr! { Factory1: [IDXGIFactory1, IDXGIFactory]}
impl_comptr! { Factory2: [IDXGIFactory2, IDXGIFactory1, IDXGIFactory]}

pub fn create_factory<I>() -> ComPtr<I>
where
    I: Interface,
{
    unsafe {
        let mut native = ComPtr::<I>::default();
        let hr = CreateDXGIFactory1(&I::uuidof(), native.getter_addrefs());
        assert!(hr == 0);
        native
    }
}

pub fn create_factory_2(debug: bool) -> Factory2 {
    let flags = if debug { DXGI_CREATE_FACTORY_DEBUG } else { 0 };

    unsafe {
        let mut factory = ComPtr::<IDXGIFactory2>::default();
        let hr = CreateDXGIFactory2(flags, &IDXGIFactory2::uuidof(), factory.getter_addrefs());
        assert!(hr == 0);
        factory.into()
    }
}

impl_comptr!( Adapter: [IDXGIAdapter, IDXGIObject, IUnknown]);
impl_comptr!(Adapter1: [IDXGIAdapter1, IDXGIAdapter, IDXGIObject, IUnknown]);

impl Factory2 {
    pub fn adapters1<'a>(&'a self) -> impl Iterator<Item = Adapter1> + 'a {
        Adapter1Iter {
            factory: self,
            curr: 0,
        }
    }
}

#[derive(Clone, Copy, Default, NewType)]
#[repr(transparent)]
pub struct AdapterDesc1(DXGI_ADAPTER_DESC1);

impl std::fmt::Debug for AdapterDesc1 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let desc_len = unsafe { lstrlenW(&self.0.Description as *const _) } as usize;
        f.debug_struct("AdapterDesc1")
            .field(
                "Description",
                &(OsString::from_wide(&self.0.Description[..desc_len])),
            )
            .field("VendorId", &self.0.VendorId)
            .field("DeviceId", &self.0.DeviceId)
            .field("SubSysId", &self.0.SubSysId)
            .field("Revision", &self.0.Revision)
            .field("DedicatedVideoMemory", &self.0.DedicatedVideoMemory)
            .field("DedicatedSystemMemory", &self.0.DedicatedSystemMemory)
            .field("SharedSystemMemory", &self.0.SharedSystemMemory)
            .field(
                "AdapterLuid",
                &format!(
                    "{}-{}",
                    self.0.AdapterLuid.HighPart, self.0.AdapterLuid.LowPart
                ),
            )
            .field("Flags", &self.0.Flags)
            .finish()
    }
}

impl Adapter1 {
    pub fn desc1(&self) -> AdapterDesc1 {
        let mut result = AdapterDesc1::default();
        let hr = unsafe { self.0.GetDesc1(&mut *result) };
        assert!(hr == 0);
        result
    }
}

struct Adapter1Iter<'a> {
    factory: &'a Factory2,
    curr: u32,
}

impl Iterator for Adapter1Iter<'_> {
    type Item = Adapter1;
    fn next(&mut self) -> Option<Self::Item> {
        let mut adapter = ComPtr::<IDXGIAdapter1>::default();
        let hr = unsafe {
            self.factory
                .0
                .EnumAdapters1(self.curr, adapter.getter_addrefs())
        };
        self.curr += 1;
        match hr {
            0 => Some(adapter.into()),
            DXGI_ERROR_NOT_FOUND => None,
            _ => unreachable!(),
        }
    }
}
