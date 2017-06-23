extern crate gbr;

use gbr::cpu::CPU;

fn main() {
    let cpu = CPU::new("../roms/tetris.gb".to_owned());
    cpu.cycle();
    println!("Hello, world!");
}
