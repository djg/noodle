mod bitmap;
mod color;
mod device;
mod device_context;
mod factory;
mod solid_color_brush;

use crate::Rect;
use derive_newtype::NewType;
use std::ops::Deref;
use winapi::um::d2d1::*;

pub use winapi::um::{
    d2d1::{
        D2D1_BRUSH_PROPERTIES, D2D1_COLOR_F, D2D1_ELLIPSE as Ellipse, D2D1_FACTORY_OPTIONS,
        D2D1_POINT_2F as Point2F,
    },
    d2d1_1::{
        D2D1_BITMAP_OPTIONS as BitmapOptions,
        D2D1_BITMAP_OPTIONS_CANNOT_DRAW as BITMAP_OPTIONS_CANNOT_DRAW,
        D2D1_BITMAP_OPTIONS_CPU_READ as BITMAP_OPTIONS_CPU_READ,
        D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE as BITMAP_OPTIONS_GDI_COMPATIBLE,
        D2D1_BITMAP_OPTIONS_TARGET as BITMAP_OPTIONS_TARGET,
        D2D1_BITMAP_PROPERTIES1 as BitmapProperties1,
    },
    dcommon::{D2D1_PIXEL_FORMAT as PixelFormat, D2D_RECT_F},
};

pub use bitmap::{Bitmap, Bitmap1};
pub use color::{Color, NamedColor};
pub use device::Device1;
pub use device_context::{DeviceContext, DeviceContext1};
pub use factory::*;
pub use solid_color_brush::*;

pub struct BrushProperties(D2D1_BRUSH_PROPERTIES);

impl Deref for BrushProperties {
    type Target = D2D1_BRUSH_PROPERTIES;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DebugLevel {
    None,
    Error,
    Warning,
    Information,
}

impl Into<D2D1_DEBUG_LEVEL> for DebugLevel {
    fn into(self) -> D2D1_DEBUG_LEVEL {
        match self {
            DebugLevel::None => D2D1_DEBUG_LEVEL_NONE,
            DebugLevel::Error => D2D1_DEBUG_LEVEL_ERROR,
            DebugLevel::Warning => D2D1_DEBUG_LEVEL_WARNING,
            DebugLevel::Information => D2D1_DEBUG_LEVEL_INFORMATION,
        }
    }
}

pub struct FactoryOptions(D2D1_FACTORY_OPTIONS);

impl Deref for FactoryOptions {
    type Target = D2D1_FACTORY_OPTIONS;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<FactoryOptions> for DebugLevel {
    fn into(self) -> FactoryOptions {
        FactoryOptions(D2D1_FACTORY_OPTIONS {
            debugLevel: self.into(),
        })
    }
}

#[derive(Clone, Copy, Default, NewType)]
pub struct RectF(D2D1_RECT_F);

impl RectF {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self(D2D1_RECT_F {
            left,
            top,
            right,
            bottom,
        })
    }
}

impl From<Rect> for RectF {
    fn from(r: Rect) -> Self {
        Self::new(r.left as f32, r.top as f32, r.right as f32, r.bottom as f32)
    }
}

impl PartialEq for RectF {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left
            && self.top == other.top
            && self.right == other.right
            && self.bottom == other.bottom
    }
}

impl Eq for RectF {}
