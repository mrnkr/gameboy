use super::bit_index::BitIndex;
use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, ArithmeticTarget}, cpu_impl::CPU, instruction::InstructionEntry
};

#[instruction(0x80, 0, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x81, 0, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x82, 0, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x83, 0, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x84, 0, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x85, 0, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x86, 0, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x87, 0, ArithmeticTarget::A, prefixed = true)]
#[instruction(0x88, 1, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x89, 1, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x8A, 1, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x8B, 1, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x8C, 1, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x8D, 1, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x8E, 1, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x8F, 1, ArithmeticTarget::A, prefixed = true)]
#[instruction(0x90, 2, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x91, 2, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x92, 2, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x93, 2, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x94, 2, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x95, 2, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x96, 2, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x97, 2, ArithmeticTarget::A, prefixed = true)]
#[instruction(0x98, 3, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x99, 3, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x9A, 3, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x9B, 3, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x9C, 3, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x9D, 3, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x9E, 3, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x9F, 3, ArithmeticTarget::A, prefixed = true)]
#[instruction(0xA0, 4, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xA1, 4, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xA2, 4, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xA3, 4, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xA4, 4, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xA5, 4, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xA6, 4, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xA7, 4, ArithmeticTarget::A, prefixed = true)]
#[instruction(0xA8, 5, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xA9, 5, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xAA, 5, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xAB, 5, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xAC, 5, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xAD, 5, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xAE, 5, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xAF, 5, ArithmeticTarget::A, prefixed = true)]
#[instruction(0xB0, 6, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xB1, 6, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xB2, 6, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xB3, 6, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xB4, 6, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xB5, 6, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xB6, 6, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xB7, 6, ArithmeticTarget::A, prefixed = true)]
#[instruction(0xB8, 7, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xB9, 7, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xBA, 7, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xBB, 7, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xBC, 7, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xBD, 7, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xBE, 7, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xBF, 7, ArithmeticTarget::A, prefixed = true)]
fn handle_res(cpu: &mut CPU, index: u8, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    res(value, index);

    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn res(value: u8, idx: u8) -> u8 {
    if let Ok(bit_idx) = BitIndex::build(idx) {
        !(0x01 << *bit_idx) & value
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0b0000_0000, 3, 0b0000_0000)]
    #[case(0b1000_0000, 7, 0b0000_0000)]
    #[case(0b1010_1010, 0, 0b1010_1010)]
    #[case(0b1010_1010, 1, 0b1010_1000)]
    #[case(0b1111_0000, 3, 0b1111_0000)]
    #[case(0b1100_1100, 6, 0b1000_1100)]
    #[case(0b1100_1100, 5, 0b1100_1100)]
    #[case(0b0000_1111, 0, 0b0000_1110)]
    #[case(0b0000_1111, 3, 0b0000_0111)]
    #[case(0b0000_1111, 4, 0b0000_1111)]
    #[case(0b1111_1111, 7, 0b0111_1111)]
    #[case(0b0000_0001, 1, 0b0000_0001)]
    #[case(0b0000_0001, 0, 0b0000_0000)]
    fn should_perform_bit_check(#[case] value: u8, #[case] idx: u8, #[case] expected_result: u8) {
        let result = res(value, idx);

        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(0b0000_0000, 8)]
    #[case(0b1000_0000, 25)]
    #[case(0b1010_1010, 10)]
    fn should_perform_noop(#[case] value: u8, #[case] idx: u8) {
        let result = res(value, idx);

        assert_eq!(result, value);
    }
}
