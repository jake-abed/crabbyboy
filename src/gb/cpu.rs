// Test function to just make sure project structure works.
pub fn hello() -> String {
    "Hello!".to_string()
}

struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
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

impl std::convert::From<FlagsRegisters> for u8 {
    fn from(flag: FlagsRegisters) -> u8 {
        let z: u8 = if flag.z { 1 } else { 0 };
        let s: u8 = if flag.s { 1 } else { 0 };
        let h: u8 = if flag.h { 1 } else { 0 };
        let c: u8 = if flag.c { 1 } else { 0 };

        z << FlagBytePositions::Zero | s << FlagBytePositions::Subtract |
            h << FlagBytePositions::HalfCarry | c << FlagBytePositions::Carry
    }
}

impl std::convert::From<u8> for FlagsRegisters {
    fn from(byte: u8) -> Self {
        let z = ((byte >> FlagBytePositions::Zero) & 1) != 0;
        let s = ((byte >> FlagBytePositions::Subtract) & 1) != 0;
        let h = ((byte >> (FlagBytePositions::HalfCarry as u8)) & 1) != 0;
        let c = ((byte >> (FlagBytePositions::Carry as u8)) & 1) != 0;

        FlagsRegisters{
            z,
            s,
            h,
            c,
        }
    }
}

impl Registers {
    fn bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, val: u16) {
        self.b = ((val & 0xFF00) >> 8) as u8;
        self.c = (val & 0x00FF) as u8;
    }
}
