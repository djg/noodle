use crate::{Offset, Size};
use derive_newtype::NewType;
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Sub};
use winapi::shared::windef::RECT;

#[derive(Clone, Copy, Default, NewType)]
#[repr(transparent)]
pub struct Rect(RECT);

#[macro_export]
macro_rules! rect {
    ($l:expr, $t:expr, $r:expr, $b:expr $(,)?) => {
        $crate::Rect::new($l, $t, $r, $b)
    };
    ($w:expr, $h:expr) => {
        rect!(0, 0, $w, $h)
    };
}

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

    pub fn is_empty(&self) -> bool {
        self.left >= self.right || self.top >= self.bottom
    }

    pub fn size(&self) -> Size {
        Size::new(self.right - self.left, self.bottom - self.top)
    }

    pub fn inset(&self, dx: i32, dy: i32) -> Rect {
        rect! {
            self.left + dx,
            self.top + dy,
            self.right - dx,
            self.bottom - dy,
        }
    }

    pub fn outset(&self, dx: i32, dy: i32) -> Rect {
        self.inset(-dx, -dy)
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

impl std::fmt::Debug for Rect {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Rect")
            .field("l", &self.left)
            .field("t", &self.top)
            .field("r", &self.right)
            .field("b", &self.bottom)
            .finish()
    }
}

impl BitAnd for Rect {
    type Output = Rect;
    fn bitand(self, rhs: Self) -> Self::Output {
        let lhs = self;
        rect! {
            std::cmp::max(lhs.left, rhs.left),
            std::cmp::max(lhs.top, rhs.top),
            std::cmp::min(lhs.right, rhs.right),
            std::cmp::min(lhs.bottom, rhs.bottom),
        }
    }
}

impl BitOr for Rect {
    type Output = Rect;
    fn bitor(self, rhs: Self) -> Self::Output {
        let lhs = self;
        rect! {
            std::cmp::min(lhs.left, rhs.left),
            std::cmp::min(lhs.top, rhs.top),
            std::cmp::max(lhs.right, rhs.right),
            std::cmp::max(lhs.bottom, rhs.bottom),
        }
    }
}

impl Add<Offset> for Rect {
    type Output = Rect;
    fn add(self, rhs: Offset) -> Self::Output {
        rect! {
            self.left + rhs.x,
            self.top + rhs.y,
            self.right + rhs.x,
            self.bottom + rhs.y,
        }
    }
}

impl Add<Size> for Rect {
    type Output = Rect;
    fn add(self, rhs: Size) -> Self::Output {
        rect! {
            self.left,
            self.top,
            self.right + rhs.cx,
            self.bottom + rhs.cy,
        }
    }
}

impl Sub for Rect {
    type Output = Vec<Rect>;
    fn sub(self, rhs: Self) -> Self::Output {
        subtract(self, rhs)
    }
}

impl Sub<Size> for Rect {
    type Output = Rect;
    fn sub(self, rhs: Size) -> Self::Output {
        rect! {
            self.left,
            self.top,
            self.right - rhs.cx,
            self.bottom - rhs.cy,
        }
    }
}

impl Mul<i32> for Rect {
    type Output = Rect;
    fn mul(self, rhs: i32) -> Self::Output {
        rect! {
            self.left * rhs,
            self.top * rhs,
            self.right * rhs,
            self.bottom * rhs,
        }
    }
}

impl Div<i32> for Rect {
    type Output = Rect;
    fn div(self, rhs: i32) -> Self::Output {
        rect! {
            self.left / rhs,
            self.top / rhs,
            self.right / rhs,
            self.bottom / rhs,
        }
    }
}

