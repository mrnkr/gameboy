use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, set_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

#[instruction(0x30, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x31, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x32, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x33, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x34, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x35, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x36, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x37, ArithmeticTarget::A, prefixed = true)]
fn handle_swap(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let new_value = swap_nibbles(value, &mut cpu.registers.f);
    set_value_in_arithmetic_target(cpu, &target, new_value);
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn swap_nibbles(value: u8, flags: &mut FlagsRegister) -> u8 {
    let upper_nibble = value & 0xF0;
    let lower_nibble = value & 0x0F;
    let result = (lower_nibble << 4) | (upper_nibble >> 4);

    flags.zero = result == 0x00;
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
    #[case(0xF0, 0x0F, false)]
    #[case(0x80, 0x08, false)]
    #[case(0xAD, 0xDA, false)]
    #[case(0x15, 0x51, false)]
    #[case(0x00, 0x00, true)]
    fn should_swap_nibbles(
        #[case] value: u8,
        #[case] expected_result: u8,
        #[case] expected_zero: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = swap_nibbles(value, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
    }
}
