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

    let mut end_found = false;
    let mut instr_count: u16 = 0;

    while !end_found {
        cpu.cycle();
        instr_count += 1;
        if cpu.end || instr_count > 260 {
            end_found = true;
        }
    }
}
