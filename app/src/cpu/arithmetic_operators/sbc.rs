use super::sub::sub;
use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

#[instruction(0x98, ArithmeticTarget::B)]
#[instruction(0x99, ArithmeticTarget::C)]
#[instruction(0x9A, ArithmeticTarget::D)]
#[instruction(0x9B, ArithmeticTarget::E)]
#[instruction(0x9C, ArithmeticTarget::H)]
#[instruction(0x9D, ArithmeticTarget::L)]
#[instruction(0x9E, ArithmeticTarget::HL)]
#[instruction(0x9F, ArithmeticTarget::A)]
#[instruction(0xDE, ArithmeticTarget::D8)]
fn handle_sbc(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let result = sbc(cpu.registers.a, value, &mut cpu.registers.f);

    cpu.registers.a = result;
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn sbc(left: u8, right: u8, flags: &mut FlagsRegister) -> u8 {
    let mut new_value = sub(left, right, flags);
    if flags.carry {
        new_value -= 0x01;
    }
    new_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x12, 0x01, 0x11, false, false, false, true)] // Normal subtraction
    #[case(0x01, 0x02, 0xFE, true, true, false, true)] // Borrow & Half-Borrow
    #[case(0x10, 0x01, 0x0F, false, true, false, true)] // Half-Borrow only
    #[case(0x80, 0x01, 0x7F, false, true, false, true)] // Large sub, half borrow
    #[case(0x01, 0x01, 0x00, false, false, true, true)] // Zero result
    #[case(0x20, 0x10, 0x10, false, false, false, true)] // No borrow, regular subtraction
    fn should_sub_with_carry(
        #[case] left: u8,
        #[case] right: u8,
        #[case] expected_result: u8,
        #[case] expected_carry: bool,
        #[case] expected_half_carry: bool,
        #[case] expected_zero: bool,
        #[case] expected_subtract: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = sbc(left, right, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, expected_half_carry);
        assert_eq!(flags.subtract, expected_subtract);
    }
}
