use super::add_hl::add_hl;
use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    instruction::InstructionEntry,
};

#[instruction(0xE8)]
fn handle_add_sp(cpu: &mut CPU) {
    let value = cpu.registers.get_sp();
    let (offset, pc_increment) =
        get_value_in_arithmetic_target(cpu, &ArithmeticTarget::D8);
    let new_value = add_hl(value, offset as u16, &mut cpu.registers.f);
    cpu.registers.f.zero = false;
    cpu.registers.set_sp(new_value);
    cpu.pc = cpu.pc.wrapping_add(pc_increment)
}
