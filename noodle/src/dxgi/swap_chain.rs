use crate::{
    dxgi::{Factory2, Output},
    impl_comptr, AsPtr, ComPtr,
};
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

impl Factory2 {
    pub fn create_swap_chain_for_composition(
        &self,
        device: &impl AsPtr<IUnknown>,
        desc: &SwapChainDesc1,
        restrict_to_output: impl Into<Option<Output>>,
    ) -> SwapChain1 {
        let restrict_to_output = restrict_to_output.into();
        unsafe {
            let mut swap_chain = ComPtr::<IDXGISwapChain1>::default();
            let hr = self.0.CreateSwapChainForComposition(
                device.as_ptr(),
                desc,
                restrict_to_output.as_ptr(),
                swap_chain.getter_addrefs(),
            );
            assert!(hr == 0);
            swap_chain.into()
        }
    }
}

impl SwapChain1 {
    pub fn get_buffer<I>(&self, buffer: u32) -> ComPtr<I>
    where
        ComPtr<I>: AsPtr<IDXGISurface>,
        I: Interface,
    {
        unsafe {
            let mut native = ComPtr::<I>::default();
            let hr = self
                .0
                .GetBuffer(buffer, &I::uuidof(), native.getter_addrefs());
            assert!(hr == 0);
            native
        }
    }

    pub fn present(&self, sync_interval: u32, flags: u32) {
        unsafe {
            let hr = self.0.Present(sync_interval, flags);
            assert!(hr == 0);
        }
    }
}
