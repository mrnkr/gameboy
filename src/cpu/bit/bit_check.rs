use crate::cpu::flag_registers::FlagsRegister;
use super::bit_index::BitIndex;

pub fn bit_check(value: u8, idx: u8, flags: &mut FlagsRegister) {
    if let Ok(bit_idx) = BitIndex::build(idx) {
        let bit = (value >> *bit_idx) & 0x01;

        flags.zero = bit == 0x00;
        flags.half_carry = true;
        flags.subtract = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0b0000_0000, 3, true)]
    #[case(0b1000_0000, 7, false)]
    #[case(0b1010_1010, 0, true)]
    #[case(0b1010_1010, 1, false)]
    #[case(0b1111_0000, 3, true)]
    #[case(0b1100_1100, 6, false)]
    #[case(0b1100_1100, 5, true)]
    #[case(0b0000_1111, 0, false)]
    #[case(0b0000_1111, 3, false)]
    #[case(0b0000_1111, 4, true)]
    #[case(0b1111_1111, 7, false)]
    #[case(0b0000_0001, 1, true)]
    #[case(0b0000_0001, 0, false)]
    fn should_perform_bit_check(
        #[case] value: u8,
        #[case] idx: u8,
        #[case] expected_zero: bool
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        bit_check(value, idx, &mut flags);

        assert_eq!(flags.zero, expected_zero);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.half_carry, true);
        assert_eq!(flags.subtract, false);
    }

    #[rstest]
    #[case(0b0000_0000, 8)]
    #[case(0b1000_0000, 25)]
    #[case(0b1010_1010, 10)]
    fn should_perform_noop(
        #[case] value: u8,
        #[case] idx: u8,
    ) {
        let mut flags = FlagsRegister::from(0x00 as u8);

        bit_check(value, idx, &mut flags);

        assert_eq!(flags.zero, false);
        assert_eq!(flags.carry, false);
        assert_eq!(flags.half_carry, false);
        assert_eq!(flags.subtract, false);
    }
}
