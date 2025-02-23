use super::flag_registers::FlagsRegister;

pub fn sub(left: u8, right: u8, flags: &mut FlagsRegister) -> u8 {
    let (new_value, did_overflow) = left.overflowing_sub(right);
    flags.zero = new_value == 0 && !did_overflow;
    flags.subtract = true;
    flags.carry = did_overflow;
    flags.half_carry = (left & 0x0F) < (right & 0x0F);
    new_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x12, 0x01, 0x11, false, false, false, true)] // Normal subtraction
    #[case(0x01, 0x02, 0xFF, true, true, false, true)] // Borrow & Half-Borrow
    #[case(0x10, 0x01, 0x0F, false, true, false, true)] // Half-Borrow only
    #[case(0x80, 0x01, 0x7F, false, true, false, true)] // Large sub, half borrow
    #[case(0x01, 0x01, 0x00, false, false, true, true)] // Zero result
    #[case(0x20, 0x10, 0x10, false, false, false, true)] // No borrow, regular subtraction
    fn should_sub_values(
        #[case] left: u8,
        #[case] right: u8,
        #[case] expected_result: u8,
        #[case] expected_carry: bool,
        #[case] expected_half_carry: bool,
        #[case] expected_zero: bool,
        #[case] expected_subtract: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = sub(left, right, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, expected_half_carry);
        assert_eq!(flags.subtract, expected_subtract);
    }
}
