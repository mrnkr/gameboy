use super::flag_registers::FlagsRegister;

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagsRegister,
    pub h: u8,
    pub l: u8,
    sp: u16,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0x00,
            b: 0x00,
            c: 0x00,
            d: 0x00,
            e: 0x00,
            f: FlagsRegister::from(0x00 as u8),
            h: 0x00,
            l: 0x00,
            sp: 0x0000,
        }
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u16::from(self.f)
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = FlagsRegister::from(value);
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_be_able_to_manipulate_bc_virtual_register() {
        let mut registers = Registers::new();
        registers.set_bc(0xABCD);
        assert_eq!(registers.b, 0xAB);
        assert_eq!(registers.c, 0xCD);
        assert_eq!(registers.get_bc(), 0xABCD);
    }

    #[test]
    fn should_be_able_to_manipulate_af_virtual_register() {
        let mut registers = Registers::new();
        registers.set_af(0xABF0);
        assert_eq!(registers.a, 0xAB);
        assert_eq!(u8::from(registers.f), 0xF0);
        assert_eq!(registers.get_af(), 0xABF0);
    }

    #[test]
    fn should_be_able_to_manipulate_de_virtual_register() {
        let mut registers = Registers::new();
        registers.set_de(0xABCD);
        assert_eq!(registers.d, 0xAB);
        assert_eq!(registers.e, 0xCD);
        assert_eq!(registers.get_de(), 0xABCD);
    }

    #[test]
    fn should_be_able_to_manipulate_hl_virtual_register() {
        let mut registers = Registers::new();
        registers.set_hl(0xABCD);
        assert_eq!(registers.h, 0xAB);
        assert_eq!(registers.l, 0xCD);
        assert_eq!(registers.get_hl(), 0xABCD);
    }
}
