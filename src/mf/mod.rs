use crate::{impl_comptr, impl_interface, AsPtr, ComPtr, ToUtf16};
use bitflags::bitflags;
use winapi::{
    shared::{
        guiddef::{GUID, REFGUID},
        winerror::{S_FALSE, S_OK},
    },
    um::{
        mfapi::{
            MFCreateAttributes, MFCreateDXGIDeviceManager, MFCreateMediaType, MFMediaType_Audio,
            MFMediaType_Video, MFShutdown, MFStartup, MFVideoFormat_NV12, MFSTARTUP_FULL,
            MF_MT_MAJOR_TYPE, MF_MT_SUBTYPE, MF_VERSION,
        },
        mfobjects::{
            IMF2DBuffer, IMFAttributes, IMFDXGIBuffer, IMFDXGIDeviceManager, IMFMediaBuffer,
            IMFMediaType, IMFSample, MF_ATTRIBUTE_BLOB, MF_ATTRIBUTE_DOUBLE, MF_ATTRIBUTE_GUID,
            MF_ATTRIBUTE_IUNKNOWN, MF_ATTRIBUTE_STRING, MF_ATTRIBUTE_UINT32, MF_ATTRIBUTE_UINT64,
        },
        mfreadwrite::{
            IMFSourceReader, MFCreateSourceReaderFromURL, MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS,
            MF_SOURCE_READERF_ALLEFFECTSREMOVED, MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED,
            MF_SOURCE_READERF_ENDOFSTREAM, MF_SOURCE_READERF_ERROR,
            MF_SOURCE_READERF_NATIVEMEDIATYPECHANGED, MF_SOURCE_READERF_NEWSTREAM,
            MF_SOURCE_READERF_STREAMTICK, MF_SOURCE_READER_ALL_STREAMS,
            MF_SOURCE_READER_ANY_STREAM, MF_SOURCE_READER_CONTROLF_DRAIN,
            MF_SOURCE_READER_D3D_MANAGER, MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING,
            MF_SOURCE_READER_FIRST_AUDIO_STREAM, MF_SOURCE_READER_FIRST_VIDEO_STREAM,
        },
        propidl::PROPVARIANT,
        unknwnbase::IUnknown,
        winnt::HRESULT,
    },
    Interface,
};

#[derive(Clone, Copy)]
pub enum Attr {
    MajorType,
    SubType,
}

#[derive(Clone, Copy)]
pub enum SourceReaderAttr {
    D3dManager,
    EnableAdvancedVideoProcessing,
}

#[derive(Clone, Copy)]
pub enum ReadWriteAttr {
    EnableHardwareTransforms,
}

#[derive(Clone, Copy)]
pub enum MajorTypeAttr {
    Audio,
    Video,
}

#[derive(Clone, Copy)]
pub enum SourceReaderStream {
    All,
    Any,
    FirstAudio,
    FirstVideo,
    Index(u32),
}

impl From<u32> for SourceReaderStream {
    fn from(index: u32) -> Self {
        assert!(index < 0x8000_0000);
        SourceReaderStream::Index(index)
    }
}

impl From<SourceReaderStream> for u32 {
    fn from(stream: SourceReaderStream) -> Self {
        use self::SourceReaderStream::*;
        match stream {
            All => MF_SOURCE_READER_ALL_STREAMS,
            Any => MF_SOURCE_READER_ANY_STREAM,
            FirstAudio => MF_SOURCE_READER_FIRST_AUDIO_STREAM,
            FirstVideo => MF_SOURCE_READER_FIRST_VIDEO_STREAM,
            Index(index) => index,
        }
    }
}

#[derive(Clone, Copy)]
pub enum VideoFormat {
    Nv12,
}

