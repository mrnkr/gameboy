use std::collections::HashMap;
use once_cell::sync::Lazy;

use super::cpu_impl::CPU;

pub struct InstructionEntry {
    pub opcode: u8,
    pub prefixed: bool,
    pub handler: fn(&mut CPU),
}

pub struct InstructionTable {
    pub prefixed: HashMap<u8, &'static InstructionEntry>,
    pub unprefixed: HashMap<u8, &'static InstructionEntry>,
}

inventory::collect!(InstructionEntry);

pub static INSTRUCTION_TABLE: Lazy<InstructionTable> = Lazy::new(|| {
    println!("Collecting instructions");
    let mut prefixed: HashMap<u8, &InstructionEntry> = HashMap::new();
    let mut unprefixed: HashMap<u8, &InstructionEntry> = HashMap::new();
    for entry in inventory::iter::<InstructionEntry> {
        println!("Opcode: {:#04x}, prefixed: {}", entry.opcode, entry.prefixed);
        if entry.prefixed {
            prefixed.insert(entry.opcode, entry);
        } else {
            unprefixed.insert(entry.opcode, entry);
        }
    }
    InstructionTable { prefixed, unprefixed }
});
