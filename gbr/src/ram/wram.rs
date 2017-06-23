pub struct Wram {
    pub map: Vec<u8>,
}
impl Wram {
    pub fn new(wram: Vec<u8>) -> Wram {
        Wram { map: wram }
    }
}
