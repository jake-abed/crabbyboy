#[derive(Debug)]
pub enum Instruction {
    BlockZero(BlockZeroInstruction),
    BlockOne(BlockOneInstruction),
    BlockTwo(BlockTwoInstruction),
    BlockThree(BlockThreeInstruction),
    Prefixed(PrefixedInstruction),
}

#[derive(Debug)]
pub enum BlockZeroInstruction {
    NOP,
    LD(u16),
    RLCA,
    RRCA,
    RLA,
    RRA,
    DAA,
    CPL,
    SCF,
    CCF,
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
            0x0F => Ok(Instruction::BlockZero(BlockZeroInstruction::RRCA)),
            0x31 => Ok(Instruction::BlockZero(BlockZeroInstruction::RRA)),
            _ => Err(InstructionError::NotFound)
        }
    }

    fn from_byte_one_block(byte: u8) -> Result<Instruction, InstructionError> {
        match byte {
            _ => Err(InstructionError::NotFound),
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
            _ => Err(InstructionError::NotFound),
        }
    }
}
