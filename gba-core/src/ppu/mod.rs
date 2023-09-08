mod lcd_regs;
mod utils;

use num_traits::{FromBytes, ToBytes, Zero};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    ppu::utils::decode_color,
    utils::{get, set, AddressableBits},
};

use lcd_regs::LcdRegs;

const SCREEN_WIDTH: u16 = 240;
const SCREEN_HEIGHT: u16 = 160;
const SCREEN_AREA: u16 = SCREEN_WIDTH * SCREEN_HEIGHT;
const H_BLANK_WIDTH: u16 = 68;
const V_BLANK_HEIGHT: u16 = 68;

const DISPSTAT_V_BLANK: usize = 0;
const DISPSTAT_H_BLANK: usize = 1;

pub struct Ppu {
    lcd_regs: LcdRegs,
    bg_obj_palette: Vec<u8>,
    pub(super) vram: Vec<u8>,
    oam: Vec<u8>,

    /* Renderer */
    // Counts 3-2-1-0
    pixel_timer: u8,
    // Current location on screen (or off-screen, during H/V-blank)
    x: u16,
    //y: u16, See lcd_regs.vcount
    //
    screen: Vec<u8>,
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
            lcd_regs: LcdRegs::default(),
            bg_obj_palette: vec![0; 0x400],
            vram: vec![0; 0x18000],
            oam: vec![0; 0x400],

            pixel_timer: 0,
            x: 0,

            screen: vec![0; usize::from(SCREEN_AREA) * 3],
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
            0x5000400..=0x5ffffff => {}
            0x6000000..=0x6ffffff => {
                set(&mut self.vram, index - 0x6000000, value);
            }
            0x7000000..=0x70003ff => set(&mut self.oam, index - 0x7000000, value),
            _ => unreachable!("{:x}", index),
        }
    }

    // Side effects out the wazoo
    pub fn read_lcd_io_regs<T, const N: usize>(&self, index: usize) -> T
    where
        T: FromBytes<Bytes = [u8; N]> + 'static + Copy,
    {
        let mut arr = [0; N];
        for i in 0..N {
            arr[i] = self.lcd_regs.read_byte(index + i);
        }
        T::from_le_bytes(&arr)
    }

    // Side effects out the wazoo
    pub fn write_lcd_io_regs<T, const N: usize>(&mut self, index: usize, value: T)
    where
        T: ToBytes<Bytes = [u8; N]>,
    {
        let arr = value.to_le_bytes();
        for i in 0..N {
            self.lcd_regs.write_byte(index + i, arr[i]);
        }
    }

    fn bg_mode(&self) -> u8 {
        self.lcd_regs.dispcnt.bits(0, 2) as u8
    }

    fn get_pixel(&self) -> [u8; 3] {
        let pixel_index: usize = usize::from(self.x + self.lcd_regs.vcount * SCREEN_WIDTH);
        match self.bg_mode() {
            3 => {
                let color = u16::from_le_bytes([
                    self.vram[2 * pixel_index],
                    self.vram[2 * pixel_index + 1],
                ]);
                decode_color(color)
            }
            4 => {
                let bg = if self.lcd_regs.dispcnt.bit(4) == 0 {
                    &self.vram[0..usize::from(SCREEN_AREA)]
                } else {
                    &self.vram[usize::from(SCREEN_AREA)..usize::from(SCREEN_AREA) * 2]
                };
                let palette = &self.bg_obj_palette;

                let palette_index = 2 * bg[pixel_index];
                let color_lo = palette[usize::from(palette_index)];
                let color_hi = palette[usize::from(palette_index) + 1];

                let color = u16::from_le_bytes([color_lo, color_hi]);
                decode_color(color)
            }
            _ => [255, 255, 255],
        }
    }

    pub fn inspect(&self) -> PpuDetails {
        PpuDetails {
            bg_mode: self.bg_mode(),
            screen: self.get_js_screen(),
        }
    }

    fn get_js_screen(&self) -> Vec<u8> {
        self.screen
            .chunks_exact(3)
            .flat_map(|chunk| [chunk[0], chunk[1], chunk[2], 255])
            .collect()
    }

    pub fn tick(&mut self) {
        if self.pixel_timer == 0 {
            self.pixel_timer = 3;

            // Draw pixel
            if self.x < SCREEN_WIDTH && self.lcd_regs.vcount < SCREEN_HEIGHT {
                let pixel = self.get_pixel();
                let pixel_index = usize::from(self.x + self.lcd_regs.vcount * SCREEN_WIDTH);
                self.screen[3 * pixel_index..3 * pixel_index + 3].clone_from_slice(&pixel);
            }

            self.x += 1;
            if self.x == SCREEN_WIDTH + H_BLANK_WIDTH {
                self.x = 0;
                self.lcd_regs.vcount += 1;
                if self.lcd_regs.vcount == SCREEN_HEIGHT + V_BLANK_HEIGHT {
                    // New frame
                    self.lcd_regs.vcount = 0;
                }
            }

            if self.lcd_regs.vcount == 0 && self.x == 0 {
                // V-Draw starts and V-Blank ends
                self.lcd_regs.dispstat.mut_bit(DISPSTAT_V_BLANK, false);
            } else if self.lcd_regs.vcount < SCREEN_HEIGHT && self.x == 0 {
                // V-Draw starts and H-Blank ends
                self.lcd_regs.dispstat.mut_bit(DISPSTAT_H_BLANK, false);
            } else if self.lcd_regs.vcount < SCREEN_HEIGHT && self.x == SCREEN_WIDTH {
                // H-Blank starts and V-Draw ends
                self.lcd_regs.dispstat.mut_bit(DISPSTAT_H_BLANK, true);
            } else if self.lcd_regs.vcount == SCREEN_HEIGHT {
                // V-Blank starts and H-Blank ends
                self.lcd_regs.dispstat.mut_bit(DISPSTAT_V_BLANK, true);
                self.lcd_regs.dispstat.mut_bit(DISPSTAT_H_BLANK, false);
            }
        } else {
            self.pixel_timer -= 1;
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
