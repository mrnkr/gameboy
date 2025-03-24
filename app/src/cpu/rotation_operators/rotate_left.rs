use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, set_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

#[instruction(0x07)]
fn handle_rlca(cpu: &mut CPU) {
    let result = rotate_left(cpu.registers.a, &mut cpu.registers.f);

    cpu.registers.a = result;
    cpu.pc = cpu.pc.wrapping_add(1);
}

#[instruction(0x00, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x01, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x02, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x03, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x04, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x05, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x06, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x07, ArithmeticTarget::A, prefixed = true)]
fn handle_rlc(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let new_value = rotate_left(value, &mut cpu.registers.f);
    cpu.registers.f.zero = new_value == 0x00;
    set_value_in_arithmetic_target(cpu, &target, new_value);
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn rotate_left(value: u8, flags: &mut FlagsRegister) -> u8 {
    let msb = (value >> 7) & 0x01;
    let result = (value << 1) | msb;

    flags.carry = msb == 0x01;
    flags.half_carry = false;
    flags.subtract = false;
    flags.zero = false;

    result
}

#[instruction(0x17)]
fn handle_rla(cpu: &mut CPU) {
    let result = rotate_left_through_carry(cpu.registers.a, &mut cpu.registers.f);

    cpu.registers.a = result;
    cpu.pc = cpu.pc.wrapping_add(1);
}

#[instruction(0x10, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x11, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x12, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x13, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x14, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x15, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x16, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x17, ArithmeticTarget::A, prefixed = true)]
fn handle_rl(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let new_value = rotate_left_through_carry(value, &mut cpu.registers.f);
    cpu.registers.f.zero = new_value == 0x00;
    set_value_in_arithmetic_target(cpu, &target, new_value);
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn rotate_left_through_carry(value: u8, flags: &mut FlagsRegister) -> u8 {
    let msb = (value >> 7) & 0x01;
    let mut result = value << 1;

    if flags.carry {
        result |= 0x01;
    }

    flags.carry = msb == 0x01;
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
    #[case(0x01, false, 0x02, false)]
    #[case(0x01, true, 0x02, false)]
    #[case(0xFF, false, 0xFF, true)]
    #[case(0xFF, true, 0xFF, true)]
    #[case(0x80, false, 0x01, true)]
    #[case(0x80, true, 0x01, true)]
    fn should_rotate_left(
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

        let result = rotate_left(value, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
        assert_eq!(flags.zero, false);
    }

    #[rstest]
    #[case(0x00, false, 0x00, false)]
    #[case(0x00, true, 0x01, false)]
    #[case(0x01, false, 0x02, false)]
    #[case(0x01, true, 0x03, false)]
    #[case(0xFF, false, 0xFE, true)]
    #[case(0xFF, true, 0xFF, true)]
    #[case(0x80, false, 0x00, true)]
    #[case(0x80, true, 0x01, true)]
    fn should_rotate_left_though_carry(
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

        let result = rotate_left_through_carry(value, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
        assert_eq!(flags.zero, false);
    }
}
