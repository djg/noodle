use super::Color;
use crate::impl_comptr;
use winapi::um::d2d1::{ID2D1Brush, ID2D1SolidColorBrush};

impl_comptr! { SolidColorBrush: [ID2D1SolidColorBrush, ID2D1Brush] }

impl SolidColorBrush {
    pub fn set_color(&self, color: &Color) {
        unsafe {
            self.0.SetColor(&**color);
        }
    }
}
