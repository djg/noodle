#![warn(clippy::all)]
#![feature(crate_visibility_modifier)]
mod macros;

mod comptr;
pub mod d2d;
pub mod d3d11;
pub mod dcomp;
pub mod dxgi;
pub mod winuser;

pub use comptr::ComPtr;
pub use winapi::shared::windef::{POINT as Point, RECT as Rect};

use std::ptr::{self};

pub trait AsPtr<T> {
    fn as_ptr(&self) -> *mut T;
}

impl<T, U> AsPtr<T> for Option<U>
where
    U: AsPtr<T>,
{
    fn as_ptr(&self) -> *mut T {
        self.as_ref().map(AsPtr::as_ptr).unwrap_or(ptr::null_mut())
    }
}

impl<'a, T, U> AsPtr<T> for &'a U
where
    U: AsPtr<T>,
{
    fn as_ptr(&self) -> *mut T {
        (*self).as_ptr()
    }
}

crate fn opt_ptr<T>(opt: Option<&'_ T>) -> *const T {
    opt.map(|v| v as *const _).unwrap_or(std::ptr::null())
}
