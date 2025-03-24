use super::cpu_impl::CPU;

pub enum ArithmeticTargetPair {
    BC,
    DE,
    HL,
    SP,
}

pub fn get_value_in_arithmetic_target_pair(cpu: &CPU, target: &ArithmeticTargetPair) -> u16 {
    match target {
        ArithmeticTargetPair::BC => cpu.registers.get_bc(),
        ArithmeticTargetPair::DE => cpu.registers.get_de(),
        ArithmeticTargetPair::HL => cpu.registers.get_hl(),
        ArithmeticTargetPair::SP => cpu.registers.get_sp(),
    }
}

pub fn set_value_in_arithmetic_target_pair(
    cpu: &mut CPU,
    target: &ArithmeticTargetPair,
    new_value: u16,
) {
    match target {
        ArithmeticTargetPair::BC => cpu.registers.set_bc(new_value),
        ArithmeticTargetPair::DE => cpu.registers.set_de(new_value),
        ArithmeticTargetPair::HL => cpu.registers.set_hl(new_value),
        ArithmeticTargetPair::SP => cpu.registers.set_sp(new_value),
    }
}
