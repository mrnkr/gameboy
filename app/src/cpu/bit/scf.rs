use instruction_macro::instruction;

use crate::cpu::{
    cpu_impl::CPU,
    instruction::InstructionEntry,
};

#[instruction(0x37)]
fn handle_scf(cpu: &mut CPU) {
    cpu.registers.f.subtract = false;
    cpu.registers.f.half_carry = false;
    cpu.registers.f.carry = true;
    cpu.pc = cpu.pc.wrapping_add(1);
}
