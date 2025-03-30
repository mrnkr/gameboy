use instruction_macro::instruction;

use crate::cpu::{
    cpu_impl::CPU,
    flag_registers::FlagsRegister,
    instruction::InstructionEntry,
};

use super::stack::{pop, push};

pub enum JumpTest {
    NotZero,
    Zero,
    NotCarry,
    Carry,
    Always,
}

#[instruction(0xC2, JumpTest::NotZero)]
#[instruction(0xC3, JumpTest::Always)]
#[instruction(0xCA, JumpTest::Zero)]
#[instruction(0xD2, JumpTest::NotCarry)]
#[instruction(0xDA, JumpTest::Carry)]
fn handle_jp(cpu: &mut CPU, test: JumpTest) {
    let next_pc = jump(cpu, test);
    cpu.pc = next_pc;
}

#[instruction(0xE9)]
fn handle_jphl(cpu: &mut CPU) {
    cpu.pc = cpu.registers.get_hl();
}

#[instruction(0x18, JumpTest::Always)]
#[instruction(0x20, JumpTest::NotZero)]
#[instruction(0x28, JumpTest::Zero)]
#[instruction(0x30, JumpTest::NotCarry)]
#[instruction(0x38, JumpTest::Carry)]
fn handle_jr(cpu: &mut CPU, test: JumpTest) {
    let next_pc = jump_relative(cpu, test);
    cpu.pc = next_pc;
}

#[instruction(0xC4, JumpTest::NotZero)]
#[instruction(0xCC, JumpTest::Zero)]
#[instruction(0xCD, JumpTest::Always)]
#[instruction(0xD4, JumpTest::NotCarry)]
#[instruction(0xDC, JumpTest::Carry)]
fn handle_call(cpu: &mut CPU, test: JumpTest) {
    let next_pc = call(cpu, test);
    cpu.pc = next_pc;
}

#[instruction(0xC0, JumpTest::NotZero)]
#[instruction(0xC8, JumpTest::Zero)]
#[instruction(0xC9, JumpTest::Always)]
#[instruction(0xD0, JumpTest::NotCarry)]
#[instruction(0xD8, JumpTest::Carry)]
fn handle_ret(cpu: &mut CPU, test: JumpTest) {
    let next_pc = return_(cpu, test);
    cpu.pc = next_pc;
}

pub fn jump(cpu: &CPU, test: JumpTest) -> u16 {
    jump_internal(cpu, test, 3, || cpu.read_next_word())
}

fn jump_relative(cpu: &CPU, test: JumpTest) -> u16 {
    jump_internal(cpu, test, 2, || {
        // The Game Boy's JR instruction uses an 8-bit signed offset relative to the current PC
        let offset = cpu.bus.read_byte(cpu.pc + 1) as i8;
        cpu.pc.wrapping_add(2).wrapping_add(offset as u16)
    })
}

pub fn call(cpu: &mut CPU, test: JumpTest) -> u16 {
    let next_pc = cpu.pc.wrapping_add(3);
    let should_jump = evaluate_test(&cpu.registers.f, test);
    if should_jump {
      push(cpu, next_pc);
      cpu.read_next_word()
    } else {
      next_pc
    }
}

pub fn return_(cpu: &mut CPU, test: JumpTest) -> u16 {
    let should_jump = evaluate_test(&cpu.registers.f, test);
    if should_jump {
      pop(cpu)
    } else {
      cpu.pc.wrapping_add(1)
    }
}

fn jump_internal<F>(cpu: &CPU, test: JumpTest, instruction_size: u16, perform_jump: F) -> u16
where
    F: Fn() -> u16,
{
    let should_jump = evaluate_test(&cpu.registers.f, test);

    if should_jump {
        perform_jump()
    } else {
        cpu.pc.wrapping_add(instruction_size)
    }
}

