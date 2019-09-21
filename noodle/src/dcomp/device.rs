use super::{Surface, Target, VirtualSurface, Visual};
use crate::{dxgi, impl_comptr, winuser};
use winapi::um::{dcomp::IDCompositionDevice, unknwnbase::IUnknown};

impl_comptr! { Device: [IDCompositionDevice, IUnknown] }

impl Device {
    pub fn commit(&self) {
        unsafe {
            let hr = self.Commit();
            assert!(hr == 0);
        }
    }

    pub fn create_target_for_hwnd(&self, window: winuser::Window, top_most: bool) -> Target {
        unsafe {
            let mut target = Target::default();
            let hr = self.CreateTargetForHwnd(
                window.as_hwnd(),
                top_most as i32,
                target.getter_addrefs(),
            );
            assert!(hr == 0);
            target
        }
    }

    pub fn create_visual(&self) -> Visual {
        unsafe {
            let mut visual = Visual::default();
            let hr = self.CreateVisual(visual.getter_addrefs());
            assert!(hr == 0);
            visual
        }
    }

    pub fn create_surface(
        &self,
        width: u32,
        height: u32,
        pixel_format: dxgi::Format,
        alpha_mode: dxgi::AlphaMode,
    ) -> Surface {
        unsafe {
            let mut surface = Surface::default();
            let hr = self.CreateSurface(
                width,
                height,
                pixel_format.into(),
                alpha_mode.into(),
                surface.getter_addrefs(),
            );
            assert!(hr == 0);
            surface
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
            let mut surface = VirtualSurface::default();
            let hr = self.CreateVirtualSurface(
                initial_width,
                initial_height,
                pixel_format.into(),
                alpha_mode.into(),
                surface.getter_addrefs(),
            );
            assert!(hr == 0);
            surface
        }
    }
}
