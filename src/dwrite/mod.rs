mod text_format;

use crate::{impl_comptr, text::ToUtf16, ComPtr};
use std::ptr;
use winapi::{
    um::{
        dwrite::{
            DWriteCreateFactory, IDWriteFactory, IDWriteTextFormat, DWRITE_FACTORY_TYPE_SHARED,
            DWRITE_FONT_STRETCH, DWRITE_FONT_STRETCH_CONDENSED, DWRITE_FONT_STRETCH_EXPANDED,
            DWRITE_FONT_STRETCH_EXTRA_CONDENSED, DWRITE_FONT_STRETCH_EXTRA_EXPANDED,
            DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STRETCH_SEMI_CONDENSED,
            DWRITE_FONT_STRETCH_SEMI_EXPANDED, DWRITE_FONT_STRETCH_ULTRA_CONDENSED,
            DWRITE_FONT_STRETCH_ULTRA_EXPANDED, DWRITE_FONT_STRETCH_UNDEFINED, DWRITE_FONT_STYLE,
            DWRITE_FONT_STYLE_ITALIC, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_STYLE_OBLIQUE,
            DWRITE_FONT_WEIGHT, DWRITE_FONT_WEIGHT_BLACK, DWRITE_FONT_WEIGHT_BOLD,
            DWRITE_FONT_WEIGHT_EXTRA_BLACK, DWRITE_FONT_WEIGHT_EXTRA_BOLD,
            DWRITE_FONT_WEIGHT_EXTRA_LIGHT, DWRITE_FONT_WEIGHT_LIGHT, DWRITE_FONT_WEIGHT_MEDIUM,
            DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_WEIGHT_SEMI_BOLD, DWRITE_FONT_WEIGHT_SEMI_LIGHT,
            DWRITE_FONT_WEIGHT_THIN, DWRITE_PARAGRAPH_ALIGNMENT, DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
            DWRITE_PARAGRAPH_ALIGNMENT_FAR, DWRITE_PARAGRAPH_ALIGNMENT_NEAR, DWRITE_TEXT_ALIGNMENT,
            DWRITE_TEXT_ALIGNMENT_CENTER, DWRITE_TEXT_ALIGNMENT_JUSTIFIED,
            DWRITE_TEXT_ALIGNMENT_LEADING, DWRITE_TEXT_ALIGNMENT_TRAILING,
        },
        unknwnbase::IUnknown,
    },
    Interface,
};

pub use text_format::TextFormat;

/// The font weight enumeration describes common values for degree of blackness or thickness of strokes of characters in a font.
/// Font weight values less than 1 or greater than 999 are considered to be invalid, and they are rejected by font API functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FontWeight {
    Thin,
    ExtraLight,
    Light,
    SemiLight,
    Normal,
    Medium,
    SemiBold,
    Bold,
    ExtraBold,
    Black,
    ExtraBlack,
}

impl Into<DWRITE_FONT_WEIGHT> for FontWeight {
    fn into(self) -> DWRITE_FONT_WEIGHT {
        use FontWeight::*;
        match self {
            Thin => DWRITE_FONT_WEIGHT_THIN,
            ExtraLight => DWRITE_FONT_WEIGHT_EXTRA_LIGHT,
            Light => DWRITE_FONT_WEIGHT_LIGHT,
            SemiLight => DWRITE_FONT_WEIGHT_SEMI_LIGHT,
            Normal => DWRITE_FONT_WEIGHT_NORMAL,
            Medium => DWRITE_FONT_WEIGHT_MEDIUM,
            SemiBold => DWRITE_FONT_WEIGHT_SEMI_BOLD,
            Bold => DWRITE_FONT_WEIGHT_BOLD,
            ExtraBold => DWRITE_FONT_WEIGHT_EXTRA_BOLD,
            Black => DWRITE_FONT_WEIGHT_BLACK,
            ExtraBlack => DWRITE_FONT_WEIGHT_EXTRA_BLACK,
        }
    }
}

/// The font stretch enumeration describes relative change from the normal aspect ratio
/// as specified by a font designer for the glyphs in a font.
/// Values less than 1 or greater than 9 are considered to be invalid, and they are rejected by font API functions.
pub enum FontStretch {
    Undefined,
    UltraCondensed,
    ExtraCondensed,
    Condensed,
    SemiCondensed,
    Normal,
    SemiExpanded,
    Expanded,
    ExtraExpanded,
    UltraExpanded,
}

