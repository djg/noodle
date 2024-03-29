/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::ptr;

use super::{Bitmap1, BitmapProperties1, BrushProperties, Color, Ellipse, RectF, SolidColorBrush};
use crate::{impl_comptr, impl_interface, opt_ptr, AsPtr, ComPtr};
use winapi::{
    shared::dxgi::IDXGISurface,
    um::{
        d2d1::{ID2D1Brush, ID2D1Image, ID2D1RenderTarget, ID2D1Resource, ID2D1SolidColorBrush},
        d2d1_1::{ID2D1Bitmap1, ID2D1DeviceContext},
        d2d1_2::ID2D1DeviceContext1,
        unknwnbase::IUnknown,
    },
};

impl_comptr! {  DeviceContext: [ID2D1DeviceContext, ID2D1RenderTarget, ID2D1Resource, IUnknown] }
impl_comptr! { DeviceContext1: [ID2D1DeviceContext1, ID2D1DeviceContext, ID2D1RenderTarget, ID2D1Resource, IUnknown] }

// ID2D1RenderTarget
impl_interface! {
    impl [DeviceContext, DeviceContext1] {
        pub fn create_solid_color_brush<'a>(
            &self,
            color: &Color,
            brush_properties: impl Into<Option<&'a BrushProperties>>,
        ) -> SolidColorBrush {
            unsafe {
                let mut native = ComPtr::<ID2D1SolidColorBrush>::default();
                let hr = self.0.CreateSolidColorBrush(
                    &**color,
                    opt_ptr(brush_properties.into()),
                    native.getter_addrefs(),
                );
                assert!(hr == 0);
                SolidColorBrush(native)
            }
        }

        pub fn clear<'a>(&self, color: impl Into<Option<&'a Color>>) {
            unsafe {
                self.0.Clear(opt_ptr(color.into()));
            }
        }

        pub fn fill_rectangle(&self, rect: &RectF, brush: &impl AsPtr<ID2D1Brush>) {
            unsafe {
                self.0.FillRectangle(&**rect, brush.as_ptr());
            }
        }

        pub fn fill_ellipse(&self, ellipse: &Ellipse, brush: &impl AsPtr<ID2D1Brush>) {
            unsafe {
                self.0.FillEllipse(ellipse, brush.as_ptr());
            }
        }

        pub fn begin_draw(&self) {
            unsafe { self.0.BeginDraw() }
        }

        pub fn end_draw(&self) {
            unsafe {
                let hr = self.0.EndDraw(ptr::null_mut(), ptr::null_mut());
                assert!(hr == 0);
            }
        }
    }
}

impl_interface! {
    impl DeviceContext1 {
        pub fn create_bitmap_from_dxgi_surface(
            &self,
            surface: &impl AsPtr<IDXGISurface>,
            bitmap_properties: &BitmapProperties1,
        ) -> Bitmap1 {
            unsafe {
                //let mut native = Bitmap1::default();
                let mut native = ComPtr::<ID2D1Bitmap1>::default();
                let hr = self.0.CreateBitmapFromDxgiSurface(
                    surface.as_ptr(),
                    bitmap_properties,
                    native.getter_addrefs(),
                );
                assert!(hr == 0);
                Bitmap1(native)
            }
        }

        pub fn draw(&self, mut f: impl FnMut(&DeviceContext1)) {
            self.begin_draw();
            f(self);
            self.end_draw();
        }

        pub fn set_target(&self, image: &impl AsPtr<ID2D1Image>) {
            unsafe {
                self.0.SetTarget(image.as_ptr());
            }
        }
    }
}
