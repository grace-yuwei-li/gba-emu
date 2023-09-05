mod utils;
use num_traits::{FromBytes, ToBytes, Zero};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    ppu::utils::bg_mode_3,
    utils::{get, set, AddressableBits},
};

use self::utils::bg_mode_4;

pub struct Ppu {
    lcd_regs: Vec<u8>,
    bg_obj_palette: Vec<u8>,
    pub(super) vram: Vec<u8>,
    oam: Vec<u8>,

    vblank_timer: u32,
}

#[wasm_bindgen]
pub struct PpuDetails {
    pub bg_mode: u8,
    screen: Vec<u8>,
}

#[wasm_bindgen]
impl PpuDetails {
    pub fn screen(&self) -> js_sys::Uint8ClampedArray {
        let bytes = &self.screen;
        let array = js_sys::Uint8ClampedArray::new_with_length(bytes.len().try_into().unwrap());
        array.copy_from(bytes);
        array
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            lcd_regs: vec![0; 0x60],
            bg_obj_palette: vec![0; 0x400],
            vram: vec![0; 0x18000],
            oam: vec![0; 0x400],

            vblank_timer: 0,
        }
    }
}

impl Ppu {
    // Access to nicely-behaved memory
    pub fn read_simple<T, const N: usize>(&self, index: usize) -> T
    where
        T: FromBytes<Bytes = [u8; N]> + 'static + Copy,
        T: Zero,
    {
        match index {
            0x5000000..=0x50003ff => get(&self.bg_obj_palette, index - 0x5000000),
            0x5000400..=0x5ffffff => T::zero(),
            0x6000000..=0x6ffffff => get(&self.vram, index - 0x6000000),
            0x7000000..=0x70003ff => get(&self.oam, index - 0x7000000),
            0x7000400..=0x7ffffff => T::zero(),
            _ => unreachable!("{:x}", index),
        }
    }

    pub fn write_simple<T, const N: usize>(&mut self, index: usize, value: T)
    where
        T: ToBytes<Bytes = [u8; N]>,
    {
        match index {
            0x5000000..=0x50003ff => set(&mut self.bg_obj_palette, index - 0x5000000, value),
            0x6000000..=0x6ffffff => {
                set(&mut self.vram, index - 0x6000000, value);
            }
            0x7000000..=0x70003ff => set(&mut self.oam, index - 0x7000000, value),
            _ => unreachable!(),
        }
    }

    // Side effects out the wazoo
    pub fn read_lcd_io_regs<T, const N: usize>(&self, index: usize) -> T
    where
        T: FromBytes<Bytes = [u8; N]> + 'static + Copy,
    {
        get(&self.lcd_regs, index - 0x4000000)
    }

    pub fn write_lcd_io_regs<T, const N: usize>(&mut self, index: usize, value: T)
    where
        T: ToBytes<Bytes = [u8; N]>,
    {
        set(&mut self.lcd_regs, index - 0x4000000, value)
    }

    fn bg_mode(&self) -> u8 {
        self.lcd_regs[0].bits(0, 2)
    }

    fn get_screen(&self) -> Vec<u8> {
        let screen = match self.bg_mode() {
            3 => bg_mode_3(&self.vram[0..240 * 160 * 2]),
            4 => bg_mode_4(&self.vram[0..240 * 160]),
            _ => vec![255; 240 * 160 * 4],
        };
        assert_eq!(screen.len(), 240 * 160 * 4);
        screen
    }

    pub fn inspect(&self) -> PpuDetails {
        PpuDetails {
            bg_mode: self.bg_mode(),
            screen: self.get_screen(),
        }
    }

    pub fn tick(&mut self) {
        // Toggle V-Blank flag every 100000 cycles
        // Not accurate at all, but lets us proceed in arm.gba
        if self.vblank_timer == 0 {
            self.vblank_timer = 100000;
            let dispstat = self.lcd_regs[4];
            let vblank = dispstat.bit(0);
            self.lcd_regs[4] = dispstat.bits(1, 15) | (!vblank & 1);
        } else {
            self.vblank_timer -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_byte_to_vram_works() {
        let mut ppu = Ppu::default();

        assert_eq!(ppu.vram[0], 0);
        ppu.write_simple::<u8, 1>(0x6000000, 0xff);
        assert_eq!(ppu.vram[0], 0xff);
        assert_eq!(ppu.read_simple::<u8, 1>(0x6000000), 0xff);
    }

    #[test]
    fn write_word_to_vram_works() {
        let mut ppu = Ppu::default();

        assert_eq!(ppu.vram[0], 0);
        assert_eq!(ppu.vram[1], 0);
        assert_eq!(ppu.vram[2], 0);
        assert_eq!(ppu.vram[3], 0);
        ppu.write_simple::<u32, 4>(0x6000000, 0x01020304);
        assert_eq!(ppu.vram[0], 4);
        assert_eq!(ppu.vram[1], 3);
        assert_eq!(ppu.vram[2], 2);
        assert_eq!(ppu.vram[3], 1);
        assert_eq!(ppu.read_simple::<u32, 4>(0x6000000), 0x01020304);
    }
}
