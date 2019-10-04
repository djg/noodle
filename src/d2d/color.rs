use std::ops::{Deref, DerefMut};
use winapi::um::d2d1::D2D1_COLOR_F;

pub struct Color(D2D1_COLOR_F);

impl Deref for Color {
    type Target = D2D1_COLOR_F;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<D2D1_COLOR_F> for Color {
    fn from(c: D2D1_COLOR_F) -> Self {
        Self(c)
    }
}

impl Color {
    pub fn with_alpha(rgba: [f32; 4]) -> Color {
        Self(D2D1_COLOR_F {
            r: rgba[0],
            g: rgba[1],
            b: rgba[2],
            a: rgba[3],
        })
    }

    pub fn new(rgb: [f32; 3]) -> Color {
        Self::with_alpha([rgb[0], rgb[1], rgb[2], 1.0])
    }
}

// Colors, this enum defines a set of predefined colors.
#[derive(Clone, Copy, Debug, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum NamedColor {
    AliceBlue,
    AntiqueWhite,
    Aqua,
    Aquamarine,
    Azure,
    Beige,
    Bisque,
    Black,
    BlanchedAlmond,
    Blue,
    BlueViolet,
    Brown,
    BurlyWood,
    CadetBlue,
    Chartreuse,
    Chocolate,
    Coral,
    CornflowerBlue,
    Cornsilk,
    Crimson,
    Cyan,
    DarkBlue,
    DarkCyan,
    DarkGoldenrod,
    DarkGray,
    DarkGreen,
    DarkKhaki,
    DarkMagenta,
    DarkOliveGreen,
    DarkOrange,
    DarkOrchid,
    DarkRed,
    DarkSalmon,
    DarkSeaGreen,
    DarkSlateBlue,
    DarkSlateGray,
    DarkTurquoise,
    DarkViolet,
    DeepPink,
    DeepSkyBlue,
    DimGray,
    DodgerBlue,
    Firebrick,
    FloralWhite,
    ForestGreen,
    Fuchsia,
    Gainsboro,
    GhostWhite,
    Gold,
    Goldenrod,
    Gray,
    Green,
    GreenYellow,
    Honeydew,
    HotPink,
    IndianRed,
    Indigo,
    Ivory,
    Khaki,
    Lavender,
    LavenderBlush,
    LawnGreen,
    LemonChiffon,
    LightBlue,
    LightCoral,
    LightCyan,
    LightGoldenrodYellow,
    LightGreen,
    LightGray,
    LightPink,
    LightSalmon,
    LightSeaGreen,
    LightSkyBlue,
    LightSlateGray,
    LightSteelBlue,
    LightYellow,
    Lime,
    LimeGreen,
    Linen,
    Magenta,
    Maroon,
    MediumAquamarine,
    MediumBlue,
    MediumOrchid,
    MediumPurple,
    MediumSeaGreen,
    MediumSlateBlue,
    MediumSpringGreen,
    MediumTurquoise,
    MediumVioletRed,
    MidnightBlue,
    MintCream,
    MistyRose,
    Moccasin,
    NavajoWhite,
    Navy,
    OldLace,
    Olive,
    OliveDrab,
    Orange,
    OrangeRed,
    Orchid,
    PaleGoldenrod,
    PaleGreen,
    PaleTurquoise,
    PaleVioletRed,
    PapayaWhip,
    PeachPuff,
    Peru,
    Pink,
    Plum,
    PowderBlue,
    Purple,
    Red,
    RosyBrown,
    RoyalBlue,
    SaddleBrown,
    Salmon,
    SandyBrown,
    SeaGreen,
    SeaShell,
    Sienna,
    Silver,
    SkyBlue,
    SlateBlue,
    SlateGray,
    Snow,
    SpringGreen,
    SteelBlue,
    Tan,
    Teal,
    Thistle,
    Tomato,
    Turquoise,
    Violet,
    Wheat,
    White,
    WhiteSmoke,
    Yellow,
    YellowGreen,
}

impl PartialEq for NamedColor {
    fn eq(&self, other: &Self) -> bool {
        let lhs: u32 = (*self).into();
        let rhs: u32 = (*other).into();
        lhs == rhs
    }
}

