use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, set_value_in_arithmetic_target, ArithmeticTarget}, cpu_impl::CPU, flag_registers::FlagsRegister, instruction::InstructionEntry
};

#[instruction(0x0F)]
fn handle_rrca(cpu: &mut CPU) {
    let result = rotate_right(cpu.registers.a, &mut cpu.registers.f);

    cpu.registers.a = result;
    cpu.registers.f.zero = false;
}

#[instruction(0x08, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x09, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x0A, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x0B, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x0C, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x0D, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x0E, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x0F, ArithmeticTarget::A, prefixed = true)]
fn handle_rrc(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let new_value = rotate_right(value, &mut cpu.registers.f);
    cpu.registers.f.zero = new_value == 0x00;
    set_value_in_arithmetic_target(cpu, &target, new_value);
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn rotate_right(value: u8, flags: &mut FlagsRegister) -> u8 {
    let lsb = value & 0x01;
    let result = (value >> 1) | (lsb << 7);

    flags.carry = lsb == 0x01;
    flags.half_carry = false;
    flags.subtract = false;
    flags.zero = false;

    result
}

#[instruction(0x1F)]
fn handle_rra(cpu: &mut CPU) {
    let result = rotate_right_through_carry(cpu.registers.a, &mut cpu.registers.f);

    cpu.registers.a = result;
    cpu.registers.f.zero = false;
}

#[instruction(0x18, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x19, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x1A, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x1B, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x1C, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x1D, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x1E, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x1F, ArithmeticTarget::A, prefixed = true)]
fn handle_rr(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let new_value = rotate_right_through_carry(value, &mut cpu.registers.f);
    cpu.registers.f.zero = new_value == 0x00;
    set_value_in_arithmetic_target(cpu, &target, new_value);
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn rotate_right_through_carry(value: u8, flags: &mut FlagsRegister) -> u8 {
    let lsb = value & 0x01;
    let mut result = value >> 1;

    if flags.carry {
        result |= 0x80;
    }

    flags.carry = lsb == 0x01;
    flags.half_carry = false;
    flags.subtract = false;
    flags.zero = false;

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x00, false, 0x00, false)]
    #[case(0x00, true, 0x00, false)]
    #[case(0x01, false, 0x80, true)]
    #[case(0x01, true, 0x80, true)]
    #[case(0xFF, false, 0xFF, true)]
    #[case(0xFF, true, 0xFF, true)]
    #[case(0x80, false, 0x40, false)]
    #[case(0x80, true, 0x40, false)]
    fn should_rotate_right(
        #[case] value: u8,
        #[case] carry_in: bool,
        #[case] expected_result: u8,
        #[case] expected_carry: bool,
    ) {
        let mut flags = FlagsRegister {
            carry: carry_in,
            half_carry: true,
            subtract: true,
            zero: true,
        };

        let result = rotate_right(value, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
        assert_eq!(flags.zero, false);
    }

    #[rstest]
    #[case(0x00, false, 0x00, false)]
    #[case(0x00, true, 0x80, false)]
    #[case(0x01, false, 0x00, true)]
    #[case(0x01, true, 0x80, true)]
    #[case(0xFF, false, 0x7F, true)]
    #[case(0xFF, true, 0xFF, true)]
    #[case(0x80, false, 0x40, false)]
    #[case(0x80, true, 0xC0, false)]
    fn should_rotate_right_though_carry(
        #[case] value: u8,
        #[case] carry_in: bool,
        #[case] expected_result: u8,
        #[case] expected_carry: bool,
    ) {
        let mut flags = FlagsRegister {
            carry: carry_in,
            half_carry: true,
            subtract: true,
            zero: true,
        };

        let result = rotate_right_through_carry(value, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
        assert_eq!(flags.zero, false);
    }
}
