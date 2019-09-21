use std::{ffi::OsStr, io::Error, os::windows::ffi::OsStrExt, ptr};
use winapi::{shared::minwindef::*, shared::windef::*, um::winuser::*};

extern "C" {
    pub static __ImageBase: u8;
}

pub fn get_module_handle() -> HINSTANCE {
    unsafe { &__ImageBase as *const u8 as HINSTANCE }
}

trait ToUtf16 {
    fn to_utf16(&self) -> Vec<u16>;
}

impl ToUtf16 for OsStr {
    fn to_utf16(&self) -> Vec<u16> {
        self.encode_wide().chain(Some(0)).collect()
    }
}

pub fn to_utf16(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0).into_iter()).collect()
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Rect(RECT);

impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self(RECT {
            left,
            top,
            right,
            bottom,
        })
    }

    pub fn width(&self) -> i32 {
        self.right - self.left
    }

    pub fn height(&self) -> i32 {
        self.bottom - self.top
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl std::ops::Deref for Rect {
    type Target = RECT;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left
            && self.top == other.top
            && self.right == other.right
            && self.bottom == other.bottom
    }
}
impl Eq for Rect {}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let window = Window::from_hwnd(hwnd).expect("Expected hwnd");

    let inner = match msg {
        WM_NCCREATE => {
            let create = &*(lparam as *const CREATESTRUCTW);
            let inner = create.lpCreateParams as *mut Inner;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, inner as _);
            inner
        }
        WM_NCDESTROY => {
            let inner = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Inner;
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
            if !inner.is_null() {
                let _ = Box::from_raw(inner);
            }
            return 0;
        }
        _ => GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Inner,
    };

    let message = Message {
        window,
        kind: (msg, wparam, lparam).into(),
    };

    if let Some(inner) = inner.as_mut() {
        inner.delegate.handle_message(message)
    } else {
        default_handle_message(message)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageKind {
    Destroy,
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
            MessageKind::Other {
                msg,
                wparam,
                lparam,
            } => (msg, wparam, lparam),
        }
    }
}
pub struct Message<'a> {
    pub window: Window<'a>,
    pub kind: MessageKind,
}

pub trait WindowDelegate {
    fn class_name(&self) -> &OsStr;
    fn handle_message(&mut self, message: Message) -> isize;
}

struct Inner<'a> {
    delegate: &'a mut dyn WindowDelegate,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Window<'a>(ptr::NonNull<HWND__>, std::marker::PhantomData<&'a mut ()>);

impl<'a> Window<'a> {
    pub fn create(
        delegate: &'a mut dyn WindowDelegate,
        window_name: &OsStr,
        style: u32,
        width: i32,
        height: i32,
    ) -> Option<Window<'a>> {
        let class_name = delegate.class_name().to_utf16();
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

        let inner = Box::leak(Box::new(Inner { delegate })) as *mut Inner;

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
                inner as *mut _,
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

    pub fn client_rect(self) -> std::io::Result<Rect> {
        let mut rect = Default::default();
        if unsafe { GetClientRect(self.as_hwnd(), &mut rect) } == 0 {
            return Err(Error::last_os_error());
        }
        Ok(Rect(rect))
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

pub fn run_loop() {
    let mut msg = Default::default();
    loop {
        let result = unsafe { GetMessageW(&mut msg, ptr::null_mut(), 0, 0) };
        match result {
            FALSE => return,
            TRUE => unsafe {
                DispatchMessageW(&msg);
            },
            _ => {}
        }
    }
}
