use crate::cpu::flag_registers::FlagsRegister;

pub fn swap_nibbles(value: u8, flags: &mut FlagsRegister) -> u8 {
    let upper_nibble = value & 0xF0;
    let lower_nibble = value & 0x0F;
    let result = (lower_nibble << 4) | (upper_nibble >> 4);

    flags.zero = result == 0x00;
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
    #[case(0xF0, 0x0F, false)]
    #[case(0x80, 0x08, false)]
    #[case(0xAD, 0xDA, false)]
    #[case(0x15, 0x51, false)]
    #[case(0x00, 0x00, true)]
    fn should_swap_nibbles(
        #[case] value: u8,
        #[case] expected_result: u8,
        #[case] expected_zero: bool
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        let result = swap_nibbles(value, &mut flags);

        assert_eq!(result, expected_result);
        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
    }
}
