use super::{arithmetic_target::ArithmeticTarget, arithmetic_target_pair::ArithmeticTargetPair};

pub enum Instruction {
    ADD(ArithmeticTarget),
    ADDHL(ArithmeticTargetPair),
    ADC(ArithmeticTarget),
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),
    INC(ArithmeticTarget),
    DEC(ArithmeticTarget),
    CCF,
    SCF,
    RRA,
    RLA,
    RRCA,
    RRLA,
}
