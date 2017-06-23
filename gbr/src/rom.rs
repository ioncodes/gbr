use std::fs::File;
use std::io::Read;
pub struct Rom {
    pub map: Vec<u8>,
}

impl Rom {
    pub fn new(rom_path: String) -> Rom {
        // load rom
        let mut f = File::open(rom_path).unwrap();
        let mut buffer = Vec::<u8>::new();
        // read the whole file
        f.read_to_end(&mut buffer);
        Rom { map: buffer }
    }
}