impl Into<u32> for NamedColor {
    fn into(self) -> u32 {
        use NamedColor::*;
        #[allow(clippy::mistyped_literal_suffixes)]
        match self {
            AliceBlue => 0xF0_F8_FF,
            AntiqueWhite => 0xFA_EB_D7,
            Aqua => 0x00_FF_FF,
            Aquamarine => 0x7F_FF_D4,
            Azure => 0xF0_FF_FF,
            Beige => 0xF5_F5_DC,
            Bisque => 0xFF_E4_C4,
            Black => 0x00_00_00,
            BlanchedAlmond => 0xFF_EB_CD,
            Blue => 0x00_00_FF,
            BlueViolet => 0x8A_2B_E2,
            Brown => 0xA5_2A_2A,
            BurlyWood => 0xDE_B8_87,
            CadetBlue => 0x5F_9E_A0,
            Chartreuse => 0x7F_FF_00,
            Chocolate => 0xD2_69_1E,
            Coral => 0xFF_7F_50,
            CornflowerBlue => 0x64_95_ED,
            Cornsilk => 0xFF_F8_DC,
            Crimson => 0xDC_14_3C,
            Cyan => 0x00_FF_FF,
            DarkBlue => 0x00_00_8B,
            DarkCyan => 0x00_8B_8B,
            DarkGoldenrod => 0xB8_86_0B,
            DarkGray => 0xA9_A9_A9,
            DarkGreen => 0x00_64_00,
            DarkKhaki => 0xBD_B7_6B,
            DarkMagenta => 0x8B_00_8B,
            DarkOliveGreen => 0x55_6B_2F,
            DarkOrange => 0xFF_8C_00,
            DarkOrchid => 0x99_32_CC,
            DarkRed => 0x8B_00_00,
            DarkSalmon => 0xE9_96_7A,
            DarkSeaGreen => 0x8F_BC_8F,
            DarkSlateBlue => 0x48_3D_8B,
            DarkSlateGray => 0x2F_4F_4F,
            DarkTurquoise => 0x00_CE_D1,
            DarkViolet => 0x94_00_D3,
            DeepPink => 0xFF_14_93,
            DeepSkyBlue => 0x00_BF_FF,
            DimGray => 0x69_69_69,
            DodgerBlue => 0x1E_90_FF,
            Firebrick => 0xB2_22_22,
            FloralWhite => 0xFF_FA_F0,
            ForestGreen => 0x22_8B_22,
            Fuchsia => 0xFF_00_FF,
            Gainsboro => 0xDC_DC_DC,
            GhostWhite => 0xF8_F8_FF,
            Gold => 0xFF_D7_00,
            Goldenrod => 0xDA_A5_20,
            Gray => 0x80_80_80,
            Green => 0x00_80_00,
            GreenYellow => 0xAD_FF_2F,
            Honeydew => 0xF0_FF_F0,
            HotPink => 0xFF_69_B4,
            IndianRed => 0xCD_5C_5C,
            Indigo => 0x4B_00_82,
            Ivory => 0xFF_FF_F0,
            Khaki => 0xF0_E6_8C,
            Lavender => 0xE6_E6_FA,
            LavenderBlush => 0xFF_F0_F5,
            LawnGreen => 0x7C_FC_00,
            LemonChiffon => 0xFF_FA_CD,
            LightBlue => 0xAD_D8_E6,
            LightCoral => 0xF0_80_80,
            LightCyan => 0xE0_FF_FF,
            LightGoldenrodYellow => 0xFA_FA_D2,
            LightGreen => 0x90_EE_90,
            LightGray => 0xD3_D3_D3,
            LightPink => 0xFF_B6_C1,
            LightSalmon => 0xFF_A0_7A,
            LightSeaGreen => 0x20_B2_AA,
            LightSkyBlue => 0x87_CE_FA,
            LightSlateGray => 0x77_88_99,
            LightSteelBlue => 0xB0_C4_DE,
            LightYellow => 0xFF_FF_E0,
            Lime => 0x00_FF_00,
            LimeGreen => 0x32_CD_32,
            Linen => 0xFA_F0_E6,
            Magenta => 0xFF_00_FF,
            Maroon => 0x80_00_00,
            MediumAquamarine => 0x66_CD_AA,
            MediumBlue => 0x00_00_CD,
            MediumOrchid => 0xBA_55_D3,
            MediumPurple => 0x93_70_DB,
            MediumSeaGreen => 0x3C_B3_71,
            MediumSlateBlue => 0x7B_68_EE,
            MediumSpringGreen => 0x00_FA_9A,
            MediumTurquoise => 0x48_D1_CC,
            MediumVioletRed => 0xC7_15_85,
            MidnightBlue => 0x19_19_70,
            MintCream => 0xF5_FF_FA,
            MistyRose => 0xFF_E4_E1,
            Moccasin => 0xFF_E4_B5,
            NavajoWhite => 0xFF_DE_AD,
            Navy => 0x00_00_80,
            OldLace => 0xFD_F5_E6,
            Olive => 0x80_80_00,
            OliveDrab => 0x6B_8E_23,
            Orange => 0xFF_A5_00,
            OrangeRed => 0xFF_45_00,
            Orchid => 0xDA_70_D6,
            PaleGoldenrod => 0xEE_E8_AA,
            PaleGreen => 0x98_FB_98,
            PaleTurquoise => 0xAF_EE_EE,
            PaleVioletRed => 0xDB_70_93,
            PapayaWhip => 0xFF_EF_D5,
            PeachPuff => 0xFF_DA_B9,
            Peru => 0xCD_85_3F,
            Pink => 0xFF_C0_CB,
            Plum => 0xDD_A0_DD,
            PowderBlue => 0xB0_E0_E6,
            Purple => 0x80_00_80,
            Red => 0xFF_00_00,
            RosyBrown => 0xBC_8F_8F,
            RoyalBlue => 0x41_69_E1,
            SaddleBrown => 0x8B_45_13,
            Salmon => 0xFA_80_72,
            SandyBrown => 0xF4_A4_60,
            SeaGreen => 0x2E_8B_57,
            SeaShell => 0xFF_F5_EE,
            Sienna => 0xA0_52_2D,
            Silver => 0xC0_C0_C0,
            SkyBlue => 0x87_CE_EB,
            SlateBlue => 0x6A_5A_CD,
            SlateGray => 0x70_80_90,
            Snow => 0xFF_FA_FA,
            SpringGreen => 0x00_FF_7F,
            SteelBlue => 0x46_82_B4,
            Tan => 0xD2_B4_8C,
            Teal => 0x00_80_80,
            Thistle => 0xD8_BF_D8,
            Tomato => 0xFF_63_47,
            Turquoise => 0x40_E0_D0,
            Violet => 0xEE_82_EE,
            Wheat => 0xF5_DE_B3,
            White => 0xFF_FF_FF,
            WhiteSmoke => 0xF5_F5_F5,
            Yellow => 0xFF_FF_00,
            YellowGreen => 0x9A_CD_32,
        }
    }
}