impl_comptr! { Attributes: [IMFAttributes, IUnknown] }
impl_comptr! { DXGIDeviceManager(u32): [IMFDXGIDeviceManager, IUnknown] }
impl_comptr! { SourceReader: [IMFSourceReader, IUnknown] }
impl_comptr! { MediaType: [IMFMediaType, IMFAttributes, IUnknown] }
impl_comptr! { Sample: [IMFSample, IMFAttributes, IUnknown] }
impl_comptr! { MediaBuffer: [IMFMediaBuffer, IUnknown] }
impl_comptr! { Buffer2D: [IMF2DBuffer, IUnknown] }
impl_comptr! { BufferDxgi: [IMFDXGIBuffer, IUnknown] }

pub fn startup() {
    unsafe {
        MFStartup(MF_VERSION, MFSTARTUP_FULL);
    }
}

pub fn shutdown() {
    unsafe {
        MFShutdown();
    }
}

pub fn create_attributes(initial_size: u32) -> Attributes {
    let mut attributes = ComPtr::<IMFAttributes>::default();
    let hr = unsafe { MFCreateAttributes(attributes.getter_addrefs(), initial_size) };
    assert!(hr == 0);
    attributes.into()
}

pub fn create_dxgi_device_manager() -> DXGIDeviceManager {
    let mut reset_token = 0u32;
    let mut manager = ComPtr::<IMFDXGIDeviceManager>::default();
    let hr = unsafe { MFCreateDXGIDeviceManager(&mut reset_token, manager.getter_addrefs()) };
    assert!(hr == 0);
    (manager, reset_token).into()
}

pub fn create_media_type() -> MediaType {
    let mut media_type = ComPtr::<IMFMediaType>::default();
    let hr = unsafe { MFCreateMediaType(media_type.getter_addrefs()) };
    assert!(hr == 0);
    media_type.into()
}

fn _create_source_reader_from_url(url: &[u16], attributes: &Attributes) -> SourceReader {
    let mut source_reader = ComPtr::<IMFSourceReader>::default();
    let hr = unsafe {
        MFCreateSourceReaderFromURL(
            url.as_ptr(),
            attributes.as_ptr(),
            source_reader.getter_addrefs(),
        )
    };
    assert!(hr == 0);
    source_reader.into()
}

pub fn create_source_reader_from_url(url: impl ToUtf16, attributes: &Attributes) -> SourceReader {
    let url = url.to_utf16();
    _create_source_reader_from_url(&url, attributes)
}

pub trait ToAttributeKey {
    fn to_key(self) -> REFGUID;
}

impl ToAttributeKey for Attr {
    fn to_key(self) -> REFGUID {
        match self {
            Attr::MajorType => &MF_MT_MAJOR_TYPE,
            Attr::SubType => &MF_MT_SUBTYPE,
        }
    }
}

impl ToAttributeKey for SourceReaderAttr {
    fn to_key(self) -> REFGUID {
        match self {
            SourceReaderAttr::D3dManager => &MF_SOURCE_READER_D3D_MANAGER,
            SourceReaderAttr::EnableAdvancedVideoProcessing => {
                &MF_SOURCE_READER_ENABLE_ADVANCED_VIDEO_PROCESSING
            }
        }
    }
}

