use super::FactoryOptions;
use crate::{comptr::ComPtr, impl_comptr, opt_ptr, AsPtr};
use winapi::{
    shared::dxgi::IDXGIDevice,
    um::{
        d2d1::{D2D1CreateFactory, ID2D1Factory, D2D1_FACTORY_TYPE_SINGLE_THREADED},
        d2d1_1::ID2D1Factory1,
        d2d1_2::ID2D1Factory2,
    },
    Interface,
};

impl_comptr! { Factory2: [ID2D1Factory2, ID2D1Factory1, ID2D1Factory]}

pub fn create_single_threaded_factory<'a, I>(
    options: impl Into<Option<&'a FactoryOptions>>,
) -> ComPtr<I>
where
    I: Interface,
{
    unsafe {
        let mut native = ComPtr::<I>::default();
        let hr = D2D1CreateFactory(
            D2D1_FACTORY_TYPE_SINGLE_THREADED,
            &I::uuidof(),
            opt_ptr(options.into()),
            native.getter_addrefs(),
        );
        assert!(hr == 0);
        native
    }
}

impl Factory2 {
    pub fn create_device<I>(&self, device: &impl AsPtr<IDXGIDevice>) -> ComPtr<I>
    where
        I: Interface,
    {
        unsafe {
            let mut native = ComPtr::<I>::default();
            let hr = self.CreateDevice(device.as_ptr(), native.getter_addrefs());
            assert!(hr == 0);
            native
        }
    }
}
