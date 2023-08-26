mod io_map;

use io_map::IoMap;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    ppu::Ppu,
    utils::{get_u32, set_u32},
};

#[wasm_bindgen]
pub struct MemoryDetails {
    vram: Vec<u8>,
}

#[wasm_bindgen]
impl MemoryDetails {
    #[wasm_bindgen(getter)]
    pub fn vram(&self) -> js_sys::Uint8Array {
        js_sys::Uint8Array::from(&self.vram[..])
    }
}

pub struct Bus {
    sys_rom: [u8; 0x4000],
    ew_ram: [u8; 0x40000],
    iw_ram: [u8; 0x8000],

    game_pak_rom: Vec<u8>,

    io_map: IoMap,

    pub(crate) ppu: Ppu,
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            sys_rom: [0; 0x4000],
            ew_ram: [0; 0x40000],
            iw_ram: [0; 0x8000],

            game_pak_rom: vec![0; 0x2000000],

            ppu: Ppu::default(),
            io_map: IoMap::new(),
        }
    }
}

impl Bus {
    pub fn inspect(&self) -> MemoryDetails {
        MemoryDetails {
            vram: self.ppu.vram.clone(),
        }
    }

    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.game_pak_rom[..bytes.len()].clone_from_slice(bytes);
        log::trace!("byte at 0x8000000 is {:#010b}", self.game_pak_rom[0])
    }

    pub fn read(&self, index: u32) -> u32 {
        let index: usize = index.try_into().unwrap();
        match index {
            0x3000000..=0x3007fff => {
                let index = index - 0x3000000;
                get_u32(&self.iw_ram, index)
            }
            0x4000000..=0x400005f => self.ppu.read_lcd_io_regs(index),
            0x4000060..=0x40003fe => self.io_map.read(index),
            0x5000000..=0x7ffffff => self.ppu.read_simple(index),
            0x8000000..=0x9ffffff => {
                let index = index - 0x8000000;
                get_u32(&self.game_pak_rom, index)
            }
            _ => 0 //todo!("index {:#x} not implemented", index),
        }
    }

    pub fn read_half(&self, index: u32) -> u16 {
        self.read(index) as u16
    }

    pub fn write(&mut self, index: u32, value: u32) {
        let index: usize = index.try_into().unwrap();
        match index {
            0x3000000..=0x3007fff => {
                let index = index - 0x3000000;
                set_u32(&mut self.iw_ram, index, value);
            }
            0x4000000..=0x400005f => self.ppu.write_lcd_io_regs(index, value),
            0x4000060..=0x40003fe => self.io_map.write(index, value),
            0x5000000..=0x7ffffff => self.ppu.write_simple(index, value),
            0x8000000..=0x9ffffff => {
                let index = index - 0x8000000;
                set_u32(&mut self.game_pak_rom, index, value);
            }
            _ => todo!("index {:#x} not implemented", index),
        }
    }

    pub fn write_half(&mut self, index: u32, value: u16) {
        let value = (self.read(index) & 0xffff0000) | value as u32;
        self.write(index, value)
    }
}
