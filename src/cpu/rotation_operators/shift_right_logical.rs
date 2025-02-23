use crate::cpu::flag_registers::FlagsRegister;

pub fn shift_right_logical(value: u8, flags: &mut FlagsRegister) -> u8 {
    let lsb = value & 0x01;
    let result = value >> 1;

    flags.zero = result == 0x00;
    flags.carry = lsb == 0x01;
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
    #[case(0x01, false, 0x00, true, true)]
    #[case(0x01, true, 0x00, true, true)]
    #[case(0xFF, false, 0x7F, true, false)]
    #[case(0xFF, true, 0x7F, true, false)]
    #[case(0x80, false, 0x40, false, false)]
    #[case(0x80, true, 0x40, false, false)]
    fn should_shift_right(
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

        let result = shift_right_logical(value, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
        assert_eq!(flags.zero, expected_zero);
    }
}
