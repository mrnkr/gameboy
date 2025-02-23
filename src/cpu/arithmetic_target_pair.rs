use super::cpu_impl::CPU;

pub enum ArithmeticTargetPair {
    BC,
    DE,
    HL,
}

pub fn get_value_in_arithmetic_target_pair(cpu: &CPU, target: &ArithmeticTargetPair) -> u16 {
    match target {
        ArithmeticTargetPair::BC => cpu.registers.get_bc(),
        ArithmeticTargetPair::DE => cpu.registers.get_de(),
        ArithmeticTargetPair::HL => cpu.registers.get_hl(),
    }
}
