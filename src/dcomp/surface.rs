use crate::{
    dcomp::{DesktopDevice, Device, Device2},
    dxgi, impl_comptr, impl_interface, opt_ptr, ComPtr, Offset, Rect,
};
use std::convert::TryInto;
use winapi::{
    shared::dxgi::IDXGISurface,
    um::{
        dcomp::{IDCompositionSurface, IDCompositionVirtualSurface},
        unknwnbase::IUnknown,
    },
    Interface,
};

impl_comptr! { Surface: [IDCompositionSurface, IUnknown] }
impl_comptr! { VirtualSurface: [IDCompositionVirtualSurface, IDCompositionSurface, IUnknown] }

impl_interface! {
    impl [Device, Device2, DesktopDevice] {
        pub fn create_surface(
            &self,
            width: u32,
            height: u32,
            pixel_format: dxgi::Format,
            alpha_mode: dxgi::AlphaMode,
        ) -> Surface {
            unsafe {
                let mut surface = ComPtr::<IDCompositionSurface>::default();
                let hr = self.0.CreateSurface(
                    width,
                    height,
                    pixel_format.into(),
                    alpha_mode.into(),
                    surface.getter_addrefs(),
                );
                assert!(hr == 0);
                surface.into()
            }
        }

        pub fn create_virtual_surface(
            &self,
            initial_width: u32,
            initial_height: u32,
            pixel_format: dxgi::Format,
            alpha_mode: dxgi::AlphaMode,
        ) -> VirtualSurface {
            unsafe {
                let mut surface = ComPtr::<IDCompositionVirtualSurface>::default();
                let hr = self.0.CreateVirtualSurface(
                    initial_width,
                    initial_height,
                    pixel_format.into(),
                    alpha_mode.into(),
                    surface.getter_addrefs(),
                );
                assert!(hr == 0);
                surface.into()
            }
        }
    }
}

impl_interface! {
    impl [Surface, VirtualSurface] {
        pub fn draw<'a>(
            &self,
            update_rect: impl Into<Option<&'a Rect>>,
            mut f: impl FnMut(&dxgi::Surface, Offset))
        {
            let mut update_object = ComPtr::<IDXGISurface>::default();
            let mut update_offset = Offset::default();
            let hr = unsafe {
                self.0.BeginDraw(
                    opt_ptr(update_rect.into()),
                    &IDXGISurface::uuidof(),
                    update_object.getter_addrefs(),
                    &mut *update_offset)
            };
            assert!(hr == 0);
            let surface = update_object.into();
            f(&surface, update_offset);
            let hr = unsafe {
                self.0.EndDraw()
            };
            assert!(hr == 0);
        }

        pub fn scroll<'a>(
            &self,
            scroll_rect: impl Into<Option<&'a Rect>>,
            clip_rect: impl Into<Option<&'a Rect>>,
            offset: (i32, i32)
        ) {
            unsafe {
                let hr = self.0.Scroll(
                    opt_ptr(scroll_rect.into()),
                    opt_ptr(clip_rect.into()),
                    offset.0,
                    offset.1
                );
                assert!(hr == 0);
            }
        }
    }
}

impl VirtualSurface {
    pub fn resize(&self, width: u32, height: u32) {
        unsafe {
            let hr = self.0.Resize(width, height);
            assert!(hr == 0);
        }
    }

    pub fn trim(&self, rects: &[Rect]) {
        unsafe {
            // Rect is a thin wrapper around RECT, so &[Rect] is &[RECT]
            let hr = self
                .0
                .Trim(rects.as_ptr() as *const _, rects.len().try_into().unwrap());
            assert!(hr == 0);
        }
    }
}
