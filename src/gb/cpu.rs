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

    // Interrupt Master Enable - CPU register for whether or not interrupts enabled.
    ime: bool,
}

#[derive(Eq, PartialEq, Debug)]
#[repr(u8)]
enum Cycles {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: reg::Registers::new(),
            memory_bus: MMU::new(),
            end: false,
            ime: false,
        }
    }

    pub fn cycle(&mut self) {
        let mut byte = self.fetch();
        let prefixed: bool = byte == 0xCB;
        if prefixed {
            byte = self.fetch();
        }
        let pc = self.registers.pc;
        println!("cycling: {pc}");
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

    fn push_sp(&mut self, sp: u16) {
        let lsb: u8 = (sp >> 4) as u8;
        let msb: u8 = (sp & 0x0F) as u8;
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory_bus.set_byte(self.registers.sp, msb);
        self.registers.sp = self.registers.sp.wrapping_sub(1);
        self.memory_bus.set_byte(self.registers.sp, lsb);
    }

    /* Parent function to execute the an instruction. Filters down through
     * successive match cases to perform the expected instruction.
     */
    fn execute(&mut self, byte: u8, prefixed: bool) -> Cycles {
        let instruction_result = Instr::from_byte(byte, prefixed);
        match instruction_result {
            Ok(Instr::Block0(instruction)) => self.execute_block_zero(instruction),
            Ok(Instr::Block1(instruction)) => self.execute_block_one(instruction),
            Ok(Instr::Block2(instruction)) => self.execute_block_two(instruction),
            Ok(Instr::Block3(instruction)) => self.execute_block_three(instruction),
            Ok(Instr::Prefixed(instruction)) => self.execute_prefixed(instruction),
            Err(error) => panic!("{error:?}"),
        }
    }

    /* Filtering functions for Blocks 0, 1, 2, 3, and prefixed to pass down
     * responsibility to individual helper functions for the specific opcode
     * families.
     */
    fn execute_block_zero(&mut self, instruction: B0Inst) -> Cycles {
        match instruction {
            B0Inst::NOP => self.nop(),
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

    fn execute_block_one(&mut self, instruction: B1Inst) -> Cycles {
        match instruction {
            B1Inst::LD { dest, source } => self.ld(dest, source),
            B1Inst::HALT => self.halt(),
        }
    }

    fn execute_block_two(&mut self, instruction: B2Inst) -> Cycles {
        match instruction {
            B2Inst::ADD(val) => self.add(val),
            B2Inst::ADC(val) => self.adc(val),
            B2Inst::SUB(val) => self.sub(val),
            B2Inst::SBC(val) => self.sbc(val),
            B2Inst::AND(val) => self.and(val),
            B2Inst::XOR(val) => self.xor(val),
            B2Inst::OR(val) => self.or(val),
            B2Inst::CP(val) => self.cp(val),
        }
    }

    fn execute_block_three(&mut self, instruction: B3Inst) -> Cycles {
        match instruction {
            B3Inst::ADDN8 => self.add_n8(),
            B3Inst::ADCN8 => self.adc_n8(),
            B3Inst::SUBN8 => self.sub_n8(),
            B3Inst::SBCN8 => self.sbc_n8(),
            B3Inst::ANDN8 => self.and_n8(),
            B3Inst::XORN8 => self.xor_n8(),
            B3Inst::ORN8 => self.or_n8(),
            B3Inst::CPN8 => self.cp_n8(),
            B3Inst::RETCOND(cond) => self.retcond(cond),
            B3Inst::RET => self.ret(),
            B3Inst::RETI => self.reti(),
            B3Inst::JPCOND(cond) => self.jpcond(cond),
            B3Inst::JPN16 => self.jp_n16(),
            B3Inst::JPHL => self.jp_hl(),
            B3Inst::CALLCONDN16(cond) => self.call_cond_n16(cond),
            B3Inst::CALLN16 => self.call_n16(),
            _ => {
                println!("IDK, {instruction:?}");
                Cycles::One
            }
        }
    }

    fn execute_prefixed(&mut self, instruction: PrefixedInst) -> Cycles {
        match instruction {
            PrefixedInst::RLC(operand) => {
                println!("Got RLC(operand) - {operand}");
                Cycles::One
            }
            _ => {
                println!("IDK, {instruction:?}");
                Cycles::One
            }
        }
    }

    // Begin Block 0 Helper Functions

    fn nop(&mut self) -> Cycles {
        Cycles::One
    }

    fn ldr16n16(&mut self, dest: u8) -> Cycles {
        let n16: u16 = self.fetch_n16();

        println!("ldr16n16 - dest: {dest} - n16: {n16}");
        match reg::R16::try_from(dest) {
            Ok(reg::R16::BC) => self.registers.set_bc(n16),
            Ok(reg::R16::DE) => self.registers.set_de(n16),
            Ok(reg::R16::HL) => self.registers.set_hl(n16),
            Ok(reg::R16::SP) => self.registers.sp = n16,
            Err(err) => panic!("{err:?}"),
        }

        Cycles::Three
    }

    fn ldr16(&mut self, dest: u8) -> Cycles {
        println!("ldr16 - dest: {dest}");
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
        };

        Cycles::Two
    }

    fn lda(&mut self, source: u8) -> Cycles {
        println!("lda - source: {source}");
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
        };

        Cycles::Two
    }

    fn ldn16sp(&mut self) -> Cycles {
        println!("ldn16sp");
        let n16 = self.fetch_n16();
        let sp_high: u8 = (self.registers.sp & 0x00FF) as u8;
        let sp_low: u8 = (self.registers.sp >> 8) as u8;
        self.memory_bus.set_byte(n16, sp_high);
        self.memory_bus.set_byte(n16.wrapping_add(1), sp_low);

        Cycles::Five
    }

    fn incr16(&mut self, operand: u8) -> Cycles {
        println!("incr16 - operand: {operand}");
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

        Cycles::Two
    }

    fn decr16(&mut self, operand: u8) -> Cycles {
        println!("decr16 - operand: {operand}");
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

        Cycles::Two
    }

    fn addhl(&mut self, operand: u8) -> Cycles {
        println!("addhl - operand: {operand}");

        let old_hl = self.registers.hl();
        let mut register_val: u16 = 0;

        match reg::R16::try_from(operand) {
            Ok(reg::R16::BC) => register_val = self.registers.bc(),
            Ok(reg::R16::DE) => register_val = self.registers.de(),
            Ok(reg::R16::HL) => register_val = self.registers.hl(),
            Ok(reg::R16::SP) => register_val = self.registers.sp,
            Err(err) => {
                panic!(
                    "operand={operand} not r16 - failed to set register_val={register_val} - {err:?}"
                );
            }
        }

        let (res, carry) = old_hl.overflowing_add(register_val);
        self.registers.f.c = carry;
        self.registers.f.s = false;

        let mask: u16 = 0b1111_1111_1111;
        self.registers.f.h = (register_val & mask) + (old_hl & mask) > mask;
        self.registers.set_hl(res);

        Cycles::Two
    }

    fn incr8(&mut self, operand: u8) -> Cycles {
        println!("incr8 - operand: {operand}");
        let reg_val: u8;
        let res: u8;
        match reg::R8::try_from(operand) {
            Ok(reg::R8::A) => {
                reg_val = self.registers.a;
                res = self.registers.a.wrapping_add(1);
                self.registers.a = res;
            }
            Ok(reg::R8::B) => {
                reg_val = self.registers.b;
                res = self.registers.b.wrapping_add(1);
                self.registers.b = res;
            }
            Ok(reg::R8::C) => {
                reg_val = self.registers.c;
                res = self.registers.c.wrapping_add(1);
                self.registers.c = res;
            }
            Ok(reg::R8::D) => {
                reg_val = self.registers.d;
                res = self.registers.d.wrapping_add(1);
                self.registers.c = res;
            }
            Ok(reg::R8::E) => {
                reg_val = self.registers.h;
                res = self.registers.e.wrapping_add(1);
                self.registers.e = res;
            }
            Ok(reg::R8::H) => {
                reg_val = self.registers.h;
                res = self.registers.h.wrapping_add(1);
                self.registers.h = res;
            }
            Ok(reg::R8::L) => {
                reg_val = self.registers.l;
                res = self.registers.l.wrapping_add(1);
                self.registers.h = res;
            }
            Ok(reg::R8::HL) => {
                let hl_val = self.memory_bus.read_byte(self.registers.hl());
                res = hl_val.wrapping_add(1);
                reg_val = hl_val;
                self.memory_bus.set_byte(self.registers.hl(), res);
            }
            Err(err) => panic!("{err:?}"),
        }

        self.registers.f.z = res == 0;
        self.registers.f.s = false;
        // Slightly esoteric but fairly smart way to do this from
        // https://www.reddit.com/r/EmuDev/comments/692n59/gb_questions_about_halfcarry_and_best/
        self.registers.f.h = ((reg_val ^ 1 ^ res) & 0x10) != 0;

        Cycles::One
    }

    fn decr8(&mut self, operand: u8) -> Cycles {
        let res: u8;
        let register_val: u8;
        println!("decr8 - operand: {operand}");
        match reg::R8::try_from(operand) {
            Ok(reg::R8::A) => {
                register_val = self.registers.a;
                res = register_val.wrapping_sub(1);
                self.registers.a = res;
            }
            Ok(reg::R8::B) => {
                register_val = self.registers.b;
                res = register_val.wrapping_sub(1);
                self.registers.b = res;
            }
            Ok(reg::R8::C) => {
                register_val = self.registers.c;
                res = register_val.wrapping_sub(1);
                self.registers.c = res;
            }
            Ok(reg::R8::D) => {
                register_val = self.registers.d;
                res = register_val.wrapping_sub(1);
                self.registers.c = res;
            }
            Ok(reg::R8::E) => {
                register_val = self.registers.e;
                res = register_val.wrapping_sub(1);
                self.registers.e = res;
            }
            Ok(reg::R8::H) => {
                register_val = self.registers.h;
                res = register_val.wrapping_sub(1);
                self.registers.h = res;
            }
            Ok(reg::R8::L) => {
                register_val = self.registers.l;
                res = register_val.wrapping_sub(1);
                self.registers.h = res;
            }
            Ok(reg::R8::HL) => {
                register_val = self.memory_bus.read_byte(self.registers.hl());
                res = register_val.wrapping_sub(1);
                self.memory_bus.set_byte(self.registers.hl(), res);
            }
            Err(err) => panic!("{err:?}"),
        }

        self.registers.f.z = res == 0;
        self.registers.f.s = true;
        self.registers.f.h = register_val & 0xF == 0;

        Cycles::One
    }

    fn ldr8n8(&mut self, dest: u8) -> Cycles {
        let n8: u8 = self.fetch();
        println!("ldr8n8 - dest: {dest}");
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

        Cycles::Two
    }

    fn rlca(&mut self) -> Cycles {
        println!("rlca");
        let bit: u8 = self.registers.a >> 7;
        let shifted: u8 = (self.registers.a & 0x7F) << 1;

        // set flags
        self.registers.f.h = false;
        self.registers.f.z = false;
        self.registers.f.s = false;
        self.registers.f.c = bit > 0;

        self.registers.a = shifted | bit;

        Cycles::One
    }

    fn rrca(&mut self) -> Cycles {
        println!("rrca");
        let bit: u8 = self.registers.a & 0x01;
        let shifted: u8 = (self.registers.a & 0xFE) >> 1;

        // set flags
        self.registers.f.h = false;
        self.registers.f.z = false;
        self.registers.f.s = false;
        self.registers.f.c = bit > 0;

        self.registers.a = shifted | (bit << 7);

        Cycles::One
    }

    fn rla(&mut self) -> Cycles {
        println!("rla");
        let bit: u8 = self.registers.a >> 7;
        let shifted: u8 = (self.registers.a & 0x7F) << 1;
        let c: u8 = if self.registers.f.c { 1 } else { 0 };

        // set flags
        self.registers.f.h = false;
        self.registers.f.z = false;
        self.registers.f.s = false;
        self.registers.f.c = bit > 0;

        self.registers.a = shifted | c;

        Cycles::One
    }

    fn rra(&mut self) -> Cycles {
        println!("rra");
        let bit: u8 = self.registers.a & 0x01;
        let shifted: u8 = (self.registers.a & 0xFE) >> 1;
        let c: u8 = if self.registers.f.c { 1 } else { 0 };

        // set flags
        self.registers.f.h = false;
        self.registers.f.z = false;
        self.registers.f.s = false;
        self.registers.f.c = bit > 0;

        self.registers.a = shifted | (c << 7);

        Cycles::One
    }

    // This one should be good, but dear god, this needs to be tested.
    fn daa(&mut self) -> Cycles {
        println!("daa");
        let sub_flag = self.registers.f.s;
        let mut adjustment: u8 = 0;

        if sub_flag {
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

        // Final universal adjustments to the zero and halfcarry flags
        self.registers.f.z = adjustment == 0;
        self.registers.f.h = false;

        Cycles::One
    }

    fn cpl(&mut self) -> Cycles {
        println!("cpl");
        self.registers.a = !self.registers.a;

        // set registers
        self.registers.f.s = true;
        self.registers.f.h = true;

        Cycles::One
    }

    // Weird instruction, justs set the carry flag, turns off sub and half.
    fn scf(&mut self) -> Cycles {
        println!("scf");
        self.registers.f.c = true;
        self.registers.f.s = false;
        self.registers.f.h = false;

        Cycles::One
    }

    // Same as above, but flips the carry flag.
    fn ccf(&mut self) -> Cycles {
        println!("ccf");
        self.registers.f.c = !self.registers.f.c;
        self.registers.f.s = false;
        self.registers.f.h = false;

        Cycles::One
    }

    fn jrn8(&mut self) -> Cycles {
        println!("jrn8");
        self.jump_relative();

        Cycles::Three
    }

    fn jrcondn8(&mut self, cond: u8) -> Cycles {
        println!("jrcondn8 - cond: {cond}");

        let cond_met = self.cond_met(cond);
        
        if cond_met {
            self.jump_relative();
            Cycles::Three
        } else {
            Cycles::Two
        }
    }

    fn stop(&mut self) -> Cycles {
        // TODO: Implement STOP. This will require
        Cycles::Zero
    }

    fn jump_relative(&mut self) {
        let n8: i8 = self.fetch() as i8;
        self.registers.sp = self.registers.sp.wrapping_add_signed(n8.into());
    }

    // Begin Block 1 Helper Functions

    // Copy from source register to destination register.
    fn ld(&mut self, dest: u8, source: u8) -> Cycles {
        println!("ld - dest: {dest} - source: {source}");
        let source_val: u8 = match reg::R8::try_from(source) {
            Ok(reg::R8::A) => self.registers.a,
            Ok(reg::R8::B) => self.registers.b,
            Ok(reg::R8::C) => self.registers.c,
            Ok(reg::R8::D) => self.registers.d,
            Ok(reg::R8::E) => self.registers.e,
            Ok(reg::R8::H) => self.registers.h,
            Ok(reg::R8::L) => self.registers.l,
            Ok(reg::R8::HL) => self.memory_bus.read_byte(self.registers.hl()),
            Err(err) => panic!("{err:?}"),
        };

        match reg::R8::try_from(dest) {
            Ok(reg::R8::A) => self.registers.a = source_val,
            Ok(reg::R8::B) => self.registers.b = source_val,
            Ok(reg::R8::C) => self.registers.c = source_val,
            Ok(reg::R8::D) => self.registers.d = source_val,
            Ok(reg::R8::E) => self.registers.e = source_val,
            Ok(reg::R8::H) => self.registers.h = source_val,
            Ok(reg::R8::L) => self.registers.l = source_val,
            Ok(reg::R8::HL) => {
                self.memory_bus.set_byte(self.registers.hl(), source_val);
            }
            Err(err) => panic!("{err:?}"),
        }

        Cycles::One
    }

    // To Do: Implement halt and IME flag. Complex instruction.
    fn halt(&mut self) -> Cycles {
        println!("Halt");

        Cycles::One
    }

    // Begin Block 2 Helper Functions

    fn add(&mut self, register: u8) -> Cycles {
        let reg_val = self.get_register_val(register);
        println!("add - val: {reg_val}");

        self.add_to_a(reg_val);

        Cycles::One
    }

    fn adc(&mut self, register: u8) -> Cycles {
        let reg_val = self.get_register_val(register);
        println!("adc - val: {reg_val}");

        self.adc_to_a(reg_val);
        // This should possibly be switched for a more terse binary evaluation.

        Cycles::One
    }

    fn sub(&mut self, register: u8) -> Cycles {
        let reg_val = self.get_register_val(register);
        println!("sub - val: {reg_val}");

        self.sub_to_a(reg_val);

        Cycles::One
    }

    fn sbc(&mut self, register: u8) -> Cycles {
        let reg_val = self.get_register_val(register);
        println!("sbc - val: {reg_val}");

        self.sbc_to_a(reg_val);

        Cycles::One
    }

    fn and(&mut self, register: u8) -> Cycles {
        let reg_val = self.get_register_val(register);
        println!("and - val: {reg_val}");

        self.and_to_a(reg_val);

        Cycles::One
    }

    fn xor(&mut self, register: u8) -> Cycles {
        let reg_val = self.get_register_val(register);
        println!("xor - val: {reg_val}");

        self.xor_to_a(reg_val);

        Cycles::One
    }

    fn or(&mut self, register: u8) -> Cycles {
        let reg_val = self.get_register_val(register);
        println!("or - val: {reg_val}");

        self.or_to_a(reg_val);

        Cycles::One
    }

    fn cp(&mut self, register: u8) -> Cycles {
        let reg_val = self.get_register_val(register);
        println!("cp - val: {reg_val}");

        self.cp_against_a(reg_val);

        Cycles::One
    }

    fn add_n8(&mut self) -> Cycles {
        let n8: u8 = self.fetch();

        self.add_to_a(n8);

        Cycles::Two
    }

    fn adc_n8(&mut self) -> Cycles {
        let n8: u8 = self.fetch();

        self.adc_to_a(n8);

        Cycles::Two
    }

    fn sub_n8(&mut self) -> Cycles {
        let n8: u8 = self.fetch();

        self.sub_to_a(n8);

        Cycles::Two
    }

    fn sbc_n8(&mut self) -> Cycles {
        let n8: u8 = self.fetch();

        self.sbc_to_a(n8);

        Cycles::Two
    }

    fn and_n8(&mut self) -> Cycles {
        let n8 = self.fetch();

        self.and_to_a(n8);

        Cycles::Two
    }

    fn xor_n8(&mut self) -> Cycles {
        let n8 = self.fetch();

        self.xor_to_a(n8);

        Cycles::Two
    }

    fn or_n8(&mut self) -> Cycles {
        let n8 = self.fetch();

        self.or_to_a(n8);

        Cycles::Two
    }

    fn cp_n8(&mut self) -> Cycles {
        let n8 = self.fetch();

        self.cp_against_a(n8);

        Cycles::Two
    }

    fn retcond(&mut self, cond: u8) -> Cycles {
        println!("recond - cond:{cond}");

        if self.cond_met(cond) {
            self.do_ret();
            Cycles::Five
        } else {
            Cycles::Two
        }
    }

    fn ret(&mut self) -> Cycles {
        println!("ret");
        self.do_ret();
        Cycles::Four
    }

    fn reti(&mut self) -> Cycles {
        println!("reti");
        self.do_ret();
        self.ime = true;

        Cycles::Four
    }

    fn jpcond(&mut self, cond: u8) -> Cycles {
        println!("jpcond: {cond}");
        let n16 = self.fetch_n16();

        if self.cond_met(cond) {
            self.do_jp(n16);
            Cycles::Four
        } else {
            Cycles::Three
        }
    }

    fn jp_n16(&mut self) -> Cycles {
        println!("jp_n16");

        let n16 = self.fetch_n16();
        self.do_jp(n16);
        Cycles::Four
    }

    fn jp_hl(&mut self) -> Cycles {
        println!("jp_hl");

        let hl = self.registers.hl();
        self.do_jp(hl);
        Cycles::One
    }

    fn call_cond_n16(&mut self, cond: u8) -> Cycles {
        println!("call_cond_n16 - cond: {cond}");
        let n16 = self.fetch_n16();

        let cond_met = self.cond_met(cond);
        if cond_met {
            let curr_pc= self.registers.pc;
            self.push_sp(curr_pc);
            self.registers.pc = n16;
            Cycles::Six
        } else {
            Cycles::Three
        }
    }

    fn call_n16(&mut self) -> Cycles {
        println!("call_n16");
        let n16 = self.fetch_n16();
        let curr_pc = self.registers.pc;

        self.push_sp(curr_pc);
        self.registers.pc = n16;
        println!("{self:?}");
        Cycles::Six
    }

    // General Helper Functions

    fn get_register_val(&self, register: u8) -> u8 {
        match reg::R8::try_from(register) {
            Ok(reg::R8::A) => self.registers.a,
            Ok(reg::R8::B) => self.registers.b,
            Ok(reg::R8::C) => self.registers.c,
            Ok(reg::R8::D) => self.registers.d,
            Ok(reg::R8::E) => self.registers.e,
            Ok(reg::R8::H) => self.registers.h,
            Ok(reg::R8::L) => self.registers.l,
            Ok(reg::R8::HL) => self.memory_bus.read_byte(self.registers.hl()),
            Err(err) => panic!("invalid register val - {err:?}"),
        }
    }

    fn add_to_a(&mut self, operand: u8) {
        // This should possibly be switched for a more terse binary evaluation.
        let (_, overflow) = self.registers.a.overflowing_add(operand);
        let res: u8 = self.registers.a.wrapping_add(operand);

        self.registers.f.z = if res == 0 { true } else { false };
        self.registers.f.s = false;
        self.registers.f.h = half_carry(operand, self.registers.a, false);
        self.registers.f.c = overflow;

        self.registers.a = res;
    }

    fn adc_to_a(&mut self, operand: u8) {
        let (_, overflow) = self.registers.a.overflowing_add(operand);
        let carry: u8 = if self.registers.f.c { 1 } else { 0 };
        let res: u8 = self.registers.a.wrapping_add(operand).wrapping_add(carry);

        self.registers.f.z = if res == 0 { true } else { false };
        self.registers.f.s = false;
        self.registers.f.h = half_carry(self.registers.a, operand, true);
        self.registers.f.c = overflow;

        self.registers.a = res;
    }

    fn sub_to_a(&mut self, operand: u8) {
        let res: u8 = self.registers.a.wrapping_sub(operand);

        self.registers.f.z = if res == 0 { true } else { false };
        self.registers.f.s = true;
        let hc = half_carry_sub(self.registers.a, operand, false);
        self.registers.f.h = hc;
        self.registers.f.c = operand > self.registers.a;

        self.registers.a = res;
    }

    fn sbc_to_a(&mut self, operand: u8) {
        let a_val: u8 = self.registers.a;
        let carry_val = if self.registers.f.c { 1 } else { 0 };
        let res: u8 = a_val.wrapping_sub(operand).wrapping_sub(carry_val);

        self.registers.f.z = res == 0;
        self.registers.f.s = true;
        let hc = half_carry_sub(a_val, operand, self.registers.f.c);
        self.registers.f.h = hc;
        self.registers.f.c = operand.wrapping_add(carry_val) > a_val;

        self.registers.a = res;
    }

    fn and_to_a(&mut self, operand: u8) {
        let new_val = self.registers.a & operand;

        self.registers.f.z = new_val == 0;
        self.registers.f.s = false;
        self.registers.f.h = true;
        self.registers.f.c = false;

        self.registers.a = new_val;
    }

    fn xor_to_a(&mut self, operand: u8) {
        let new_val = self.registers.a ^ operand;

        self.registers.f.z = new_val == 0;
        self.registers.f.s = false;
        self.registers.f.h = false;
        self.registers.f.c = false;

        self.registers.a = new_val;
    }

    fn or_to_a(&mut self, operand: u8) {
        let new_val = self.registers.a | operand;

        self.registers.f.z = new_val == 0;
        self.registers.f.s = false;
        self.registers.f.h = false;
        self.registers.f.c = false;

        self.registers.a = new_val;
    }

    fn cp_against_a(&mut self, operand: u8) {
        let res: u8 = self.registers.a.wrapping_sub(operand);

        self.registers.f.z = if res == 0 { true } else { false };
        self.registers.f.s = true;
        let hc = half_carry_sub(self.registers.a, operand, false);
        self.registers.f.h = hc;
        self.registers.f.c = operand > self.registers.a;
    }

    fn cond_met(&mut self, cond: u8) -> bool {
        let parsed_cond: reg::Cond = cond.try_into().unwrap();
        let mut cond_met: bool = false;

        match parsed_cond {
            reg::Cond::Z => {
                if self.registers.f.z {
                    cond_met = true
                }
            }
            reg::Cond::NZ => {
                if !self.registers.f.z {
                    cond_met = true
                }
            }
            reg::Cond::C => {
                if self.registers.f.c {
                    cond_met = true
                }
            }
            reg::Cond::NC => {
                if !self.registers.f.c {
                    cond_met = true
                }
            }
        }

        cond_met
    }

    fn do_ret(&mut self) {
        let lsb = self.memory_bus.read_byte(self.registers.sp);
        self.registers.sp += 1;
        let msb = self.memory_bus.read_byte(self.registers.sp);
        self.registers.sp += 1;

        let new_pc: u16 = ((msb as u16) << 8) & (lsb as u16);
        self.registers.pc = new_pc;
    }

    fn do_jp(&mut self, target: u16) {
        self.registers.pc = target;
    }
}

fn half_carry(a: u8, b: u8, carry: bool) -> bool {
    if carry {
        ((a & 0xF) + (b & 0xF) + (1 & 0xF)) & 0x10 != 0
    } else {
        ((a & 0xF) + (b & 0xF)) & 0x10 != 0
    }
}

fn half_carry_sub(a: u8, b: u8, carry: bool) -> bool {
    if carry {
        ((a & 0xF).wrapping_sub(b & 0xF).wrapping_sub(1 & 0xF)) & 0x10 != 0
    } else {
        ((a & 0xF).wrapping_sub(b & 0xF)) & 0x10 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cpu() {
        let cpu = CPU::new();
        assert_eq!(cpu.end, false);
        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.b, 0);
        assert_eq!(cpu.registers.c, 0);
        assert_eq!(cpu.registers.d, 0);
        assert_eq!(cpu.registers.e, 0);
        assert_eq!(cpu.registers.h, 0);
        assert_eq!(cpu.registers.l, 0);
        assert_eq!(cpu.registers.f.h, false);
        assert_eq!(cpu.registers.f.c, false);
        assert_eq!(cpu.registers.f.z, false);
        assert_eq!(cpu.registers.f.s, false);
    }

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        cpu.registers.a = 1;
        cpu.registers.b = 10;

        let register_int = reg::R8::B.try_into().unwrap();
        let cycles = cpu.add(register_int);

        assert_eq!(cpu.registers.a, 11);
        assert_eq!(cycles, Cycles::One);
    }

    #[test]
    fn test_adc() {
        let mut cpu = CPU::new();
        cpu.registers.a = 10;
        cpu.registers.d = 1;
        cpu.registers.f.c = true;

        let reg_int = reg::R8::D.try_into().unwrap();

        let cycles = cpu.adc(reg_int);

        assert_eq!(cpu.registers.a, 12);
        assert_eq!(cycles, Cycles::One);
    }

    #[test]
    fn test_sub() {
        let mut cpu = CPU::new();
        cpu.registers.a = 17;
        cpu.registers.c = 1;
        cpu.registers.d = 17;

        let reg_int = reg::R8::C.try_into().unwrap();
        let cycles = cpu.sub(reg_int);

        assert_eq!(cpu.registers.a, 16);
        assert_eq!(cpu.registers.f.h, false);
        assert_eq!(cpu.registers.f.z, false);
        assert_eq!(cycles, Cycles::One);

        cpu.sub(reg::R8::D.try_into().unwrap());

        assert_eq!(cpu.registers.a, 255);
        assert_eq!(cpu.registers.f.h, true);
        assert_eq!(cpu.registers.f.c, true);
    }

    #[test]
    fn test_sbc() {
        let mut cpu = CPU::new();
        cpu.registers.a = 10;
        cpu.registers.h = 1;
        cpu.registers.f.c = true;
        cpu.registers.l = 9;

        let reg_int = reg::R8::H.try_into().unwrap();
        let cycles = cpu.sbc(reg_int);

        assert_eq!(cpu.registers.a, 8);
        assert_eq!(cycles, Cycles::One);

        cpu.sbc(reg::R8::L.try_into().unwrap());

        assert_eq!(cpu.registers.a, 255);
    }

    #[test]
    fn test_and() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b1111;
        cpu.registers.b = 0b0101;
        cpu.registers.c = 0b0000;

        let reg_int = reg::R8::B.try_into().unwrap();
        let cycles = cpu.and(reg_int);

        assert_eq!(cpu.registers.a, 0b0101);
        assert_eq!(cycles, Cycles::One);

        cpu.and(reg::R8::C.try_into().unwrap());

        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.f.z, true);
    }

    #[test]
    fn test_xor() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b0101;
        cpu.registers.e = 0b0100;
        cpu.registers.h = 0b0001;

        let reg_int = reg::R8::E.try_into().unwrap();
        let cycles = cpu.xor(reg_int);

        assert_eq!(cpu.registers.a, 0b0001);
        assert_eq!(cycles, Cycles::One);

        cpu.xor(reg::R8::H.try_into().unwrap());

        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.f.z, true);
    }

    #[test]
    fn test_or() {
        let mut cpu = CPU::new();
        cpu.registers.a = 0b0000;
        cpu.registers.b = 0b0000;
        cpu.registers.c = 0b0101;

        let reg_int = reg::R8::B.try_into().unwrap();
        let cycles = cpu.xor(reg_int);

        assert_eq!(cpu.registers.a, 0);
        assert_eq!(cpu.registers.f.z, true);
        assert_eq!(cycles, Cycles::One);

        cpu.xor(reg::R8::C.try_into().unwrap());

        assert_eq!(cpu.registers.a, 0b0101);
    }

    #[test]
    fn test_cp() {
        let mut cpu = CPU::new();
        cpu.registers.a = 17;
        cpu.registers.b = 1;
        cpu.registers.c = 2;
        cpu.registers.d = 18;

        let reg_int = reg::R8::B.try_into().unwrap();
        let cycles = cpu.cp(reg_int);

        assert_eq!(cpu.registers.a, 17);
        assert_eq!(cpu.registers.f.h, false);
        assert_eq!(cycles, Cycles::One);

        cpu.cp(reg::R8::C.try_into().unwrap());

        assert_eq!(cpu.registers.f.h, true);
        assert_eq!(cpu.registers.f.c, false);

        cpu.cp(reg::R8::D.try_into().unwrap());

        assert_eq!(cpu.registers.f.c, true);
        assert_eq!(cpu.registers.f.h, true);
    }
}
