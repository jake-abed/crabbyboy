/*
 * This crate's job is to take in a byte and tidily organize it into
 * a nested Enum that the CPU can use to process the instructions.
 * All necesary values will be passed along (aside from N16 and N8, which the
 * CPU will have to grab from memory)
 */

#[derive(Debug)]
pub enum Instruction {
    Block0(B0Instruction),
    Block1(B1Instruction),
    Block2(B2Instruction),
    Block3(B3Instruction),
    Prefixed(PrefixedInstruction),
}

/* Likely the messiest part of this entire thing so far.
 * Block Zero is massive compared to the other ones and some of the names
 * have "collisions so to speak. Here's a little guide to the names:
 * R16: 16 byte register address of bc, de, hl, or the stack pointer.
 * b3: A bit index of 3-bits (stored as a u8)
 * n8: The next 8 bits.
 * n16: The next 16 bits.
 */
#[derive(Debug)]
pub enum B0Instruction {
    NOP,
    LDR16N16(u8),
    LDR16A(u8),
    LDA(u8),
    LDN16SP,
    INCR16(u8),
    DECR16(u8),
    ADDHL(u8),
    INCR8(u8),
    DECR8(u8),
    LDR8N8(u8),
    RLCA,
    RRCA,
    RLA,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
    JRN8,
    JRCONDN8(u8),
    STOP,
}

#[derive(Debug)]
pub enum B1Instruction {
    LD { dest: u8, source: u8 },
    HALT,
}

#[derive(Debug)]
pub enum B2Instruction {
    ADDA(u8),
    ADCA(u8),
    SUBA(u8),
    SBCA(u8),
    ANDA(u8,),
    XORA(u8),
    ORA(u8),
    CPA(u8),
}

#[derive(Debug)]
pub enum B3Instruction {
    ADDAN8,
    ADCAN8,
    SUBAN8,
    SBCAN8,
    ANDAN8,
    XORAN8,
    ORAN8,
    CPAN8,
    RETCOND(u8),
    RET,
    RETI,
    JPCONDN8(u8),
    JPN16,
    JPHL,
    CALLCONDN8(u8),
    CALLN16,
    RST(u8),
    POP(u8),
    PUSH(u8),
    LDHCA,
    LDHN8A,
    LDN16A,
    LDHAC,
    LDHAN8,
    LDAN16,
    ADDSPN8,
    LDHLSPN8,
    LDSPHL,
    DI,
    EI,
}

#[derive(Debug)]
pub enum PrefixedInstruction {
    RLC(u8),
    RRC(u8),
    RL(u8),
    RR(u8),
    SLA(u8),
    SRA(u8),
    SWAP(u8),
    SRL(u8),
    BIT { b3: u8, operand: u8 },
    RES { b3: u8, operand: u8 },
    SET { b3: u8, operand: u8 },
}

