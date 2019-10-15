use crate::{hr, impl_comptr, impl_newtype, opt_ptr, AsPtr, ComPtr};
use bitflags::bitflags;
use derive_newtype::NewType;
use std::ptr;
use winapi::um::{
    d3d11::{
        D3D11CreateDevice, ID3D11Buffer, ID3D11Device, ID3D11DeviceChild, ID3D11DeviceContext,
        ID3D11InputLayout, ID3D11PixelShader, ID3D11RenderTargetView, ID3D11Resource,
        ID3D11SamplerState, ID3D11ShaderResourceView, ID3D11Texture2D, ID3D11VertexShader,
        ID3D11View, D3D11_BUFFER_DESC, D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_DEBUG,
        D3D11_CREATE_DEVICE_DEBUGGABLE, D3D11_CREATE_DEVICE_SINGLETHREADED,
        D3D11_CREATE_DEVICE_VIDEO_SUPPORT, D3D11_INPUT_ELEMENT_DESC, D3D11_RENDER_TARGET_VIEW_DESC,
        D3D11_SAMPLER_DESC, D3D11_SDK_VERSION, D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC,
        D3D11_VIEWPORT,
    },
    d3d11_4::ID3D11Multithread,
    d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
    unknwnbase::IUnknown,
};

impl_comptr! { Buffer: [ID3D11Buffer, ID3D11Resource, ID3D11DeviceChild, IUnknown] }
impl_comptr! { Device: [ID3D11Device, IUnknown] }
impl_comptr! { DeviceContext: [ID3D11DeviceContext, ID3D11DeviceChild, IUnknown] }
impl_comptr! { InputLayout: [ID3D11InputLayout, ID3D11DeviceChild, IUnknown] }
impl_comptr! { Multithread: [ID3D11Multithread, IUnknown] }
impl_comptr! { PixelShader: [ID3D11PixelShader, ID3D11DeviceChild, IUnknown] }
impl_comptr! { RenderTargetView: [ID3D11RenderTargetView, ID3D11View, ID3D11DeviceChild, IUnknown] }
impl_comptr! { SamplerState: [ID3D11SamplerState, ID3D11DeviceChild, IUnknown] }
impl_comptr! { ShaderResourceView: [ID3D11ShaderResourceView, ID3D11View, ID3D11DeviceChild, IUnknown] }
impl_comptr! { Texture2d: [ID3D11Texture2D, ID3D11Resource, ID3D11DeviceChild, IUnknown] }
impl_comptr! { VertexShader: [ID3D11VertexShader, ID3D11DeviceChild, IUnknown] }

bitflags! {
    pub struct CreateDevice: u32 {
        const SINGLE_THREADED = D3D11_CREATE_DEVICE_SINGLETHREADED;
        const DEBUG = D3D11_CREATE_DEVICE_DEBUG;
        const BGRA_SUPPORT = D3D11_CREATE_DEVICE_BGRA_SUPPORT;
        const DEVICE_DEBUGGABLE = D3D11_CREATE_DEVICE_DEBUGGABLE;
        const VIDEO_SUPPORT = D3D11_CREATE_DEVICE_VIDEO_SUPPORT;
    }
}

impl_newtype! {
    pub struct BufferDesc(D3D11_BUFFER_DESC);
    pub struct InputElementDesc(D3D11_INPUT_ELEMENT_DESC);
    pub struct RenderTargetViewDesc(D3D11_RENDER_TARGET_VIEW_DESC);
    pub struct SamplerDesc(D3D11_SAMPLER_DESC);
    pub struct SubresourceData(D3D11_SUBRESOURCE_DATA);
    pub struct Viewport(D3D11_VIEWPORT);
}

pub fn create_hardware_device(create_flags: CreateDevice) -> Device {
    let mut device = ComPtr::<ID3D11Device>::default();
    hr!(D3D11CreateDevice(
        ptr::null_mut(),
        D3D_DRIVER_TYPE_HARDWARE,
        ptr::null_mut(),
        create_flags.bits(),
        ptr::null_mut(),
        0,
        D3D11_SDK_VERSION,
        device.getter_addrefs(),
        ptr::null_mut(),
        ptr::null_mut(),
    ));
    device.into()
}

