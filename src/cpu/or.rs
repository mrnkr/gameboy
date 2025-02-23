use super::flag_registers::FlagsRegister;

pub fn or(left: u8, right: u8, flags: &mut FlagsRegister) -> u8 {
    let result = left | right;

    flags.zero = result == 0;
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
    #[case(0b0000_0000, 0b0000_0000, 0b0000_0000, true)] // Both zero
    #[case(0b1111_1111, 0b1111_1111, 0b1111_1111, false)] // Both all ones
    #[case(0b1010_1010, 0b0101_0101, 0b1111_1111, false)] // Alternating bits
    #[case(0b1111_0000, 0b0000_1111, 0b1111_1111, false)] // Non-overlapping bits
    #[case(0b1100_1100, 0b1010_1010, 0b1110_1110, false)] // Partial overlap
    #[case(0b0000_1111, 0b1111_0000, 0b1111_1111, false)] // Non-overlapping bits (inverse of case above)
    #[case(0b1111_1111, 0b0000_0000, 0b1111_1111, false)] // OR with zero
    #[case(0b0000_0001, 0b0000_0001, 0b0000_0001, false)] // Single-bit set
    fn should_perform_or_to_register_in_a(
        #[case] left:u8,
        #[case] right: u8,
        #[case] expected_result: u8,
        #[case] expected_zero: bool
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = or(left, right, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
    }
}
