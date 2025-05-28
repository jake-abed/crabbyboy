use crate::gb::mmu::MMU;

pub struct CPU {
    registers: Registers,
    pub memory_bus: MMU,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            memory_bus: MMU::new([0; 65536]),
        }
    }
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: FlagsRegisters,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

enum FlagBytePositions {
    Zero = 7,
    Subtract = 6,
    HalfCarry = 5,
    Carry = 4,
}

impl std::ops::Shr<FlagBytePositions> for u8 {
    type Output = u8;

    fn shr(self, rhs: FlagBytePositions) -> u8 {
        self >> rhs as u8
    }
}

impl std::ops::Shl<FlagBytePositions> for u8 {
    type Output = u8;

    fn shl(self, lhs: FlagBytePositions) -> u8 {
        self << lhs as u8
    }
}

// 0x11110000 -> where the first four bits correspond to `zshc` in the struct.
struct FlagsRegisters {
    z: bool, // Zero
    s: bool, // Subtract
    h: bool, // Half Carry
    c: bool, // Carry
}

impl FlagsRegisters {
    fn new() -> FlagsRegisters {
        FlagsRegisters{
            z: false,
            s: false,
            h: false,
            c: false,
        }
    }
}

impl std::convert::From<FlagsRegisters> for u8 {
    fn from(flag: FlagsRegisters) -> u8 {
        let z: u8 = if flag.z { 1 } else { 0 };
        let s: u8 = if flag.s { 1 } else { 0 };
        let h: u8 = if flag.h { 1 } else { 0 };
        let c: u8 = if flag.c { 1 } else { 0 };

        z << FlagBytePositions::Zero
            | s << FlagBytePositions::Subtract
            | h << FlagBytePositions::HalfCarry
            | c << FlagBytePositions::Carry
    }
}

impl std::convert::From<u8> for FlagsRegisters {
    fn from(byte: u8) -> Self {
        let z = ((byte >> FlagBytePositions::Zero) & 1) != 0;
        let s = ((byte >> FlagBytePositions::Subtract) & 1) != 0;
        let h = ((byte >> (FlagBytePositions::HalfCarry as u8)) & 1) != 0;
        let c = ((byte >> (FlagBytePositions::Carry as u8)) & 1) != 0;

        FlagsRegisters { z, s, h, c }
    }
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x0000,
            b: 0x0000,
            c: 0x0000,
            d: 0x0000,
            e: 0x0000,
            f: FlagsRegisters::new(),
            h: 0x0000,
            l: 0x0000,
            sp: 0x00000000,
            pc: 0x00000000,
        }
    }

    fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, val: u16) {
        self.b = ((val & 0xFF00) >> 8) as u8;
        self.c = (val & 0x00FF) as u8;
    }

    fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    fn set_de(&mut self, val: u16) {
        self.d = ((val & 0xFF00) >> 8) as u8;
        self.e = (val & 0x00FF) as u8;
    }

    fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    fn set_hl(&mut self, val: u16) {
        self.h = ((val & 0xFF00) >> 8) as u8;
        self.l = (val & 0x00FF) as u8;
    }
}