const RED_SHIFT: u32 = 16;
const GREEN_SHIFT: u32 = 8;
const BLUE_SHIFT: u32 = 0;

const RED_MASK: u32 = 0xFF << RED_SHIFT;
const GREEN_MASK: u32 = 0xFF << GREEN_SHIFT;
const BLUE_MASK: u32 = 0xFF << BLUE_SHIFT;

// private helper to convert u32 color into float representation.
fn init(rgb: u32, a: f32) -> Color {
    let r = ((rgb & RED_MASK) >> RED_SHIFT) as f32 / 255.0;
    let g = ((rgb & GREEN_MASK) >> GREEN_SHIFT) as f32 / 255.0;
    let b = ((rgb & BLUE_MASK) >> BLUE_SHIFT) as f32 / 255.0;
    Color(D2D1_COLOR_F { r, g, b, a })
}

impl From<NamedColor> for Color {
    fn from(named: NamedColor) -> Self {
        init(named.into(), 1.0)
    }
}

impl From<(NamedColor, f32)> for Color {
    fn from(col: (NamedColor, f32)) -> Self {
        let (named, a) = col;
        init(named.into(), a)
    }
}

#[cfg(test)]
mod tests {
    use super::NamedColor;
    #[test]
    fn color_match() {
        assert_eq!(NamedColor::Aqua, NamedColor::Cyan);
        assert_eq!(NamedColor::Fuchsia, NamedColor::Magenta);
    }
}
