use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

#[instruction(0xA0, ArithmeticTarget::B)]
#[instruction(0xA1, ArithmeticTarget::C)]
#[instruction(0xA2, ArithmeticTarget::D)]
#[instruction(0xA3, ArithmeticTarget::E)]
#[instruction(0xA4, ArithmeticTarget::H)]
#[instruction(0xA5, ArithmeticTarget::L)]
#[instruction(0xA6, ArithmeticTarget::HL)]
#[instruction(0xA7, ArithmeticTarget::A)]
#[instruction(0xE6, ArithmeticTarget::D8)]
fn handle_and(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let result = and(cpu.registers.a, value, &mut cpu.registers.f);

    cpu.registers.a = result;
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn and(left: u8, right: u8, flags: &mut FlagsRegister) -> u8 {
    let result = left & right;

    flags.zero = result == 0x00;
    flags.carry = false;
    flags.half_carry = true;
    flags.subtract = false;

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0b0000_0000, 0b0000_0000, 0b0000_0000, true)] // Both zero
    #[case(0b1111_1111, 0b1111_1111, 0b1111_1111, false)] // Both all ones
    #[case(0b1010_1010, 0b0101_0101, 0b0000_0000, true)] // Alternating bits
    #[case(0b1111_0000, 0b0000_1111, 0b0000_0000, true)] // Non-overlapping bits
    #[case(0b1100_1100, 0b1010_1010, 0b1000_1000, false)] // Partial overlap
    #[case(0b0000_1111, 0b1111_0000, 0b0000_0000, true)] // Non-overlapping bits (inverse of case above)
    #[case(0b1111_1111, 0b0000_0000, 0b0000_0000, true)] // AND with zero
    #[case(0b0000_0001, 0b0000_0001, 0b0000_0001, false)] // Single-bit set
    fn should_perform_and_operation(
        #[case] left: u8,
        #[case] right: u8,
        #[case] expected_result: u8,
        #[case] expected_zero: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = and(left, right, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.half_carry, true);
        assert_eq!(flags.subtract, false);
    }
}
