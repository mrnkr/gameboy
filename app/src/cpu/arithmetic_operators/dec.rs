use super::{sub::sub, IncDecTarget};
use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, set_value_in_arithmetic_target, ArithmeticTarget}, arithmetic_target_pair::{get_value_in_arithmetic_target_pair, set_value_in_arithmetic_target_pair, ArithmeticTargetPair}, cpu_impl::CPU, instruction::InstructionEntry
};

#[instruction(0x05, IncDecTarget::Byte(ArithmeticTarget::B))]
#[instruction(0x0D, IncDecTarget::Byte(ArithmeticTarget::C))]
#[instruction(0x15, IncDecTarget::Byte(ArithmeticTarget::D))]
#[instruction(0x1D, IncDecTarget::Byte(ArithmeticTarget::E))]
#[instruction(0x25, IncDecTarget::Byte(ArithmeticTarget::H))]
#[instruction(0x2D, IncDecTarget::Byte(ArithmeticTarget::L))]
#[instruction(0x35, IncDecTarget::Byte(ArithmeticTarget::HL))]
#[instruction(0x3D, IncDecTarget::Byte(ArithmeticTarget::A))]
#[instruction(0x0B, IncDecTarget::Word(ArithmeticTargetPair::BC))]
#[instruction(0x1B, IncDecTarget::Word(ArithmeticTargetPair::DE))]
#[instruction(0x2B, IncDecTarget::Word(ArithmeticTargetPair::HL))]
#[instruction(0x3B, IncDecTarget::Word(ArithmeticTargetPair::SP))]
fn handle_dec(cpu: &mut CPU, target: IncDecTarget) {
    match target {
        IncDecTarget::Byte(target) => {
            let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
            let new_value = sub(value, 0x01, &mut cpu.registers.f);
            set_value_in_arithmetic_target(cpu, &target, new_value);
            cpu.pc = cpu.pc.wrapping_add(pc_increment)
        }
        IncDecTarget::Word(target) => {
            let value = get_value_in_arithmetic_target_pair(cpu, &target);
            set_value_in_arithmetic_target_pair(cpu, &target, value - 0x0001);
            cpu.pc = cpu.pc.wrapping_add(1)
        }
    }
}
