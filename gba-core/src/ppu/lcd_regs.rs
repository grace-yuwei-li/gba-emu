use crate::utils::AddressableBits;

use super::masked_byte::Masked;

pub struct LcdRegs {
    placeholder: Reg,
    pub dispcnt: Reg,
    pub green_swap: Reg,
    pub dispstat: Reg,
    pub vcount: Reg,
    pub bgcnt: [Reg; 4],
    pub bgofs: [Reg; 8],
}

impl Default for LcdRegs {
    fn default() -> Self {
        Self {
            placeholder: Reg::Placeholder,
            dispcnt: Reg::Simple(0),
            green_swap: Reg::Simple(0),
            dispstat: Reg::Masked(Masked::new(0xfff8)),
            vcount: Reg::Simple(0),
            bgcnt: [Reg::Simple(0); 4],
            bgofs: [Reg::Masked(Masked::new(0x01ff)); 8],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Reg {
    Simple(u16),
    Masked(Masked<u16>),
    Placeholder,
}

impl Reg {
    pub fn read(&self) -> u16 {
        match self {
            Reg::Placeholder => 0,
            Reg::Simple(x) => *x,
            Reg::Masked(m) => m.read(),
        }
    }

    pub fn write(&mut self, value: u16) {
        match self {
            Reg::Placeholder => {}
            Reg::Simple(_) => *self = Reg::Simple(value),
            Reg::Masked(m) => m.write(value),
        }
    }

    pub fn force_write(&mut self, value: u16) {
        match self {
            Reg::Masked(ref mut m) => m.force_write(value),
            _ => panic!("Force write only works on masked"),
        }
    }
}

pub(super) trait LcdReg {
    fn read(&self) -> u16;
    fn write(&mut self, value: u16);
}

impl LcdReg for u16 {
    fn read(&self) -> u16 {
        *self
    }

    fn write(&mut self, value: u16) {
        *self = value;
    }
}

impl LcdRegs {
    fn get_halfword(&self, index: usize) -> &Reg {
        // Index should be halfword aligned
        assert!(index % 2 == 0);
        match index - 0x4000000 {
            0x0 => &self.dispcnt,
            0x2 => &self.green_swap,
            0x4 => &self.dispstat,
            0x6 => &self.vcount,
            0x8..=0xf => &self.bgcnt[(index - 0x4000008) / 2],
            0x10..=0x1f => &self.bgofs[(index - 0x4000010) / 2],
            _ => &self.placeholder,
        }
    }

    fn get_halfword_mut(&mut self, index: usize) -> &mut Reg {
        // Index should be halfword aligned
        assert!(index % 2 == 0);
        match index - 0x4000000 {
            0x0 => &mut self.dispcnt,
            0x2 => &mut self.green_swap,
            0x4 => &mut self.dispstat,
            0x6 => &mut self.vcount,
            0x8..=0xf => &mut self.bgcnt[(index - 0x4000008) / 2],
            0x10..=0x1f => &mut self.bgofs[(index - 0x4000010) / 2],
            _ => &mut self.placeholder,
        }
    }

    pub fn read_byte(&self, index: usize) -> u8 {
        if index & 1 == 0 {
            self.get_halfword(index).read() as u8
        } else {
            (self.get_halfword(index - 1).read() >> 8) as u8
        }
    }

    pub fn write_byte(&mut self, index: usize, value: u8) {
        if index & 1 == 0 {
            let mem = self.get_halfword_mut(index);
            mem.write(mem.read().bits(8, 15) | u16::from(value));
        } else {
            let mem = self.get_halfword_mut(index - 1);
            mem.write(mem.read().bits(0, 7) | (u16::from(value) << 8));
        }
    }
}
