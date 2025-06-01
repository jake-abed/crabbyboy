mod gb;

use std::fs;
use crate::gb::cpu::CPU;

fn main() {
    // Read the boot rom
    let bin = fs::read("./roms/dmg_boot.bin").unwrap();
    let _boot_rom = String::new();

    // Make a new CPU
    let mut cpu = CPU::new();

    // Make an iterator out of the binary file
    let iterator = bin.iter();

    // Load the iterator into memory
    cpu.memory_bus.load_rom(iterator);

    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
    cpu.cycle();
}
