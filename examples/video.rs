use noodle::{d3d11, d3dcompiler, mf};
use std::ptr;
use winapi::um::combaseapi::{CoInitializeEx, CoUninitialize};
use winapi::um::d3d11::D3D11_FILTER_MIN_MAG_MIP_LINEAR;

const VS_HLSL_SRC: &'static str = r#"
cbuffer cbPerObject : register(b0) {
	matrix		g_mWorldViewProjection	: packoffset( c0 );
	matrix		g_mWorld				: packoffset( c4 );
};

struct VS_INPUT {
	float4 vPosition	: POSITION;
	float3 vNormal		: NORMAL;
	float2 vTexcoord	: TEXCOORD0;
};

struct VS_OUTPUT {
	float3 vNormal		: NORMAL;
	float2 vTexcoord	: TEXCOORD0;
    float4 vPosition	: SV_POSITION;
};

VS_OUTPUT VSMain(VS_INPUT Input) {
	VS_OUTPUT Output;
	
	Output.vPosition = mul( Input.vPosition, g_mWorldViewProjection );
	Output.vNormal = mul( Input.vNormal, (float3x3)g_mWorld );
	Output.vTexcoord = Input.vTexcoord;

	return Output;
}
"#;

const PS_HLSL_SRC: &'static str = r#"
cbuffer cbPerObject: register(b0) {
	float4		g_vObjectColor			: packoffset(c0);
};

cbuffer cbPerFrame: register(b1) {
	float3		g_vLightDir				: packoffset( c0 );
	float		g_fAmbient				: packoffset( c0.w );
};

Texture2D	g_txDiffuse: register(t0);
SamplerState g_samLinear: register(s0);

struct PS_INPUT {
	float3 vNormal		: NORMAL;
	float2 vTexcoord	: TEXCOORD0;
};

float4 PSMain(PS_INPUT Input) : SV_TARGET {
	float4 vDiffuse = g_txDiffuse.Sample( g_samLinear, Input.vTexcoord );
	
	float fLighting = saturate( dot( g_vLightDir, Input.vNormal ) );
	fLighting = max( fLighting, g_fAmbient );
	
	return vDiffuse * fLighting;
}
"#;

macro_rules! format_guid {
    ($guid:expr) => {
        format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            $guid.Data1,
            $guid.Data2,
            $guid.Data3,
            $guid.Data4[0],
            $guid.Data4[1],
            $guid.Data4[2],
            $guid.Data4[3],
            $guid.Data4[4],
            $guid.Data4[5],
            $guid.Data4[6],
            $guid.Data4[7]
        )
    };
}

fn com_initialize() {
    use winapi::um::objbase::{COINIT_APARTMENTTHREADED, COINIT_DISABLE_OLE1DDE};
    unsafe {
        CoInitializeEx(
            ptr::null_mut(),
            COINIT_APARTMENTTHREADED | COINIT_DISABLE_OLE1DDE,
        );
    }
}

fn com_uninitialize() {
    unsafe {
        CoUninitialize();
    }
}

fn main() {
    com_initialize();
    mf::startup();

    {
        let device: d3d11::Device = d3d11::create_hardware_device(
            d3d11::CreateDevice::VIDEO_SUPPORT | d3d11::CreateDevice::BGRA_SUPPORT,
        );

        let multithread: Option<d3d11::Multithread> = device.as_();
        if let Some(multithread) = multithread {
            println!("Setting multithread protection");
            multithread.set_multithread_protected(true);
        }

        let vs_byte_code = d3dcompiler::Compiler::default()
            .name("vs_hlsl_src")
            .text(VS_HLSL_SRC)
            .entry_point("VSMain")
            .target("vs_4_0_level_9_3")
            .compile();

        let vs_byte_code = match vs_byte_code {
            Ok(vs) => vs,
            Err(s) => panic!("{}", s),
        };

        let vertex_shader = device.create_vertex_shader(&vs_byte_code);

        let ps_byte_code = d3dcompiler::Compiler::default()
             .text(PS_HLSL_SRC)
             .entry_point("PSMain")
             .target("ps_4_0_level_9_1")
             .compile().unwrap();

        let pixel_shader = device.create_pixel_shader(&ps_byte_code);

        // Create our vertex input layout
        let layout = [
            input_element_desc!{"POSITION", 0, dxgi::Format::Rg32F, 0, 0, D, 0},
            input_element_desc!{"TEXCOORD", 0, dxgi::Format::Rg32F, 0, 8, D, 0},
        ];

        // Create a sampler state
        let mut sam_desc = d3d11::SamplerDesc::default();
        sam_desc.Filter = D3D11_FILTER_MIN_MAG_MIP_LINEAR;

        let manager = mf::create_dxgi_device_manager();
        manager.reset_device(&device);

        let mut attributes = mf::create_attributes(3);
        attributes.set(mf::SourceReaderAttr::D3dManager, &manager);
        attributes.set(mf::ReadWriteAttr::EnableHardwareTransforms, true);
        attributes.set(mf::SourceReaderAttr::EnableAdvancedVideoProcessing, true);

        let reader = mf::create_source_reader_from_url(
            "Rogue One - A Star Wars Story - Trailer.mp4",
            &attributes,
        );

        let curr_media_type = reader.current_media_type(mf::SourceReaderStream::FirstVideo);
        println!(
            "curr media type: {}",
            format_guid!(curr_media_type.major_type())
        );
        let attr = curr_media_type.get(mf::Attr::SubType);
        if let Some(guid) = attr.as_guid() {
            println!("SubType = {}", format_guid!(guid));
        }

        let mut output_type = mf::create_media_type();
        output_type.set(mf::Attr::MajorType, mf::MajorTypeAttr::Video);
        output_type.set(mf::Attr::SubType, mf::VideoFormat::Nv12);
        reader.set_current_media_type(mf::SourceReaderStream::FirstVideo, &output_type);

        let mut frame_count = 0;

        println!("Started processing frames");

        loop {
            let (_, flags, _, sample) = reader.read_sample(
                mf::SourceReaderStream::FirstVideo,
                mf::SourceReaderControlFlag::empty(),
            );

            if flags.contains(mf::SourceReaderFlag::END_OF_STREAM) || sample.is_none() {
                break;
            }

            if let Some(sample) = sample {
                println!("Frame {}", frame_count);
                frame_count += 1;

                let buffer = sample.convert_to_contiguous_buffer();
                let dxgi_buffer: Option<mf::BufferDxgi> = buffer.as_();
                if let Some(dxgi_buffer) = dxgi_buffer {
                    println!("It's a DXGI buffer");
                }
            }
        }

        println!("Finished processing frames");
    }

    mf::shutdown();
    com_uninitialize();
}
