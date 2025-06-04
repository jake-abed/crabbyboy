#[derive(Debug)]
pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegisters,
    pub h: u8,
    pub l: u8,
    pub sp: u16,
    pub pc: u16,
}

#[repr(u8)]
pub enum R8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HL,
    A,
}

impl TryFrom<u8> for R8 {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(R8::B),
            1 => Ok(R8::C),
            2 => Ok(R8::D),
            3 => Ok(R8::E),
            4 => Ok(R8::H),
            5 => Ok(R8::L),
            6 => Ok(R8::HL),
            7 => Ok(R8::A),
            _ => Err(()),
        }
    }

}

#[repr(u8)]
pub enum R16 {
    BC,
    DE,
    HL,
    SP,
}

impl TryFrom<u8> for R16 {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(R16::BC),
            1 => Ok(R16::DE),
            2 => Ok(R16::HL),
            3 => Ok(R16::SP),
            _ => Err(()),
        }
    }

}

#[repr(u8)]
pub enum R16Stk {
    BC,
    DE,
    HL,
    AF,
}

#[repr(u8)]
pub enum R16Mem {
    BC,
    DE,
    HLI,
    HLD,
}

impl TryFrom<u8> for R16Mem {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(R16Mem::BC),
            1 => Ok(R16Mem::DE),
            2 => Ok(R16Mem::HLI),
            3 => Ok(R16Mem::HLD),
            _ => Err(()),
        }
    }

}

#[repr(u8)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
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
#[derive(Debug)]
pub struct FlagsRegisters {
    pub z: bool, // Zero
    pub s: bool, // Subtract
    pub h: bool, // Half Carry
    pub c: bool, // Carry
}

impl FlagsRegisters {
    fn new() -> FlagsRegisters {
        FlagsRegisters {
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

    pub fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = ((val & 0xFF00) >> 8) as u8;
        self.c = (val & 0x00FF) as u8;
    }

    pub fn de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = ((val & 0xFF00) >> 8) as u8;
        self.e = (val & 0x00FF) as u8;
    }

    pub fn hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = ((val & 0xFF00) >> 8) as u8;
        self.l = (val & 0x00FF) as u8;
    }

    pub fn advance_pc(&mut self) {
        self.pc += 1;
    }
}