fn subtract(lhs: Rect, rhs: Rect) -> Vec<Rect> {
    // intersection of lhs & rhs
    let i = lhs & rhs;
    if i.is_empty() {
        return vec![lhs];
    }

    let code = (lhs.left != i.left) as i32 * 0x8
        | (lhs.top != i.top) as i32 * 0x4
        | (lhs.right != i.right) as i32 * 0x2
        | (lhs.bottom != i.bottom) as i32 * 0x1;

    match code {
        0b0000 => vec![],
        //
        0b1000 => vec![/* L */ rect!(lhs.left, lhs.top, i.left, lhs.bottom)],
        0b0100 => vec![/* T */ Rect::new(lhs.left, lhs.top, lhs.right, i.top)],
        0b0010 => vec![/* R */ rect!(i.right, lhs.top, lhs.right, lhs.bottom)],
        0b0001 => vec![
            /* B */ rect!(lhs.left, i.bottom, lhs.right, lhs.bottom),
        ],
        //
        0b1100 => vec![
            /* T */ rect!(lhs.left, lhs.top, lhs.right, i.top),
            /* L */ rect!(lhs.left, i.top, i.left, lhs.bottom),
        ],
        0b1010 => vec![
            /* L */ rect!(lhs.left, lhs.top, i.left, lhs.bottom),
            /* R */ rect!(i.right, lhs.top, lhs.right, lhs.bottom),
        ],
        0b1001 => vec![
            /* L */ rect!(lhs.left, lhs.top, i.left, i.bottom),
            /* B */ rect!(lhs.left, i.bottom, lhs.right, lhs.bottom),
        ],
        0b0110 => vec![
            /* T */ rect!(lhs.left, lhs.top, lhs.right, i.top),
            /* R */ rect!(i.right, i.top, lhs.right, lhs.bottom),
        ],
        0b0101 => vec![
            /* T */ rect!(lhs.left, lhs.top, lhs.right, i.top),
            /* B */ rect!(lhs.left, i.bottom, lhs.right, lhs.bottom),
        ],
        0b0011 => vec![
            /* R */ rect!(i.right, lhs.top, lhs.right, i.bottom),
            /* B */ rect!(lhs.left, i.bottom, lhs.right, lhs.bottom),
        ],
        //
        0b1110 => vec![
            /* L */ rect!(lhs.left, i.top, i.left, lhs.bottom),
            /* T */ rect!(lhs.left, lhs.top, lhs.right, i.top),
            /* R */ rect!(i.right, i.top, lhs.right, lhs.bottom),
        ],
        0b1101 => vec![
            /* L */ rect!(lhs.left, i.top, i.left, i.bottom),
            /* T */ rect!(lhs.left, lhs.top, lhs.right, i.top),
            /* B */ rect!(lhs.left, i.bottom, lhs.right, lhs.bottom),
        ],
        0b1011 => vec![
            /* L */ rect!(lhs.left, i.left, lhs.top, i.bottom),
            /* R */ rect!(i.right, lhs.right, lhs.top, i.bottom),
            /* B */ rect!(lhs.left, i.bottom, lhs.right, lhs.bottom),
        ],
        0b0111 => vec![
            /* T */ rect!(lhs.left, lhs.top, lhs.right, i.top),
            /* R */ rect!(i.right, i.top, lhs.right, i.bottom),
            /* B */ rect!(lhs.left, i.bottom, lhs.right, lhs.bottom),
        ],
        //
        0b1111 => vec![
            /* T */ rect!(lhs.left, lhs.top, lhs.right, i.top),
            /* L */ rect!(lhs.left, i.top, i.left, i.bottom),
            /* R */ rect!(i.right, i.top, lhs.right, i.bottom),
            /* B */ rect!(lhs.left, i.bottom, lhs.right, lhs.bottom),
        ],

        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn disjoint() {
        let d = rect!(0, 0, 2, 2);
        let r = rect!(2, 2, 4, 4);
        let v = r - d;
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], rect!(2, 2, 4, 4));
    }

    #[test]
    fn enlarged() {
        let d = rect!(1, 1, 2, 2);
        let r = rect!(0, 0, 3, 3);
        let v = r - d;
        assert_eq!(v.len(), 4);
        assert_eq!(v[0], rect!(0, 0, 3, 1));
        assert_eq!(v[1], rect!(0, 1, 1, 2));
        assert_eq!(v[2], rect!(2, 1, 3, 2));
        assert_eq!(v[3], rect!(0, 2, 3, 3));
    }

    #[test]
    fn exists() {
        let d = rect!(0, 0, 3, 3);
        let r = rect!(1, 1, 2, 2);
        let v = r - d;
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn corners() {
        let d = rect!(0, 0, 2, 2);
        let r = rect!(1, 1, 3, 3);
        let v = r - d;
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], rect!(2, 1, 3, 2));
        assert_eq!(v[1], rect!(1, 2, 3, 3));

        let d = rect!(1, 0, 3, 2);
        let r = rect!(0, 1, 2, 3);
        let v = r - d;
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], rect!(0, 1, 1, 2));
        assert_eq!(v[1], rect!(0, 2, 2, 3));

        let d = rect!(0, 1, 2, 3);
        let r = rect!(1, 0, 3, 2);
        let v = r - d;
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], rect!(1, 0, 3, 1));
        assert_eq!(v[1], rect!(2, 1, 3, 2));

        let d = rect!(1, 1, 3, 3);
        let r = rect!(0, 0, 2, 2);
        let v = r - d;
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], rect!(0, 0, 2, 1));
        assert_eq!(v[1], rect!(0, 1, 1, 2));
    }

    #[test]
    fn sides() {
        let d = rect!(0, 0, 2, 2);
        let r = rect!(1, 0, 3, 2);
        let v = r - d;
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], rect!(2, 0, 3, 2));

        let d = rect!(0, 0, 2, 2);
        let r = rect!(-1, 0, 1, 2);
        let v = r - d;
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], rect!(-1, 0, 0, 2));

        let d = rect!(0, 0, 2, 2);
        let r = rect!(0, 1, 2, 3);
        let v = r - d;
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], rect!(0, 2, 2, 3));

        let d = rect!(0, 0, 2, 2);
        let r = rect!(0, -1, 2, 1);
        let v = r - d;
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], rect!(0, -1, 2, 0));
    }

    #[test]
    fn middle() {
        let d = rect!(1, 0, 2, 2);
        let r = rect!(0, 0, 3, 2);
        let v = r - d;
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], rect!(0, 0, 1, 2));
        assert_eq!(v[1], rect!(2, 0, 3, 2));

        let d = rect!(0, 1, 2, 2);
        let r = rect!(0, 0, 2, 3);
        let v = r - d;
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], rect!(0, 0, 2, 1));
        assert_eq!(v[1], rect!(0, 2, 2, 3));
    }
}
