use instruction_macro::instruction;

use crate::cpu::{
    cpu_impl::CPU,
    instruction::InstructionEntry,
};

#[instruction(0x00)]
fn handle_nop(cpu: &mut CPU) {
    cpu.pc = cpu.pc.wrapping_add(1);
}
