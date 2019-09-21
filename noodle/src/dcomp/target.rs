use crate::{impl_comptr, AsPtr};
use winapi::um::{
    dcomp::{IDCompositionTarget, IDCompositionVisual},
    unknwnbase::IUnknown,
};

impl_comptr! { Target: [IDCompositionTarget, IUnknown] }

impl Target {
    pub fn set_root(&self, visual: &impl AsPtr<IDCompositionVisual>) {
        unsafe {
            let hr = self.SetRoot(visual.as_ptr());
            assert!(hr == 0);
        }
    }
}
