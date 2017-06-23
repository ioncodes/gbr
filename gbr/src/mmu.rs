use bios::Bios;
use rom::Rom;
use ram::Ram;
use registers::Registers;

pub struct Mmu {
    pub bios: Bios,
    pub rom: Rom,
    pub ram: Ram,
    pub registers: Registers,
}
impl Mmu {
    fn read8(&mut self, addr: u8) -> u8 {
        match addr & 0xF000 {
            0x000 => {
                if self.bios.in_bios {
                    if addr < 0x0100 {
                        return self.bios.map[addr as usize];
                    } else if self.registers.pc == 0x0100 {
                        self.bios.in_bios = false;
                    }
                }
                self.rom.map[addr as usize]
            }
            _ => {
                println!("{:?}", "Unknown opcode");
                return 0x00;
            }
        }
    }
    fn read16(&mut self, addr: u8) -> u16 {
        return (self.read8(addr) as u16 + ((self.read8(addr + 1) as u16) << 8)) as u16;
    }
    fn write8(addr: u16, value: u8) {}
    fn write16(addr: u16, value: u16) {}
}