fn evaluate_test(flags: &FlagsRegister, test: JumpTest) -> bool {
    match test {
        JumpTest::NotZero => !flags.zero,
        JumpTest::NotCarry => !flags.carry,
        JumpTest::Zero => flags.zero,
        JumpTest::Carry => flags.carry,
        JumpTest::Always => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(JumpTest::NotZero, 0xBA99, true, true, 0x0003)]
    #[case(JumpTest::NotZero, 0xBA99, false, true, 0xBA99)]
    #[case(JumpTest::NotCarry, 0xBA99, true, true, 0x0003)]
    #[case(JumpTest::NotCarry, 0xBA99, true, false, 0xBA99)]
    #[case(JumpTest::Carry, 0xBA99, true, false, 0x0003)]
    #[case(JumpTest::Carry, 0xBA99, true, true, 0xBA99)]
    #[case(JumpTest::Zero, 0xBA99, false, false, 0x0003)]
    #[case(JumpTest::Zero, 0xBA99, true, true, 0xBA99)]
    #[case(JumpTest::Always, 0xBA99, true, true, 0xBA99)]
    #[case(JumpTest::Always, 0xBA99, true, false, 0xBA99)]
    #[case(JumpTest::Always, 0xBA99, false, true, 0xBA99)]
    #[case(JumpTest::Always, 0xBA99, false, false, 0xBA99)]
    fn should_jump(
        #[case] test: JumpTest,
        #[case] requested_pc: u16,
        #[case] zero: bool,
        #[case] carry: bool,
        #[case] expected_pc: u16,
    ) {
        let mut cpu = CPU::new();

        cpu.bus
            .write_word(cpu.pc + 1, requested_pc);

        cpu.registers.f.zero = zero;
        cpu.registers.f.carry = carry;

        let next_pc = jump(&cpu, test);

        assert_eq!(next_pc, expected_pc);
    }

    #[rstest]
    #[case(JumpTest::NotZero, 0x8, true, true, 0xBA9B)]
    #[case(JumpTest::NotZero, -0x8, true, true, 0xBA9B)]
    #[case(JumpTest::NotZero, 0x8, false, true, 0xBAA3)]
    #[case(JumpTest::NotZero, -0x8, false, true, 0xBA93)]
    #[case(JumpTest::NotCarry, 0x8, true, true, 0xBA9B)]
    #[case(JumpTest::NotCarry, -0x8, true, true, 0xBA9B)]
    #[case(JumpTest::NotCarry, 0x8, true, false, 0xBAA3)]
    #[case(JumpTest::NotCarry, -0x8, true, false, 0xBA93)]
    #[case(JumpTest::Carry, 0x8, true, false, 0xBA9B)]
    #[case(JumpTest::Carry, -0x8, true, false, 0xBA9B)]
    #[case(JumpTest::Carry, 0x8, true, true, 0xBAA3)]
    #[case(JumpTest::Carry, -0x8, true, true, 0xBA93)]
    #[case(JumpTest::Zero, 0x8, false, true, 0xBA9B)]
    #[case(JumpTest::Zero, -0x8, false, true, 0xBA9B)]
    #[case(JumpTest::Zero, 0x8, true, true, 0xBAA3)]
    #[case(JumpTest::Zero, -0x8, true, true, 0xBA93)]
    #[case(JumpTest::Always, 0x8, true, true, 0xBAA3)]
    #[case(JumpTest::Always, -0x8, true, true, 0xBA93)]
    #[case(JumpTest::Always, 0x8, true, false, 0xBAA3)]
    #[case(JumpTest::Always, -0x8, true, false, 0xBA93)]
    #[case(JumpTest::Always, 0x8, false, true, 0xBAA3)]
    #[case(JumpTest::Always, -0x8, false, true, 0xBA93)]
    #[case(JumpTest::Always, 0x8, false, false, 0xBAA3)]
    #[case(JumpTest::Always, -0x8, false, false, 0xBA93)]
    fn should_jump_relative(
        #[case] test: JumpTest,
        #[case] requested_offset: i8,
        #[case] zero: bool,
        #[case] carry: bool,
        #[case] expected_pc: u16,
    ) {
        let mut cpu = CPU::new();

        cpu.pc = 0xBA99;

        cpu.bus.write_byte(cpu.pc + 1, requested_offset as u8);

        cpu.registers.f.zero = zero;
        cpu.registers.f.carry = carry;

        let next_pc = jump_relative(&cpu, test);

        assert_eq!(next_pc, expected_pc);
    }

    #[rstest]
    #[case(JumpTest::NotZero, 0xBA99, true, true, 0x0003)]
    #[case(JumpTest::NotZero, 0xBA99, false, true, 0xBA99)]
    #[case(JumpTest::NotCarry, 0xBA99, true, true, 0x0003)]
    #[case(JumpTest::NotCarry, 0xBA99, true, false, 0xBA99)]
    #[case(JumpTest::Carry, 0xBA99, true, false, 0x0003)]
    #[case(JumpTest::Carry, 0xBA99, true, true, 0xBA99)]
    #[case(JumpTest::Zero, 0xBA99, false, false, 0x0003)]
    #[case(JumpTest::Zero, 0xBA99, true, true, 0xBA99)]
    #[case(JumpTest::Always, 0xBA99, true, true, 0xBA99)]
    #[case(JumpTest::Always, 0xBA99, true, false, 0xBA99)]
    #[case(JumpTest::Always, 0xBA99, false, true, 0xBA99)]
    #[case(JumpTest::Always, 0xBA99, false, false, 0xBA99)]
    fn should_call(
        #[case] test: JumpTest,
        #[case] requested_pc: u16,
        #[case] zero: bool,
        #[case] carry: bool,
        #[case] expected_pc: u16,
    ) {
        let mut cpu = CPU::new();

        cpu.bus
            .write_word(cpu.pc + 1, requested_pc);

        cpu.registers.f.zero = zero;
        cpu.registers.f.carry = carry;

        let next_pc = call(&mut cpu, test);

        assert_eq!(next_pc, expected_pc);
    }

    #[rstest]
    #[case(JumpTest::NotZero, true, true, 0x0001)]
    #[case(JumpTest::NotZero, false, true, 0xBA99)]
    #[case(JumpTest::NotCarry, true, true, 0x0001)]
    #[case(JumpTest::NotCarry, true, false, 0xBA99)]
    #[case(JumpTest::Carry, true, false, 0x0001)]
    #[case(JumpTest::Carry, true, true, 0xBA99)]
    #[case(JumpTest::Zero, false, false, 0x0001)]
    #[case(JumpTest::Zero, true, true, 0xBA99)]
    #[case(JumpTest::Always, true, true, 0xBA99)]
    #[case(JumpTest::Always, true, false, 0xBA99)]
    #[case(JumpTest::Always, false, true, 0xBA99)]
    #[case(JumpTest::Always, false, false, 0xBA99)]
    fn should_return(
        #[case] test: JumpTest,
        #[case] zero: bool,
        #[case] carry: bool,
        #[case] expected_pc: u16,
    ) {
        let mut cpu = CPU::new();

        cpu.sp = cpu.sp.wrapping_sub(2);
        cpu.bus.write_word(cpu.sp, 0xBA99);

        cpu.registers.f.zero = zero;
        cpu.registers.f.carry = carry;

        let next_pc = return_(&mut cpu, test);

        assert_eq!(next_pc, expected_pc);
    }
}
