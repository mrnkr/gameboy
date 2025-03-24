use super::{add::add, IncDecTarget};
use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, set_value_in_arithmetic_target, ArithmeticTarget}, arithmetic_target_pair::{get_value_in_arithmetic_target_pair, set_value_in_arithmetic_target_pair, ArithmeticTargetPair}, cpu_impl::CPU, instruction::InstructionEntry
};

#[instruction(0x04, IncDecTarget::Byte(ArithmeticTarget::B))]
#[instruction(0x0C, IncDecTarget::Byte(ArithmeticTarget::C))]
#[instruction(0x14, IncDecTarget::Byte(ArithmeticTarget::D))]
#[instruction(0x1C, IncDecTarget::Byte(ArithmeticTarget::E))]
#[instruction(0x24, IncDecTarget::Byte(ArithmeticTarget::H))]
#[instruction(0x2C, IncDecTarget::Byte(ArithmeticTarget::L))]
#[instruction(0x34, IncDecTarget::Byte(ArithmeticTarget::HL))]
#[instruction(0x3C, IncDecTarget::Byte(ArithmeticTarget::A))]
#[instruction(0x03, IncDecTarget::Word(ArithmeticTargetPair::BC))]
#[instruction(0x13, IncDecTarget::Word(ArithmeticTargetPair::DE))]
#[instruction(0x23, IncDecTarget::Word(ArithmeticTargetPair::HL))]
#[instruction(0x33, IncDecTarget::Word(ArithmeticTargetPair::SP))]
fn handle_inc(cpu: &mut CPU, target: IncDecTarget) {
    match target {
        IncDecTarget::Byte(target) => {
            let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
            let new_value = add(value, 0x01, &mut cpu.registers.f);
            set_value_in_arithmetic_target(cpu, &target, new_value);
            cpu.pc = cpu.pc.wrapping_add(pc_increment);
        }
        IncDecTarget::Word(target) => {
            let value = get_value_in_arithmetic_target_pair(cpu, &target);
            set_value_in_arithmetic_target_pair(cpu, &target, value + 0x0001);
            cpu.pc = cpu.pc.wrapping_add(1);
        }
    }
}
