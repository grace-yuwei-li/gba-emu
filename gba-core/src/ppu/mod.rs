mod utils;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{utils::{get_u32, set_u32, AddressableBits}, ppu::utils::bg_mode_3};

use self::utils::bg_mode_4;

pub struct Ppu {
    lcd_regs: Vec<u8>,
    bg_obj_palette: Vec<u8>,
    pub(super) vram: Vec<u8>,
    oam: Vec<u8>,

    vblank_timer: u8,
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

            vblank_timer: 100,
        }
    }
}

impl Ppu {
    // Access to nicely-behaved memory
    pub fn read_simple(&self, index: usize) -> u32 {
        match index {
            0x5000000..=0x50003ff => get_u32(&self.bg_obj_palette, index - 0x5000000),
            0x5000400..=0x5ffffff => 0,
            0x6000000..=0x6017fff => get_u32(&self.vram, index - 0x6000000),
            0x6018000..=0x6ffffff => 0,
            0x7000000..=0x70003ff => get_u32(&self.oam, index - 0x7000000),
            0x7000400..=0x7ffffff => 0,
            _ => unreachable!("{:x}", index),
        }
    }
    pub fn write_simple(&mut self, index: usize, value: u32) {
        match index {
            0x5000000..=0x50003ff => set_u32(&mut self.bg_obj_palette, index - 0x5000000, value),
            0x6000000..=0x6017fff => {
                set_u32(&mut self.vram, index - 0x6000000, value);
            }
            0x7000000..=0x70003ff => set_u32(&mut self.oam, index - 0x7000000, value),
            _ => unreachable!(),
        }
    }

    // Side effects out the wazoo
    pub fn read_lcd_io_regs(&self, index: usize) -> u32 {
        //web_sys::console::log_1(&format!("read from lcdio {:x}", index).into());
        get_u32(&self.lcd_regs, index - 0x4000000)
    }
    pub fn write_lcd_io_regs(&mut self, index: usize, value: u32) {
        //web_sys::console::log_1(&format!("write to lcdio {:x} {:#034b}", index, value).into());
        set_u32(&mut self.lcd_regs, index - 0x4000000, value)
    }

    fn bg_mode(&self) -> u8 {
        self.lcd_regs[0].bits(0, 2)
    }

    fn get_screen(&self) -> Vec<u8> {
        match self.bg_mode() {
            3 => bg_mode_3(&self.vram[0..240 * 160 * 2]),
            4 => bg_mode_4(&self.vram[0..240 * 160]),
            _ => vec![0; 240 * 160 * 4],
        }
    }

    pub fn inspect(&self) -> PpuDetails {
        PpuDetails {
            bg_mode: self.bg_mode(),
            screen: self.get_screen(),
        }
    }

    pub fn tick(&mut self) {
        // Toggle V-Blank flag every 100 cycles
        // Not accurate at all, but lets us proceed in arm.gba
        if self.vblank_timer == 0 {
            self.vblank_timer = 100;
            let dispstat = self.lcd_regs[4];
            let vblank = dispstat.bit(0);
            self.lcd_regs[4] = dispstat.bits(1, 15) | (!vblank & 1);
        } else {
            self.vblank_timer -= 1;
        }
    }
}
