/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::ops::{Deref, DerefMut};
use std::ptr;
use winapi::{
    shared::winerror::{E_NOINTERFACE, S_OK},
    um::unknwnbase::IUnknown,
    Interface,
};

#[derive(Debug)]
pub struct ComPtr<T>
where
    T: Interface,
{
    ptr: *mut T,
}

impl<T> ComPtr<T>
where
    T: Interface,
{
    pub fn from_ptr(ptr: *mut T) -> Self {
        unsafe {
            if !ptr.is_null() {
                (*(ptr as *mut IUnknown)).AddRef();
            }
            ComPtr { ptr }
        }
    }

    pub unsafe fn already_addrefed(ptr: *mut T) -> Self {
        ComPtr { ptr }
    }

    pub unsafe fn getter_addrefs<Q>(&mut self) -> *mut *mut Q {
        self.release();
        &mut self.ptr as *mut *mut _ as *mut *mut Q
    }

    pub fn as_ptr(&self) -> *mut T {
        self.ptr
    }

    pub fn query_interface<Q>(&self) -> Option<ComPtr<Q>>
    where
        Q: Interface,
    {
        if self.ptr.is_null() {
            return None;
        }

        unsafe {
            let mut p = ComPtr::<Q>::default();
            let hr =
                (*(self.ptr as *mut IUnknown)).QueryInterface(&Q::uuidof(), p.getter_addrefs());
            if hr == S_OK {
                return Some(p);
            }
            assert!(hr == E_NOINTERFACE);
            None
        }
    }

    pub unsafe fn add_ref(&self) {
        assert!(!self.ptr.is_null());
        let unknwn = self.ptr as *mut IUnknown;
        (*unknwn).AddRef();
    }

    pub unsafe fn release(&self) {
        if !self.ptr.is_null() {
            let unknwn = self.ptr as *mut IUnknown;
            (*unknwn).Release();
        }
    }

    pub fn forget(&mut self) -> *mut T {
        let ptr = self.ptr;
        self.ptr = ptr::null_mut();
        ptr
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
}

impl<T> Clone for ComPtr<T>
where
    T: Interface,
{
    fn clone(&self) -> Self {
        if !self.ptr.is_null() {
            unsafe {
                self.add_ref();
            }
        }
        ComPtr { ptr: self.ptr }
    }
}

impl<T> Default for ComPtr<T>
where
    T: Interface,
{
    fn default() -> Self {
        Self {
            ptr: ptr::null_mut(),
        }
    }
}
impl<T> Deref for ComPtr<T>
where
    T: Interface,
{
    type Target = T;
    fn deref(&self) -> &T {
        assert!(!self.ptr.is_null());
        unsafe { &mut *self.ptr }
    }
}

impl<T> DerefMut for ComPtr<T>
where
    T: Interface,
{
    fn deref_mut(&mut self) -> &mut T {
        assert!(!self.ptr.is_null());
        unsafe { &mut *self.ptr }
    }
}

impl<T> PartialEq for ComPtr<T>
where
    T: Interface,
{
    fn eq(&self, other: &ComPtr<T>) -> bool {
        self.ptr == other.ptr
    }
}

impl<T> Drop for ComPtr<T>
where
    T: Interface,
{
    fn drop(&mut self) {
        unsafe {
            self.release();
        }
    }
}

unsafe impl<T> Send for ComPtr<T> where T: Interface {}