impl ToAttributeKey for ReadWriteAttr {
    fn to_key(self) -> REFGUID {
        match self {
            ReadWriteAttr::EnableHardwareTransforms => &MF_READWRITE_ENABLE_HARDWARE_TRANSFORMS,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum AttributeValue {
    Uint32(u32),
    Uint64(u64),
    Double(f64),
    Guid(REFGUID),
    //String(LPWSTR),
    //Blob(&[u8]),
    Unknown(*mut IUnknown),
}

impl AttributeValue {
    pub fn as_u32(&self) -> Option<u32> {
        use self::AttributeValue::*;
        match self {
            Uint32(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        use self::AttributeValue::*;
        match self {
            Uint64(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        use self::AttributeValue::*;
        match self {
            Double(v) => Some(*v),
            _ => None,
        }
    }

    pub fn as_guid(&self) -> Option<&'static GUID> {
        use self::AttributeValue::*;
        match self {
            Guid(guid) => Some(unsafe { &**guid }),
            _ => None,
        }
    }
}

pub trait ToAttributeValue {
    fn to_value(self) -> AttributeValue;
}

impl ToAttributeValue for bool {
    fn to_value(self) -> AttributeValue {
        AttributeValue::Uint32(self as _)
    }
}

impl ToAttributeValue for u32 {
    fn to_value(self) -> AttributeValue {
        AttributeValue::Uint32(self)
    }
}

impl ToAttributeValue for MajorTypeAttr {
    fn to_value(self) -> AttributeValue {
        use self::MajorTypeAttr::*;
        match self {
            Audio => AttributeValue::Guid(&MFMediaType_Audio),
            Video => AttributeValue::Guid(&MFMediaType_Video),
        }
    }
}

impl ToAttributeValue for VideoFormat {
    fn to_value(self) -> AttributeValue {
        use self::VideoFormat::*;
        match self {
            Nv12 => AttributeValue::Guid(&MFVideoFormat_NV12),
        }
    }
}

impl<'a, T> ToAttributeValue for &'a T
where
    T: AsPtr<IUnknown>,
{
    fn to_value(self) -> AttributeValue {
        AttributeValue::Unknown(self.as_ptr())
    }
}

impl ToAttributeValue for PROPVARIANT {
    fn to_value(self) -> AttributeValue {
        match self.vt as u32 {
            MF_ATTRIBUTE_UINT32 => AttributeValue::Uint32(unsafe { *self.data.uintVal() }),
            MF_ATTRIBUTE_UINT64 => AttributeValue::Uint64(unsafe { *self.data.uhVal().QuadPart() }),
            MF_ATTRIBUTE_DOUBLE => AttributeValue::Double(unsafe { *self.data.dblVal() }),
            MF_ATTRIBUTE_GUID => AttributeValue::Guid(unsafe { *self.data.puuid() }),
            MF_ATTRIBUTE_STRING => AttributeValue::Uint32(0), // TODO
            MF_ATTRIBUTE_BLOB => AttributeValue::Uint32(0),   // TODO
            MF_ATTRIBUTE_IUNKNOWN => AttributeValue::Unknown(unsafe { *self.data.punkVal() }),
            _ => unreachable!(),
        }
    }
}

impl_interface! {
    impl [Attributes, MediaType, Sample] {
        fn _get(&self, key: REFGUID) -> AttributeValue {
            let mut item = PROPVARIANT::default();
            let hr = unsafe { self.0.GetItem(key, &mut item) };
            assert!(hr == S_OK);
            item.to_value()
        }

        pub fn get(&self, key: impl ToAttributeKey) -> AttributeValue {
            self._get(key.to_key())
        }

        fn _set(&mut self, key: REFGUID, value: AttributeValue) {
            let hr = match value {
                AttributeValue::Guid(guid) => unsafe { self.0.SetGUID(key, guid) },
                AttributeValue::Uint32(value) => unsafe { self.0.SetUINT32(key, value) },
                AttributeValue::Uint64(value) => unsafe { self.0.SetUINT64(key, value ) },
                AttributeValue::Unknown(interface) => unsafe { self.0.SetUnknown(key, interface) },
                _ => S_OK,
            };
            assert!(hr == S_OK);
        }

        pub fn set(&mut self, key: impl ToAttributeKey, value: impl ToAttributeValue) {
            self._set(key.to_key(), value.to_value())
        }

        pub fn len(&self) -> usize {
            let mut items = 0;
            let hr = unsafe { self.0.GetCount(&mut items) };
            assert!(hr == S_OK);
            items as _
        }

        pub fn iter<'a>(&'a self) -> impl Iterator<Item=(GUID, AttributeValue)> + 'a {
            let mut total = 0;
            let hr = unsafe {
                self.0.GetCount(&mut total)
            };
            assert!(hr == S_OK);
            AttributeIter { attrs: &self.0, state: None }
        }
    }
}

impl DXGIDeviceManager {
    pub fn reset_device(&self, device: &impl AsPtr<IUnknown>) {
        let hr = unsafe { self.0.ResetDevice(device.as_ptr(), self.1) };
        assert!(hr == 0);
    }
}

bitflags! {
    pub struct SourceReaderControlFlag: u32 {
        const DRAIN = MF_SOURCE_READER_CONTROLF_DRAIN;
    }
}

impl Into<u32> for SourceReaderControlFlag {
    fn into(self) -> u32 {
        self.bits()
    }
}

bitflags! {
    pub struct SourceReaderFlag: u32 {
        const ERROR = MF_SOURCE_READERF_ERROR;
        const END_OF_STREAM = MF_SOURCE_READERF_ENDOFSTREAM;
        const NEWSTREAM = MF_SOURCE_READERF_NEWSTREAM;
        const NATIVE_MEDIA_TYPE_CHANGED = MF_SOURCE_READERF_NATIVEMEDIATYPECHANGED;
        const CURRENT_MEDIA_TYPE_CHANGED = MF_SOURCE_READERF_CURRENTMEDIATYPECHANGED;
        const STREAM_TICK = MF_SOURCE_READERF_STREAMTICK;
        const ALL_EFFECTS_REMOVED = MF_SOURCE_READERF_ALLEFFECTSREMOVED;
    }
}

impl From<u32> for SourceReaderFlag {
    fn from(raw: u32) -> Self {
        Self::from_bits_truncate(raw)
    }
}

impl SourceReader {
    /*
        virtual HRESULT STDMETHODCALLTYPE GetStreamSelection(
            /* [annotation][in] */
            _In_  DWORD dwStreamIndex,
            /* [annotation][out] */
            _Out_  BOOL *pfSelected) = 0;

        virtual HRESULT STDMETHODCALLTYPE SetStreamSelection(
            /* [annotation][in] */
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */
            _In_  BOOL fSelected) = 0;
    */
    pub fn native_media_types<'a>(
        &'a self,
        stream: impl Into<SourceReaderStream>,
    ) -> impl Iterator<Item = MediaType> + 'a {
        NativeMediaTypeIter {
            reader: self,
            stream: stream.into(),
            index: 0,
        }
    }

    /*
        virtual HRESULT STDMETHODCALLTYPE GetNativeMediaType(
            /* [annotation][in] */
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */
            _In_  DWORD dwMediaTypeIndex,
            /* [annotation][out] */
            _Out_  IMFMediaType **ppMediaType) = 0;
    */

