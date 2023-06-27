use rz_sys as sys;

use std::ffi::{CString, CStr};

pub struct RzAnalysis {
    ptr: *mut sys::rz_analysis_t
}

impl RzAnalysis {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { sys::rz_analysis_new() }
        }
    }

    pub fn purge(&mut self) {
        unsafe {
            sys::rz_analysis_purge(self.ptr);
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

impl Drop for RzAnalysis {
    fn drop(&mut self) {
        unsafe {
            sys::rz_analysis_free(self.ptr);
        }
    }
}