#[derive(Debug)]
pub enum InstructionError {
    NotFound,
    Invalid,
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Result<Instruction, InstructionError> {
        if prefixed {
            println!("prefixed");
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_cb(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Result<Instruction, InstructionError> {
        let block: u8 = byte >> 6;
        let operand: u8 = (byte & 0x0F) as u8;
        let b3: u8 = (byte >> 3) & 0b00111;
        match block {
            0b00 => Instruction::from_cb_zero_block(byte, operand),
            0b01 => Ok(Instruction::Prefixed(PrefixedInstruction::BIT {
                b3,
                operand,
            })),
            0b10 => Ok(Instruction::Prefixed(PrefixedInstruction::RES {
                b3,
                operand,
            })),
            0b11 => Ok(Instruction::Prefixed(PrefixedInstruction::SET {
                b3,
                operand,
            })),
            _ => Err(InstructionError::NotFound),
        }
    }

    fn from_cb_zero_block(byte: u8, operand: u8) -> Result<Instruction, InstructionError> {
        let u5: u8 = byte >> 3;
        
        match u5 {
            0x00 => Ok(Instruction::Prefixed(PrefixedInstruction::RLC(operand))),
            0x01 => Ok(Instruction::Prefixed(PrefixedInstruction::RRC(operand))),
            0x02 => Ok(Instruction::Prefixed(PrefixedInstruction::RL(operand))),
            0x03 => Ok(Instruction::Prefixed(PrefixedInstruction::RR(operand))),
            0x04 => Ok(Instruction::Prefixed(PrefixedInstruction::SLA(operand))),
            0x05 => Ok(Instruction::Prefixed(PrefixedInstruction::SRA(operand))),
            0x06 => Ok(Instruction::Prefixed(PrefixedInstruction::SWAP(operand))),
            0x07 => Ok(Instruction::Prefixed(PrefixedInstruction::SRL(operand))),
            _ => Err(InstructionError::NotFound),
        }
    }

    fn from_byte_not_cb(byte: u8) -> Result<Instruction, InstructionError> {
        let block = byte >> 6;
        match block {
            0b00 => Instruction::from_byte_zero_block(byte),
            0b01 => Instruction::from_byte_one_block(byte),
            0b10 => Instruction::from_byte_two_block(byte),
            0b11 => Instruction::from_byte_three_block(byte),
            _ => Err(InstructionError::NotFound),
        }
    }

    fn from_byte_zero_block(byte: u8) -> Result<Instruction, InstructionError> {
        match byte {
            0x00 => Ok(Instruction::Block0(B0Instruction::NOP)),
            0x07 => Ok(Instruction::Block0(B0Instruction::RLCA)),
            0x08 => Ok(Instruction::Block0(B0Instruction::LDN16SP)),
            0x0F => Ok(Instruction::Block0(B0Instruction::RRCA)),
            0x10 => Ok(Instruction::Block0(B0Instruction::STOP)),
            0x17 => Ok(Instruction::Block0(B0Instruction::RLA)),
            0x18 => Ok(Instruction::Block0(B0Instruction::JRN8)),
            0x1F => Ok(Instruction::Block0(B0Instruction::RRA)),
            0x27 => Ok(Instruction::Block0(B0Instruction::DAA)),
            0x2F => Ok(Instruction::Block0(B0Instruction::CPL)),
            0x37 => Ok(Instruction::Block0(B0Instruction::SCF)),
            0x3F => Ok(Instruction::Block0(B0Instruction::CCF)),
            _ => Instruction::from_byte_zero_block_u4(byte),
        }
    }

    fn from_byte_zero_block_u4(byte: u8) -> Result<Instruction, InstructionError> {
        let r16: u8 = (byte >> 4) & 0x3;
        match byte & 0x0F {
            0x1 => Ok(Instruction::Block0(B0Instruction::LDR16N16(r16))),
            0x2 => Ok(Instruction::Block0(B0Instruction::LDR16A(r16))),
            0x3 => Ok(Instruction::Block0(B0Instruction::INCR16(r16))),
            0x8 => Ok(Instruction::Block0(B0Instruction::LDN16SP)),
            0x9 => Ok(Instruction::Block0(B0Instruction::ADDHL(r16))),
            0xA => Ok(Instruction::Block0(B0Instruction::LDA(r16))),
            0xB => Ok(Instruction::Block0(B0Instruction::DECR16(r16))),
            _ => Instruction::from_byte_zero_block_u3(byte),
        }
    }

    fn from_byte_zero_block_u3(byte: u8) -> Result<Instruction, InstructionError> {
        let r8: u8 = (byte >> 3) & 0x7;
        match byte & 0x7 {
            0x0 => {
                let cond: u8 = r8 & 0x3;
                Ok(Instruction::Block0(B0Instruction::JRCONDN8(cond)))
            }
            0x4 => Ok(Instruction::Block0(B0Instruction::INCR8(r8))),
            0x5 => Ok(Instruction::Block0(B0Instruction::DECR8(r8))),
            0x6 => Ok(Instruction::Block0(B0Instruction::LDR8N8(r8))),
            _ => Err(InstructionError::NotFound)
        }
    }

    fn from_byte_one_block(byte: u8) -> Result<Instruction, InstructionError> {
        let dest: u8 = (byte >> 3) & 0x7;
        let source: u8 = byte & 0x7;
        match byte {
            0x76 => Ok(Instruction::Block1(B1Instruction::HALT)),
            _ => Ok(Instruction::Block1(B1Instruction::LD{ dest, source })),
        }
    }

    fn from_byte_two_block(byte: u8) -> Result<Instruction, InstructionError> {
        let first_five: u8 = byte >> 3;
        let operand: u8 = byte & 0x7;
        match first_five {
            0x10 => Ok(Instruction::Block2(B2Instruction::ADDA(operand))),
            0x11 => Ok(Instruction::Block2(B2Instruction::ADCA(operand))),
            0x12 => Ok(Instruction::Block2(B2Instruction::SUBA(operand))),
            0x13 => Ok(Instruction::Block2(B2Instruction::SBCA(operand))),
            0x14 => Ok(Instruction::Block2(B2Instruction::ANDA(operand))),
            0x15 => Ok(Instruction::Block2(B2Instruction::XORA(operand))),
            0x16 => Ok(Instruction::Block2(B2Instruction::ORA(operand))),
            0x17 => Ok(Instruction::Block2(B2Instruction::CPA(operand))),
            _ => Err(InstructionError::NotFound),
        }
    }
    fn from_byte_three_block(byte: u8) -> Result<Instruction, InstructionError> {
        match byte {
            0xC3 => Ok(Instruction::Block3(B3Instruction::JPN16)),
            0xC6 => Ok(Instruction::Block3(B3Instruction::ADDAN8)),
            0xC9 => Ok(Instruction::Block3(B3Instruction::RET)),
            0xCD => Ok(Instruction::Block3(B3Instruction::CALLN16)),
            0xCE => Ok(Instruction::Block3(B3Instruction::ADCAN8)),
            0xD6 => Ok(Instruction::Block3(B3Instruction::SUBAN8)),
            0xD9 => Ok(Instruction::Block3(B3Instruction::RETI)),
            0xDE => Ok(Instruction::Block3(B3Instruction::SBCAN8)),
            0xE0 => Ok(Instruction::Block3(B3Instruction::LDHN8A)),
            0xE2 => Ok(Instruction::Block3(B3Instruction::LDHCA)),
            0xE6 => Ok(Instruction::Block3(B3Instruction::ANDAN8)),
            0xE8 => Ok(Instruction::Block3(B3Instruction::ADDSPN8)),
            0xE9 => Ok(Instruction::Block3(B3Instruction::JPHL)),
            0xEA => Ok(Instruction::Block3(B3Instruction::LDN16A)),
            0xEE => Ok(Instruction::Block3(B3Instruction::XORAN8)),
            0xF0 => Ok(Instruction::Block3(B3Instruction::LDHAN8)),
            0xF2 => Ok(Instruction::Block3(B3Instruction::LDHAC)),
            0xF3 => Ok(Instruction::Block3(B3Instruction::DI)),
            0xF6 => Ok(Instruction::Block3(B3Instruction::ORAN8)),
            0xF8 => Ok(Instruction::Block3(B3Instruction::LDHLSPN8)),
            0xF9 => Ok(Instruction::Block3(B3Instruction::LDSPHL)),
            0xFA => Ok(Instruction::Block3(B3Instruction::LDAN16)),
            0xFB => Ok(Instruction::Block3(B3Instruction::EI)),
            0xFE => Ok(Instruction::Block3(B3Instruction::CPAN8)),
            _ => Instruction::from_byte_three_block_u3(byte),
        }
    }

    fn from_byte_three_block_u3(byte: u8) -> Result<Instruction, InstructionError> {
        let cond: u8 = (byte >> 3) & 0x3;
        let register: u8 = (byte >> 4) &0x3;
        let tgt3: u8 = (byte >> 3) & 0x7;

        let bottom_three: u8 = byte & 0x7;
        match bottom_three {
            0x0 => Ok(Instruction::Block3(B3Instruction::RETCOND(cond))),
            0x1 => Ok(Instruction::Block3(B3Instruction::POP(register))),
            0x2 => Ok(Instruction::Block3(B3Instruction::JPCONDN8(cond))),
            0x4 => Ok(Instruction::Block3(B3Instruction::CALLCONDN8(cond))),
            0x5 => Ok(Instruction::Block3(B3Instruction::PUSH(register))),
            0x7 => Ok(Instruction::Block3(B3Instruction::RST(tgt3))),
            _ => Err(InstructionError::NotFound),
        }
    }
}
