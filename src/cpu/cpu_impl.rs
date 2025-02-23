use super::{arithmetic_operators::{add::add, add_c::add_c, add_hl::add_hl, sub::sub, sub_c::sub_c}, arithmetic_target::{get_value_in_arithmetic_target, set_value_in_arithmetic_target}, arithmetic_target_pair::{get_value_in_arithmetic_target_pair, set_value_in_arithmetic_target_pair}, bit::{bit_check::bit_check, bit_reset::bit_reset, bit_set::bit_set}, complement::complement, instruction::{IndDecTarget, Instruction}, logical_operators::{and::and, or::or, xor::xor}, memory_bus::MemoryBus, registers::Registers, rotation_operators::{rotate_left::{rotate_left, rotate_left_through_carry}, rotate_right::{rotate_right, rotate_right_through_carry}, shift_left::shift_left, shift_right_arithmetic::shift_right_arithmetic, shift_right_logical::shift_right_logical, swap_nibbles::swap_nibbles}};

pub struct CPU {
    pub registers: Registers,
    pc: u16,
    bus: MemoryBus,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
            pc: 0,
            bus: MemoryBus::new(),
        }
    }

    fn step(&mut self) {
        let instruction_byte = self.bus.read_byte(self.pc);

        let next_pc = if let Ok(instruction) = Instruction::from_byte(instruction_byte) {
            self.execute(instruction);
        } else {
            panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
        };

        // self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ADD(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = add(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::ADDHL(target) => {
                let value = get_value_in_arithmetic_target_pair(self, &target);
                let new_value = add_hl(self.registers.get_hl(), value, &mut self.registers.f);
                self.registers.set_hl(new_value);
            }
            Instruction::ADDSP(offset) => {
                let value = self.registers.get_sp();
                let new_value = add_hl(value, offset as u16, &mut self.registers.f);
                self.registers.f.zero = false;
                self.registers.set_hl(new_value);
            }
            Instruction::ADC(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = add_c(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::SUB(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = sub(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::SBC(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = sub_c(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::AND(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = and(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::OR(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = or(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::XOR(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = xor(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::CP(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                sub(self.registers.a, value, &mut self.registers.f);
            }
            Instruction::INC(target) => {
                match target {
                    IndDecTarget::Byte(target) => {
                        let value = get_value_in_arithmetic_target(self, &target);
                        let new_value = add(value, 0x01, &mut self.registers.f);
                        set_value_in_arithmetic_target(self, &target, new_value);
                    }
                    IndDecTarget::Word(target) => {
                        let value = get_value_in_arithmetic_target_pair(self, &target);
                        set_value_in_arithmetic_target_pair(self, &target, value + 0x0001);
                    }
                }
            }
            Instruction::DEC(target) => {
                match target {
                    IndDecTarget::Byte(target) => {
                        let value = get_value_in_arithmetic_target(self, &target);
                        let new_value = sub(value, 0x01, &mut self.registers.f);
                        set_value_in_arithmetic_target(self, &target, new_value);
                    }
                    IndDecTarget::Word(target) => {
                        let value = get_value_in_arithmetic_target_pair(self, &target);
                        set_value_in_arithmetic_target_pair(self, &target, value - 0x0001);
                    }
                }
            }
            Instruction::CCF => self.registers.f.carry = !self.registers.f.carry,
            Instruction::SCF => self.registers.f.carry = true,
            Instruction::RRA => {
                let new_value = rotate_right_through_carry(self.registers.a, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::RLA => {
                let new_value = rotate_left_through_carry(self.registers.a, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::RRCA => {
                let new_value = rotate_right(self.registers.a, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::RLCA => {
                let new_value = rotate_left(self.registers.a, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::CPL => {
                let new_value = complement(self.registers.a, &mut self.registers.f);
                self.registers.a = new_value;
            }
            Instruction::BIT(idx, target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                bit_check(value, idx, &mut self.registers.f);
            }
            Instruction::RES(idx, target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = bit_reset(value, idx);
                set_value_in_arithmetic_target(self, &target, new_value);
            }
            Instruction::SET(idx, target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = bit_set(value, idx);
                set_value_in_arithmetic_target(self, &target, new_value);
            }
            Instruction::SRL(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = shift_right_logical(value, &mut self.registers.f);
                set_value_in_arithmetic_target(self, &target, new_value);
            }
            Instruction::RR(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = rotate_right_through_carry(value, &mut self.registers.f);
                self.registers.f.zero = new_value == 0x00;
                set_value_in_arithmetic_target(self, &target, new_value);
            }
            Instruction::RL(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = rotate_left_through_carry(value, &mut self.registers.f);
                self.registers.f.zero = new_value == 0x00;
                set_value_in_arithmetic_target(self, &target, new_value);
            }
            Instruction::RRC(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = rotate_right(value, &mut self.registers.f);
                self.registers.f.zero = new_value == 0x00;
                set_value_in_arithmetic_target(self, &target, new_value);
            }
            Instruction::RLC(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = rotate_left(value, &mut self.registers.f);
                self.registers.f.zero = new_value == 0x00;
                set_value_in_arithmetic_target(self, &target, new_value);
            }
            Instruction::SRA(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = shift_right_arithmetic(value, &mut self.registers.f);
                set_value_in_arithmetic_target(self, &target, new_value);
            }
            Instruction::SLA(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = shift_left(value, &mut self.registers.f);
                set_value_in_arithmetic_target(self, &target, new_value);
            }
            Instruction::SWAP(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = swap_nibbles(value, &mut self.registers.f);
                set_value_in_arithmetic_target(self, &target, new_value);
            }
        }
    }
}
