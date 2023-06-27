use std::ffi::{CString, CStr};

pub struct RzIO {
    ptr: *mut rz_sys::RzIO
}

impl RzIO {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { rz_sys::rz_io_init(rz_sys::rz_io_new()) }
        }
    }

    pub fn open(&mut self, path: &str) {
        println!("RzIO: Opening {}", path);
        let path = CString::new(path).unwrap();
        unsafe {
            let desc = rz_sys::rz_io_open(self.ptr, path.as_ptr(), 0, 0);
            if desc == std::ptr::null_mut() {
                panic!("Error opening thing");
            } else {
                println!("Opened thing");
            }
        }
    }

    pub fn open_at(&mut self) {
        todo!()
    }

    pub fn is_mapped(&self, vaddr: u64) -> bool {
        unsafe {
            rz_sys::rz_io_addr_is_mapped(self.ptr, vaddr)
        }
    }

    pub fn read_at(&self, addr: u64, size: usize) -> Result<Vec<u8>, ()> {
        println!("RzIO: Reading {} bytes at {}", size, addr);
        unsafe {
            let mut vec = vec![0; size];
            let success = rz_sys::rz_io_read_at(self.ptr, addr, vec.as_mut_ptr(), size as i32);

            if success {
                Ok(vec)
            } else {
                Err(())
            }
        }
    }

    pub fn read_at_mapped(&self, addr: u64, size: usize) -> Result<Vec<u8>, ()> {
        println!("RzIO: Reading {} bytes mapped at {}", size, addr);
        unsafe {
            let mut vec = vec![0; size];
            let success = rz_sys::rz_io_read_at_mapped(self.ptr, addr, vec.as_mut_ptr(), size as i32);

            if success {
                Ok(vec)
            } else {
                Err(())
            }
        }
    }
}

impl Drop for RzIO {
    fn drop(&mut self) {
        unsafe {
            rz_sys::rz_io_free(self.ptr);
        }
    }
}

pub struct RzIODesc {
    ptr: *mut rz_sys::RzIODesc
}

impl RzIODesc {
    /*pub fn new() -> Self {
        Self {
            ptr: unsafe { rz_io_desc_new() }
        }
    }*/
}