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
        let core = RzCore::new();

        println!("Dropping rizin core");
        drop(core);
    }
}
