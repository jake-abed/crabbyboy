use crate::gb::instructions::Instruction as Instr;
use crate::gb::instructions::{
    B0Instruction as B0Inst, B1Instruction as B1Inst, B2Instruction as B2Inst,
    B3Instruction as B3Inst, PrefixedInstruction as PrefixedInst,
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

    /* Grabs and returns one single byte from the address stored at the program
     * counter, then advances the program counter once.
     */
    fn fetch(&mut self) -> u8 {
        let byte = self.memory_bus.read_byte(self.registers.pc);
        self.registers.advance_pc();
        byte
    }

    /* Grabs and returns two bytes as a u16 from the address stored at the
     * prgram counter, then advances the program counter twice.
     */
    fn fetch_n16(&mut self) -> u16 {
        let n16: u16 = self.memory_bus.read_word(self.registers.pc);
        self.registers.advance_pc();
        self.registers.advance_pc();
        n16
    }

    /* Parent function to execute the an instruction. Filters down through
     * successive match cases to perform the expected instruction.
     */
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

    /* Filtering functions for Blocks 0, 1, 2, 3, and prefixed to pass down
     * responsibility to individual helper functions for the specific opcode
     * families.
     */
    fn execute_block_zero(&mut self, instruction: B0Inst) {
        match instruction {
            B0Inst::NOP => println!("Got NOP"),
            B0Inst::LDR16N16(dest) => self.ldr16n16(dest),
            B0Inst::LDR16(dest) => self.ldr16(dest),
            B0Inst::LDA(source) => self.lda(source),
            B0Inst::LDN16SP => self.ldn16sp(),
            B0Inst::INCR16(operand) => self.incr16(operand),
            B0Inst::DECR16(operand) => self.decr16(operand),
            B0Inst::ADDHL(operand) => self.addhl(operand),
            B0Inst::INCR8(operand) => self.incr8(operand),
            B0Inst::DECR8(operand) => self.decr8(operand),
            B0Inst::LDR8N8(dest) => self.ldr8n8(dest),
            B0Inst::RLCA => self.rlca(),
            B0Inst::RRCA => self.rrca(),
            B0Inst::RLA => self.rla(),
            B0Inst::RRA => self.rra(),
            B0Inst::DAA => self.daa(),
            B0Inst::CPL => self.cpl(),
            B0Inst::SCF => self.scf(),
            B0Inst::CCF => self.ccf(),
            B0Inst::JRN8 => self.jrn8(),
            B0Inst::JRCONDN8(cond) => self.jrcondn8(cond),
            B0Inst::STOP => self.stop(),
        }
    }

    fn execute_block_one(&mut self, instruction: B1Inst) {
        match instruction {
            B1Inst::LD { dest, source } => println!("GOT LD: {dest} - {source}"),
            B1Inst::HALT => println!("Got HALT"),
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

    // Begin Block 0 Helper Functions
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
            Ok(reg::R16Mem::BC) => {
                self.memory_bus
                    .set_byte(self.registers.bc(), self.registers.a);
            }
            Ok(reg::R16Mem::DE) => {
                self.memory_bus
                    .set_byte(self.registers.de(), self.registers.a);
            }
            Ok(reg::R16Mem::HLI) => {
                let hl = self.registers.hl();
                self.memory_bus.set_byte(hl, self.registers.a);
                self.registers.set_hl(hl.wrapping_add(1));
            }
            Ok(reg::R16Mem::HLD) => {
                let hl = self.registers.hl();
                self.memory_bus.set_byte(hl, self.registers.a);
                self.registers.set_hl(hl.wrapping_sub(1));
            }
            Err(err) => panic!("{err:?}"),
        }
    }

    fn lda(&mut self, source: u8) {
        match reg::R16Mem::try_from(source) {
            Ok(reg::R16Mem::BC) => {
                self.registers.a = self.memory_bus.read_byte(self.registers.bc())
            }
            Ok(reg::R16Mem::DE) => {
                self.registers.a = self.memory_bus.read_byte(self.registers.de())
            }
            Ok(reg::R16Mem::HLI) => {
                let hl = self.registers.hl();
                self.registers.a = self.memory_bus.read_byte(hl);
                self.registers.set_hl(hl.wrapping_add(1));
            }
            Ok(reg::R16Mem::HLD) => {
                let hl = self.registers.hl();
                self.registers.a = self.memory_bus.read_byte(hl);
                self.registers.set_hl(hl.wrapping_sub(1));
            }
            Err(err) => panic!("{err:?}"),
        }
    }

    fn ldn16sp(&mut self) {
        let n16 = self.fetch_n16();
        let sp_high: u8 = (self.registers.sp & 0x00FF) as u8;
        let sp_low: u8 = (self.registers.sp >> 8) as u8;
        self.memory_bus.set_byte(n16, sp_high);
        self.memory_bus.set_byte(n16.wrapping_add(1), sp_low);
    }

    fn incr16(&mut self, operand: u8) {
        match reg::R16::try_from(operand) {
            Ok(reg::R16::BC) => {
                let current_bc = self.registers.bc();
                self.registers.set_bc(current_bc.wrapping_add(1));
            }
            Ok(reg::R16::DE) => {
                let current_de = self.registers.de();
                self.registers.set_de(current_de.wrapping_add(1));
            }
            Ok(reg::R16::HL) => {
                let current_hl = self.registers.hl();
                self.registers.set_hl(current_hl.wrapping_add(1));
            }
            Ok(reg::R16::SP) => {
                let current_sp = self.registers.sp;
                self.registers.sp = current_sp.wrapping_add(1);
            }
            Err(err) => panic!("{err:?}"),
        }
    }

    fn decr16(&mut self, operand: u8) {
        match reg::R16::try_from(operand) {
            Ok(reg::R16::BC) => {
                let current_bc = self.registers.bc();
                self.registers.set_bc(current_bc.wrapping_sub(1));
            }
            Ok(reg::R16::DE) => {
                let current_de = self.registers.de();
                self.registers.set_de(current_de.wrapping_sub(1));
            }
            Ok(reg::R16::HL) => {
                let current_hl = self.registers.hl();
                self.registers.set_hl(current_hl.wrapping_sub(1));
            }
            Ok(reg::R16::SP) => {
                let current_sp = self.registers.sp;
                self.registers.sp = current_sp.wrapping_sub(1);
            }
            Err(err) => panic!("{err:?}"),
        }
    }

    fn addhl(&mut self, operand: u8) {
        let old_hl = self.registers.hl();
        let mut register_val: u16 = 0;
        match reg::R16::try_from(operand) {
            Ok(reg::R16::BC) => register_val = self.registers.bc(),
            Ok(reg::R16::DE) => register_val = self.registers.de(),
            Ok(reg::R16::HL) => register_val = self.registers.hl(),
            Ok(reg::R16::SP) => register_val = self.registers.sp,
            _ => {}
        }
        let (res, carry) = old_hl.overflowing_add(register_val);
        self.registers.f.c = carry;
        self.registers.f.s = false;

        let mask: u16 = 0b1111_1111_1111;
        self.registers.f.h = (register_val & mask) + (old_hl & mask) > mask;

        self.registers.set_hl(res);
    }

    fn incr8(&mut self, operand: u8) {}

    fn decr8(&mut self, operand: u8) {}

    fn ldr8n8(&mut self, dest: u8) {
        let n8: u8 = self.fetch();
        match reg::R8::try_from(dest) {
            Ok(reg::R8::B) => self.registers.b = n8,
            Ok(reg::R8::C) => self.registers.c = n8,
            Ok(reg::R8::D) => self.registers.d = n8,
            Ok(reg::R8::E) => self.registers.e = n8,
            Ok(reg::R8::H) => self.registers.h = n8,
            Ok(reg::R8::L) => self.registers.l = n8,
            Ok(reg::R8::HL) => {
                self.memory_bus.set_byte(self.registers.hl(), n8);
            }
            Ok(reg::R8::A) => self.registers.a = n8,
            Err(err) => panic!("{err:?}"),
        }
    }

    fn rlca(&mut self) {
        let bit: u8 = self.registers.a >> 7;
        let shifted: u8 = (self.registers.a & 0x7F) << 1;
        self.registers.f.h = false;
        self.registers.f.z = false;
        self.registers.f.s = false;
        self.registers.f.c = bit > 0;
        self.registers.a = shifted | bit;
    }

    fn rrca(&mut self) {
        let bit: u8 = self.registers.a & 0x01;
        let shifted: u8 = (self.registers.a & 0xFE) >> 1;
        self.registers.f.h = false;
        self.registers.f.z = false;
        self.registers.f.s = false;
        self.registers.f.c = bit > 0;
        self.registers.a = shifted | (bit << 7);
    }

    fn rla(&mut self) {
        let bit: u8 = self.registers.a >> 7;
        let shifted: u8 = (self.registers.a & 0x7F) << 1;
        let c: u8 = if self.registers.f.c { 1 } else { 0 };
        self.registers.f.h = false;
        self.registers.f.z = false;
        self.registers.f.s = false;
        self.registers.f.c = bit > 0;
        self.registers.a = shifted | c;
    }

    fn rra(&mut self) {
        let bit: u8 = self.registers.a & 0x01;
        let shifted: u8 = (self.registers.a & 0xFE) >> 1;
        let c: u8 = if self.registers.f.c { 1 } else { 0 };
        self.registers.f.h = false;
        self.registers.f.z = false;
        self.registers.f.s = false;
        self.registers.f.c = bit > 0;
        self.registers.a = shifted | (c << 7);
    }

    fn daa(&mut self) {
        let n = self.registers.f.s;
        let mut adjustment: u8 = 0;

        if n {
            if self.registers.f.h {
                adjustment |= 0x6;
            }
            if self.registers.f.c {
                adjustment |= 0x60;
                self.registers.f.c = false;
            }
            self.registers.a = self.registers.a.wrapping_sub(adjustment);
        } else {
            if self.registers.f.h || (self.registers.a & 0xF) > 0x9 {
                adjustment |= 0x6;
            }
            if self.registers.f.c || self.registers.a > 0x99 {
                adjustment |= 0x60;
                self.registers.f.c = true;
            }

            self.registers.a = self.registers.a.wrapping_add(adjustment);
        }

        // If the adjustment was
        self.registers.f.z = adjustment == 0;
        self.registers.f.h = false;
    }

    fn cpl(&mut self) {
        self.registers.a = !self.registers.a;
        self.registers.f.s = true;
        self.registers.f.h = true;
    }

    fn scf(&mut self) {
        self.registers.f.c = true;
        self.registers.f.s = false;
        self.registers.f.h = false;
    }

    fn ccf(&mut self) {
        self.registers.f.c = !self.registers.f.c;
        self.registers.f.s = false;
        self.registers.f.h = false;
    }

    fn jrn8(&mut self) {
        self.jump_relative();
    }

    fn jrcondn8(&mut self, cond: u8) {
        match cond {
            0x0 => {
                if !self.registers.f.z {
                    self.jump_relative();
                }
            }
            0x1 => {
                if self.registers.f.z {
                    self.jump_relative();
                }
            }
            0x2 => {
                if !self.registers.f.c {
                    self.jump_relative();
                }
            }
            0x3 => {
                if self.registers.f.c {
                    self.jump_relative();
                }
            }
            _ => {
                panic!("cond: {cond} not valid, could not jump")
            }
        }
    }

    fn stop(&mut self) {
        // TODO: Implement STOP. This will require
    }

    fn jump_relative(&mut self) {
        let n8: i8 = self.fetch() as i8;
        self.registers.sp = self.registers.sp.wrapping_add_signed(n8.into());
    }
    // Begin Block 1 Helper Functions
}
