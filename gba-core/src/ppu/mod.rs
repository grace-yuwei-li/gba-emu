mod lcd_regs;
mod utils;

use std::char;

use num_traits::{FromBytes, ToBytes, Zero};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    ppu::utils::decode_color,
    utils::{get, set, AddressableBits, sub_overflows}, bus::{Bus, Interrupt, IoMap},
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
        match self.bg_mode() {
            0 => {
                let bg0cnt = self.lcd_regs.bgcnt[0];
                let character_base_block = usize::from(bg0cnt.bits(2, 3)) * 0x4000;
                let screen_base_block = usize::from(bg0cnt.bits(8, 12)) * 0x800;
                let scroll_x = self.lcd_regs.bgofs[0].bits(0, 8);
                let scroll_y = self.lcd_regs.bgofs[1].bits(0, 8);

                let background_x = self.x + scroll_x;
                let background_y = self.lcd_regs.vcount + scroll_y;

                let mut tile_x = background_x / 8;
                let mut tile_y = background_y / 8;
                let screenblock = self.reg_screenblock(tile_x.into(), tile_y.into());
                tile_x %= 32;
                tile_y %= 32;

                let tile_index = usize::from(tile_x + tile_y * 32);
                let tm_data = u16::from_le_bytes([
                    self.vram[screen_base_block + 0x800 * screenblock + tile_index * 2],
                    self.vram[screen_base_block + 0x800 * screenblock + tile_index * 2 + 1]
                ]);

                let flip_vertical = tm_data.bit(11) == 1;
                let flip_horizontal = tm_data.bit(10) == 1;

                let mut subpixel_x = usize::from(background_x % 8);
                let mut subpixel_y = usize::from(background_y % 8);
                if flip_horizontal {
                    subpixel_x = 7 - subpixel_x;
                }
                if flip_vertical {
                    subpixel_y = 7 - subpixel_y;
                }

                let ts_index: usize = usize::from(tm_data.bits(0, 9));
                let ts_byte = self.vram[character_base_block + 32 * ts_index + 4 * subpixel_y + subpixel_x / 2];

                let palette_offset = if subpixel_x % 2 == 0 {
                    ts_byte.bits(0, 3)
                } else {
                    ts_byte.bits(4, 7)
                };

                let palette_bank = tm_data.bits(12, 15);
                let color = self.palette_lookup(palette_offset.into(), palette_bank.into());

                color
            }
            1 => [128, 128, 0],
            2 => [0, 0, 255],
            3 => {
                let pixel_index: usize = usize::from(self.x + self.lcd_regs.vcount * SCREEN_WIDTH);
                let color = u16::from_le_bytes([
                    self.vram[2 * pixel_index],
                    self.vram[2 * pixel_index + 1],
                ]);
                decode_color(color)
            }
            4 => {
                let pixel_index: usize = usize::from(self.x + self.lcd_regs.vcount * SCREEN_WIDTH);
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

    fn reg_screenblock(&self, tile_x: usize, tile_y: usize) -> usize {
        match self.lcd_regs.bgcnt[0].bits(14, 15) {
            0 => 0,
            1 => (tile_x % 64) / 32,
            2 => (tile_y % 64) / 32,
            3 => ((tile_y % 64) / 32) * 2 + (tile_x % 64)/ 32,
            _ => unreachable!(),
        }
    }

    fn palette_lookup(&self, offset: usize, palette_bank: usize) -> [u8; 3] {
        let index = palette_bank * 32 + 2 * offset;
        let color = u16::from_le_bytes(self.bg_obj_palette[index ..= index + 1].try_into().unwrap());
        decode_color(color.into())
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

    pub fn tick(&mut self, io_map: &mut IoMap) {
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
                io_map.set_interrupt(Interrupt::VBlank, true);
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
