use std::ops::Deref;

use crate::emulator_error::EmulatorError;

#[derive(Debug, PartialEq)]
pub struct BitIndex(pub u8);

impl BitIndex {
    pub fn build(value: u8) -> Result<Self, EmulatorError> {
        match value {
            0..=7 => Ok(BitIndex(value)),
            _ => Err(EmulatorError::OutOfBoundsIndex(value)),
        }
    }
}

impl Deref for BitIndex {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(0, Ok(BitIndex(0)))]
    #[case(1, Ok(BitIndex(1)))]
    #[case(3, Ok(BitIndex(3)))]
    #[case(4, Ok(BitIndex(4)))]
    #[case(6, Ok(BitIndex(6)))]
    #[case(7, Ok(BitIndex(7)))]
    #[case(8, Err(EmulatorError::OutOfBoundsIndex(8)))]
    #[case(10, Err(EmulatorError::OutOfBoundsIndex(10)))]
    #[case(25, Err(EmulatorError::OutOfBoundsIndex(25)))]
    fn should_build_bit_index(
        #[case] idx: u8,
        #[case] expected_result: Result<BitIndex, EmulatorError>,
    ) {
        let result = BitIndex::build(idx);
        assert_eq!(result, expected_result);
    }
}