    pub fn current_media_type(&self, stream_index: impl Into<SourceReaderStream>) -> MediaType {
        let stream_index = stream_index.into();
        let mut media_type = ComPtr::<IMFMediaType>::default();
        let hr = unsafe {
            self.0
                .GetCurrentMediaType(stream_index.into(), media_type.getter_addrefs())
        };
        assert!(hr == 0);
        media_type.into()
    }
    pub fn set_current_media_type(
        &self,
        stream_index: impl Into<SourceReaderStream>,
        media_type: &MediaType,
    ) {
        let stream_index = stream_index.into();
        let hr = unsafe {
            self.0.SetCurrentMediaType(
                stream_index.into(),
                std::ptr::null_mut(),
                media_type.as_ptr(),
            )
        };
        assert!(hr == 0);
    }

    /*
        virtual HRESULT STDMETHODCALLTYPE SetCurrentPosition(
            /* [annotation][in] */
            _In_  REFGUID guidTimeFormat,
            /* [annotation][in] */
            _In_  REFPROPVARIANT varPosition) = 0;
    */
    pub fn read_sample(
        &self,
        stream_index: impl Into<SourceReaderStream>,
        control_flags: SourceReaderControlFlag,
    ) -> (SourceReaderStream, SourceReaderFlag, i64, Option<Sample>) {
        let stream_index = stream_index.into();
        let mut actual_stream_index = 0;
        let mut stream_flags = 0;
        let mut timestamp = 0;
        let mut sample = ComPtr::<IMFSample>::default();
        let hr = unsafe {
            self.0.ReadSample(
                stream_index.into(),
                control_flags.into(),
                &mut actual_stream_index,
                &mut stream_flags,
                &mut timestamp,
                sample.getter_addrefs(),
            )
        };
        if hr < 0 {
            println!("hr = 0x{:x}", hr);
        }
        assert!(hr == 0);
        let sample = if sample.is_null() {
            None
        } else {
            Some(sample.into())
        };
        (
            actual_stream_index.into(),
            stream_flags.into(),
            timestamp,
            sample,
        )
    }

    pub fn read_sample_async(
        &self,
        stream_index: impl Into<SourceReaderStream>,
        control_flags: SourceReaderControlFlag,
    ) {
        let stream_index = stream_index.into();
        let hr = unsafe {
            self.0.ReadSample(
                stream_index.into(),
                control_flags.into(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            )
        };
        assert!(hr == 0);
    }

    /*
        virtual HRESULT STDMETHODCALLTYPE Flush(
            /* [annotation][in] */
            _In_  DWORD dwStreamIndex) = 0;

        virtual HRESULT STDMETHODCALLTYPE GetServiceForStream(
            /* [annotation][in] */
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */
            _In_  REFGUID guidService,
            /* [annotation][in] */
            _In_  REFIID riid,
            /* [annotation][out] */
            _Out_  LPVOID *ppvObject) = 0;

        virtual HRESULT STDMETHODCALLTYPE GetPresentationAttribute(
            /* [annotation][in] */
            _In_  DWORD dwStreamIndex,
            /* [annotation][in] */
            _In_  REFGUID guidAttribute,
            /* [annotation][out] */
            _Out_  PROPVARIANT *pvarAttribute) = 0;
    */
}

