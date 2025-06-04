// This is a draft version of the MMU. Obviously not the real thing,
// but we need to start somewhere.
#[derive(Debug)]
pub struct MemoryManagementUnit {
    memory: [u8; 65536],
}

/* May not use at all, but these will be the regions of memory.
pub enum Regions {
    Bank0,
    Bank1,
    VRAM,
    ExRam, // AKA External RAM from cartridge, switchable bank if any.
    WRAM,
    EchoRAM // Weird unused area of ram mirroring C000-DDFF
    OAM,
    IORegisters
    HighRAM,
    IERegister // Interrupt enable register
}
*/

impl MemoryManagementUnit {
    pub fn new() -> MemoryManagementUnit {
        MemoryManagementUnit {
            memory: [0; 65536],
        }
    }

    pub fn load_rom<'a>(&mut self, rom: impl Iterator<Item=&'a u8>) {
        let mut pos: usize = 0;
        for byte in rom {
            self.memory[pos] = *byte;
            pos += 1;
        }
    }

    /* Reads any address from the memory, regardless of where it belongs */
    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        (self.memory[address as usize] as u16) << 8 |
            self.memory[(address + 1) as usize] as u16
    }

    pub fn set_byte(&mut self, address: u16, val: u8) {
        self.memory[address as usize] = val;
    }
}

