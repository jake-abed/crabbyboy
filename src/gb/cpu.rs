use crate::gb::instructions;
use crate::gb::mmu::MemoryManagementUnit as MMU; // Use the acronym for space.
use crate::gb::registers;

pub struct CPU {
    registers: registers::Registers,
    pub memory_bus: MMU,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: registers::Registers::new(),
            memory_bus: MMU::new(),
        }
    }

    pub fn cycle(&mut self) {
        let mut byte = self.fetch();
        let prefixed: bool = byte == 0xCB;
        if prefixed {
            byte = self.fetch();
        }
        self.execute(byte, prefixed);
    }

    fn fetch(&mut self) -> u8 {
        let byte = self.memory_bus.read_byte(self.registers.pc);
        self.registers.advance_pc();
        byte
    }

    fn execute(&mut self, byte: u8, prefixed: bool) {
        let instruction = instructions::Instruction::from_byte(byte, prefixed);
        println!("{byte} - {instruction:?}");
    }
}
