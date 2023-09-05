use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::AddressableBits;

pub struct IoMap {
    mock: [u8; 0x400],
    keyinput: u8,
}

#[wasm_bindgen]
pub enum Key {
    A,
    B,
    Select,
    Start,
    Right,
    Left,
    Up,
    Down,
    R,
    L,
}

impl Key {
    pub fn bit(&self) -> usize {
        match *self {
            Self::A => 0,
            Self::B => 1,
            Self::Select => 2,
            Self::Start => 3,
            Self::Right => 4,
            Self::Left => 5,
            Self::Up => 6,
            Self::Down => 7,
            Self::R => 8,
            Self::L => 9,
        }
    }
}

const BASE_ADDR: usize = 0x4000000;

impl IoMap {
    pub fn new() -> Self {
        Self { 
            mock: [0; 0x400],
            keyinput: 0xff,
        }
    }

    pub fn set_key(&mut self, key: Key, pressed: bool) {
        self.keyinput.mut_bit(key.bit(), !pressed);
    }

    pub fn read(&self, index: usize) -> u32 {
        u32::from_le_bytes([
            self.read_byte(index),
            self.read_byte(index + 1),
            self.read_byte(index + 2),
            self.read_byte(index + 3),
        ])
    }

    fn read_byte(&self, index: usize) -> u8 {
        match index {
            0x4000130 => self.keyinput,
            _ => {
                let index = index - BASE_ADDR;
                self.mock[index]
            }
        }
    }

    pub fn write(&mut self, index: usize, value: u32) {
        let index = index - BASE_ADDR;
        self.mock[index..index + 4].clone_from_slice(&value.to_le_bytes());
    }
}
