mod io_map;

pub use io_map::Key;
pub use io_map::{Interrupt, IoMap};
use num_traits::{AsPrimitive, FromBytes, ToBytes, Zero};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    cpu::Cpu,
    ppu::Ppu,
    utils::{get, set, AddressableBits},
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
    bios: [u8; 0x4000],
    ew_ram: [u8; 0x40000],
    iw_ram: [u8; 0x8000],

    game_pak_rom: Vec<u8>,

    pub io_map: IoMap,

    pub(crate) ppu: Ppu,
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            bios: include_bytes!("../../og-bios.bin").clone(),
            ew_ram: [0; 0x40000],
            iw_ram: [0; 0x8000],

            game_pak_rom: vec![0; 0x2000000],

            ppu: Ppu::default(),
            io_map: IoMap::new(),
        }
    }
}

impl Bus {
    pub fn set_bios(&mut self, bios: &[u8]) {
        self.bios.clone_from_slice(bios);
    }

    pub fn set_key(&mut self, key: Key, pressed: bool) {
        self.io_map.set_key(key, pressed);
    }

    pub fn inspect(&self) -> MemoryDetails {
        MemoryDetails {
            vram: self.ppu.vram.clone(),
        }
    }

    pub fn load_rom(&mut self, bytes: &[u8]) {
        self.game_pak_rom[..bytes.len()].clone_from_slice(bytes);
    }

    fn read_internal<T, const N: usize>(&self, address: u32, cpu: &Cpu) -> T
    where
        T: FromBytes<Bytes = [u8; N]> + 'static + Copy + AsPrimitive<T>,
        T: Zero,
        u32: AsPrimitive<T>,
    {
        assert_eq!(address % u32::try_from(N).unwrap(), 0);
        let index: usize = address.try_into().unwrap();
        match index {
            0x0000000..=0x1ffffff => get(&self.bios, index & 0x3ffc),
            0x2000000..=0x2ffffff => get(&self.ew_ram, index & 0x3ffff),
            0x3000000..=0x3ffffff => get(&self.iw_ram, index & 0x7fff),
            0x4000000..=0x4ffffff => match index & 0x3ff {
                0..=0x5f => self.ppu.read_lcd_io_regs::<T, N>(index & 0x40003ff),
                0x60..=0x3fe => self.io_map.read(index & 0x40003ff),
                0x3ff => todo!(),
                _ => unreachable!(),
            },
            0x5000000..=0x7ffffff => self.ppu.read_simple::<T, N>(index),
            0x8000000..=0x9ffffff => get(&self.game_pak_rom, index - 0x8000000),
            // There's some timing stuff about these mirrored game pak sections but I'm ignoring
            // that for now.
            0xa000000..=0xbffffff => get(&self.game_pak_rom, index - 0xa000000),
            0xc000000..=0xdffffff => get(&self.game_pak_rom, index - 0xc000000),
            // TODO: Cartridge SRAM
            0xe000000..=0xe00ffff => T::zero(),
            0x1000_0000..=0xffff_ffff => cpu.prefetched_instruction().as_(),
            _ => T::zero(),
        }
    }

    pub fn read(&self, index: u32, cpu: &Cpu) -> u32 {
        let aligned_index = index & 0xfffffffc;
        let value: u32 = self.read_internal(aligned_index, cpu);
        value.rotate_right(8 * index.bits(0, 1))
    }

    pub fn read_half(&self, index: u32, cpu: &Cpu) -> u32 {
        let aligned_index = index & 0xfffffffe;
        let value: u16 = self.read_internal(aligned_index, cpu);
        u32::from(value).rotate_right(8 * index.bit(0))
    }

    pub fn read_signed_half(&self, index: u32, cpu: &Cpu) -> u32 {
        let aligned_index = index & 0xfffffffe;
        let value: u16 = self.read_internal(aligned_index, cpu);
        let extended_value = i32::from(value as i16);
        extended_value.rotate_right(8 * index.bit(0)) as u32
    }

    pub fn read_byte(&self, index: u32, cpu: &Cpu) -> u8 {
        self.read_internal(index, cpu)
    }

    fn write_internal<T, const N: usize>(&mut self, index: u32, value: T)
    where
        T: ToBytes<Bytes = [u8; N]>,
    {
        assert_eq!(index % u32::try_from(N).unwrap(), 0);
        let index: usize = index.try_into().unwrap();
        match index {
            // Don't write to bios.
            0x0000000..=0x1ffffff => {}
            0x2000000..=0x2ffffff => set(&mut self.ew_ram, index & 0x3ffff, value),
            0x3000000..=0x3ffffff => set(&mut self.iw_ram, index & 0x7fff, value),
            0x4000000..=0x4ffffff => match index & 0x3ff {
                0..=0x5f => self.ppu.write_lcd_io_regs(index & 0x40003ff, value),
                0x60..=0x3fe => self.io_map.write(index & 0x40003ff, value),
                0x3ff => todo!(),
                _ => unreachable!(),
            },
            0x5000000..=0x7ffffff => self.ppu.write_simple(index, value),
            // Cartridge ROM - read only?
            0x8000000..=0xdffffff => {}
            // TODO: Cartridge SRAM
            0xe000000..=0xe00ffff => {}
            0x1000_0000..=0xffff_ffff => {}
            _ => todo!("index {:#x} not implemented", index),
        }
    }

    pub fn write(&mut self, index: u32, value: u32) {
        self.write_internal(index & 0xfffffffc, value);
    }

    pub fn write_half(&mut self, index: u32, value: u16) {
        self.write_internal(index & 0xfffffffe, value);
    }

    pub fn write_byte(&mut self, index: u32, value: u8) {
        self.write_internal(index, value);
    }
}
