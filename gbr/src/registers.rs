use flags::Flags;

pub struct Registers {
    pub pc: u16, // program_counter
    pub sp: u16, // stack_pointer
    pub a: u8,
    pub f: Flags, // flags
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            pc: 0,
            sp: 0,
            a: 0,
            f: Flags {
                c: 0,
                h: 0,
                n: 0,
                z: 0,
            },
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,
        }
    }
}
