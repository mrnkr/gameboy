use super::add::add;
use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

#[instruction(0x88, ArithmeticTarget::B)]
#[instruction(0x89, ArithmeticTarget::C)]
#[instruction(0x8A, ArithmeticTarget::D)]
#[instruction(0x8B, ArithmeticTarget::E)]
#[instruction(0x8C, ArithmeticTarget::H)]
#[instruction(0x8D, ArithmeticTarget::L)]
#[instruction(0x8E, ArithmeticTarget::HL)]
#[instruction(0x8F, ArithmeticTarget::A)]
#[instruction(0xCE, ArithmeticTarget::D8)]
fn handle_adc(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let result = adc(cpu.registers.a, value, &mut cpu.registers.f);

    cpu.registers.a = result;
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn adc(left: u8, right: u8, flags: &mut FlagsRegister) -> u8 {
    let mut new_value = add(left, right, flags);
    if flags.carry {
        new_value += 0x01;
    }
    new_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x12, 0x01, 0x13, false, false, false, false)] // Normal addition
    #[case(0xFF, 0x01, 0x01, true, true, false, false)] // Carry & Half-Carry
    #[case(0x0F, 0x01, 0x10, false, true, false, false)] // Half-Carry only
    #[case(0x7F, 0x01, 0x80, false, true, false, false)] // Large add with Half-Carry
    #[case(0x10, 0x01, 0x11, false, false, false, false)] // No carry, small add
    #[case(0x00, 0x00, 0x00, false, false, true, false)] // Zero
    fn should_add_with_carry(
        #[case] right: u8,
        #[case] left: u8,
        #[case] expected_result: u8,
        #[case] expected_carry: bool,
        #[case] expected_half_carry: bool,
        #[case] expected_zero: bool,
        #[case] expected_subtract: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = adc(left, right, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, expected_half_carry);
        assert_eq!(flags.subtract, expected_subtract);
    }
}
