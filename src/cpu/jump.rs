use super::{cpu_impl::CPU, flag_registers::FlagsRegister, instruction::JumpTest};

pub fn jump(cpu: &CPU, test: JumpTest) -> u16 {
    jump_internal(cpu, test, 3, || {
        // Gameboy is little endian so read pc + 2 as most significant bit
        // and pc + 1 as least significant bit
        let least_significant_byte = cpu.bus.read_byte(cpu.pc + 1) as u16;
        let most_significant_byte = cpu.bus.read_byte(cpu.pc + 2) as u16;
        (most_significant_byte << 8) | least_significant_byte
    })
}

pub fn jump_relative(cpu: &CPU, test: JumpTest) -> u16 {
    jump_internal(cpu, test, 2, || {
        // The Game Boy's JR instruction uses an 8-bit signed offset relative to the current PC
        let offset = cpu.bus.read_byte(cpu.pc + 1) as i8;
        cpu.pc.wrapping_add(2).wrapping_add(offset as u16)
    })
}

fn jump_internal<F>(cpu: &CPU, test: JumpTest, instruction_size: u16, perform_jump: F) -> u16
    where F: Fn() -> u16
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
