use crate::{impl_comptr, AsPtr};
use winapi::um::{
    dcomp::{IDCompositionVisual, IDCompositionVisual2, IDCompositionVisual3},
    unknwnbase::IUnknown,
};

impl_comptr! { Visual: [IDCompositionVisual, IUnknown] }
impl_comptr! { Visual2: [IDCompositionVisual2, IDCompositionVisual, IUnknown] }
impl_comptr! { Visual3: [IDCompositionVisual3, IDCompositionVisual2, IDCompositionVisual, IUnknown] }

impl Visual {
    pub fn set_content(&self, content: &impl AsPtr<IUnknown>) {
        unsafe {
            let hr = self.SetContent(content.as_ptr());
            assert!(hr == 0);
        }
    }
}