impl MediaType {
    pub fn major_type(&self) -> GUID {
        let mut major_type = GUID::default();
        let hr = unsafe { self.0.GetMajorType(&mut major_type) };
        assert!(hr == S_OK);
        major_type
    }

    pub fn is_compressed_format(&self) -> bool {
        let mut compressed = 0;
        let hr = unsafe { self.0.IsCompressedFormat(&mut compressed) };
        assert!(hr == S_OK);
        compressed != 0
    }
}

impl PartialEq for MediaType {
    fn eq(&self, other: &Self) -> bool {
        let mut dummy = 0;
        let hr = unsafe { self.0.IsEqual(other.as_ptr(), &mut dummy) };
        assert!(hr == S_OK || hr == S_FALSE);
        hr == S_OK
    }
}

impl Sample {
    pub fn buffer_count(&self) -> usize {
        let mut count = 0;
        let hr = unsafe { self.0.GetBufferCount(&mut count) };
        assert!(hr == 0);
        count as _
    }

    pub fn buffer(&self, index: usize) -> MediaBuffer {
        debug_assert!(index < self.buffer_count());
        let mut buffer = ComPtr::<IMFMediaBuffer>::default();
        let hr = unsafe { self.0.GetBufferByIndex(index as _, buffer.getter_addrefs()) };
        assert!(hr == 0);
        buffer.into()
    }

    pub fn convert_to_contiguous_buffer(&self) -> MediaBuffer {
        let mut buffer = ComPtr::<IMFMediaBuffer>::default();
        let hr = unsafe { self.0.ConvertToContiguousBuffer(buffer.getter_addrefs()) };
        assert!(hr == 0);
        buffer.into()
    }
}

