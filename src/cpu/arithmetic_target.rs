use super::cpu_impl::CPU;

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

pub fn get_value_in_arithmetic_target(cpu: &CPU, target: &ArithmeticTarget) -> u8 {
    match target {
        ArithmeticTarget::A => cpu.registers.a,
        ArithmeticTarget::B => cpu.registers.b,
        ArithmeticTarget::C => cpu.registers.c,
        ArithmeticTarget::D => cpu.registers.d,
        ArithmeticTarget::E => cpu.registers.e,
        ArithmeticTarget::H => cpu.registers.h,
        ArithmeticTarget::L => cpu.registers.l,
    }
}

pub fn set_value_in_arithmetic_target(cpu: &mut CPU, target: &ArithmeticTarget, new_value: u8) {
    match target {
        ArithmeticTarget::A => cpu.registers.a = new_value,
        ArithmeticTarget::B => cpu.registers.b = new_value,
        ArithmeticTarget::C => cpu.registers.c = new_value,
        ArithmeticTarget::D => cpu.registers.d = new_value,
        ArithmeticTarget::E => cpu.registers.e = new_value,
        ArithmeticTarget::H => cpu.registers.h = new_value,
        ArithmeticTarget::L => cpu.registers.l = new_value,
    }
}
