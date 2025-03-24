use super::{
    instruction::INSTRUCTION_TABLE,
    memory_bus::MemoryBus,
    registers::Registers,
};

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub bus: MemoryBus,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0,
            bus: MemoryBus::new(),
        }
    }

    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;

        let instruction_entry = if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
            INSTRUCTION_TABLE.prefixed.get(&instruction_byte)
        } else {
            INSTRUCTION_TABLE.unprefixed.get(&instruction_byte)
        };

        if let Some(entry) = instruction_entry {
            let handler = entry.handler;
            handler(self);
        } else {
            panic!("Unkown instruction found for: {:#04x}", instruction_byte);
        };
    }

    pub fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    pub fn read_next_word(&self) -> u16 {
        self.bus.read_word(self.pc + 1)
    }
}
