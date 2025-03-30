use instruction_macro::instruction;

use crate::cpu::{
    cpu_impl::CPU,
    instruction::InstructionEntry,
};

#[instruction(0x76)]
fn handle_halt(cpu: &mut CPU) {
    cpu.is_halted = true;
    cpu.pc = cpu.pc.wrapping_add(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_halt() {
        let mut cpu = CPU::new();
        handle_halt(&mut cpu);
        assert!(cpu.is_halted);
    }
}