impl Device {
    pub fn create_buffer<'a>(
        &self,
        desc: &BufferDesc,
        initial_data: impl Into<Option<&'a SubresourceData>>,
    ) -> Buffer {
        let initial_data = initial_data.into();
        let mut buffer = ComPtr::<ID3D11Buffer>::default();
        hr!(self
            .0
            .CreateBuffer(&**desc, opt_ptr(initial_data), buffer.getter_addrefs()));
        buffer.into()
    }
    /*
        virtual HRESULT STDMETHODCALLTYPE CreateTexture1D( 
            _In_  const D3D11_TEXTURE1D_DESC *pDesc,
            _In_reads_opt_(_Inexpressible_(pDesc->MipLevels * pDesc->ArraySize))  const D3D11_SUBRESOURCE_DATA *pInitialData,
            _COM_Outptr_opt_  ID3D11Texture1D **ppTexture1D) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTexture2D( 
            _In_  const D3D11_TEXTURE2D_DESC *pDesc,
            _In_reads_opt_(_Inexpressible_(pDesc->MipLevels * pDesc->ArraySize))  const D3D11_SUBRESOURCE_DATA *pInitialData,
            _COM_Outptr_opt_  ID3D11Texture2D **ppTexture2D) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateTexture3D( 
            _In_  const D3D11_TEXTURE3D_DESC *pDesc,
            _In_reads_opt_(_Inexpressible_(pDesc->MipLevels))  const D3D11_SUBRESOURCE_DATA *pInitialData,
            _COM_Outptr_opt_  ID3D11Texture3D **ppTexture3D) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateShaderResourceView( 
            _In_  ID3D11Resource *pResource,
            _In_opt_  const D3D11_SHADER_RESOURCE_VIEW_DESC *pDesc,
            _COM_Outptr_opt_  ID3D11ShaderResourceView **ppSRView) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateUnorderedAccessView( 
            _In_  ID3D11Resource *pResource,
            _In_opt_  const D3D11_UNORDERED_ACCESS_VIEW_DESC *pDesc,
            _COM_Outptr_opt_  ID3D11UnorderedAccessView **ppUAView) = 0;
    */
        pub fn create_render_target_view(
        &self,
        resource: impl AsPtr<ID3D11Resource>,
        desc: &RenderTargetViewDesc,
    ) -> RenderTargetView {
        let mut view = ComPtr::<ID3D11RenderTargetView>::default();
        hr!(self
            .0
            .CreateRenderTargetView(resource.as_ptr(), &**desc, view.getter_addrefs()));
        view.into()
    }
    /*
        virtual HRESULT STDMETHODCALLTYPE CreateDepthStencilView( 
            _In_  ID3D11Resource *pResource,
            _In_opt_  const D3D11_DEPTH_STENCIL_VIEW_DESC *pDesc,
            _COM_Outptr_opt_  ID3D11DepthStencilView **ppDepthStencilView) = 0;
    */        
    pub fn create_input_layout(&self, descs: &[InputElementDesc], byte_code: &[u8]) -> InputLayout {
        let mut layout = ComPtr::<ID3D11InputLayout>::default();
        hr!(self.0.CreateInputLayout(
            descs.as_ptr() as *const _,
            descs.len() as _,
            byte_code.as_ptr() as *const _,
            byte_code.len() as _,
            layout.getter_addrefs()
        ));
        layout.into()
    }

    pub fn create_vertex_shader(&self, byte_code: &[u8]) -> VertexShader {
        let mut shader = ComPtr::<ID3D11VertexShader>::default();
        hr!(self.0.CreateVertexShader(
            byte_code.as_ptr() as *const _,
            byte_code.len() as _,
            ptr::null_mut(),
            shader.getter_addrefs()
        ));
        shader.into()
    }

    /*
        virtual HRESULT STDMETHODCALLTYPE CreateGeometryShader( 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            _In_  SIZE_T BytecodeLength,
            _In_opt_  ID3D11ClassLinkage *pClassLinkage,
            _COM_Outptr_opt_  ID3D11GeometryShader **ppGeometryShader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateGeometryShaderWithStreamOutput( 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            _In_  SIZE_T BytecodeLength,
            _In_reads_opt_(NumEntries)  const D3D11_SO_DECLARATION_ENTRY *pSODeclaration,
            _In_range_( 0, D3D11_SO_STREAM_COUNT * D3D11_SO_OUTPUT_COMPONENT_COUNT )  UINT NumEntries,
            _In_reads_opt_(NumStrides)  const UINT *pBufferStrides,
            _In_range_( 0, D3D11_SO_BUFFER_SLOT_COUNT )  UINT NumStrides,
            _In_  UINT RasterizedStream,
            _In_opt_  ID3D11ClassLinkage *pClassLinkage,
            _COM_Outptr_opt_  ID3D11GeometryShader **ppGeometryShader) = 0;
    */
    pub fn create_pixel_shader(&self, byte_code: &[u8]) -> PixelShader {
        let mut shader = ComPtr::<ID3D11PixelShader>::default();
        hr!(self.0.CreatePixelShader(
            byte_code.as_ptr() as *const _,
            byte_code.len() as _,
            ptr::null_mut(),
            shader.getter_addrefs()
        ));
        shader.into()
    }
    /*    
        virtual HRESULT STDMETHODCALLTYPE CreateHullShader( 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            _In_  SIZE_T BytecodeLength,
            _In_opt_  ID3D11ClassLinkage *pClassLinkage,
            _COM_Outptr_opt_  ID3D11HullShader **ppHullShader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDomainShader( 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            _In_  SIZE_T BytecodeLength,
            _In_opt_  ID3D11ClassLinkage *pClassLinkage,
            _COM_Outptr_opt_  ID3D11DomainShader **ppDomainShader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateComputeShader( 
            _In_reads_(BytecodeLength)  const void *pShaderBytecode,
            _In_  SIZE_T BytecodeLength,
            _In_opt_  ID3D11ClassLinkage *pClassLinkage,
            _COM_Outptr_opt_  ID3D11ComputeShader **ppComputeShader) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateClassLinkage( 
            _COM_Outptr_  ID3D11ClassLinkage **ppLinkage) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateBlendState( 
            _In_  const D3D11_BLEND_DESC *pBlendStateDesc,
            _COM_Outptr_opt_  ID3D11BlendState **ppBlendState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDepthStencilState( 
            _In_  const D3D11_DEPTH_STENCIL_DESC *pDepthStencilDesc,
            _COM_Outptr_opt_  ID3D11DepthStencilState **ppDepthStencilState) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateRasterizerState( 
            _In_  const D3D11_RASTERIZER_DESC *pRasterizerDesc,
            _COM_Outptr_opt_  ID3D11RasterizerState **ppRasterizerState) = 0;
    */        
    pub fn create_sampler_state(&self, desc: &SamplerDesc) -> SamplerState {
        let mut ss = ComPtr::<ID3D11SamplerState>::default();
        hr!(self.0.CreateSamplerState(&**desc, ss.getter_addrefs()));
        ss.into()
    }
    /*
        virtual HRESULT STDMETHODCALLTYPE CreateQuery( 
            _In_  const D3D11_QUERY_DESC *pQueryDesc,
            _COM_Outptr_opt_  ID3D11Query **ppQuery) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreatePredicate( 
            _In_  const D3D11_QUERY_DESC *pPredicateDesc,
            _COM_Outptr_opt_  ID3D11Predicate **ppPredicate) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateCounter( 
            _In_  const D3D11_COUNTER_DESC *pCounterDesc,
            _COM_Outptr_opt_  ID3D11Counter **ppCounter) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CreateDeferredContext( 
            UINT ContextFlags,
            _COM_Outptr_opt_  ID3D11DeviceContext **ppDeferredContext) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE OpenSharedResource( 
            _In_  HANDLE hResource,
            _In_  REFIID ReturnedInterface,
            _COM_Outptr_opt_  void **ppResource) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckFormatSupport( 
            _In_  DXGI_FORMAT Format,
            _Out_  UINT *pFormatSupport) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckMultisampleQualityLevels( 
            _In_  DXGI_FORMAT Format,
            _In_  UINT SampleCount,
            _Out_  UINT *pNumQualityLevels) = 0;
        
        virtual void STDMETHODCALLTYPE CheckCounterInfo( 
            _Out_  D3D11_COUNTER_INFO *pCounterInfo) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckCounter( 
            _In_  const D3D11_COUNTER_DESC *pDesc,
            _Out_  D3D11_COUNTER_TYPE *pType,
            _Out_  UINT *pActiveCounters,
            _Out_writes_opt_(*pNameLength)  LPSTR szName,
            _Inout_opt_  UINT *pNameLength,
            _Out_writes_opt_(*pUnitsLength)  LPSTR szUnits,
            _Inout_opt_  UINT *pUnitsLength,
            _Out_writes_opt_(*pDescriptionLength)  LPSTR szDescription,
            _Inout_opt_  UINT *pDescriptionLength) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE CheckFeatureSupport( 
            D3D11_FEATURE Feature,
            _Out_writes_bytes_(FeatureSupportDataSize)  void *pFeatureSupportData,
            UINT FeatureSupportDataSize) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetPrivateData( 
            _In_  REFGUID guid,
            _Inout_  UINT *pDataSize,
            _Out_writes_bytes_opt_(*pDataSize)  void *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrivateData( 
            _In_  REFGUID guid,
            _In_  UINT DataSize,
            _In_reads_bytes_opt_(DataSize)  const void *pData) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE SetPrivateDataInterface( 
            _In_  REFGUID guid,
            _In_opt_  const IUnknown *pData) = 0;
        
        virtual D3D_FEATURE_LEVEL STDMETHODCALLTYPE GetFeatureLevel( void) = 0;
        
        virtual UINT STDMETHODCALLTYPE GetCreationFlags( void) = 0;
        
        virtual HRESULT STDMETHODCALLTYPE GetDeviceRemovedReason( void) = 0;
        
        virtual void STDMETHODCALLTYPE GetImmediateContext( 
            _Outptr_  ID3D11DeviceContext **ppImmediateContext) = 0;
    */
    pub fn immediate_context(&self) -> DeviceContext {
        let mut context = ComPtr::<ID3D11DeviceContext>::default();
        unsafe { self.0.GetImmediateContext(context.getter_addrefs()) };
        context.into()
    }
    /*    
        virtual HRESULT STDMETHODCALLTYPE SetExceptionMode( 
            UINT RaiseFlags) = 0;
        
        virtual UINT STDMETHODCALLTYPE GetExceptionMode( void) = 0;
    */
}

