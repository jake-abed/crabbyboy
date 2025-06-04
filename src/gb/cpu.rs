use crate::gb::instructions::Instruction as Instr;
use crate::gb::instructions::{
    B0Instruction as B0Inst,
    B1Instruction as B1Inst,
    B2Instruction as B2Inst,
    B3Instruction as B3Inst,
    PrefixedInstruction as PrefixedInst,
};
use crate::gb::mmu::MemoryManagementUnit as MMU; // Use the acronym for space.
use crate::gb::registers as reg;

#[derive(Debug)]
pub struct CPU {
    registers: reg::Registers,
    pub memory_bus: MMU,
    pub end: bool,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: reg::Registers::new(),
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

    fn fetch_n16(&mut self) -> u16 {
        let n16: u16 = self.memory_bus.read_word(self.registers.pc);
        self.registers.advance_pc();
        self.registers.advance_pc();
        n16
    }

    fn execute(&mut self, byte: u8, prefixed: bool) {
        let instruction_result = Instr::from_byte(byte, prefixed);
        match instruction_result {
            Ok(Instr::Block0(instruction)) => self.execute_block_zero(instruction),
            Ok(Instr::Block1(instruction)) => self.execute_block_one(instruction),
            Ok(Instr::Block2(instruction)) => self.execute_block_two(instruction),
            Ok(Instr::Block3(instruction)) => self.execute_block_three(instruction),
            Ok(Instr::Prefixed(instruction)) => self.execute_prefixed(instruction),
            Err(error) => println!("{error:?}"),
        }
    }

    fn execute_block_zero(&mut self, instruction: B0Inst) {
        match instruction {
            B0Inst::NOP => println!("Got NOP"),
            B0Inst::LDR16N16(dest) => self.ldr16n16(dest),
            B0Inst::LDR16(dest) => self.ldr16(dest),
            _ => println!("Idk"),
        }
    }

    fn execute_block_one(&mut self, instruction: B1Inst) {
        match instruction {
            B1Inst::LD{dest, source} => println!("GOT LD: {dest} - {source}"),
            _ => println!("Idk"),
        }
    }

    fn execute_block_two(&mut self, instruction: B2Inst) {
        match instruction {
            B2Inst::ADD(val) => println!("GOT ADD: {val}"),
            _ => println!("Idk"),
        }
    }

    fn execute_block_three(&mut self, instruction: B3Inst) {
        match instruction {
            B3Inst::ADDN8 => println!("GOT ADDN8)"),
            _ => println!("Idk"),
        }
    }

    fn execute_prefixed(&mut self, instruction: PrefixedInst) {
        match instruction {
            PrefixedInst::RLC(operand) => println!("Got RLC(operand) - {operand}"),
            _ => println!("Idk"),
        }
    }

    fn ldr16n16(&mut self, dest: u8) {
        let n16: u16 = self.fetch_n16();
        match reg::R16::try_from(dest) {
            Ok(reg::R16::BC) => self.registers.set_bc(n16),
            Ok(reg::R16::DE) => self.registers.set_de(n16),
            Ok(reg::R16::HL) => self.registers.set_hl(n16),
            Ok(reg::R16::SP) => self.registers.sp = n16,
            Err(err) => panic!("{err:?}"),
        }
    }

    fn ldr16(&mut self, dest: u8) {
        match reg::R16Mem::try_from(dest) {
            Ok(reg::R16Mem::BC) => self.registers.a = self.memory_bus.read_byte(self.registers.bc()),
            Ok(reg::R16Mem::DE) => self.registers.a = self.memory_bus.read_byte(self.registers.de()),
            Ok(reg::R16Mem::HLI) => {
                let hl = self.registers.hl();
                self.registers.a = self.memory_bus.read_byte(hl);
                self.registers.set_hl(hl.wrapping_add(1));
            },
            Ok(reg::R16Mem::HLD) => {
                let hl = self.registers.hl();
                self.registers.a = self.memory_bus.read_byte(hl);
                self.registers.set_hl(hl.wrapping_sub(1));
            },
            Err(err) => panic!("{err:?}"),
        }
    }

}
