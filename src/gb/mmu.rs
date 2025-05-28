// This is a draft version of the MMU. Obviously not the real thing,
// but we need to start somewhere.
pub struct MMU {
    pub memory: [u8; 65536]
}

impl MMU {
    pub fn new(memory: [u8; 65536]) -> MMU {
        MMU {
            memory,
        }
    }

    pub fn load_rom<'a>(&mut self, rom: impl Iterator<Item=&'a u8>) {
        let mut pos: usize = 0;
        for byte in rom {
            self.memory[pos] = *byte;
            pos += 1;
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        (self.memory[address as usize] as u16) << 8 |
            self.memory[(address + 1) as usize] as u16
    }
}

