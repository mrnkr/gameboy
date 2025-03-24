use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::cpu_impl::CPU;

#[derive(Clone, Copy, Debug)]
pub struct InstructionEntry {
    pub opcode: u8,
    pub prefixed: bool,
    pub handler: fn(&mut CPU),
}

#[derive(Debug)]
pub struct InstructionTable {
    pub prefixed: HashMap<u8, InstructionEntry>,
    pub unprefixed: HashMap<u8, InstructionEntry>,
}

inventory::collect!(InstructionEntry);

pub static INSTRUCTION_TABLE: Lazy<InstructionTable> = Lazy::new(|| {
    let mut prefixed: HashMap<u8, InstructionEntry> = HashMap::new();
    let mut unprefixed: HashMap<u8, InstructionEntry> = HashMap::new();
    for entry in inventory::iter::<InstructionEntry> {
        if entry.prefixed {
            prefixed.insert(entry.opcode, *entry);
        } else {
            unprefixed.insert(entry.opcode, *entry);
        }
    }
    InstructionTable { prefixed, unprefixed }
});
