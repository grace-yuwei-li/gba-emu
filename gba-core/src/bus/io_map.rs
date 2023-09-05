use num_traits::{FromBytes, ToBytes};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::{set, AddressableBits};

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

    pub fn read<T, const N: usize>(&self, index: usize) -> T
    where
        T: FromBytes<Bytes = [u8; N]> + 'static + Copy,
    {
        let mut bytes = [0; N];
        for i in 0..N {
            bytes[i] = self.read_byte(index + i);
        }
        T::from_le_bytes(&bytes)
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

    pub fn write<T, const N: usize>(&mut self, index: usize, value: T)
    where
        T: ToBytes<Bytes = [u8; N]>,
    {
        set(&mut self.mock, index - BASE_ADDR, value);
    }
}
