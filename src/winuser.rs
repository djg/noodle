use crate::{text::ToUtf16, Rect};
use std::{cell::Cell, ffi::OsStr, ptr};
use winapi::{shared::minwindef::*, shared::windef::*, um::winuser::*};

extern "C" {
    pub static __ImageBase: u8;
}

pub fn get_module_handle() -> HINSTANCE {
    unsafe { &__ImageBase as *const u8 as HINSTANCE }
}

#[derive(Clone, Copy)]
enum State {
    // WndProc handler not defined
    Unarmed,
    // WndProc handler defined
    Armed(*mut dyn FnMut(&Message) -> bool),
}

thread_local! {
    static WND_PROC: Cell<State> = Cell::new(State::Unarmed)
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let window = Window::from_hwnd(hwnd).expect("Expected hwnd");
    let message = Message {
        window,
        kind: (msg, wparam, lparam).into(),
    };
    let handled = WND_PROC.with(|wnd_proc| match wnd_proc.get() {
        State::Armed(handler_ptr) => {
            let handler = &mut *handler_ptr;
            handler(&message)
        }
        State::Unarmed => false,
    });

    if handled {
        0
    } else {
        default_handle_message(message)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageKind {
    Destroy,
    Paint,
    Size {
        width: i32,
        height: i32,
    },
    Other {
        msg: UINT,
        wparam: WPARAM,
        lparam: LPARAM,
    },
}

impl From<(UINT, WPARAM, LPARAM)> for MessageKind {
    fn from(t: (UINT, WPARAM, LPARAM)) -> Self {
        let (msg, wparam, lparam) = t;
        match msg {
            WM_DESTROY => MessageKind::Destroy,
            WM_PAINT => MessageKind::Paint,
            WM_SIZE => MessageKind::Size {
                width: LOWORD(lparam as DWORD) as i32,
                height: HIWORD(lparam as DWORD) as i32,
            },
            _ => MessageKind::Other {
                msg,
                wparam,
                lparam,
            },
        }
    }
}

impl From<MessageKind> for (UINT, WPARAM, LPARAM) {
    fn from(msg: MessageKind) -> (UINT, WPARAM, LPARAM) {
        match msg {
            MessageKind::Destroy => (WM_DESTROY, 0, 0),
            MessageKind::Paint => (WM_PAINT, 0, 0),
            MessageKind::Size { width, height } => (
                WM_SIZE,
                0,
                MAKELONG(width as WORD, height as WORD) as LPARAM,
            ),
            MessageKind::Other {
                msg,
                wparam,
                lparam,
            } => (msg, wparam, lparam),
        }
    }
}

#[derive(Debug)]
pub struct Message<'a> {
    pub window: Window<'a>,
    pub kind: MessageKind,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Window<'a>(ptr::NonNull<HWND__>, std::marker::PhantomData<&'a mut ()>);

impl std::fmt::Debug for Window<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Window({:p}", &self.0.as_ptr())
    }
}

impl<'a> Window<'a> {
    pub fn create(
        class_name: &OsStr,
        window_name: &OsStr,
        style: u32,
        width: i32,
        height: i32,
    ) -> Option<Window<'a>> {
        let class_name = class_name.to_utf16();
        let window_name = window_name.to_utf16();

        let mut wc = WNDCLASSW::default();
        wc.lpfnWndProc = Some(wnd_proc);
        wc.hCursor = unsafe { LoadCursorW(ptr::null_mut(), IDC_ARROW) };
        wc.hInstance = get_module_handle();
        wc.lpszClassName = class_name.as_ptr();
        wc.style = CS_HREDRAW | CS_VREDRAW;
        unsafe {
            RegisterClassW(&wc);
        }

        let ex_style = WS_EX_NOREDIRECTIONBITMAP;
        let x = CW_USEDEFAULT;
        let y = CW_USEDEFAULT;
        let wnd_parent = ptr::null_mut();
        let menu = ptr::null_mut();

        let hwnd = unsafe {
            CreateWindowExW(
                ex_style,
                class_name.as_ptr(),
                window_name.as_ptr(),
                style,
                x,
                y,
                width,
                height,
                wnd_parent,
                menu,
                get_module_handle(),
                ptr::null_mut(),
            )
        };

        Window::from_hwnd(hwnd)
    }

    fn from_hwnd(hwnd: HWND) -> Option<Window<'a>> {
        if !hwnd.is_null() {
            // Safe because guarded by `is_null()` above.
            unsafe {
                Some(Window(
                    ptr::NonNull::new_unchecked(hwnd),
                    std::marker::PhantomData,
                ))
            }
        } else {
            None
        }
    }

    #[inline]
    crate fn as_hwnd(self) -> HWND {
        self.0.as_ptr()
    }

    pub fn client_rect(self) -> Rect {
        let mut rect = Default::default();
        let ok = unsafe { GetClientRect(self.as_hwnd(), &mut rect) };
        assert!(ok == TRUE);
        rect.into()
    }

    pub fn window_rect(self) -> Rect {
        let mut rect = Default::default();
        let ok = unsafe { GetWindowRect(self.as_hwnd(), &mut rect) };
        assert!(ok == TRUE);
        rect.into()
    }
}

pub fn default_handle_message(message: Message) -> LRESULT {
    let hwnd = message.window.as_hwnd();
    let (msg, wparam, lparam) = message.kind.into();

    unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }
}

pub fn post_quit_message(exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) }
}

pub fn process_pending_events<F>(mut f: F) -> bool
where
    F: FnMut(&Message) -> bool,
{
    struct Reset<'a>(&'a Cell<State>, State);
    impl Drop for Reset<'_> {
        fn drop(&mut self) {
            self.0.set(self.1);
        }
    }

    unsafe fn hide_lt<'a>(
        p: *mut (dyn FnMut(&Message) -> bool + 'a),
    ) -> *mut (dyn FnMut(&Message) -> bool + 'static) {
        #[allow(clippy::transmute_ptr_to_ptr)]
        std::mem::transmute(p)
    }

    unsafe {
        WND_PROC.with(|cell| {
            let was = cell.get();
            let _reset = Reset(cell, was);

            let handler_ptr = hide_lt(&mut f as &mut _ as *mut _);
            cell.set(State::Armed(handler_ptr));

            let mut msg = Default::default();
            while PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, PM_REMOVE) > FALSE {
                DispatchMessageW(&msg);

                if msg.message == WM_QUIT {
                    return true;
                }
            }

            false
        })
    }
}

/*
pub fn run_loop() {
    let mut msg = Default::default();
    unsafe {
        loop {
            let result = GetMessageW(&mut msg, ptr::null_mut(), 0, 0);
            match result {
                FALSE => return,
                TRUE => {
                    DispatchMessageW(&msg);
                }
                n => panic!("GetMessageW returned error: {}", n),
            }
        }
    }
}
*/