impl Into<DWRITE_FONT_STRETCH> for FontStretch {
    fn into(self) -> DWRITE_FONT_STRETCH {
        use FontStretch::*;
        match self {
            Undefined => DWRITE_FONT_STRETCH_UNDEFINED,
            UltraCondensed => DWRITE_FONT_STRETCH_ULTRA_CONDENSED,
            ExtraCondensed => DWRITE_FONT_STRETCH_EXTRA_CONDENSED,
            Condensed => DWRITE_FONT_STRETCH_CONDENSED,
            SemiCondensed => DWRITE_FONT_STRETCH_SEMI_CONDENSED,
            Normal => DWRITE_FONT_STRETCH_NORMAL,
            SemiExpanded => DWRITE_FONT_STRETCH_SEMI_EXPANDED,
            Expanded => DWRITE_FONT_STRETCH_EXPANDED,
            ExtraExpanded => DWRITE_FONT_STRETCH_EXTRA_EXPANDED,
            UltraExpanded => DWRITE_FONT_STRETCH_ULTRA_EXPANDED,
        }
    }
}

/// The font style enumeration describes the slope style of a font face, such as Normal, Italic or Oblique.
/// Values other than the ones defined in the enumeration are considered to be invalid, and they are rejected by font API functions.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FontStyle {
    Normal,
    Oblique,
    Italic,
}

impl Into<DWRITE_FONT_STYLE> for FontStyle {
    fn into(self) -> DWRITE_FONT_STYLE {
        use FontStyle::*;
        match self {
            Normal => DWRITE_FONT_STYLE_NORMAL,
            Oblique => DWRITE_FONT_STYLE_OBLIQUE,
            Italic => DWRITE_FONT_STYLE_ITALIC,
        }
    }
}

/// Alignment of paragraph text along the reading direction axis relative to
/// the leading and trailing edge of the layout box.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TextAlignment {
    /// The leading edge of the paragraph text is aligned to the layout box's leading edge.
    Leading,
    /// The trailing edge of the paragraph text is aligned to the layout box's trailing edge.
    Trailing,
    /// The center of the paragraph text is aligned to the center of the layout box.
    Center,
    /// Align text to the leading side, and also justify text to fill the lines.
    Justified,
}

impl Into<DWRITE_TEXT_ALIGNMENT> for TextAlignment {
    fn into(self) -> DWRITE_TEXT_ALIGNMENT {
        use TextAlignment::*;
        match self {
            Leading => DWRITE_TEXT_ALIGNMENT_LEADING,
            Trailing => DWRITE_TEXT_ALIGNMENT_TRAILING,
            Center => DWRITE_TEXT_ALIGNMENT_CENTER,
            Justified => DWRITE_TEXT_ALIGNMENT_JUSTIFIED,
        }
    }
}

/// Alignment of paragraph text along the flow direction axis relative to the
/// flow's beginning and ending edge of the layout box.
pub enum ParagraphAlignment {
    /// The first line of paragraph is aligned to the flow's beginning edge of the layout box.
    Near,
    /// The last line of paragraph is aligned to the flow's ending edge of the layout box.
    Far,
    /// The center of the paragraph is aligned to the center of the flow of the layout box.
    Center,
}

impl Into<DWRITE_PARAGRAPH_ALIGNMENT> for ParagraphAlignment {
    fn into(self) -> DWRITE_PARAGRAPH_ALIGNMENT {
        use ParagraphAlignment::*;
        match self {
            Near => DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
            Far => DWRITE_PARAGRAPH_ALIGNMENT_FAR,
            Center => DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
        }
    }
}

impl_comptr! { Factory: [IDWriteFactory, IUnknown] }

pub fn create_shared_factory<T, I>() -> T
where
    T: From<ComPtr<I>>,
    I: Interface,
{
    unsafe {
        let mut factory = ComPtr::<I>::default();
        let hr = DWriteCreateFactory(
            DWRITE_FACTORY_TYPE_SHARED,
            &I::uuidof(),
            factory.getter_addrefs(),
        );
        assert!(hr == 0);
        factory.into()
    }
}

impl Factory {
    pub fn create_text_format(
        &self,
        family_name: &str,
        /*collection,*/ weight: FontWeight,
        style: FontStyle,
        stretch: FontStretch,
        size: f32,
        locale_name: &str,
    ) -> TextFormat {
        let family_name = family_name.to_utf16();
        let locale_name = locale_name.to_utf16();
        let mut text_format = ComPtr::<IDWriteTextFormat>::default();
        let hr = unsafe {
            self.0.CreateTextFormat(
                family_name.as_ptr(),
                ptr::null_mut(),
                weight.into(),
                style.into(),
                stretch.into(),
                size,
                locale_name.as_ptr(),
                text_format.getter_addrefs(),
            )
        };
        assert!(hr == 0);
        text_format.into()
    }
}
