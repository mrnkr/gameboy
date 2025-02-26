use super::cpu_impl::CPU;

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

pub enum LoadWordTarget {
    BC,
    DE,
    HL,
    SP,
}

pub enum LoadWordSource {
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

pub fn load(cpu: &mut CPU, load_type: &LoadType) -> u16 {
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
                LoadWordSource::D16 => cpu.read_next_word(),
            };
            match target {
                LoadWordTarget::BC => cpu.registers.set_bc(source_value),
                LoadWordTarget::DE => cpu.registers.set_de(source_value),
                LoadWordTarget::HL => cpu.registers.set_hl(source_value),
                LoadWordTarget::SP => cpu.registers.set_sp(source_value),
            };
            match source {
                LoadWordSource::D16 => cpu.pc.wrapping_add(3),
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
