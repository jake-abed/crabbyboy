/*
 * This crate's job is to take in a byte and tidily organize it into
 * a nested Enum that the CPU can use to process the instructions.
 * All necesary values will be passed along (aside from N16 and N8, which the
 * CPU will have to grab from memory)
 */

#[derive(Debug)]
pub enum Instruction {
    BlockZero(BlockZeroInstruction),
    BlockOne(BlockOneInstruction),
    BlockTwo(BlockTwoInstruction),
    BlockThree(BlockThreeInstruction),
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
pub enum BlockZeroInstruction {
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
pub enum BlockOneInstruction {
    LD { dest: u8, source: u8 },
    HALT,
}

#[derive(Debug)]
pub enum BlockTwoInstruction {
    // Add instructions
}

#[derive(Debug)]
pub enum BlockThreeInstruction {
    CP,
    RST(u8)
}

#[derive(Debug)]
pub enum PrefixedInstruction {
    RLC(u8),
    RRC(u8),
    RL(u8),
    RR(u8),
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
            0x00 => Instruction::from_cb_zero_block(byte, operand),
            0x01 => Ok(Instruction::Prefixed(PrefixedInstruction::BIT {
                b3,
                operand,
            })),
            0x10 => Ok(Instruction::Prefixed(PrefixedInstruction::RES {
                b3,
                operand,
            })),
            0x11 => Ok(Instruction::Prefixed(PrefixedInstruction::SET {
                b3,
                operand,
            })),
            _ => Err(InstructionError::NotFound),
        }
    }

    fn from_cb_zero_block(byte: u8, operand: u8) -> Result<Instruction, InstructionError> {
        let prefix = byte >> 3;
        match prefix {
            0x000 => Ok(Instruction::Prefixed(PrefixedInstruction::RLC(operand))),
            0x001 => Ok(Instruction::Prefixed(PrefixedInstruction::RRC(operand))),
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
            0x00 => Ok(Instruction::BlockZero(BlockZeroInstruction::NOP)),
            0x07 => Ok(Instruction::BlockZero(BlockZeroInstruction::RLCA)),
            0x08 => Ok(Instruction::BlockZero(BlockZeroInstruction::LDN16SP)),
            0x0F => Ok(Instruction::BlockZero(BlockZeroInstruction::RRCA)),
            0x10 => Ok(Instruction::BlockZero(BlockZeroInstruction::STOP)),
            0x17 => Ok(Instruction::BlockZero(BlockZeroInstruction::RLA)),
            0x18 => Ok(Instruction::BlockZero(BlockZeroInstruction::JRN8)),
            0x27 => Ok(Instruction::BlockZero(BlockZeroInstruction::DAA)),
            0x2F => Ok(Instruction::BlockZero(BlockZeroInstruction::CPL)),
            0x37 => Ok(Instruction::BlockZero(BlockZeroInstruction::SCF)),
            0x3F => Ok(Instruction::BlockZero(BlockZeroInstruction::CCF)),
            0x31 => Ok(Instruction::BlockZero(BlockZeroInstruction::RRA)),
            _ => Instruction::from_byte_zero_block_u4(byte),
        }
    }

    fn from_byte_zero_block_u4(byte: u8) -> Result<Instruction, InstructionError> {
        let r16: u8 = (byte >> 4) & 0x3;
        match byte & 0x0F {
            0x1 => Ok(Instruction::BlockZero(BlockZeroInstruction::LDR16N16(r16))),
            0x2 => Ok(Instruction::BlockZero(BlockZeroInstruction::LDR16A(r16))),
            0x3 => Ok(Instruction::BlockZero(BlockZeroInstruction::INCR16(r16))),
            0x8 => Ok(Instruction::BlockZero(BlockZeroInstruction::LDN16SP)),
            0x9 => Ok(Instruction::BlockZero(BlockZeroInstruction::ADDHL(r16))),
            0xA => Ok(Instruction::BlockZero(BlockZeroInstruction::LDA(r16))),
            0xB => Ok(Instruction::BlockZero(BlockZeroInstruction::DECR16(r16))),
            _ => Instruction::from_byte_zero_block_u3(byte),
        }
    }

    fn from_byte_zero_block_u3(byte: u8) -> Result<Instruction, InstructionError> {
        let r8: u8 = (byte >> 3) & 0x7;
        match byte & 0x7 {
            0x0 => {
                let cond: u8 = r8 & 0x3;
                Ok(Instruction::BlockZero(BlockZeroInstruction::JRCONDN8(cond)))
            }
            0x3 => Ok(Instruction::BlockZero(BlockZeroInstruction::INCR8(r8))),
            0xB => Ok(Instruction::BlockZero(BlockZeroInstruction::DECR8(r8))),
            0x9 => Ok(Instruction::BlockZero(BlockZeroInstruction::LDR8N8(r8))),
            _ => Err(InstructionError::NotFound)
        }
    }

    fn from_byte_one_block(byte: u8) -> Result<Instruction, InstructionError> {
        let dest: u8 = (byte >> 3) & 0x7;
        let source: u8 = byte & 0x7;
        match byte {
            0x76 => Ok(Instruction::BlockOne(BlockOneInstruction::HALT)),
            _ => Ok(Instruction::BlockOne(BlockOneInstruction::LD{ dest, source })),
        }
    }

    fn from_byte_two_block(byte: u8) -> Result<Instruction, InstructionError> {
        match byte {
            _ => Err(InstructionError::NotFound),
        }
    }
    fn from_byte_three_block(byte: u8) -> Result<Instruction, InstructionError> {
        match byte {
            0xFE => Ok(Instruction::BlockThree(BlockThreeInstruction::CP)),
            0xFF => {
                let tgt3 = (byte >> 3) & 0b00111;
                Ok(Instruction::BlockThree(BlockThreeInstruction::RST(tgt3)))
            }
            _ => Err(InstructionError::NotFound),
        }
    }
}
