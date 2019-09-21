mod bitmap;
mod device;
mod device_context;
mod factory;
mod solid_color_brush;

use winapi::um::d2d1::*;

pub use winapi::um::d2d1::{
    D2D1_BRUSH_PROPERTIES as BrushProperties, D2D1_COLOR_F as ColorF, D2D1_ELLIPSE as Ellipse,
    D2D1_FACTORY_OPTIONS as FactoryOptions, D2D1_POINT_2F as Point2F,
};
pub use winapi::um::d2d1_1::{
    D2D1_BITMAP_OPTIONS as BitmapOptions,
    D2D1_BITMAP_OPTIONS_CANNOT_DRAW as BITMAP_OPTIONS_CANNOT_DRAW,
    D2D1_BITMAP_OPTIONS_CPU_READ as BITMAP_OPTIONS_CPU_READ,
    D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE as BITMAP_OPTIONS_GDI_COMPATIBLE,
    D2D1_BITMAP_OPTIONS_TARGET as BITMAP_OPTIONS_TARGET,
    D2D1_BITMAP_PROPERTIES1 as BitmapProperties1,
};
pub use winapi::um::dcommon::D2D1_PIXEL_FORMAT as PixelFormat;

pub use bitmap::{Bitmap, Bitmap1};
pub use device::Device1;
pub use device_context::DeviceContext;
pub use factory::*;
pub use solid_color_brush::*;

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

impl Into<FactoryOptions> for DebugLevel {
    fn into(self) -> FactoryOptions {
        FactoryOptions {
            debugLevel: self.into(),
        }
    }
}
