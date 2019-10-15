#![warn(clippy::all)]
#![feature(crate_visibility_modifier)]
mod macros;

mod comptr;
pub mod d2d;
pub mod d3d11;
pub mod d3dcompiler;
pub mod dcomp;
pub mod dwrite;
pub mod dxgi;
pub mod mf;
pub mod winuser;

mod rect;
mod text;

pub use comptr::ComPtr;
use derive_newtype::NewType;
pub use rect::Rect;
use std::{
    ops::{Add, AddAssign, Div, Mul, Neg, Sub},
    ptr,
};
pub use text::ToUtf16;
use winapi::shared::windef::{POINT, SIZE};

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

crate fn opt_ptr<T, U>(opt: Option<&'_ U>) -> *const T
where
    U: std::ops::Deref<Target = T>,
{
    opt.map(|v| v.deref() as *const _)
        .unwrap_or(std::ptr::null())
}

#[macro_export]
macro_rules! point {
    ($x:expr, $y:expr) => {
        $crate::Point::new($x, $y)
    };
}

#[derive(Clone, Copy, Default, NewType)]
#[repr(transparent)]
pub struct Point(POINT);

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(POINT { x, y })
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Point {}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

#[macro_export]
macro_rules! offset {
    ($dx:expr, $dy:expr) => {
        $crate::Offset::new($dx, $dy)
    };
}

#[derive(Clone, Copy, Default, NewType)]
#[repr(transparent)]
pub struct Offset(POINT);

impl Offset {
    pub fn new(x: i32, y: i32) -> Self {
        Self(POINT { x, y })
    }
}

impl PartialEq for Offset {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl Eq for Offset {}

#[macro_export]
macro_rules! size {
    ($w:expr, $h:expr) => {
        $crate::Size::new($w, $h)
    };
    ($w:expr) => {
        size!($w, $w)
    };
}

#[derive(Clone, Copy, Default, NewType)]
#[repr(transparent)]
pub struct Size(SIZE);

impl Size {
    pub const fn new(width: i32, height: i32) -> Self {
        Self(SIZE {
            cx: width,
            cy: height,
        })
    }

    pub fn width(self) -> i32 {
        self.0.cx
    }

    pub fn height(self) -> i32 {
        self.0.cy
    }
}

impl PartialEq for Size {
    fn eq(&self, other: &Self) -> bool {
        self.cx == other.cx && self.cy == other.cy
    }
}
impl Eq for Size {}

impl std::fmt::Debug for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Size")
            .field("w", &self.cx)
            .field("h", &self.cy)
            .finish()
    }
}

//===== =====
impl Neg for Offset {
    type Output = Offset;
    fn neg(self) -> Self::Output {
        offset!(-self.x, -self.y)
    }
}

impl Sub for Offset {
    type Output = Offset;
    fn sub(self, rhs: Self) -> Self::Output {
        offset!(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Sub<[i32; 2]> for Offset {
    type Output = Offset;
    fn sub(self, rhs: [i32; 2]) -> Self::Output {
        offset!(self.x - rhs[0], self.y - rhs[1])
    }
}

impl Sub<(i32, i32)> for Offset {
    type Output = Offset;
    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        offset!(self.x - rhs.0, self.y - rhs.1)
    }
}

impl Add<Offset> for Point {
    type Output = Point;
    fn add(self, rhs: Offset) -> Self::Output {
        point!(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign<Offset> for Point {
    fn add_assign(&mut self, rhs: Offset) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Size> for Point {
    type Output = Rect;
    fn add(self, rhs: Size) -> Self::Output {
        rect!(self.x, self.y, self.x + rhs.cx, self.y + rhs.cy)
    }
}

impl Sub<Offset> for Point {
    type Output = Point;
    fn sub(self, rhs: Offset) -> Self::Output {
        point!(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul<i32> for Point {
    type Output = Point;
    fn mul(self, rhs: i32) -> Self::Output {
        point!(self.x * rhs, self.y * rhs)
    }
}

impl Add<i32> for Size {
    type Output = Size;
    fn add(self, rhs: i32) -> Self::Output {
        size!(self.width() + rhs, self.height() + rhs)
    }
}

impl Sub<i32> for Size {
    type Output = Size;
    fn sub(self, rhs: i32) -> Self::Output {
        size!(self.width() - rhs, self.height() - rhs)
    }
}

impl Mul<i32> for Size {
    type Output = Size;
    fn mul(self, rhs: i32) -> Self::Output {
        size!(self.cx * rhs, self.cy * rhs)
    }
}

impl Div<i32> for Size {
    type Output = Size;
    fn div(self, rhs: i32) -> Self::Output {
        size!(self.cx / rhs, self.cy / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::rect;

    #[test]
    fn sub_rect_size() {
        assert_eq!(rect!(0, 0, 5, 10) - size!(1, 2), rect!(0, 0, 4, 8));
    }

    #[test]
    fn add_point_size() {
        assert_eq!(point!(5, 10) + size!(1, 2), rect!(5, 10, 6, 12))
    }
}
