use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

#[instruction(0x90, ArithmeticTarget::B)]
#[instruction(0x91, ArithmeticTarget::C)]
#[instruction(0x92, ArithmeticTarget::D)]
#[instruction(0x93, ArithmeticTarget::E)]
#[instruction(0x94, ArithmeticTarget::H)]
#[instruction(0x95, ArithmeticTarget::L)]
#[instruction(0x96, ArithmeticTarget::HL)]
#[instruction(0x97, ArithmeticTarget::A)]
#[instruction(0xD6, ArithmeticTarget::D8)]
fn handle_sub(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let result = sub(cpu.registers.a, value, &mut cpu.registers.f);

    cpu.registers.a = result;
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn sub(left: u8, right: u8, flags: &mut FlagsRegister) -> u8 {
    let (new_value, did_overflow) = left.overflowing_sub(right);
    flags.zero = new_value == 0 && !did_overflow;
    flags.subtract = true;
    flags.carry = did_overflow;
    flags.half_carry = (left & 0x0F) < (right & 0x0F);
    new_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x12, 0x01, 0x11, false, false, false, true)] // Normal subtraction
    #[case(0x01, 0x02, 0xFF, true, true, false, true)] // Borrow & Half-Borrow
    #[case(0x10, 0x01, 0x0F, false, true, false, true)] // Half-Borrow only
    #[case(0x80, 0x01, 0x7F, false, true, false, true)] // Large sub, half borrow
    #[case(0x01, 0x01, 0x00, false, false, true, true)] // Zero result
    #[case(0x20, 0x10, 0x10, false, false, false, true)] // No borrow, regular subtraction
    fn should_sub_values(
        #[case] left: u8,
        #[case] right: u8,
        #[case] expected_result: u8,
        #[case] expected_carry: bool,
        #[case] expected_half_carry: bool,
        #[case] expected_zero: bool,
        #[case] expected_subtract: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = sub(left, right, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, expected_half_carry);
        assert_eq!(flags.subtract, expected_subtract);
    }
}
