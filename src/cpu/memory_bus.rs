pub struct MemoryBus {
    memory: [u8; 0xFFFF],
}

impl MemoryBus {
    pub fn new() -> MemoryBus {
        MemoryBus {
            memory: [0x00; 0xFFFF],
        }
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        // Gameboy is little endian so read pc + 2 as most significant bit
        // and pc + 1 as least significant bit
        let least_significant_byte = self.read_byte(address) as u16;
        let most_significant_byte = self.read_byte(address + 1) as u16;
        (most_significant_byte << 8) | least_significant_byte
    }

    pub fn write_byte(&mut self, address: u16, new_value: u8) {
        self.memory[address as usize] = new_value;
    }
}
