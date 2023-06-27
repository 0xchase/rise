use rz_core::*;
use rz_analysis::*;

pub struct Rise {
    core: RzCore
}

impl Rise {
    pub fn new() -> Self {
        Self {
            core: RzCore::new()
        }
    }

    pub fn load(&mut self, path: &str) {
        println!("Loading a thing");
        self.core.open_load(path);
    }
}