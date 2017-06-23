pub struct Zram {
    pub map: Vec<u8>,
}
impl Zram {
    pub fn new(zram: Vec<u8>) -> Zram {
        Zram { map: zram }
    }
}
