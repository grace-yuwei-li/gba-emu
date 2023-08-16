mod io_map;
mod display_map;

use io_map::IoMap;
use display_map::DisplayMap;

pub struct Bus {
    sys_rom: [u8; 0x4000],
    ew_ram: [u8; 0x40000],
    iw_ram: [u8; 0x8000],

    vram: Vec<u8>,

    game_pak_rom: Vec<u8>,

    io_map: IoMap,
    display_map: DisplayMap,
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            sys_rom: [0; 0x4000],
            ew_ram: [0; 0x40000],
            iw_ram: [0; 0x8000],

            vram: vec![0; 0x18000],

            game_pak_rom: vec![0; 0x2000000],

            io_map: IoMap::new(),
            display_map: DisplayMap::new(),
        }
    }
}

impl Bus {
    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.game_pak_rom[..bytes.len()].clone_from_slice(bytes);
        log::trace!("byte at 0x8000000 is {:#010b}", self.game_pak_rom[0])
    }

    fn get_u32(slice: &[u8], index: usize) -> u32 {
        u32::from_le_bytes(slice[index .. index + 4].try_into().unwrap())
    }

    fn set_u32(slice: &mut[u8], index: usize, value: u32) {
        slice[index .. index + 4].copy_from_slice(&value.to_le_bytes());
    }

    pub fn get(&self, index: u32) -> u32 {
        let index: usize = index.try_into().unwrap();
        match index {
            0x3000000 ..= 0x3007fff => {
                let index = index - 0x3000000;
                Self::get_u32(&self.iw_ram, index)
            }
            0x4000000 ..= 0x40003fe => self.io_map.get(index),
            0x5000000 ..= 0x50003ff => self.display_map.get(index),
            0x6000000 ..= 0x6017fff => Self::get_u32(&self.vram, index - 0x6000000),
            0x8000000 ..= 0x9ffffff => {
                let index = index - 0x8000000;
                Self::get_u32(&self.game_pak_rom, index)
            },
            _ => todo!("index {:#x} not implemented", index),
        }
    }

    pub fn get_half(&self, index: u32) -> u16 {
        self.get(index) as u16
    }

    pub fn set(&mut self, index: u32, value: u32) {
        let index: usize = index.try_into().unwrap();
        match index {
            0x3000000 ..= 0x3007fff => {
                let index = index - 0x3000000;
                Self::set_u32(&mut self.iw_ram, index, value);
            },
            0x4000000 ..= 0x40003fe => self.io_map.set(index, value),
            0x5000000 ..= 0x50003ff => self.display_map.set(index, value),
            0x6000000 ..= 0x6017fff => Self::set_u32(&mut self.vram, index - 0x6000000, value),
            0x8000000 ..= 0x9ffffff => {
                let index = index - 0x8000000;
                Self::set_u32(&mut self.game_pak_rom, index, value);
            },
            _ => todo!("index {:#x} not implemented", index)
        }
    }

    pub fn set_half(&mut self, index: u32, value: u16) {
        let value = (self.get(index) & 0xffff0000) | value as u32;
        self.set(index, value)
    }
}