impl DeviceContext {
    /*
        virtual void STDMETHODCALLTYPE VSSetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _In_reads_opt_(NumBuffers)  ID3D11Buffer *const *ppConstantBuffers) = 0;

        virtual void STDMETHODCALLTYPE PSSetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _In_reads_opt_(NumViews)  ID3D11ShaderResourceView *const *ppShaderResourceViews) = 0;
    */
    pub fn ps_set_shader<'a>(&self, shader: impl Into<Option<&'a PixelShader>>) {
        let shader = shader.into();
        unsafe {
            self.0.PSSetShader(shader.as_ptr(), ptr::null(), 0);
        }
    }
    /*
        virtual void STDMETHODCALLTYPE PSSetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _In_reads_opt_(NumSamplers)  ID3D11SamplerState *const *ppSamplers) = 0;
    */
    pub fn vs_set_shader<'a>(&self, shader: impl Into<Option<&'a VertexShader>>) {
        let shader = shader.into();
        unsafe {
            self.0.VSSetShader(shader.as_ptr(), ptr::null(), 0);
        }
    }

    pub fn draw_indexed(
        &self,
        index_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
    ) {
        unsafe {
            self.0
                .DrawIndexed(index_count, start_index_location, base_vertex_location);
        }
    }
    /*
        virtual void STDMETHODCALLTYPE Draw(
            _In_  UINT VertexCount,
            _In_  UINT StartVertexLocation) = 0;

        virtual HRESULT STDMETHODCALLTYPE Map(
            _In_  ID3D11Resource *pResource,
            _In_  UINT Subresource,
            _In_  D3D11_MAP MapType,
            _In_  UINT MapFlags,
            _Out_opt_  D3D11_MAPPED_SUBRESOURCE *pMappedResource) = 0;

        virtual void STDMETHODCALLTYPE Unmap(
            _In_  ID3D11Resource *pResource,
            _In_  UINT Subresource) = 0;

        virtual void STDMETHODCALLTYPE PSSetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _In_reads_opt_(NumBuffers)  ID3D11Buffer *const *ppConstantBuffers) = 0;

        virtual void STDMETHODCALLTYPE IASetInputLayout(
            _In_opt_  ID3D11InputLayout *pInputLayout) = 0;

        virtual void STDMETHODCALLTYPE IASetVertexBuffers(
            _In_range_( 0, D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _In_reads_opt_(NumBuffers)  ID3D11Buffer *const *ppVertexBuffers,
            _In_reads_opt_(NumBuffers)  const UINT *pStrides,
            _In_reads_opt_(NumBuffers)  const UINT *pOffsets) = 0;

        virtual void STDMETHODCALLTYPE IASetIndexBuffer(
            _In_opt_  ID3D11Buffer *pIndexBuffer,
            _In_  DXGI_FORMAT Format,
            _In_  UINT Offset) = 0;

        virtual void STDMETHODCALLTYPE DrawIndexedInstanced(
            _In_  UINT IndexCountPerInstance,
            _In_  UINT InstanceCount,
            _In_  UINT StartIndexLocation,
            _In_  INT BaseVertexLocation,
            _In_  UINT StartInstanceLocation) = 0;

        virtual void STDMETHODCALLTYPE DrawInstanced(
            _In_  UINT VertexCountPerInstance,
            _In_  UINT InstanceCount,
            _In_  UINT StartVertexLocation,
            _In_  UINT StartInstanceLocation) = 0;

        virtual void STDMETHODCALLTYPE GSSetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _In_reads_opt_(NumBuffers)  ID3D11Buffer *const *ppConstantBuffers) = 0;

        virtual void STDMETHODCALLTYPE GSSetShader(
            _In_opt_  ID3D11GeometryShader *pShader,
            _In_reads_opt_(NumClassInstances)  ID3D11ClassInstance *const *ppClassInstances,
            UINT NumClassInstances) = 0;

        virtual void STDMETHODCALLTYPE IASetPrimitiveTopology(
            _In_  D3D11_PRIMITIVE_TOPOLOGY Topology) = 0;

        virtual void STDMETHODCALLTYPE VSSetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _In_reads_opt_(NumViews)  ID3D11ShaderResourceView *const *ppShaderResourceViews) = 0;

        virtual void STDMETHODCALLTYPE VSSetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _In_reads_opt_(NumSamplers)  ID3D11SamplerState *const *ppSamplers) = 0;

        virtual void STDMETHODCALLTYPE Begin(
            _In_  ID3D11Asynchronous *pAsync) = 0;

        virtual void STDMETHODCALLTYPE End(
            _In_  ID3D11Asynchronous *pAsync) = 0;

        virtual HRESULT STDMETHODCALLTYPE GetData(
            _In_  ID3D11Asynchronous *pAsync,
            _Out_writes_bytes_opt_( DataSize )  void *pData,
            _In_  UINT DataSize,
            _In_  UINT GetDataFlags) = 0;

        virtual void STDMETHODCALLTYPE SetPredication(
            _In_opt_  ID3D11Predicate *pPredicate,
            _In_  BOOL PredicateValue) = 0;

        virtual void STDMETHODCALLTYPE GSSetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _In_reads_opt_(NumViews)  ID3D11ShaderResourceView *const *ppShaderResourceViews) = 0;

        virtual void STDMETHODCALLTYPE GSSetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _In_reads_opt_(NumSamplers)  ID3D11SamplerState *const *ppSamplers) = 0;
    */
    pub fn om_set_render_targets(&self, views: &[RenderTargetView]) {
        unsafe {
            self.0.OMSetRenderTargets(
                views.len() as _,
                views.as_ptr() as *const _,
                ptr::null_mut(),
            )
        }
    }
    /*
        virtual void STDMETHODCALLTYPE OMSetRenderTargetsAndUnorderedAccessViews(
            _In_  UINT NumRTVs,
            _In_reads_opt_(NumRTVs)  ID3D11RenderTargetView *const *ppRenderTargetViews,
            _In_opt_  ID3D11DepthStencilView *pDepthStencilView,
            _In_range_( 0, D3D11_1_UAV_SLOT_COUNT - 1 )  UINT UAVStartSlot,
            _In_  UINT NumUAVs,
            _In_reads_opt_(NumUAVs)  ID3D11UnorderedAccessView *const *ppUnorderedAccessViews,
            _In_reads_opt_(NumUAVs)  const UINT *pUAVInitialCounts) = 0;

        virtual void STDMETHODCALLTYPE OMSetBlendState(
            _In_opt_  ID3D11BlendState *pBlendState,
            _In_opt_  const FLOAT BlendFactor[ 4 ],
            _In_  UINT SampleMask) = 0;

        virtual void STDMETHODCALLTYPE OMSetDepthStencilState(
            _In_opt_  ID3D11DepthStencilState *pDepthStencilState,
            _In_  UINT StencilRef) = 0;

        virtual void STDMETHODCALLTYPE SOSetTargets(
            _In_range_( 0, D3D11_SO_BUFFER_SLOT_COUNT)  UINT NumBuffers,
            _In_reads_opt_(NumBuffers)  ID3D11Buffer *const *ppSOTargets,
            _In_reads_opt_(NumBuffers)  const UINT *pOffsets) = 0;
        virtual void STDMETHODCALLTYPE DrawAuto( void) = 0;

        virtual void STDMETHODCALLTYPE DrawIndexedInstancedIndirect(
            _In_  ID3D11Buffer *pBufferForArgs,
            _In_  UINT AlignedByteOffsetForArgs) = 0;

        virtual void STDMETHODCALLTYPE DrawInstancedIndirect(
            _In_  ID3D11Buffer *pBufferForArgs,
            _In_  UINT AlignedByteOffsetForArgs) = 0;

        virtual void STDMETHODCALLTYPE Dispatch(
            _In_  UINT ThreadGroupCountX,
            _In_  UINT ThreadGroupCountY,
            _In_  UINT ThreadGroupCountZ) = 0;

        virtual void STDMETHODCALLTYPE DispatchIndirect(
            _In_  ID3D11Buffer *pBufferForArgs,
            _In_  UINT AlignedByteOffsetForArgs) = 0;

        virtual void STDMETHODCALLTYPE RSSetState(
            _In_opt_  ID3D11RasterizerState *pRasterizerState) = 0;
    */
    pub fn rs_set_viewports(&self, viewports: &[Viewport]) {
        unsafe {
            self.0
                .RSSetViewports(viewports.len() as _, viewports.as_ptr() as *const _)
        };
    }
    /*
        virtual void STDMETHODCALLTYPE RSSetScissorRects(
            _In_range_(0, D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE)  UINT NumRects,
            _In_reads_opt_(NumRects)  const D3D11_RECT *pRects) = 0;

        virtual void STDMETHODCALLTYPE CopySubresourceRegion(
            _In_  ID3D11Resource *pDstResource,
            _In_  UINT DstSubresource,
            _In_  UINT DstX,
            _In_  UINT DstY,
            _In_  UINT DstZ,
            _In_  ID3D11Resource *pSrcResource,
            _In_  UINT SrcSubresource,
            _In_opt_  const D3D11_BOX *pSrcBox) = 0;

        virtual void STDMETHODCALLTYPE CopyResource(
            _In_  ID3D11Resource *pDstResource,
            _In_  ID3D11Resource *pSrcResource) = 0;

        virtual void STDMETHODCALLTYPE UpdateSubresource(
            _In_  ID3D11Resource *pDstResource,
            _In_  UINT DstSubresource,
            _In_opt_  const D3D11_BOX *pDstBox,
            _In_  const void *pSrcData,
            _In_  UINT SrcRowPitch,
            _In_  UINT SrcDepthPitch) = 0;

        virtual void STDMETHODCALLTYPE CopyStructureCount(
            _In_  ID3D11Buffer *pDstBuffer,
            _In_  UINT DstAlignedByteOffset,
            _In_  ID3D11UnorderedAccessView *pSrcView) = 0;

        virtual void STDMETHODCALLTYPE ClearRenderTargetView(
            _In_  ID3D11RenderTargetView *pRenderTargetView,
            _In_  const FLOAT ColorRGBA[ 4 ]) = 0;

        virtual void STDMETHODCALLTYPE ClearUnorderedAccessViewUint(
            _In_  ID3D11UnorderedAccessView *pUnorderedAccessView,
            _In_  const UINT Values[ 4 ]) = 0;

        virtual void STDMETHODCALLTYPE ClearUnorderedAccessViewFloat(
            _In_  ID3D11UnorderedAccessView *pUnorderedAccessView,
            _In_  const FLOAT Values[ 4 ]) = 0;

        virtual void STDMETHODCALLTYPE ClearDepthStencilView(
            _In_  ID3D11DepthStencilView *pDepthStencilView,
            _In_  UINT ClearFlags,
            _In_  FLOAT Depth,
            _In_  UINT8 Stencil) = 0;

        virtual void STDMETHODCALLTYPE GenerateMips(
            _In_  ID3D11ShaderResourceView *pShaderResourceView) = 0;

        virtual void STDMETHODCALLTYPE SetResourceMinLOD(
            _In_  ID3D11Resource *pResource,
            FLOAT MinLOD) = 0;

        virtual FLOAT STDMETHODCALLTYPE GetResourceMinLOD(
            _In_  ID3D11Resource *pResource) = 0;

        virtual void STDMETHODCALLTYPE ResolveSubresource(
            _In_  ID3D11Resource *pDstResource,
            _In_  UINT DstSubresource,
            _In_  ID3D11Resource *pSrcResource,
            _In_  UINT SrcSubresource,
            _In_  DXGI_FORMAT Format) = 0;

        virtual void STDMETHODCALLTYPE ExecuteCommandList(
            _In_  ID3D11CommandList *pCommandList,
            BOOL RestoreContextState) = 0;

        virtual void STDMETHODCALLTYPE HSSetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _In_reads_opt_(NumViews)  ID3D11ShaderResourceView *const *ppShaderResourceViews) = 0;

        virtual void STDMETHODCALLTYPE HSSetShader(
            _In_opt_  ID3D11HullShader *pHullShader,
            _In_reads_opt_(NumClassInstances)  ID3D11ClassInstance *const *ppClassInstances,
            UINT NumClassInstances) = 0;

        virtual void STDMETHODCALLTYPE HSSetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _In_reads_opt_(NumSamplers)  ID3D11SamplerState *const *ppSamplers) = 0;

        virtual void STDMETHODCALLTYPE HSSetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _In_reads_opt_(NumBuffers)  ID3D11Buffer *const *ppConstantBuffers) = 0;

        virtual void STDMETHODCALLTYPE DSSetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _In_reads_opt_(NumViews)  ID3D11ShaderResourceView *const *ppShaderResourceViews) = 0;

        virtual void STDMETHODCALLTYPE DSSetShader(
            _In_opt_  ID3D11DomainShader *pDomainShader,
            _In_reads_opt_(NumClassInstances)  ID3D11ClassInstance *const *ppClassInstances,
            UINT NumClassInstances) = 0;

        virtual void STDMETHODCALLTYPE DSSetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _In_reads_opt_(NumSamplers)  ID3D11SamplerState *const *ppSamplers) = 0;

        virtual void STDMETHODCALLTYPE DSSetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _In_reads_opt_(NumBuffers)  ID3D11Buffer *const *ppConstantBuffers) = 0;

        virtual void STDMETHODCALLTYPE CSSetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _In_reads_opt_(NumViews)  ID3D11ShaderResourceView *const *ppShaderResourceViews) = 0;

        virtual void STDMETHODCALLTYPE CSSetUnorderedAccessViews(
            _In_range_( 0, D3D11_1_UAV_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_1_UAV_SLOT_COUNT - StartSlot )  UINT NumUAVs,
            _In_reads_opt_(NumUAVs)  ID3D11UnorderedAccessView *const *ppUnorderedAccessViews,
            _In_reads_opt_(NumUAVs)  const UINT *pUAVInitialCounts) = 0;

        virtual void STDMETHODCALLTYPE CSSetShader(
            _In_opt_  ID3D11ComputeShader *pComputeShader,
            _In_reads_opt_(NumClassInstances)  ID3D11ClassInstance *const *ppClassInstances,
            UINT NumClassInstances) = 0;

        virtual void STDMETHODCALLTYPE CSSetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _In_reads_opt_(NumSamplers)  ID3D11SamplerState *const *ppSamplers) = 0;

        virtual void STDMETHODCALLTYPE CSSetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _In_reads_opt_(NumBuffers)  ID3D11Buffer *const *ppConstantBuffers) = 0;

        virtual void STDMETHODCALLTYPE VSGetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _Out_writes_opt_(NumBuffers)  ID3D11Buffer **ppConstantBuffers) = 0;

        virtual void STDMETHODCALLTYPE PSGetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _Out_writes_opt_(NumViews)  ID3D11ShaderResourceView **ppShaderResourceViews) = 0;

        virtual void STDMETHODCALLTYPE PSGetShader(
            _Outptr_result_maybenull_  ID3D11PixelShader **ppPixelShader,
            _Out_writes_opt_(*pNumClassInstances)  ID3D11ClassInstance **ppClassInstances,
            _Inout_opt_  UINT *pNumClassInstances) = 0;

        virtual void STDMETHODCALLTYPE PSGetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _Out_writes_opt_(NumSamplers)  ID3D11SamplerState **ppSamplers) = 0;

        virtual void STDMETHODCALLTYPE VSGetShader(
            _Outptr_result_maybenull_  ID3D11VertexShader **ppVertexShader,
            _Out_writes_opt_(*pNumClassInstances)  ID3D11ClassInstance **ppClassInstances,
            _Inout_opt_  UINT *pNumClassInstances) = 0;

        virtual void STDMETHODCALLTYPE PSGetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _Out_writes_opt_(NumBuffers)  ID3D11Buffer **ppConstantBuffers) = 0;

        virtual void STDMETHODCALLTYPE IAGetInputLayout(
            _Outptr_result_maybenull_  ID3D11InputLayout **ppInputLayout) = 0;

        virtual void STDMETHODCALLTYPE IAGetVertexBuffers(
            _In_range_( 0, D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_IA_VERTEX_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _Out_writes_opt_(NumBuffers)  ID3D11Buffer **ppVertexBuffers,
            _Out_writes_opt_(NumBuffers)  UINT *pStrides,
            _Out_writes_opt_(NumBuffers)  UINT *pOffsets) = 0;

        virtual void STDMETHODCALLTYPE IAGetIndexBuffer(
            _Outptr_opt_result_maybenull_  ID3D11Buffer **pIndexBuffer,
            _Out_opt_  DXGI_FORMAT *Format,
            _Out_opt_  UINT *Offset) = 0;

        virtual void STDMETHODCALLTYPE GSGetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _Out_writes_opt_(NumBuffers)  ID3D11Buffer **ppConstantBuffers) = 0;

        virtual void STDMETHODCALLTYPE GSGetShader(
            _Outptr_result_maybenull_  ID3D11GeometryShader **ppGeometryShader,
            _Out_writes_opt_(*pNumClassInstances)  ID3D11ClassInstance **ppClassInstances,
            _Inout_opt_  UINT *pNumClassInstances) = 0;

        virtual void STDMETHODCALLTYPE IAGetPrimitiveTopology(
            _Out_  D3D11_PRIMITIVE_TOPOLOGY *pTopology) = 0;

        virtual void STDMETHODCALLTYPE VSGetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _Out_writes_opt_(NumViews)  ID3D11ShaderResourceView **ppShaderResourceViews) = 0;

        virtual void STDMETHODCALLTYPE VSGetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _Out_writes_opt_(NumSamplers)  ID3D11SamplerState **ppSamplers) = 0;

        virtual void STDMETHODCALLTYPE GetPredication(
            _Outptr_opt_result_maybenull_  ID3D11Predicate **ppPredicate,
            _Out_opt_  BOOL *pPredicateValue) = 0;

        virtual void STDMETHODCALLTYPE GSGetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _Out_writes_opt_(NumViews)  ID3D11ShaderResourceView **ppShaderResourceViews) = 0;

        virtual void STDMETHODCALLTYPE GSGetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _Out_writes_opt_(NumSamplers)  ID3D11SamplerState **ppSamplers) = 0;

        virtual void STDMETHODCALLTYPE OMGetRenderTargets(
            _In_range_( 0, D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT )  UINT NumViews,
            _Out_writes_opt_(NumViews)  ID3D11RenderTargetView **ppRenderTargetViews,
            _Outptr_opt_result_maybenull_  ID3D11DepthStencilView **ppDepthStencilView) = 0;

        virtual void STDMETHODCALLTYPE OMGetRenderTargetsAndUnorderedAccessViews(
            _In_range_( 0, D3D11_SIMULTANEOUS_RENDER_TARGET_COUNT )  UINT NumRTVs,
            _Out_writes_opt_(NumRTVs)  ID3D11RenderTargetView **ppRenderTargetViews,
            _Outptr_opt_result_maybenull_  ID3D11DepthStencilView **ppDepthStencilView,
            _In_range_( 0, D3D11_PS_CS_UAV_REGISTER_COUNT - 1 )  UINT UAVStartSlot,
            _In_range_( 0, D3D11_PS_CS_UAV_REGISTER_COUNT - UAVStartSlot )  UINT NumUAVs,
            _Out_writes_opt_(NumUAVs)  ID3D11UnorderedAccessView **ppUnorderedAccessViews) = 0;

        virtual void STDMETHODCALLTYPE OMGetBlendState(
            _Outptr_opt_result_maybenull_  ID3D11BlendState **ppBlendState,
            _Out_opt_  FLOAT BlendFactor[ 4 ],
            _Out_opt_  UINT *pSampleMask) = 0;

        virtual void STDMETHODCALLTYPE OMGetDepthStencilState(
            _Outptr_opt_result_maybenull_  ID3D11DepthStencilState **ppDepthStencilState,
            _Out_opt_  UINT *pStencilRef) = 0;

        virtual void STDMETHODCALLTYPE SOGetTargets(
            _In_range_( 0, D3D11_SO_BUFFER_SLOT_COUNT )  UINT NumBuffers,
            _Out_writes_opt_(NumBuffers)  ID3D11Buffer **ppSOTargets) = 0;

        virtual void STDMETHODCALLTYPE RSGetState(
            _Outptr_result_maybenull_  ID3D11RasterizerState **ppRasterizerState) = 0;

        virtual void STDMETHODCALLTYPE RSGetViewports(
            _Inout_ /*_range(0, D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE )*/   UINT *pNumViewports,
            _Out_writes_opt_(*pNumViewports)  D3D11_VIEWPORT *pViewports) = 0;

        virtual void STDMETHODCALLTYPE RSGetScissorRects(
            _Inout_ /*_range(0, D3D11_VIEWPORT_AND_SCISSORRECT_OBJECT_COUNT_PER_PIPELINE )*/   UINT *pNumRects,
            _Out_writes_opt_(*pNumRects)  D3D11_RECT *pRects) = 0;

        virtual void STDMETHODCALLTYPE HSGetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _Out_writes_opt_(NumViews)  ID3D11ShaderResourceView **ppShaderResourceViews) = 0;

        virtual void STDMETHODCALLTYPE HSGetShader(
            _Outptr_result_maybenull_  ID3D11HullShader **ppHullShader,
            _Out_writes_opt_(*pNumClassInstances)  ID3D11ClassInstance **ppClassInstances,
            _Inout_opt_  UINT *pNumClassInstances) = 0;

        virtual void STDMETHODCALLTYPE HSGetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _Out_writes_opt_(NumSamplers)  ID3D11SamplerState **ppSamplers) = 0;

        virtual void STDMETHODCALLTYPE HSGetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _Out_writes_opt_(NumBuffers)  ID3D11Buffer **ppConstantBuffers) = 0;

        virtual void STDMETHODCALLTYPE DSGetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _Out_writes_opt_(NumViews)  ID3D11ShaderResourceView **ppShaderResourceViews) = 0;

        virtual void STDMETHODCALLTYPE DSGetShader(
            _Outptr_result_maybenull_  ID3D11DomainShader **ppDomainShader,
            _Out_writes_opt_(*pNumClassInstances)  ID3D11ClassInstance **ppClassInstances,
            _Inout_opt_  UINT *pNumClassInstances) = 0;

        virtual void STDMETHODCALLTYPE DSGetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _Out_writes_opt_(NumSamplers)  ID3D11SamplerState **ppSamplers) = 0;

        virtual void STDMETHODCALLTYPE DSGetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _Out_writes_opt_(NumBuffers)  ID3D11Buffer **ppConstantBuffers) = 0;

        virtual void STDMETHODCALLTYPE CSGetShaderResources(
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_INPUT_RESOURCE_SLOT_COUNT - StartSlot )  UINT NumViews,
            _Out_writes_opt_(NumViews)  ID3D11ShaderResourceView **ppShaderResourceViews) = 0;

        virtual void STDMETHODCALLTYPE CSGetUnorderedAccessViews(
            _In_range_( 0, D3D11_1_UAV_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_1_UAV_SLOT_COUNT - StartSlot )  UINT NumUAVs,
            _Out_writes_opt_(NumUAVs)  ID3D11UnorderedAccessView **ppUnorderedAccessViews) = 0;

        virtual void STDMETHODCALLTYPE CSGetShader(
            _Outptr_result_maybenull_  ID3D11ComputeShader **ppComputeShader,
            _Out_writes_opt_(*pNumClassInstances)  ID3D11ClassInstance **ppClassInstances,
            _Inout_opt_  UINT *pNumClassInstances) = 0;

        virtual void STDMETHODCALLTYPE CSGetSamplers(
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_SAMPLER_SLOT_COUNT - StartSlot )  UINT NumSamplers,
            _Out_writes_opt_(NumSamplers)  ID3D11SamplerState **ppSamplers) = 0;

        virtual void STDMETHODCALLTYPE CSGetConstantBuffers(
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - 1 )  UINT StartSlot,
            _In_range_( 0, D3D11_COMMONSHADER_CONSTANT_BUFFER_API_SLOT_COUNT - StartSlot )  UINT NumBuffers,
            _Out_writes_opt_(NumBuffers)  ID3D11Buffer **ppConstantBuffers) = 0;
        virtual void STDMETHODCALLTYPE ClearState( void) = 0;
        virtual void STDMETHODCALLTYPE Flush( void) = 0;
        virtual D3D11_DEVICE_CONTEXT_TYPE STDMETHODCALLTYPE GetType( void) = 0;
        virtual UINT STDMETHODCALLTYPE GetContextFlags( void) = 0;

        virtual HRESULT STDMETHODCALLTYPE FinishCommandList(
            BOOL RestoreDeferredContextState,
            _COM_Outptr_opt_  ID3D11CommandList **ppCommandList) = 0;
    */
}

impl Multithread {
    pub fn set_multithread_protected(&self, protect: bool) -> bool {
        let result = unsafe { self.0.SetMultithreadProtected(protect as _) };
        result != 0
    }
}

#[derive(Clone, Copy, Default, NewType)]
#[repr(transparent)]
pub struct Texture2dDesc(D3D11_TEXTURE2D_DESC);

impl Texture2d {
    pub fn desc(&self) -> Texture2dDesc {
        let mut desc = D3D11_TEXTURE2D_DESC::default();
        unsafe { self.0.GetDesc(&mut desc) };
        desc.into()
    }
}
