use super::Target;
use crate::{impl_comptr, impl_interface, winuser, ComPtr};
use winapi::um::{
    dcomp::{
        IDCompositionDesktopDevice, IDCompositionDevice, IDCompositionDevice2,
        IDCompositionDeviceDebug, IDCompositionTarget,
    },
    unknwnbase::IUnknown,
};

impl_comptr! { Device: [IDCompositionDevice, IUnknown] }
impl_comptr! { Device2: [IDCompositionDevice2, IUnknown] }
impl_comptr! { DesktopDevice: [IDCompositionDesktopDevice, IDCompositionDevice2, IUnknown] }
impl_comptr! { DeviceDebug: [IDCompositionDeviceDebug, IUnknown] }

impl_interface! {
    impl [Device, DesktopDevice] {
        pub fn create_target_for_hwnd(&self, window: winuser::Window, top_most: bool) -> Target {
            unsafe {
                let mut target = ComPtr::<IDCompositionTarget>::default();
                let hr = self.0.CreateTargetForHwnd(
                    window.as_hwnd(),
                    top_most as i32,
                    target.getter_addrefs(),
                );
                assert!(hr == 0);
                target.into()
            }
        }
    }
}

impl_interface! {
    impl [Device, Device2, DesktopDevice] {
        pub fn commit(&self) {
            unsafe {
                let hr = self.0.Commit();
                assert!(hr == 0);
            }
        }

        pub fn wait_for_commit_completion(&self) {
            unsafe {
                let hr = self.0.WaitForCommitCompletion();
                assert!(hr == 0);
            }
        }
    }
}

impl_interface! {
    impl DeviceDebug {
        pub fn disable_debug_counters(&self) {
            unsafe {
                let hr = self.0.DisableDebugCounters();
                assert!(hr == 0);
            }
        }

        pub fn enable_debug_counters(&self) {
            unsafe {
                let hr = self.0.EnableDebugCounters();
                assert!(hr == 0);
            }
        }
    }
}
