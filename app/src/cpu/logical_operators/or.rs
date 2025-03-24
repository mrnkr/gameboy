use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

#[instruction(0xB0, ArithmeticTarget::B)]
#[instruction(0xB1, ArithmeticTarget::C)]
#[instruction(0xB2, ArithmeticTarget::D)]
#[instruction(0xB3, ArithmeticTarget::E)]
#[instruction(0xB4, ArithmeticTarget::H)]
#[instruction(0xB5, ArithmeticTarget::L)]
#[instruction(0xB6, ArithmeticTarget::HL)]
#[instruction(0xB7, ArithmeticTarget::A)]
#[instruction(0xF6, ArithmeticTarget::D8)]
fn handle_or(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let result = or(cpu.registers.a, value, &mut cpu.registers.f);

    cpu.registers.a = result;
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn or(left: u8, right: u8, flags: &mut FlagsRegister) -> u8 {
    let result = left | right;

    flags.zero = result == 0;
    flags.carry = false;
    flags.half_carry = false;
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
    #[case(0b1010_1010, 0b0101_0101, 0b1111_1111, false)] // Alternating bits
    #[case(0b1111_0000, 0b0000_1111, 0b1111_1111, false)] // Non-overlapping bits
    #[case(0b1100_1100, 0b1010_1010, 0b1110_1110, false)] // Partial overlap
    #[case(0b0000_1111, 0b1111_0000, 0b1111_1111, false)] // Non-overlapping bits (inverse of case above)
    #[case(0b1111_1111, 0b0000_0000, 0b1111_1111, false)] // OR with zero
    #[case(0b0000_0001, 0b0000_0001, 0b0000_0001, false)] // Single-bit set
    fn should_perform_or_operation(
        #[case] left: u8,
        #[case] right: u8,
        #[case] expected_result: u8,
        #[case] expected_zero: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = or(left, right, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
    }
}
