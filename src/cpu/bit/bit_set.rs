use super::bit_index::BitIndex;

pub fn bit_set(value: u8, idx: u8) -> u8 {
    if let Ok(bit_idx) = BitIndex::build(idx) {
        (0x01 << *bit_idx) | value
    } else {
        value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0b0000_0000, 3, 0b0000_1000)]
    #[case(0b1000_0000, 7, 0b1000_0000)]
    #[case(0b1010_1010, 0, 0b1010_1011)]
    #[case(0b1010_1010, 1, 0b1010_1010)]
    #[case(0b1111_0000, 3, 0b1111_1000)]
    #[case(0b1100_1100, 6, 0b1100_1100)]
    #[case(0b1100_1100, 5, 0b1110_1100)]
    #[case(0b0000_1111, 0, 0b0000_1111)]
    #[case(0b0000_1111, 3, 0b0000_1111)]
    #[case(0b0000_1111, 4, 0b0001_1111)]
    #[case(0b1111_1111, 7, 0b1111_1111)]
    #[case(0b0000_0001, 1, 0b0000_0011)]
    #[case(0b0000_0001, 0, 0b0000_0001)]
    fn should_perform_bit_check(#[case] value: u8, #[case] idx: u8, #[case] expected_result: u8) {
        let result = bit_set(value, idx);

        assert_eq!(result, expected_result);
    }

    #[rstest]
    #[case(0b0000_0000, 8)]
    #[case(0b1000_0000, 25)]
    #[case(0b1010_1010, 10)]
    fn should_perform_noop(#[case] value: u8, #[case] idx: u8) {
        let result = bit_set(value, idx);

        assert_eq!(result, value);
    }
}
