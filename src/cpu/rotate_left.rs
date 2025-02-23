use super::flag_registers::FlagsRegister;

pub fn rotate_left(value: u8, flags: &mut FlagsRegister) -> u8 {
    let msb = (value >> 7) & 0x01;
    let result = value << 1;

    flags.carry = msb == 0x01;
    flags.half_carry = false;
    flags.subtract = false;

    result
}

pub fn rotate_left_through_carry(value: u8, flags: &mut FlagsRegister) -> u8 {
    let prev_carry = flags.carry;
    let mut result = rotate_left(value, flags);

    if prev_carry {
        result += 0x01;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x00, false, 0x00, false)]
    #[case(0x00, true,  0x00, false)]
    #[case(0x01, false, 0x02, false)]
    #[case(0x01, true,  0x02, false)]
    #[case(0xFF, false, 0xFE, true)]
    #[case(0xFF, true,  0xFE, true)]
    #[case(0x80, false, 0x00, true)]
    #[case(0x80, true,  0x00, true)]
    fn should_rotate_left(
        #[case] value: u8,
        #[case] carry_in: bool,
        #[case] expected_result: u8,
        #[case] expected_carry: bool
    ) {
        let mut flags = FlagsRegister {
            carry: carry_in,
            half_carry: true,
            subtract: true,
            zero: true
        };

        let result = rotate_left(value, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
        assert_eq!(flags.zero, true);
    }

    #[rstest]
    #[case(0x00, false, 0x00, false)]
    #[case(0x00, true,  0x01, false)]
    #[case(0x01, false, 0x02, false)]
    #[case(0x01, true,  0x03, false)]
    #[case(0xFF, false, 0xFE, true)]
    #[case(0xFF, true,  0xFF, true)]
    #[case(0x80, false, 0x00, true)]
    #[case(0x80, true,  0x01, true)]
    fn should_rotate_left_though_carry(
        #[case] value: u8,
        #[case] carry_in: bool,
        #[case] expected_result: u8,
        #[case] expected_carry: bool
    ) {
        let mut flags = FlagsRegister {
            carry: carry_in,
            half_carry: true,
            subtract: true,
            zero: true
        };

        let result = rotate_left_through_carry(value, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
        assert_eq!(flags.zero, true);
    }
}
