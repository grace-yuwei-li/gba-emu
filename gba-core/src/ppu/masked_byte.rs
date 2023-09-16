use std::ops::BitAnd;

use super::lcd_regs::LcdReg;

#[derive(Debug, Clone, Copy)]
pub struct Masked<T> 
{
    value: T,
    mask: T,
}

impl<T> Masked<T> 
where T: Default + BitAnd<Output = T>
{
    pub fn new(mask: T) -> Self {
        Self {
            value: T::default(),
            mask,
        }
    }
}

impl LcdReg for Masked<u16> {
    fn read(&self) -> u16 {
        self.value
    }

    fn write(&mut self, value: u16) {
        self.value = value & self.mask;
    }
}

impl Masked<u16> {
    pub fn force_write(&mut self, value: u16) {
        self.value = value;
    }
}
