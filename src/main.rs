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

    // Take a quick slice of the memory from 0x0000 to 0x0100
    let slice = &cpu.memory_bus.memory[0..256];

    // And take a peek! Should start with 49 and end with 80!
    println!("Dumping: \n{0:?}", slice);
}
