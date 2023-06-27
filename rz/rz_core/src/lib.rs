use rz_sys as sys;

use std::ffi::{CString, CStr};

pub struct RzCore {
    ptr: *mut sys::rz_core_t
}

impl RzCore {
    pub fn new() -> Self {
        RzCore {
            ptr: unsafe { sys::rz_core_new() }
        }
    }

    pub fn init(&mut self) -> bool {
        unsafe {
            sys::rz_core_init(self.ptr)
        }
    }

    pub fn rebase(&mut self, addr: u64) -> bool {
        unsafe {
            sys::rz_core_bin_rebase(self.ptr, addr)
        }
    }

    /*pub fn open(&mut self, path: &str) -> RzCoreFile {
        let path = CString::new(path).unwrap();
        unsafe {
            RzCoreFile {
                ptr: sys::rz_core_file_open(self.ptr, path.as_ptr(), 0, 0)
            }
        }
    }*/

    pub fn open_load(&mut self, path: &str) {
        let path = CString::new(path).unwrap();
        unsafe {
            sys::rz_core_file_open_load(self.ptr, path.as_ptr(), 0, 0, false);
        }
    }

    pub fn is_debug(&self) -> bool {
        unsafe {
            sys::rz_core_is_debug(self.ptr)
        }
    }

    pub fn reopen(args: &str) {
        unsafe {
            // sys::rz_core_file_reopen(self.ptr, ...);
        }
    }

    pub fn reopen_debug(args: &str) {
        unsafe {
            // sys::rz_core_file_reopen_debug(self.ptr, ...);
        }
    }

    /*pub fn is_project(&self) -> bool {
        unsafe {
            rz_core_is_project(self.ptr)
        }
    }*/

    /*pub fn hex_of_assembly(&self, asm: &str) {
        unsafe {
            rz_core_hex_of_assembly(self.ptr, );
        }
    }*/
}

impl Drop for RzCore {
    fn drop(&mut self) {
        unsafe {
            sys::rz_core_free(self.ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
