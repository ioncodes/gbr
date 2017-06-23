extern crate gbr;

use gbr::cpu::CPU;

fn main() {
    let cpu = CPU::new("C:\\Users\\lucam\\Downloads\\Tetris.gb".to_owned());
    cpu.cycle();
    println!("Hello, world!");
}
