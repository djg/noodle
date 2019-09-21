use super::{Output, SwapChain1, SwapChainDesc1};
use crate::{comptr::ComPtr, impl_comptr, AsPtr};
use winapi::{
    shared::{
        dxgi::{CreateDXGIFactory1, IDXGIFactory, IDXGIFactory1},
        dxgi1_2::IDXGIFactory2,
        dxgi1_3::{CreateDXGIFactory2, DXGI_CREATE_FACTORY_DEBUG},
    },
    um::unknwnbase::IUnknown,
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

pub fn create_factory_2<I>(debug: bool) -> ComPtr<I>
where
    I: Interface,
{
    let flags = if debug { DXGI_CREATE_FACTORY_DEBUG } else { 0 };

    unsafe {
        let mut native = ComPtr::<I>::default();
        let hr = CreateDXGIFactory2(flags, &I::uuidof(), native.getter_addrefs());
        assert!(hr == 0);
        native
    }
}

impl Factory2 {
    pub fn create_swap_chain_for_composition(
        &self,
        device: &impl AsPtr<IUnknown>,
        desc: &SwapChainDesc1,
        restrict_to_output: impl Into<Option<Output>>,
    ) -> SwapChain1 {
        let restrict_to_output = restrict_to_output.into();
        unsafe {
            let mut swap_chain = SwapChain1::default();
            let hr = self.CreateSwapChainForComposition(
                device.as_ptr(),
                desc,
                restrict_to_output.as_ptr(),
                swap_chain.getter_addrefs(),
            );
            assert!(hr == 0);
            swap_chain
        }
    }
}
