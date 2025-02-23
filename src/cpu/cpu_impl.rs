use super::{add::add, add_c::add_c, add_hl::add_hl, and::and, arithmetic_target::{get_value_in_arithmetic_target, set_value_in_arithmetic_target}, arithmetic_target_pair::get_value_in_arithmetic_target_pair, instruction::Instruction, or::or, registers::Registers, sub::sub, sub_c::sub_c, xor::xor};

pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            registers: Registers::new(),
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
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
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = add(value, 0x01, &mut self.registers.f);
                set_value_in_arithmetic_target(self, &target, new_value);
            }
            Instruction::DEC(target) => {
                let value = get_value_in_arithmetic_target(self, &target);
                let new_value = sub(value, 0x01, &mut self.registers.f);
                set_value_in_arithmetic_target(self, &target, new_value);
            }
        }
    }
}
