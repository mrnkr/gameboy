use super::bit_index::BitIndex;
use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    instruction::InstructionEntry,
};

#[instruction(0xC0, 0, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xC1, 0, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xC2, 0, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xC3, 0, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xC4, 0, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xC5, 0, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xC6, 0, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xC7, 0, ArithmeticTarget::A, prefixed = true)]
#[instruction(0xC8, 1, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xC9, 1, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xCA, 1, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xCB, 1, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xCC, 1, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xCD, 1, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xCE, 1, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xCF, 1, ArithmeticTarget::A, prefixed = true)]
#[instruction(0xD0, 2, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xD1, 2, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xD2, 2, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xD3, 2, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xD4, 2, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xD5, 2, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xD6, 2, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xD7, 2, ArithmeticTarget::A, prefixed = true)]
#[instruction(0xD8, 3, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xD9, 3, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xDA, 3, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xDB, 3, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xDC, 3, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xDD, 3, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xDE, 3, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xDF, 3, ArithmeticTarget::A, prefixed = true)]
#[instruction(0xE0, 4, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xE1, 4, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xE2, 4, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xE3, 4, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xE4, 4, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xE5, 4, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xE6, 4, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xE7, 4, ArithmeticTarget::A, prefixed = true)]
#[instruction(0xE8, 5, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xE9, 5, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xEA, 5, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xEB, 5, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xEC, 5, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xED, 5, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xEE, 5, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xEF, 5, ArithmeticTarget::A, prefixed = true)]
#[instruction(0xF0, 6, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xF1, 6, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xF2, 6, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xF3, 6, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xF4, 6, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xF5, 6, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xF6, 6, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xF7, 6, ArithmeticTarget::A, prefixed = true)]
#[instruction(0xF8, 7, ArithmeticTarget::B, prefixed = true)]
#[instruction(0xF9, 7, ArithmeticTarget::C, prefixed = true)]
#[instruction(0xFA, 7, ArithmeticTarget::D, prefixed = true)]
#[instruction(0xFB, 7, ArithmeticTarget::E, prefixed = true)]
#[instruction(0xFC, 7, ArithmeticTarget::H, prefixed = true)]
#[instruction(0xFD, 7, ArithmeticTarget::L, prefixed = true)]
#[instruction(0xFE, 7, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0xFF, 7, ArithmeticTarget::A, prefixed = true)]
fn handle_set(cpu: &mut CPU, index: u8, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    set(value, index);

    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn set(value: u8, idx: u8) -> u8 {
    if let Ok(bit_idx) = BitIndex::build(idx) {
        (0x01 << *bit_idx) | value
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0b0000_0000, 3, 0b0000_1000)]
    #[case(0b1000_0000, 7, 0b1000_0000)]
    #[case(0b1010_1010, 0, 0b1010_1011)]
    #[case(0b1010_1010, 1, 0b1010_1010)]
    #[case(0b1111_0000, 3, 0b1111_1000)]
    #[case(0b1100_1100, 6, 0b1100_1100)]
    #[case(0b1100_1100, 5, 0b1110_1100)]
    #[case(0b0000_1111, 0, 0b0000_1111)]
    #[case(0b0000_1111, 3, 0b0000_1111)]
    #[case(0b0000_1111, 4, 0b0001_1111)]
    #[case(0b1111_1111, 7, 0b1111_1111)]
    #[case(0b0000_0001, 1, 0b0000_0011)]
    #[case(0b0000_0001, 0, 0b0000_0001)]
    fn should_perform_bit_check(#[case] value: u8, #[case] idx: u8, #[case] expected_result: u8) {
        let result = set(value, idx);

        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(0b0000_0000, 8)]
    #[case(0b1000_0000, 25)]
    #[case(0b1010_1010, 10)]
    fn should_perform_noop(#[case] value: u8, #[case] idx: u8) {
        let result = set(value, idx);

        assert_eq!(result, value);
    }
}
