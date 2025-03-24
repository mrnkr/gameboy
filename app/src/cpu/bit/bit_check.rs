use super::bit_index::BitIndex;
use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

#[instruction(0x40, 0, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x41, 0, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x42, 0, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x43, 0, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x44, 0, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x45, 0, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x46, 0, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x47, 0, ArithmeticTarget::A, prefixed = true)]
#[instruction(0x48, 1, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x49, 1, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x4A, 1, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x4B, 1, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x4C, 1, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x4D, 1, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x4E, 1, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x4F, 1, ArithmeticTarget::A, prefixed = true)]
#[instruction(0x50, 2, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x51, 2, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x52, 2, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x53, 2, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x54, 2, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x55, 2, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x56, 2, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x57, 2, ArithmeticTarget::A, prefixed = true)]
#[instruction(0x58, 3, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x59, 3, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x5A, 3, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x5B, 3, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x5C, 3, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x5D, 3, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x5E, 3, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x5F, 3, ArithmeticTarget::A, prefixed = true)]
#[instruction(0x60, 4, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x61, 4, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x62, 4, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x63, 4, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x64, 4, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x65, 4, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x66, 4, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x67, 4, ArithmeticTarget::A, prefixed = true)]
#[instruction(0x68, 5, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x69, 5, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x6A, 5, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x6B, 5, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x6C, 5, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x6D, 5, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x6E, 5, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x6F, 5, ArithmeticTarget::A, prefixed = true)]
#[instruction(0x70, 6, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x71, 6, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x72, 6, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x73, 6, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x74, 6, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x75, 6, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x76, 6, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x77, 6, ArithmeticTarget::A, prefixed = true)]
#[instruction(0x78, 7, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x79, 7, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x7A, 7, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x7B, 7, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x7C, 7, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x7D, 7, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x7E, 7, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x7F, 7, ArithmeticTarget::A, prefixed = true)]
fn handle_bit(cpu: &mut CPU, index: u8, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    bit(value, index, &mut cpu.registers.f);

    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn bit(value: u8, idx: u8, flags: &mut FlagsRegister) {
    if let Ok(bit_idx) = BitIndex::build(idx) {
        let bit = (value >> *bit_idx) & 0x01;

        flags.zero = bit == 0x00;
        flags.half_carry = true;
        flags.subtract = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0b0000_0000, 3, true)]
    #[case(0b1000_0000, 7, false)]
    #[case(0b1010_1010, 0, true)]
    #[case(0b1010_1010, 1, false)]
    #[case(0b1111_0000, 3, true)]
    #[case(0b1100_1100, 6, false)]
    #[case(0b1100_1100, 5, true)]
    #[case(0b0000_1111, 0, false)]
    #[case(0b0000_1111, 3, false)]
    #[case(0b0000_1111, 4, true)]
    #[case(0b1111_1111, 7, false)]
    #[case(0b0000_0001, 1, true)]
    #[case(0b0000_0001, 0, false)]
    fn should_perform_bit_check(#[case] value: u8, #[case] idx: u8, #[case] expected_zero: bool) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        bit(value, idx, &mut flags);

        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.half_carry, true);
        assert_eq!(flags.subtract, false);
    }

    #[rstest]
    #[case(0b0000_0000, 8)]
    #[case(0b1000_0000, 25)]
    #[case(0b1010_1010, 10)]
    fn should_perform_noop(#[case] value: u8, #[case] idx: u8) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        bit(value, idx, &mut flags);

        assert_eq!(flags.zero, false);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
    }
}
