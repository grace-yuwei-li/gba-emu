use crate::utils::AddressableBits;

#[derive(Default)]
pub struct LcdRegs {
    pub dispcnt: u16,
    pub green_swap: u16,
    pub dispstat: u16,
    pub vcount: u16,
}

impl LcdRegs {
    fn read_halfword(&self, index: usize) -> u16 {
        match index - 0x4000000 {
            0x0 => self.dispcnt,
            0x2 => self.green_swap,
            0x4 => self.dispstat,
            0x6 => self.vcount,
            _ => 0,
        }
    }

    fn get_halfword_mut(&mut self, index: usize) -> Option<&mut u16> {
        match index - 0x4000000 {
            0x0 => Some(&mut self.dispcnt),
            0x2 => Some(&mut self.green_swap),
            0x4 => Some(&mut self.dispstat),
            0x6 => None,
            _ => None,
        }
    }

    pub fn read_byte(&self, index: usize) -> u8 {
        if index & 1 == 0 {
            self.read_halfword(index) as u8
        } else {
            (self.read_halfword(index & 0xfffffffe) >> 8) as u8
        }
    }

    pub fn write_byte(&mut self, index: usize, value: u8) {
        if index & 1 == 0 {
            if let Some(mem) = self.get_halfword_mut(index) {
                *mem = mem.bits(8, 15) | u16::from(value);
            }
        } else {
            if let Some(mem) = self.get_halfword_mut(index) {
                *mem = mem.bits(0, 7) | (u16::from(value) << 8);
            }
        }
    }
}
