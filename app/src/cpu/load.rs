use instruction_macro::instruction;

use crate::cpu::{
    cpu_impl::CPU,
    instruction::InstructionEntry,
};

pub enum LoadByteTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    HLI,
}

pub enum LoadByteSource {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    D8,
    HLI,
}

#[derive(PartialEq)]
pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
    D16,
}

#[derive(PartialEq)]
pub enum LoadWordSource {
    SP,
    D16,
}

pub enum LoadAFromIndirectSource {
    BC,
    DE,
    HLD,
    HLI,
    D16,
}

pub enum LoadIndirectFromATarget {
    BC,
    DE,
    HLD,
    HLI,
    D16,
}

pub enum AFromByteAddressSource {
    C,
    D8,
}

pub enum ByteAddressFromATarget {
    C,
    D8,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource),
    Word(LoadWordTarget, LoadWordSource),
    AFromIndirect(LoadAFromIndirectSource),
    IndirectFromA(LoadIndirectFromATarget),
    AFromByteAddress(AFromByteAddressSource),
    ByteAddressFromA(ByteAddressFromATarget),
}

#[instruction(0x01, LoadType::Word(LoadWordTarget::BC, LoadWordSource::D16))]
#[instruction(0x11, LoadType::Word(LoadWordTarget::DE, LoadWordSource::D16))]
#[instruction(0x21, LoadType::Word(LoadWordTarget::HL, LoadWordSource::D16))]
#[instruction(0x31, LoadType::Word(LoadWordTarget::SP, LoadWordSource::D16))]
#[instruction(0x06, LoadType::Byte(LoadByteTarget::B, LoadByteSource::D8))]
#[instruction(0x0E, LoadType::Byte(LoadByteTarget::C, LoadByteSource::D8))]
#[instruction(0x16, LoadType::Byte(LoadByteTarget::D, LoadByteSource::D8))]
#[instruction(0x1E, LoadType::Byte(LoadByteTarget::E, LoadByteSource::D8))]
#[instruction(0x26, LoadType::Byte(LoadByteTarget::H, LoadByteSource::D8))]
#[instruction(0x2E, LoadType::Byte(LoadByteTarget::L, LoadByteSource::D8))]
#[instruction(0x3E, LoadType::Byte(LoadByteTarget::A, LoadByteSource::D8))]
#[instruction(0x40, LoadType::Byte(LoadByteTarget::B, LoadByteSource::B))]
#[instruction(0x41, LoadType::Byte(LoadByteTarget::B, LoadByteSource::C))]
#[instruction(0x42, LoadType::Byte(LoadByteTarget::B, LoadByteSource::D))]
#[instruction(0x43, LoadType::Byte(LoadByteTarget::B, LoadByteSource::E))]
#[instruction(0x44, LoadType::Byte(LoadByteTarget::B, LoadByteSource::H))]
#[instruction(0x45, LoadType::Byte(LoadByteTarget::B, LoadByteSource::L))]
#[instruction(0x46, LoadType::Byte(LoadByteTarget::B, LoadByteSource::HLI))]
#[instruction(0x47, LoadType::Byte(LoadByteTarget::B, LoadByteSource::A))]
#[instruction(0x48, LoadType::Byte(LoadByteTarget::C, LoadByteSource::B))]
#[instruction(0x49, LoadType::Byte(LoadByteTarget::C, LoadByteSource::C))]
#[instruction(0x4A, LoadType::Byte(LoadByteTarget::C, LoadByteSource::D))]
#[instruction(0x4B, LoadType::Byte(LoadByteTarget::C, LoadByteSource::E))]
#[instruction(0x4C, LoadType::Byte(LoadByteTarget::C, LoadByteSource::H))]
#[instruction(0x4D, LoadType::Byte(LoadByteTarget::C, LoadByteSource::L))]
#[instruction(0x4E, LoadType::Byte(LoadByteTarget::C, LoadByteSource::HLI))]
#[instruction(0x4F, LoadType::Byte(LoadByteTarget::C, LoadByteSource::A))]
#[instruction(0x50, LoadType::Byte(LoadByteTarget::D, LoadByteSource::B))]
#[instruction(0x51, LoadType::Byte(LoadByteTarget::D, LoadByteSource::C))]
#[instruction(0x52, LoadType::Byte(LoadByteTarget::D, LoadByteSource::D))]
#[instruction(0x53, LoadType::Byte(LoadByteTarget::D, LoadByteSource::E))]
#[instruction(0x54, LoadType::Byte(LoadByteTarget::D, LoadByteSource::H))]
#[instruction(0x55, LoadType::Byte(LoadByteTarget::D, LoadByteSource::L))]
#[instruction(0x56, LoadType::Byte(LoadByteTarget::D, LoadByteSource::HLI))]
#[instruction(0x57, LoadType::Byte(LoadByteTarget::D, LoadByteSource::A))]
#[instruction(0x58, LoadType::Byte(LoadByteTarget::E, LoadByteSource::B))]
#[instruction(0x59, LoadType::Byte(LoadByteTarget::E, LoadByteSource::C))]
#[instruction(0x5A, LoadType::Byte(LoadByteTarget::E, LoadByteSource::D))]
#[instruction(0x5B, LoadType::Byte(LoadByteTarget::E, LoadByteSource::E))]
#[instruction(0x5C, LoadType::Byte(LoadByteTarget::E, LoadByteSource::H))]
#[instruction(0x5D, LoadType::Byte(LoadByteTarget::E, LoadByteSource::L))]
#[instruction(0x5E, LoadType::Byte(LoadByteTarget::E, LoadByteSource::HLI))]
#[instruction(0x5F, LoadType::Byte(LoadByteTarget::E, LoadByteSource::A))]
#[instruction(0x60, LoadType::Byte(LoadByteTarget::H, LoadByteSource::B))]
#[instruction(0x61, LoadType::Byte(LoadByteTarget::H, LoadByteSource::C))]
#[instruction(0x62, LoadType::Byte(LoadByteTarget::H, LoadByteSource::D))]
#[instruction(0x63, LoadType::Byte(LoadByteTarget::H, LoadByteSource::E))]
#[instruction(0x64, LoadType::Byte(LoadByteTarget::H, LoadByteSource::H))]
#[instruction(0x65, LoadType::Byte(LoadByteTarget::H, LoadByteSource::L))]
#[instruction(0x66, LoadType::Byte(LoadByteTarget::H, LoadByteSource::HLI))]
#[instruction(0x67, LoadType::Byte(LoadByteTarget::H, LoadByteSource::A))]
#[instruction(0x68, LoadType::Byte(LoadByteTarget::L, LoadByteSource::B))]
#[instruction(0x69, LoadType::Byte(LoadByteTarget::L, LoadByteSource::C))]
#[instruction(0x6A, LoadType::Byte(LoadByteTarget::L, LoadByteSource::D))]
#[instruction(0x6B, LoadType::Byte(LoadByteTarget::L, LoadByteSource::E))]
#[instruction(0x6C, LoadType::Byte(LoadByteTarget::L, LoadByteSource::H))]
#[instruction(0x6D, LoadType::Byte(LoadByteTarget::L, LoadByteSource::L))]
#[instruction(0x6E, LoadType::Byte(LoadByteTarget::L, LoadByteSource::HLI))]
#[instruction(0x6F, LoadType::Byte(LoadByteTarget::L, LoadByteSource::A))]
#[instruction(0x70, LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::B))]
#[instruction(0x71, LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::C))]
#[instruction(0x72, LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::D))]
#[instruction(0x73, LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::E))]
#[instruction(0x74, LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::H))]
#[instruction(0x75, LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::L))]
#[instruction(0x76, LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::HLI))]
#[instruction(0x77, LoadType::Byte(LoadByteTarget::HLI, LoadByteSource::A))]
#[instruction(0x78, LoadType::Byte(LoadByteTarget::A, LoadByteSource::B))]
#[instruction(0x79, LoadType::Byte(LoadByteTarget::A, LoadByteSource::C))]
#[instruction(0x7A, LoadType::Byte(LoadByteTarget::A, LoadByteSource::D))]
#[instruction(0x7B, LoadType::Byte(LoadByteTarget::A, LoadByteSource::E))]
#[instruction(0x7C, LoadType::Byte(LoadByteTarget::A, LoadByteSource::H))]
#[instruction(0x7D, LoadType::Byte(LoadByteTarget::A, LoadByteSource::L))]
#[instruction(0x7E, LoadType::Byte(LoadByteTarget::A, LoadByteSource::HLI))]
#[instruction(0x7F, LoadType::Byte(LoadByteTarget::A, LoadByteSource::A))]
#[instruction(0x0A, LoadType::AFromIndirect(LoadAFromIndirectSource::BC))]
#[instruction(0x1A, LoadType::AFromIndirect(LoadAFromIndirectSource::DE))]
#[instruction(0x2A, LoadType::AFromIndirect(LoadAFromIndirectSource::HLI))]
#[instruction(0x3A, LoadType::AFromIndirect(LoadAFromIndirectSource::HLD))]
#[instruction(0xFA, LoadType::AFromIndirect(LoadAFromIndirectSource::D16))]
#[instruction(0x02, LoadType::IndirectFromA(LoadIndirectFromATarget::BC))]
#[instruction(0x12, LoadType::IndirectFromA(LoadIndirectFromATarget::DE))]
#[instruction(0x22, LoadType::IndirectFromA(LoadIndirectFromATarget::HLI))]
#[instruction(0x32, LoadType::IndirectFromA(LoadIndirectFromATarget::HLD))]
#[instruction(0xEA, LoadType::IndirectFromA(LoadIndirectFromATarget::D16))]
#[instruction(0xF0, LoadType::AFromByteAddress(AFromByteAddressSource::D8))]
#[instruction(0xF2, LoadType::AFromByteAddress(AFromByteAddressSource::C))]
#[instruction(0xE0, LoadType::ByteAddressFromA(ByteAddressFromATarget::D8))]
#[instruction(0xE2, LoadType::ByteAddressFromA(ByteAddressFromATarget::C))]
fn handle_load(cpu: &mut CPU, load_type: LoadType) {
    cpu.pc = load(cpu, load_type);
}