impl MediaBuffer {
    fn _lock_read(&self) -> &[u8] {
        let mut length = 0;
        let mut buffer = std::ptr::null_mut();
        let hr = unsafe { self.0.Lock(&mut buffer, std::ptr::null_mut(), &mut length) };
        assert!(hr == 0);
        unsafe { std::slice::from_raw_parts(buffer, length as _) }
    }
    fn _unlock(&self) {
        let hr = unsafe { self.0.Unlock() };
        assert!(hr == 0);
    }

    pub fn lock(&self, f: impl Fn(&[u8])) {
        let buffer = self._lock_read();
        f(buffer);
        self._unlock();
    }

    pub fn current_length(&self) -> usize {
        let mut length = 0;
        let hr = unsafe { self.0.GetCurrentLength(&mut length) };
        assert!(hr == 0);
        length as _
    }
    pub fn max_length(&self) -> usize {
        let mut length = 0;
        let hr = unsafe { self.0.GetMaxLength(&mut length) };
        assert!(hr == 0);
        length as _
    }
}

struct AttributeIter<'a> {
    attrs: &'a IMFAttributes,
    state: Option<(u32, u32)>,
}

impl<'a> AttributeIter<'a> {
    fn init_state(&mut self) -> bool {
        assert!(self.state.is_none());
        // Lock the attributes so the call to the retrieve the item count is
        // correct. (As opposed to relying on getting the count when creating
        // the iterator)
        let hr = unsafe { self.attrs.LockStore() };
        if hr != S_OK {
            return false;
        }
        let mut total = 0;
        let hr = unsafe { self.attrs.GetCount(&mut total) };
        if hr != S_OK {
            return false;
        }
        self.state = Some((0, total));
        true
    }
}

impl<'a> Drop for AttributeIter<'a> {
    fn drop(&mut self) {
        if self.state.is_some() {
            unsafe { self.attrs.UnlockStore() };
        }
    }
}

impl<'a> Iterator for AttributeIter<'a> {
    type Item = (GUID, AttributeValue);
    fn next(&mut self) -> Option<Self::Item> {
        if self.state.is_none() && !self.init_state() {
            return None;
        }
        match self.state {
            Some((ref mut index, total)) => {
                if *index >= total {
                    return None;
                }
                let mut key = GUID::default();
                let mut value = PROPVARIANT::default();
                let hr = unsafe { self.attrs.GetItemByIndex(*index, &mut key, &mut value) };
                assert!(hr == S_OK);
                *index += 1;
                Some((key, value.to_value()))
            }
            _ => None,
        }
    }
}

struct NativeMediaTypeIter<'a> {
    reader: &'a SourceReader,
    stream: SourceReaderStream,
    index: u32,
}

#[allow(overflowing_literals)]
const MF_E_NO_MORE_TYPES: HRESULT = 0xc00d_36b9;

impl<'a> Iterator for NativeMediaTypeIter<'a> {
    type Item = MediaType;
    fn next(&mut self) -> Option<Self::Item> {
        let mut media_type = ComPtr::<IMFMediaType>::default();
        let hr = unsafe {
            self.reader.0.GetNativeMediaType(
                self.stream.into(),
                self.index,
                media_type.getter_addrefs(),
            )
        };
        self.index += 1;
        match hr {
            S_OK => Some(media_type.into()),
            MF_E_NO_MORE_TYPES => None,
            _ => panic!(),
        }
    }
}

impl BufferDxgi {
    pub fn resource<T, I>(&self) -> T
    where
        T: From<ComPtr<I>>,
        I: Interface,
    {
        let mut result = ComPtr::<I>::default();
        let hr = unsafe { self.0.GetResource(&I::uuidof(), result.getter_addrefs()) };
        assert!(hr == S_OK);
        result.into()
    }

    pub fn subresource_index(&self) -> u32 {
        let mut subresource = 0;
        let hr = unsafe { self.0.GetSubresourceIndex(&mut subresource) };
        assert!(hr == S_OK);
        subresource
    }
}
