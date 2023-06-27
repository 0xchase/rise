use rz_analysis::*;
use rz_bin::*;
use rz_core::*;
use rz_io::*;

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

    pub fn test(&mut self) {
        println!("Testing a thing");
        let mut io = RzIO::new();
        io.open("/bin/ls");

        let data = io.read_at(0, 16).unwrap();
        println!("{:#?}", data);
    }
}