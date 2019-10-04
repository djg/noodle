use std::{ffi::OsStr, os::windows::ffi::OsStrExt};

pub trait ToUtf16 {
    fn to_utf16(&self) -> Vec<u16>;
}

impl ToUtf16 for OsStr {
    fn to_utf16(&self) -> Vec<u16> {
        self.encode_wide().chain(Some(0)).collect()
    }
}

impl ToUtf16 for str {
    fn to_utf16(&self) -> Vec<u16> {
        self.encode_utf16().chain(Some(0)).collect()
    }
}
