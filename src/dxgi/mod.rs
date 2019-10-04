mod device;
mod factory;
mod output;
mod surface;
mod swap_chain;

use bitflags::bitflags;
use winapi::shared::{dxgi::*, dxgi1_2::*, dxgiformat::*, dxgitype::*};

pub use device::{Device, Device1, Device2, Device3, Device4};
pub use factory::{create_factory, create_factory_2, Factory2};
pub use output::Output;
pub use surface::{Surface, Surface1, Surface2};
pub use swap_chain::{SwapChain1, SwapChainDesc1};

// ===== DXGI_FORMAT =====
#[derive(Clone, Copy)]
pub enum AlphaMode {
    Unspecified,
    Premultiplied,
    Straight,
    Ignore,
}

/*
impl Into<DXGI_ALPHA_MODE> for AlphaMode {
    fn into(self) -> DXGI_ALPHA_MODE {
        match self {
            AlphaMode::Unspecified => DXGI_ALPHA_MODE_UNSPECIFIED,
            AlphaMode::Premultiplied => DXGI_ALPHA_MODE_PREMULTIPLIED,
            AlphaMode::Straight => DXGI_ALPHA_MODE_STRAIGHT,
            AlphaMode::Ignore => DXGI_ALPHA_MODE_IGNORE,

        }
    }
}
*/
impl From<AlphaMode> for DXGI_ALPHA_MODE {
    fn from(mode: AlphaMode) -> Self {
        use AlphaMode::*;
        match mode {
            Unspecified => DXGI_ALPHA_MODE_UNSPECIFIED,
            Premultiplied => DXGI_ALPHA_MODE_PREMULTIPLIED,
            Straight => DXGI_ALPHA_MODE_STRAIGHT,
            Ignore => DXGI_ALPHA_MODE_IGNORE,
        }
    }
}

#[derive(Clone, Copy)]
pub enum Format {
    Bgra8,
    Other(DXGI_FORMAT),
}

impl Into<DXGI_FORMAT> for Format {
    fn into(self) -> DXGI_FORMAT {
        match self {
            Format::Bgra8 => DXGI_FORMAT_B8G8R8A8_UNORM,
            Format::Other(fmt) => fmt,
        }
    }
}

bitflags! {
    pub struct Usage: DXGI_USAGE {
        const SHADER_INPUT = DXGI_USAGE_SHADER_INPUT;
        const RENDER_TARGET_OUTPUT = DXGI_USAGE_RENDER_TARGET_OUTPUT;
        const BACK_BUFFER = DXGI_USAGE_BACK_BUFFER;
        const SHARED = DXGI_USAGE_SHARED;
        const READ_ONLY = DXGI_USAGE_READ_ONLY;
        const DISCARD_ON_PRESENT = DXGI_USAGE_DISCARD_ON_PRESENT;
        const UNORDERED_ACCESS = DXGI_USAGE_UNORDERED_ACCESS;
    }
}

impl From<Usage> for DXGI_USAGE {
    fn from(usage: Usage) -> DXGI_USAGE {
        usage.bits()
    }
}

#[derive(Clone, Copy)]
pub enum SampleDesc {
    NoAntiAliasing,
}

impl Into<DXGI_SAMPLE_DESC> for SampleDesc {
    fn into(self) -> DXGI_SAMPLE_DESC {
        match self {
            SampleDesc::NoAntiAliasing => DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
        }
    }
}

#[derive(Clone, Copy)]
pub enum SwapEffect {
    Discard,
    Sequential,
    FlipDiscard,
    FlipSequential,
}

impl Into<DXGI_SWAP_EFFECT> for SwapEffect {
    fn into(self) -> DXGI_SWAP_EFFECT {
        match self {
            SwapEffect::Discard => DXGI_SWAP_EFFECT_DISCARD,
            SwapEffect::Sequential => DXGI_SWAP_EFFECT_SEQUENTIAL,
            SwapEffect::FlipDiscard => DXGI_SWAP_EFFECT_FLIP_DISCARD,
            SwapEffect::FlipSequential => DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        }
    }
}

// ===== IDXGIObject =====
// ===== IDXGIDeviceSubObject =====
// ===== IDXGIResource =====
// ===== IDXGIKeyedMutex =====
// ===== IDXGISurface =====
// ===== IDXGIAdapter =====
// ===== IDXGIOutput =====
// ===== IDXGISwapChain =====
// ===== IDXGISwapChain1 =====
// ===== IDXGIFactory =====
// ===== IDXGIDevice =====
// ===== IDXGIFactory1 =====
// ===== IDXGIFactory2 =====
// ===== IDXGIAdapter1 =====
// ===== IDXGIDevice1 =====
