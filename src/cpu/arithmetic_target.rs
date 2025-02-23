use super::cpu_impl::CPU;

pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    /// operations that affect the value in memory at position [HL]
    HL,
    Constant,
}

pub fn get_value_in_arithmetic_target(cpu: &mut CPU, target: &ArithmeticTarget) -> (u8, u16) {
    match target {
        ArithmeticTarget::A => (cpu.registers.a, 1),
        ArithmeticTarget::B => (cpu.registers.b, 1),
        ArithmeticTarget::C => (cpu.registers.c, 1),
        ArithmeticTarget::D => (cpu.registers.d, 1),
        ArithmeticTarget::E => (cpu.registers.e, 1),
        ArithmeticTarget::H => (cpu.registers.h, 1),
        ArithmeticTarget::L => (cpu.registers.l, 1),
        ArithmeticTarget::HL => {
            let value = cpu.bus.read_byte(cpu.registers.get_hl());
            (value, 1)
        }
        ArithmeticTarget::Constant => (cpu.bus.read_byte(cpu.pc + 1), 2),
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
        ArithmeticTarget::HL => cpu.bus.write_byte(cpu.registers.get_hl(), new_value),
        ArithmeticTarget::Constant => (),
    }
}
