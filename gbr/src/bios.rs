pub struct Bios {
    pub map: Vec<u8>,
    pub in_bios: bool,
}
impl Bios {
    pub fn new(bios: Vec<u8>) -> Bios {
        Bios {
            map: bios,
            in_bios: true,
        }
    }
}
