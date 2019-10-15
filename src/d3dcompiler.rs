use crate::{ComPtr};
use std::{ffi::CString, ptr, slice};
use winapi::um::{
    d3dcompiler::D3DCompile,
    d3dcommon::ID3DBlob,
};

#[derive(Default)]
pub struct Compiler {
    src: Vec<u8>,
    source_name: Option<CString>,
    entry_point: Option<CString>,
    target: Option<CString>,
}

impl Compiler {
    pub fn text(mut self, src: impl Into<Vec<u8>>) -> Self {
        self.src = CString::new(src.into()).unwrap().into_bytes_with_nul();
        self
    }

    pub fn name(mut self, name: impl Into<Vec<u8>>) -> Self {
        self.source_name = Some(CString::new(name.into()).unwrap());
        self
    }

    pub fn entry_point(mut self, name: impl Into<Vec<u8>>) -> Self {
        self.entry_point = Some(CString::new(name.into()).unwrap());
        self
    }
    
    pub fn target(mut self, target: impl Into<Vec<u8>>) -> Self {
        self.target = dbg!(Some(CString::new(target.into()).unwrap()));
        self
    }
    
    pub fn compile(self) -> Result<Vec<u8>, String> {
        let source_name = self.source_name.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        let entry_point = self.entry_point.as_ref().map(|s| s.as_ptr()).unwrap_or(ptr::null());
        let target = self.target.as_ref().map(|s| dbg!(s).as_ptr()).unwrap_or(ptr::null());

        let mut code = ComPtr::<ID3DBlob>::default();
        let mut error_msgs = ComPtr::<ID3DBlob>::default();

        let hr = unsafe { D3DCompile(
            self.src.as_ptr() as *const _,
            self.src.len(),
            source_name,
            ptr::null(),
            ptr::null_mut(),
            entry_point,
            target,
            0,
            0,
            code.getter_addrefs(),
            error_msgs.getter_addrefs()) };
        if hr == 0 {
            let code = unsafe { slice::from_raw_parts(code.GetBufferPointer() as *const _,code.GetBufferSize() as _) };
            Ok(code.to_vec())
        } else {
            let errors = unsafe { slice::from_raw_parts(error_msgs.GetBufferPointer() as *const _, error_msgs.GetBufferSize() as _) };
            Err(unsafe { CString::from_vec_unchecked(errors.to_vec()) }.to_string_lossy().into_owned())
        }
    }
}
