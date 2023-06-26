use rizin_sys::*;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
struct RzCore {
    ptr: *mut rz_core_t
}

impl RzCore {
    pub fn new() -> Self {
        RzCore {
            ptr: unsafe { rz_core_new() }
        }
    }

    pub fn init(&mut self) -> bool {
        unsafe {
            rz_core_init(self.ptr)
        }
    }

    pub fn rebase(&mut self, addr: u64) -> bool {
        unsafe {
            rz_core_bin_rebase(self.ptr, addr)
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
            rz_core_free(self.ptr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("Creating rizin core");

        let mut core = RzCore::new();

        core.rebase(0);
        core.rebase(0x40000);

        drop(core);
    }
}
