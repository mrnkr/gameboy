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
        #[case] expected_pc: u16
    ) {
        let mut cpu = CPU::new();
        
        cpu.bus.write_byte(cpu.pc + 1, (requested_pc & 0x00FF) as u8);
        cpu.bus.write_byte(cpu.pc + 2, ((requested_pc & 0xFF00) >> 8) as u8);

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
        #[case] expected_pc: u16
    ) {
        let mut cpu = CPU::new();
        
        cpu.pc = 0xBA99;

        cpu.bus.write_byte(cpu.pc + 1, requested_offset as u8);

        cpu.registers.f.zero = zero;
        cpu.registers.f.carry = carry;

        let next_pc = jump_relative(&cpu, test);

        assert_eq!(next_pc, expected_pc);
    }
}
