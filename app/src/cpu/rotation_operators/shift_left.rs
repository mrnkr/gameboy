use instruction_macro::instruction;

use crate::cpu::{
    arithmetic_target::{get_value_in_arithmetic_target, set_value_in_arithmetic_target, ArithmeticTarget},
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

#[instruction(0x20, ArithmeticTarget::B, prefixed = true)]
#[instruction(0x21, ArithmeticTarget::C, prefixed = true)]
#[instruction(0x22, ArithmeticTarget::D, prefixed = true)]
#[instruction(0x23, ArithmeticTarget::E, prefixed = true)]
#[instruction(0x24, ArithmeticTarget::H, prefixed = true)]
#[instruction(0x25, ArithmeticTarget::L, prefixed = true)]
#[instruction(0x26, ArithmeticTarget::HL, prefixed = true)]
#[instruction(0x27, ArithmeticTarget::A, prefixed = true)]
fn handle_sla(cpu: &mut CPU, target: ArithmeticTarget) {
    let (value, pc_increment) = get_value_in_arithmetic_target(cpu, &target);
    let new_value = shift_left(value, &mut cpu.registers.f);
    set_value_in_arithmetic_target(cpu, &target, new_value);
    cpu.pc = cpu.pc.wrapping_add(pc_increment);
}

pub fn shift_left(value: u8, flags: &mut FlagsRegister) -> u8 {
    let msb = (value >> 7) & 0x01;
    let result = value << 1;

    flags.zero = result == 0x00;
    flags.carry = msb == 0x01;
    flags.half_carry = false;
    flags.subtract = false;

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x00, false, 0x00, false, true)]
    #[case(0x00, true, 0x00, false, true)]
    #[case(0x01, false, 0x02, false, false)]
    #[case(0x01, true, 0x02, false, false)]
    #[case(0xFF, false, 0xFE, true, false)]
    #[case(0xFF, true, 0xFE, true, false)]
    #[case(0x80, false, 0x00, true, true)]
    #[case(0x80, true, 0x00, true, true)]
    fn should_shift_left(
        #[case] value: u8,
        #[case] carry_in: bool,
        #[case] expected_result: u8,
        #[case] expected_carry: bool,
        #[case] expected_zero: bool,
    ) {
        let mut flags = FlagsRegister {
            carry: carry_in,
            half_carry: true,
            subtract: true,
            zero: true,
        };

        let result = shift_left(value, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
        assert_eq!(flags.zero, expected_zero);
    }
}
