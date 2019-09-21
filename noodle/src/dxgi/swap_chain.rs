use crate::{comptr::ComPtr, impl_comptr, AsPtr};
use winapi::{
    shared::{
        dxgi::{IDXGIDeviceSubObject, IDXGIObject, IDXGISurface, IDXGISwapChain},
        dxgi1_2::IDXGISwapChain1,
    },
    um::unknwnbase::IUnknown,
    Interface,
};

pub use winapi::shared::dxgi1_2::DXGI_SWAP_CHAIN_DESC1 as SwapChainDesc1;

impl_comptr! { SwapChain : [IDXGISwapChain, IDXGIDeviceSubObject, IDXGIObject, IUnknown] }
impl_comptr! { SwapChain1: [IDXGISwapChain1, IDXGISwapChain, IDXGIDeviceSubObject, IDXGIObject, IUnknown] }

impl SwapChain1 {
    pub fn get_buffer<I>(&self, buffer: u32) -> ComPtr<I>
    where
        ComPtr<I>: AsPtr<IDXGISurface>,
        I: Interface,
    {
        unsafe {
            let mut native = ComPtr::<I>::default();
            let hr = self.GetBuffer(buffer, &I::uuidof(), native.getter_addrefs());
            assert!(hr == 0);
            native
        }
    }

    pub fn present(&self, sync_interval: u32, flags: u32) {
        unsafe {
            let hr = self.Present(sync_interval, flags);
            assert!(hr == 0);
        }
    }
}
