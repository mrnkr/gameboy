use super::{arithmetic_target::ArithmeticTarget, arithmetic_target_pair::ArithmeticTargetPair};

pub enum IncDecTarget {
    Byte(ArithmeticTarget),
    Word(ArithmeticTargetPair),
}

pub mod adc;
pub mod add_hl;
pub mod add_sp;
pub mod add;
pub mod cp;
pub mod dec;
pub mod inc;
pub mod sub;
pub mod sbc;
