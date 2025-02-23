use crate::cpu::flag_registers::FlagsRegister;

pub fn rotate_right(value: u8, flags: &mut FlagsRegister) -> u8 {
    let lsb = value & 0x01;
    let result = (value >> 1) | (lsb << 7);

    flags.carry = lsb == 0x01;
    flags.half_carry = false;
    flags.subtract = false;
    flags.zero = false;

    result
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
