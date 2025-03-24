use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

#[instruction(0xA8, ArithmeticTarget::B)]
#[instruction(0xA9, ArithmeticTarget::C)]
#[instruction(0xAA, ArithmeticTarget::D)]
#[instruction(0xAB, ArithmeticTarget::E)]
#[instruction(0xAC, ArithmeticTarget::H)]
#[instruction(0xAD, ArithmeticTarget::L)]
#[instruction(0xAE, ArithmeticTarget::HL)]
#[instruction(0xAF, ArithmeticTarget::A)]
#[instruction(0xEE, ArithmeticTarget::D8)]
fn handle_xor(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let result = xor(cpu.registers.a, value, &mut cpu.registers.f);

    cpu.registers.a = result;
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn xor(left: u8, right: u8, flags: &mut FlagsRegister) -> u8 {
    let result = left ^ right;

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
    #[case(0b1111_1111, 0b1111_1111, 0b0000_0000, true)] // Both all ones
    #[case(0b1010_1010, 0b0101_0101, 0b1111_1111, false)] // Alternating bits
    #[case(0b1111_0000, 0b0000_1111, 0b1111_1111, false)] // Non-overlapping bits
    #[case(0b1100_1100, 0b1010_1010, 0b0110_0110, false)] // Partial overlap
    #[case(0b0000_1111, 0b1111_0000, 0b1111_1111, false)] // Non-overlapping bits (inverse of case above)
    #[case(0b1111_1111, 0b0000_0000, 0b1111_1111, false)] // XOR with zero
    #[case(0b0000_0001, 0b0000_0001, 0b0000_0000, true)] // Single-bit set
    fn should_perform_xor_operation(
        #[case] left: u8,
        #[case] right: u8,
        #[case] expected_result: u8,
        #[case] expected_zero: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = xor(left, right, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
    }
}
