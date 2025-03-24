use instruction_macro::instruction;

use crate::cpu::{
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

#[instruction(0x2F)]
fn handle_cpl(cpu: &mut CPU) {
    let result = complement(cpu.registers.a, &mut cpu.registers.f);

    cpu.registers.a = result;
    cpu.pc = cpu.pc.wrapping_add(1);
}

pub fn complement(value: u8, flags: &mut FlagsRegister) -> u8 {
    let new_value = !value;

    flags.zero = new_value == 0;
    flags.half_carry = true;
    flags.subtract = true;

    new_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x00, 0xFF, false)]
    #[case(0x01, 0xFE, false)]
    #[case(0x7F, 0x80, false)]
    #[case(0x80, 0x7F, false)]
    #[case(0xFE, 0x01, false)]
    #[case(0xFF, 0x00, true)]
    #[case(0xAB, 0x54, false)]
    #[case(0x00, 0xFF, false)]
    fn should_negate_each_bit(
        #[case] value: u8,
        #[case] expected_result: u8,
        #[case] expected_zero: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = complement(value, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.half_carry, true);
        assert_eq!(flags.subtract, true);
    }
}
