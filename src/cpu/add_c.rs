use super::{add::add, flag_registers::FlagsRegister};

pub fn add_c(left: u8, right: u8, flags: &mut FlagsRegister) -> u8 {
    let mut new_value = add(left, right, flags);
    if flags.carry {
        new_value += 0x01;
    }
    new_value
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0x12, 0x01, 0x13, false, false, false, false)] // Normal addition
    #[case(0xFF, 0x01, 0x01, true, true, false, false)] // Carry & Half-Carry
    #[case(0x0F, 0x01, 0x10, false, true, false, false)] // Half-Carry only
    #[case(0x7F, 0x01, 0x80, false, true, false, false)] // Large add with Half-Carry
    #[case(0x10, 0x01, 0x11, false, false, false, false)] // No carry, small add
    #[case(0x00, 0x00, 0x00, false, false, true, false)] // Zero
    fn should_add_with_carry_value_to_register_a(
        #[case] right: u8,
        #[case] left: u8,
        #[case] expected_result: u8,
        #[case] expected_carry: bool,
        #[case] expected_half_carry: bool,
        #[case] expected_zero: bool,
        #[case] expected_subtract: bool,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = add_c(left, right, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, expected_carry);
        assert_eq!(flags.half_carry, expected_half_carry);
        assert_eq!(flags.subtract, expected_subtract);
    }
}
