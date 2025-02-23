#[derive(Debug, PartialEq)]
pub enum EmulatorError {
    OutOfBoundsIndex(u8),
    UnknownInstruction(u8),
}
