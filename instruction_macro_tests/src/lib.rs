use std::collections::HashMap;
use once_cell::sync::Lazy;

use instruction_macro::instruction;

pub struct CPU {}

pub struct InstructionEntry {
    pub opcode: u8,
    pub prefixed: bool,
    pub handler: fn(&mut CPU) -> String,
}

pub struct InstructionTable {
    pub prefixed: HashMap<u8, &'static InstructionEntry>,
    pub unprefixed: HashMap<u8, &'static InstructionEntry>,
}

inventory::collect!(InstructionEntry);

pub static INSTRUCTION_TABLE: Lazy<InstructionTable> = Lazy::new(|| {
    let mut prefixed: HashMap<u8, &InstructionEntry> = HashMap::new();
    let mut unprefixed: HashMap<u8, &InstructionEntry> = HashMap::new();
    for entry in inventory::iter::<InstructionEntry> {
        if entry.prefixed {
            prefixed.insert(entry.opcode, entry);
        } else {
            unprefixed.insert(entry.opcode, entry);
        }
    }
    InstructionTable { prefixed, unprefixed }
});

#[instruction(0x00)]
fn nop(cpu: &CPU) -> String {
    "NOP".to_string()
}

#[instruction(0x80, "B")]
#[instruction(0x81, "C")]
fn add(cpu: &CPU, registry: &str) -> String {
    format!("ADD {}", registry)
}

#[instruction(0x80, "B", prefixed = true)]
#[instruction(0x81, "C", prefixed = true)]
fn res(cpu: &CPU, registry: &str) -> String {
    format!("RES {}", registry)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x00, "NOP")]
    #[case(0x80, "ADD B")]
    #[case(0x81, "ADD C")]
    fn should_register_unprefixed_instruction(
        #[case] opcode: u8,
        #[case] expected: &str,
    ) {
        let mut cpu = CPU {};
        let handler = INSTRUCTION_TABLE.unprefixed.get(&opcode).unwrap().handler;
        let result = handler(&mut cpu);
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(0x80, "RES B")]
    #[case(0x81, "RES C")]
    fn should_register_prefixed_instruction(
        #[case] opcode: u8,
        #[case] expected: &str,
    ) {
        let mut cpu = CPU {};
        let handler = INSTRUCTION_TABLE.prefixed.get(&opcode).unwrap().handler;
        let result = handler(&mut cpu);
        assert_eq!(result, expected);
    }
}
