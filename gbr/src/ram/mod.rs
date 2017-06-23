mod eram;
mod wram;
mod zram;

use self::eram::Eram;
use self::wram::Wram;
use self::zram::Zram;

pub struct Ram {
    pub eram: Eram,
    pub wram: Wram,
    pub zram: Zram,
}
impl Ram {
    pub fn new() -> Ram {
        Ram {
            eram: Eram::new(Vec::new()),
            wram: Wram::new(Vec::new()),
            zram: Zram::new(Vec::new()),
        }
    }
}
