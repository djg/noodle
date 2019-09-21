use crate::impl_comptr;
use winapi::um::{
    dcomp::{IDCompositionSurface, IDCompositionVirtualSurface},
    unknwnbase::IUnknown,
};

impl_comptr! { Surface: [IDCompositionSurface, IUnknown] }
impl_comptr! { VirtualSurface: [IDCompositionVirtualSurface, IUnknown] }
