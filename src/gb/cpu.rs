use crate::gb::instructions::Instruction as Instr;
use crate::gb::instructions::{BlockThreeInstruction as B3Inst, BlockZeroInstruction as B0Inst};
use crate::gb::mmu::MemoryManagementUnit as MMU; // Use the acronym for space.
use crate::gb::registers;

pub struct CPU {
    registers: registers::Registers,
    pub memory_bus: MMU,
    pub end: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: registers::Registers::new(),
            memory_bus: MMU::new(),
            end: false,
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
        let instruction = Instr::from_byte(byte, prefixed);
        match instruction {
            Ok(Instr::BlockZero(B0Inst::LDR16N16(val))) => {
                println!("{byte} - {instruction:?} - {val}")
            }
            Ok(Instr::BlockThree(B3Inst::RST(val))) => {
                println!("{byte} - {instruction:?} - {val}")
            }
            Err(_) => {
                self.end = true;
                println!("ending! {instruction:?} - byte: {byte:x}")
            },
            _ => println!("{byte} - {instruction:?}"),
        }
    }
}
