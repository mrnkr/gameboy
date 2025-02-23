use super::{
    arithmetic_operators::{add::add, add_c::add_c, add_hl::add_hl, sub::sub, sub_c::sub_c},
    arithmetic_target::{
        get_value_in_arithmetic_target, set_value_in_arithmetic_target, ArithmeticTarget,
    },
    arithmetic_target_pair::{
        get_value_in_arithmetic_target_pair, set_value_in_arithmetic_target_pair,
    },
    bit::{bit_check::bit_check, bit_reset::bit_reset, bit_set::bit_set},
    complement::complement,
    instruction::{IncDecTarget, Instruction},
    jump::{jump, jump_relative},
    logical_operators::{and::and, or::or, xor::xor},
    memory_bus::MemoryBus,
    registers::Registers,
    rotation_operators::{
        rotate_left::{rotate_left, rotate_left_through_carry},
        rotate_right::{rotate_right, rotate_right_through_carry},
        shift_left::shift_left,
        shift_right_arithmetic::shift_right_arithmetic,
        shift_right_logical::shift_right_logical,
        swap_nibbles::swap_nibbles,
    },
};

pub struct CPU {
    pub registers: Registers,
    pub pc: u16,
    pub bus: MemoryBus,
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
        let mut instruction_byte = self.bus.read_byte(self.pc);
        let prefixed = instruction_byte == 0xCB;
        if prefixed {
            instruction_byte = self.bus.read_byte(self.pc.wrapping_add(1));
        }

        let next_pc = if let Ok(instruction) = Instruction::from_byte(instruction_byte, prefixed) {
            self.execute(instruction)
        } else {
            panic!("Unkown instruction found for: 0x{:x}", instruction_byte);
        };

        self.pc = next_pc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::NOP => self.pc.wrapping_add(1),
            Instruction::JP(test) => jump(self, test),
            Instruction::JPHL => self.registers.get_hl(),
            Instruction::JR(test) => jump_relative(self, test),
            Instruction::ADD(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = add(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::ADDHL(target) => {
                let value = get_value_in_arithmetic_target_pair(self, &target);
                let new_value = add_hl(self.registers.get_hl(), value, &mut self.registers.f);
                self.registers.set_hl(new_value);
                self.pc.wrapping_add(1)
            }
            Instruction::ADDSP => {
                let value = self.registers.get_sp();
                let (offset, pc_increment) =
                    get_value_in_arithmetic_target(self, &ArithmeticTarget::Constant);
                let new_value = add_hl(value, offset as u16, &mut self.registers.f);
                self.registers.f.zero = false;
                self.registers.set_sp(new_value);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::ADC(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = add_c(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::SUB(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = sub(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::SBC(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = sub_c(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::AND(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = and(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::OR(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = or(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::XOR(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = xor(self.registers.a, value, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::CP(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                sub(self.registers.a, value, &mut self.registers.f);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::INC(target) => match target {
                IncDecTarget::Byte(target) => {
                    let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                    let new_value = add(value, 0x01, &mut self.registers.f);
                    set_value_in_arithmetic_target(self, &target, new_value);
                    self.pc.wrapping_add(pc_increment)
                }
                IncDecTarget::Word(target) => {
                    let value = get_value_in_arithmetic_target_pair(self, &target);
                    set_value_in_arithmetic_target_pair(self, &target, value + 0x0001);
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::DEC(target) => match target {
                IncDecTarget::Byte(target) => {
                    let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                    let new_value = sub(value, 0x01, &mut self.registers.f);
                    set_value_in_arithmetic_target(self, &target, new_value);
                    self.pc.wrapping_add(pc_increment)
                }
                IncDecTarget::Word(target) => {
                    let value = get_value_in_arithmetic_target_pair(self, &target);
                    set_value_in_arithmetic_target_pair(self, &target, value - 0x0001);
                    self.pc.wrapping_add(1)
                }
            },
            Instruction::CCF => {
                self.registers.f.carry = !self.registers.f.carry;
                self.pc.wrapping_add(1)
            }
            Instruction::SCF => {
                self.registers.f.carry = true;
                self.pc.wrapping_add(1)
            }
            Instruction::RRA => {
                let new_value = rotate_right_through_carry(self.registers.a, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RLA => {
                let new_value = rotate_left_through_carry(self.registers.a, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RRCA => {
                let new_value = rotate_right(self.registers.a, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RLCA => {
                let new_value = rotate_left(self.registers.a, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::CPL => {
                let new_value = complement(self.registers.a, &mut self.registers.f);
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::BIT(idx, target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                bit_check(value, idx, &mut self.registers.f);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::RES(idx, target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = bit_reset(value, idx);
                set_value_in_arithmetic_target(self, &target, new_value);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::SET(idx, target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = bit_set(value, idx);
                set_value_in_arithmetic_target(self, &target, new_value);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::SRL(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = shift_right_logical(value, &mut self.registers.f);
                set_value_in_arithmetic_target(self, &target, new_value);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::RR(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = rotate_right_through_carry(value, &mut self.registers.f);
                self.registers.f.zero = new_value == 0x00;
                set_value_in_arithmetic_target(self, &target, new_value);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::RL(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = rotate_left_through_carry(value, &mut self.registers.f);
                self.registers.f.zero = new_value == 0x00;
                set_value_in_arithmetic_target(self, &target, new_value);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::RRC(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = rotate_right(value, &mut self.registers.f);
                self.registers.f.zero = new_value == 0x00;
                set_value_in_arithmetic_target(self, &target, new_value);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::RLC(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = rotate_left(value, &mut self.registers.f);
                self.registers.f.zero = new_value == 0x00;
                set_value_in_arithmetic_target(self, &target, new_value);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::SRA(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = shift_right_arithmetic(value, &mut self.registers.f);
                set_value_in_arithmetic_target(self, &target, new_value);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::SLA(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = shift_left(value, &mut self.registers.f);
                set_value_in_arithmetic_target(self, &target, new_value);
                self.pc.wrapping_add(pc_increment)
            }
            Instruction::SWAP(target) => {
                let (value, pc_increment) = get_value_in_arithmetic_target(self, &target);
                let new_value = swap_nibbles(value, &mut self.registers.f);
                set_value_in_arithmetic_target(self, &target, new_value);
                self.pc.wrapping_add(pc_increment)
            }
        }
    }
}
