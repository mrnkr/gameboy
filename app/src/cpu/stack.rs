use instruction_macro::instruction;

use super::{instruction::InstructionEntry, cpu_impl::CPU};

enum StackTarget {
    BC,
    DE,
    HL,
    AF,
}

#[instruction(0xC5, StackTarget::BC)]
#[instruction(0xD5, StackTarget::DE)]
#[instruction(0xE5, StackTarget::HL)]
#[instruction(0xF5, StackTarget::AF)]
fn handle_push(cpu: &mut CPU, target: StackTarget) {
    let value = match target {
        StackTarget::BC => cpu.registers.get_bc(),
        StackTarget::DE => cpu.registers.get_de(),
        StackTarget::HL => cpu.registers.get_hl(),
        StackTarget::AF => cpu.registers.get_af(),
    };
    push(cpu, value);
    cpu.pc = cpu.pc.wrapping_add(1)
}

pub fn push(cpu: &mut CPU, value: u16) {
    cpu.sp = cpu.sp.wrapping_sub(2);
    cpu.bus.write_word(cpu.sp, value);
}

#[instruction(0xC1, StackTarget::BC)]
#[instruction(0xD1, StackTarget::DE)]
#[instruction(0xE1, StackTarget::HL)]
#[instruction(0xF1, StackTarget::AF)]
fn handle_pop(cpu: &mut CPU, target: StackTarget) {
    let value = pop(cpu);
    match target {
        StackTarget::BC => cpu.registers.set_bc(value),
        StackTarget::DE => cpu.registers.set_de(value),
        StackTarget::HL => cpu.registers.set_hl(value),
        StackTarget::AF => cpu.registers.set_af(value),
    }
    cpu.pc = cpu.pc.wrapping_add(1)
}

pub fn pop(cpu: &mut CPU) -> u16 {
    let value = cpu.bus.read_word(cpu.sp);
    cpu.sp = cpu.sp.wrapping_add(2);
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_push() {
        let mut cpu = CPU::new();

        push(&mut cpu, 0x1234);

        assert_eq!(cpu.sp, 0xFFFC);
        assert_eq!(cpu.bus.read_word(0xFFFC), 0x1234);
    }

    #[test]
    fn should_pop() {
        let mut cpu = CPU::new();
        cpu.sp = 0xFFFC;

        cpu.bus.write_word(0xFFFC, 0x1234);

        let value = pop(&mut cpu);

        assert_eq!(cpu.sp, 0xFFFE);
        assert_eq!(value, 0x1234);
    }
}
