use super::sub::sub;
use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    instruction::InstructionEntry,
};

#[instruction(0xB8, ArithmeticTarget::B)]
#[instruction(0xB9, ArithmeticTarget::C)]
#[instruction(0xBA, ArithmeticTarget::D)]
#[instruction(0xBB, ArithmeticTarget::E)]
#[instruction(0xBC, ArithmeticTarget::H)]
#[instruction(0xBD, ArithmeticTarget::L)]
#[instruction(0xBE, ArithmeticTarget::HL)]
#[instruction(0xBF, ArithmeticTarget::A)]
#[instruction(0xFE, ArithmeticTarget::D8)]
fn handle_cp(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    sub(cpu.registers.a, value, &mut cpu.registers.f);

    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}
