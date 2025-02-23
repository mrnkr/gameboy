use crate::cpu::flag_registers::FlagsRegister;

pub fn add(left: u8, right: u8, flags: &mut FlagsRegister) -> u8 {
    let (result, did_overflow) = left.overflowing_add(right);
    flags.zero = result == 0 && !did_overflow;
    flags.subtract = false;
    flags.carry = did_overflow;
    // Half Carry is set if adding the lower nibbles of the value and register A
    // together result in a value bigger than 0xF. If the result is larger than 0xF
    // than the addition caused a carry from the lower nibble to the upper nibble.
    flags.half_carry = (left & 0xF) + (right & 0xF) > 0xF;
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x12, 0x01, 0x13, false, false, false, false)] // Normal addition
    #[case(0xFF, 0x01, 0x00, true, true, false, false)] // Carry & Half-Carry
    #[case(0x0F, 0x01, 0x10, false, true, false, false)] // Half-Carry only
    #[case(0x7F, 0x01, 0x80, false, true, false, false)] // Large add with Half-Carry
    #[case(0x10, 0x01, 0x11, false, false, false, false)] // No carry, small add
    #[case(0x00, 0x00, 0x00, false, false, true, false)] // Zero
    fn should_add_values(
        #[case] right: u8,
        #[case] left: u8,
        #[case] expected_result: u8,
        #[case] expected_carry: bool,
        #[case] expected_half_carry: bool,
        #[case] expected_zero: bool,
        #[case] expected_subtract: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = add(left, right, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, expected_half_carry);
        assert_eq!(flags.subtract, expected_subtract);
    }
}