pub fn load(cpu: &mut CPU, load_type: LoadType) -> u16 {
    match load_type {
        LoadType::Byte(target, source) => {
            let source_value = match source {
                LoadByteSource::A => cpu.registers.a,
                LoadByteSource::B => cpu.registers.b,
                LoadByteSource::C => cpu.registers.c,
                LoadByteSource::D => cpu.registers.d,
                LoadByteSource::E => cpu.registers.e,
                LoadByteSource::H => cpu.registers.h,
                LoadByteSource::L => cpu.registers.l,
                LoadByteSource::HLI => cpu.bus.read_byte(cpu.registers.get_hl()),
                LoadByteSource::D8 => cpu.read_next_byte(),
            };
            match target {
                LoadByteTarget::A => cpu.registers.a = source_value,
                LoadByteTarget::B => cpu.registers.b = source_value,
                LoadByteTarget::C => cpu.registers.c = source_value,
                LoadByteTarget::D => cpu.registers.d = source_value,
                LoadByteTarget::E => cpu.registers.e = source_value,
                LoadByteTarget::H => cpu.registers.h = source_value,
                LoadByteTarget::L => cpu.registers.l = source_value,
                LoadByteTarget::HLI => cpu.bus.write_byte(cpu.registers.get_hl(), source_value),
            };
            match source {
                LoadByteSource::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1),
            }
        }
        LoadType::Word(target, source) => {
            let source_value = match source {
                LoadWordSource::SP => cpu.registers.get_sp(),
                LoadWordSource::D16 => cpu.read_next_word(),
            };
            match target {
                LoadWordTarget::BC => cpu.registers.set_bc(source_value),
                LoadWordTarget::DE => cpu.registers.set_de(source_value),
                LoadWordTarget::HL => cpu.registers.set_hl(source_value),
                LoadWordTarget::SP => cpu.registers.set_sp(source_value),
                LoadWordTarget::D16 => cpu.bus.write_word(cpu.read_next_word(), source_value),
            };

            assert!(target != LoadWordTarget::D16 || source != LoadWordSource::D16);

            if target == LoadWordTarget::D16 || source == LoadWordSource::D16 {
                cpu.pc.wrapping_add(3)
            } else {
                cpu.pc.wrapping_add(1)
            }
        }
        LoadType::AFromIndirect(source) => {
            let source_address = match source {
                LoadAFromIndirectSource::BC => cpu.registers.get_bc(),
                LoadAFromIndirectSource::DE => cpu.registers.get_de(),
                LoadAFromIndirectSource::HLD | LoadAFromIndirectSource::HLI => {
                    cpu.registers.get_hl()
                }
                LoadAFromIndirectSource::D16 => cpu.read_next_word(),
            };
            cpu.registers.a = cpu.bus.read_byte(source_address);
            match source {
                LoadAFromIndirectSource::HLD => {
                    cpu.registers.set_hl(source_address.wrapping_sub(1))
                }
                LoadAFromIndirectSource::HLI => {
                    cpu.registers.set_hl(source_address.wrapping_add(1))
                }
                _ => (),
            }
            match source {
                LoadAFromIndirectSource::D16 => cpu.pc.wrapping_add(3),
                _ => cpu.pc.wrapping_add(1),
            }
        }
        LoadType::IndirectFromA(target) => {
            let source_value = cpu.registers.a;
            match target {
                LoadIndirectFromATarget::BC => {
                    cpu.bus.write_byte(cpu.registers.get_bc(), source_value)
                }
                LoadIndirectFromATarget::DE => {
                    cpu.bus.write_byte(cpu.registers.get_de(), source_value)
                }
                LoadIndirectFromATarget::HLD | LoadIndirectFromATarget::HLI => {
                    cpu.bus.write_byte(cpu.registers.get_hl(), source_value)
                }
                LoadIndirectFromATarget::D16 => {
                    cpu.bus.write_byte(cpu.read_next_word(), source_value)
                }
            }
            match target {
                LoadIndirectFromATarget::HLD => {
                    cpu.registers.set_hl(cpu.registers.get_hl().wrapping_sub(1))
                }
                LoadIndirectFromATarget::HLI => {
                    cpu.registers.set_hl(cpu.registers.get_hl().wrapping_add(1))
                }
                _ => (),
            }
            match target {
                LoadIndirectFromATarget::D16 => cpu.pc.wrapping_add(3),
                _ => cpu.pc.wrapping_add(1),
            }
        }
        LoadType::ByteAddressFromA(target) => {
            let source_value = cpu.registers.a;
            match target {
                ByteAddressFromATarget::C => cpu
                    .bus
                    .write_byte(0xFF00 + ((cpu.registers.c & 0x00FF) as u16), source_value),
                ByteAddressFromATarget::D8 => cpu.bus.write_byte(
                    0xFF00 + ((cpu.read_next_byte() & 0x00FF) as u16),
                    source_value,
                ),
            }
            match target {
                ByteAddressFromATarget::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1),
            }
        }
        LoadType::AFromByteAddress(source) => {
            let source_value = match source {
                AFromByteAddressSource::C => cpu
                    .bus
                    .read_byte(0xFF00 + ((cpu.registers.c & 0x00FF) as u16)),
                AFromByteAddressSource::D8 => cpu
                    .bus
                    .read_byte(0xFF00 + ((cpu.read_next_byte() & 0x00FF) as u16)),
            };
            cpu.registers.a = source_value;
            match source {
                AFromByteAddressSource::D8 => cpu.pc.wrapping_add(2),
                _ => cpu.pc.wrapping_add(1),
            }
        }
    }
}
