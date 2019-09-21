use crate::impl_comptr;
use winapi::um::{
    d2d1::{ID2D1Bitmap, ID2D1Image},
    d2d1_1::ID2D1Bitmap1,
};

impl_comptr! {  Bitmap: [ID2D1Bitmap, ID2D1Image]}
impl_comptr! { Bitmap1: [ID2D1Bitmap1, ID2D1Bitmap, ID2D1Image]}
