/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use super::{ParagraphAlignment, TextAlignment};
use crate::{
    d2d::{DeviceContext, DeviceContext1, RectF},
    impl_comptr, impl_interface, AsPtr,
};
use winapi::um::{
    d2d1::{ID2D1Brush, D2D1_DRAW_TEXT_OPTIONS_NONE},
    dcommon::DWRITE_MEASURING_MODE_NATURAL,
    dwrite::IDWriteTextFormat,
    unknwnbase::IUnknown,
};

impl_comptr! { TextFormat: [IDWriteTextFormat, IUnknown] }

impl_interface! {
    impl TextFormat {
        /// Set alignment option of text relative to layout box's leading and trailing edge.
        pub fn set_text_alignment(&self, alignment: TextAlignment) {
            let hr = unsafe {
                self.0.SetTextAlignment(alignment.into())
            };
            assert!(hr == 0);
        }

        /// Set alignment option of paragraph relative to layout box's top and bottom edge.
        pub fn set_paragraph_alignment(&self, alignment: ParagraphAlignment) {
            let hr = unsafe {
                self.0.SetParagraphAlignment(alignment.into())
            };
            assert!(hr == 0);
        }
    }
}

// ID2D1RenderTarget
impl_interface! {
    impl [DeviceContext, DeviceContext1] {
        /// Draws the text within the given layout rectangle and by default also performs
        /// baseline snapping.
        pub fn draw_text(
            &self,
            text: &[u16],
            text_format: &TextFormat,
            layout_rect: &RectF,
            default_fill_brush: &impl AsPtr<ID2D1Brush>,
        ) {
            unsafe {
                self.0.DrawText(
                    text.as_ptr(),
                    text.len() as u32,
                    text_format.as_ptr(),
                    &**layout_rect,
                    default_fill_brush.as_ptr(),
                    D2D1_DRAW_TEXT_OPTIONS_NONE,
                    DWRITE_MEASURING_MODE_NATURAL,
                )
            }
        }
    }
}
