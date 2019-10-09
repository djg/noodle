use crate::{
    dxgi::{Adapter1, Format, OverlaySupport},
    impl_comptr, impl_interface, AsPtr, ComPtr,
};
use winapi::{
    shared::{
        dxgi::{IDXGIObject, IDXGIOutput},
        dxgi1_2::IDXGIOutput1,
        dxgi1_3::{IDXGIOutput2, IDXGIOutput3},
        dxgi1_4::IDXGIOutput4,
        dxgi1_5::IDXGIOutput5,
        dxgi1_6::IDXGIOutput6,
        minwindef::FALSE,
        winerror::DXGI_ERROR_NOT_FOUND,
    },
    um::unknwnbase::IUnknown,
};

impl Adapter1 {
    pub fn outputs<'a>(&'a self) -> impl Iterator<Item = Output> + 'a {
        OutputIter {
            adapter: self,
            curr: 0,
        }
    }
}

impl_comptr!( Output: [IDXGIOutput, IDXGIObject, IUnknown]);
impl_comptr!(Output1: [IDXGIOutput1, IDXGIOutput, IDXGIObject, IUnknown]);
impl_comptr!(Output2: [IDXGIOutput2, IDXGIOutput1, IDXGIOutput, IDXGIObject, IUnknown]);
impl_comptr!(Output3: [IDXGIOutput3, IDXGIOutput2, IDXGIOutput1, IDXGIOutput, IDXGIObject, IUnknown]);
impl_comptr!(Output4: [IDXGIOutput4, IDXGIOutput3, IDXGIOutput2, IDXGIOutput1, IDXGIOutput, IDXGIObject, IUnknown]);
impl_comptr!(Output5: [IDXGIOutput5, IDXGIOutput4, IDXGIOutput3, IDXGIOutput2, IDXGIOutput1, IDXGIOutput, IDXGIObject, IUnknown]);
impl_comptr!(Output6: [IDXGIOutput6, IDXGIOutput5, IDXGIOutput4, IDXGIOutput3, IDXGIOutput2, IDXGIOutput1, IDXGIOutput, IDXGIObject, IUnknown]);

struct OutputIter<'a> {
    adapter: &'a Adapter1,
    curr: u32,
}

impl Iterator for OutputIter<'_> {
    type Item = Output;
    fn next(&mut self) -> Option<Self::Item> {
        let mut output = ComPtr::<IDXGIOutput>::default();
        let hr = unsafe {
            self.adapter
                .0
                .EnumOutputs(self.curr, output.getter_addrefs())
        };
        self.curr += 1;
        match hr {
            0 => Some(output.into()),
            DXGI_ERROR_NOT_FOUND => None,
            _ => unreachable!(),
        }
    }
}

impl_interface! {
    impl [Output2, Output3, Output4, Output5, Output6] {
        pub fn supports_overlays(&self) -> bool {
            unsafe { self.0.SupportsOverlays() != FALSE }
        }
    }
}

impl_interface! {
    impl [Output3, Output4, Output5, Output6] {
        pub fn check_overlay_support(
            &self,
            format: Format,
            concerned_device: &impl AsPtr<IUnknown>,
        ) -> OverlaySupport {
            let mut flags = 0;
            let hr = unsafe {
                self.0
                    .CheckOverlaySupport(format.into(), concerned_device.as_ptr(), &mut flags)
            };
            assert!(hr == 0);
            flags.into()
        }
    }
}