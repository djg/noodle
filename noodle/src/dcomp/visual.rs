use crate::{
    dcomp::{DesktopDevice, Device, Device2},
    impl_comptr, impl_interface, AsPtr, ComPtr,
};
use winapi::{
    shared::minwindef::{BOOL, FALSE, TRUE},
    um::{
        dcomp::{
            IDCompositionVisual, IDCompositionVisual2, IDCompositionVisual3,
            IDCompositionVisualDebug,
        },
        unknwnbase::IUnknown,
    },
};

impl_comptr! { Visual: [IDCompositionVisual, IUnknown] }
impl_comptr! { Visual2: [IDCompositionVisual2, IDCompositionVisual, IUnknown] }
impl_comptr! { VisualDebug: [IDCompositionVisualDebug, IDCompositionVisual2, IDCompositionVisual, IUnknown] }
impl_comptr! { Visual3: [IDCompositionVisual3, IDCompositionVisualDebug, IDCompositionVisual2, IDCompositionVisual, IUnknown] }

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Insert {
    Below,
    Above,
}

impl From<Insert> for BOOL {
    fn from(insert: Insert) -> Self {
        match insert {
            Insert::Below => FALSE,
            Insert::Above => TRUE,
        }
    }
}

impl_interface! {
    impl Device {
        pub fn create_visual(&self) -> Visual {
            unsafe {
                let mut visual = ComPtr::<IDCompositionVisual>::default();
                let hr = self.0.CreateVisual(visual.getter_addrefs());
                assert!(hr == 0);
                visual.into()
            }
        }
    }
}

impl_interface! {
    impl [Device2, DesktopDevice] {
        pub fn create_visual(&self) -> Visual2 {
            unsafe {
                let mut visual = ComPtr::<IDCompositionVisual2>::default();
                let hr = self.0.CreateVisual(visual.getter_addrefs());
                assert!(hr == 0);
                visual.into()
            }
        }
    }
}

impl_interface! {
    impl [Visual, Visual2, Visual3] {
        fn _set_content(&self, content: *mut IUnknown) {
            unsafe {
                let hr = self.0.SetContent(content);
                assert!(hr == 0);
            }
        }

        pub fn set_content(&self, content: &impl AsPtr<IUnknown>) {
            self._set_content(content.as_ptr());
        }

        pub fn clear_content(&self) {
            self._set_content(std::ptr::null_mut());
        }

        pub fn add_visual<'a, V: 'a>(
            &self,
            visual: impl AsPtr<IDCompositionVisual>,
            insert: Insert,
            reference_visual: impl Into<Option<&'a V>>)
        where
            V: AsPtr<IDCompositionVisual>
        {
            unsafe {
                let hr = self.0.AddVisual(
                    visual.as_ptr(),
                    insert.into(),
                    reference_visual.into().as_ptr()
                );
                assert!(hr == 0);
            }
        }

        pub fn remove_visual(&self, visual: impl AsPtr<IDCompositionVisual>) {
            unsafe {
                let hr = self.0.RemoveVisual(visual.as_ptr());
                assert!(hr == 0);
            }
        }

        pub fn remove_all_visuals(&self) {
            unsafe {
                let hr = self.0.RemoveAllVisuals();
                assert!(hr == 0);
            }
        }
    }
}

impl_interface! {
    impl [Visual3, VisualDebug] {
        pub fn disable_redraw_regions(&self) {
            unsafe {
                let hr = self.0.DisableRedrawRegions();
                assert!(hr == 0);
            }
        }

        pub fn enable_redraw_regions(&self) {
            unsafe {
                let hr = self.0.EnableRedrawRegions();
                assert!(hr == 0);
            }
        }
    }
}
