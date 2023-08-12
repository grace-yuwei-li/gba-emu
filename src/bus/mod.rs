use crate::ppu::Ppu;

pub struct Bus {
    sys_rom: [u8; 0x4000],
    ew_ram: [u8; 0x40000],
    iw_ram: [u8; 0x8000],

    game_pak_rom: Vec<u8>,

    ppu: Ppu,
    // TODO
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            sys_rom: [0; 0x4000],
            ew_ram: [0; 0x40000],
            iw_ram: [0; 0x8000],

            game_pak_rom: vec![0; 0x2000000],

            ppu: Ppu {}
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

    pub fn get(&self, index: usize) -> u32 {
        match index {
            0x3000000 ..= 0x3007fff => {
                let index = index - 0x3000000;
                Self::get_u32(&self.iw_ram, index)
            }
            0x8000000 ..= 0x9ffffff => {
                let index = index - 0x8000000;
                Self::get_u32(&self.game_pak_rom, index)
            },
            _ => todo!("index {:#x} not implemented", index)
        }
    }

    pub fn set(&mut self, index: usize, value: u32) {
        match index {
            0x3000000 ..= 0x3007fff => {
                let index = index - 0x3000000;
                Self::set_u32(&mut self.iw_ram, index, value);
            },
            0x8000000 ..= 0x9ffffff => {
                let index = index - 0x8000000;
                Self::set_u32(&mut self.game_pak_rom, index, value);
            },
            _ => todo!("index {:#x} not implemented", index)
        }
    }
}
