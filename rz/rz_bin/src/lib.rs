use rz_sys as sys;

use std::ffi::{CString, CStr};

pub struct RzList<T> {
    ptr: *mut sys::RzList,
    phantom: std::marker::PhantomData<T>
}

impl<T> RzList<T> {
    pub fn from_raw(ptr: *mut sys::RzList) -> Self {
        Self {
            ptr,
            phantom: std::marker::PhantomData
        }
    }
}

impl<T> Drop for RzList<T> {
    fn drop(&mut self) {
        unsafe {
            sys::rz_list_free(self.ptr);
        }
    }
}

pub struct RzBin {
    ptr: *mut sys::rz_bin_t
}

impl RzBin {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { sys::rz_bin_new() }
        }
    }

    pub fn set_baddr(&mut self, addr: u64) {
        unsafe {
            sys::rz_bin_set_baddr(self.ptr, addr);
        }
    }

    pub fn get_size(&self) -> u64 {
        unsafe {
            sys::rz_bin_get_size(self.ptr)
        }
    }

    /*pub fn get_reg_profile(&self) -> String {
        unsafe {
            let c = rz_analysis_get_reg_profile(self.ptr);
            let c_str: &CStr = unsafe { CStr::from_ptr(c) };
            let str_slice: &str = c_str.to_str().unwrap();
            return str_slice.to_owned();
        }
    }*/
}

impl Drop for RzBin {
    fn drop(&mut self) {
        unsafe {
            sys::rz_bin_free(self.ptr);
        }
    }
}
