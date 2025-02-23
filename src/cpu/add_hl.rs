use super::flag_registers::FlagsRegister;

pub fn add_hl(left: u16, right: u16, flags: &mut FlagsRegister) -> u16 {
    let (new_value, did_overflow) = left.overflowing_add(right);
    flags.zero = new_value == 0 && !did_overflow;
    flags.subtract = false;
    flags.carry = did_overflow;
    // Half Carry is set if adding the lower nibbles of the value and register A
    // together result in a value bigger than 0xF. If the result is larger than 0xF
    // than the addition caused a carry from the lower nibble to the upper nibble.
    flags.half_carry = (left & 0xFFF) + (right & 0xFFF) > 0xFFF;
    new_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x1234, 0x0101, 0x1335, false, false, false, false)] // Normal addition
    #[case(0xFFFF, 0x0001, 0x0000, true, true, false, false)] // Carry & Half-Carry
    #[case(0x0FFF, 0x0001, 0x1000, false, true, false, false)] // Half-Carry only
    #[case(0x7FFF, 0x0001, 0x8000, false, true, false, false)] // Large add with Half-Carry
    #[case(0x08FF, 0x0701, 0x1000, false, true, false, false)] // Large add, Half-Carry
    #[case(0x1000, 0x0001, 0x1001, false, false, false, false)] // No carry, small add
    #[case(0x0000, 0x0000, 0x0000, false, false, true, false)] // Zero
    fn should_add_value(
        #[case] left: u16,
        #[case] right: u16,
        #[case] expected_result: u16,
        #[case] expected_carry: bool,
        #[case] expected_half_carry: bool,
        #[case] expected_zero: bool,
        #[case] expected_subtract: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = add_hl(left, right, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, expected_half_carry);
        assert_eq!(flags.subtract, expected_subtract);
    }
}
