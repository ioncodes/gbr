use registers::Registers;
use rom::Rom;

pub struct CPU {
    pub opcode: u8,
    pub rom: Rom,
}

impl CPU {
    pub fn new(rom_path: String) -> CPU {
        CPU {
            opcode: 0x00,
            rom: Rom::new(rom_path.to_owned()),
        }
    }

    pub fn cycle(&self) {
        // load next opcode
    }

    pub fn execute(&self) {
        // execute opcode
    }
}
