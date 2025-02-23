use crate::emulator_error::EmulatorError;

use super::{arithmetic_target::ArithmeticTarget, arithmetic_target_pair::ArithmeticTargetPair};

pub enum IndDecTarget {
    Byte(ArithmeticTarget),
    Word(ArithmeticTargetPair)
}

pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTargetPair),
    ADDSP(i8),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(IndDecTarget),
    DEC(IndDecTarget),
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RLCA,
    CPL,
    BIT(u8, ArithmeticTarget),
    RES(u8, ArithmeticTarget),
    SET(u8, ArithmeticTarget),
    SRL(ArithmeticTarget),
    RR(ArithmeticTarget),
    RL(ArithmeticTarget),
    RRC(ArithmeticTarget),
    RLC(ArithmeticTarget),
    SRA(ArithmeticTarget),
    SLA(ArithmeticTarget),
    SWAP(ArithmeticTarget),
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Result<Instruction, EmulatorError> {
        match byte {
            // 0x03 => Ok(Instruction::INC(IndDecTarget::)),
            // 0x13 => Ok(Instruction::INC(ArithmeticTargetPair::DE)),
            _ => Err(EmulatorError::UnknownInstruction(byte))
        }
    }
}
