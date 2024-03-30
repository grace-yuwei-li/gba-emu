use num_traits::{FromBytes, ToBytes};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::utils::AddressableBits;

pub enum Interrupt {
    VBlank,
    HBlank,
    VCount,
}

pub struct IoMap {
    mock: [u8; 0x400],
    keyinput: u8,
    ime: [u8; 4],
    ie: [u8; 2],
    // Normally called 'IF', but 'if' is a keyword.
    pub irq_flags: [u8; 2],
}

#[cfg_attr(feature="debugger", wasm_bindgen)]
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
            ime: [0; 4],
            ie: [0; 2],
            irq_flags: [0; 2],
        }
    }

    pub fn set_interrupt(&mut self, interrupt: Interrupt, value: bool) {
        let bit: usize = match interrupt {
            Interrupt::VBlank => 0,
            Interrupt::HBlank => 1,
            Interrupt::VCount => 2,
        };

        if bit < 8 {
            self.irq_flags[0].mut_bit(bit, value);
        } else {
            self.irq_flags[1].mut_bit(bit - 8, value);
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
        assert!(index >= 0x4000000);
        assert!(index < 0x4000400);

        match index {
            0..=0x3ffffff => {
                unreachable!()
            }
            0x4000130 => self.keyinput,
            0x4000200..=0x4000201 => self.ie[index - 0x4000200],
            0x4000202..=0x4000203 => self.irq_flags[index - 0x4000202],
            0x4000208..=0x400020b => self.ime[index - 0x4000208],
            0x4000000..=0x40003ff => {
                let index = index - BASE_ADDR;
                self.mock[index]
            }
            _ => {
                unreachable!()
            }
        }
    }

    pub fn write<T, const N: usize>(&mut self, index: usize, value: T)
    where
        T: ToBytes<Bytes = [u8; N]>,
    {
        for (i, b) in value.to_le_bytes().into_iter().enumerate() {
            self.write_byte(index + i, b);
        }
    }

    fn write_byte(&mut self, index: usize, value: u8) {
        assert!(index >= 0x4000000);
        assert!(index < 0x4000400);

        match index {
            0..=0x3ffffff => {
                unreachable!()
            }
            0x4000130 => self.keyinput = value,
            0x4000200..=0x4000201 => self.ie[index - 0x4000200] = value,
            0x4000202..=0x4000203 => self.irq_flags[index - 0x4000202] &= !value,
            0x4000208..=0x400020b => self.ime[index - 0x4000208] = value,
            0x4000000..=0x40003ff => {
                let index = index - BASE_ADDR;
                self.mock[index] = value;
            }
            _ => {
                unreachable!()
            }
        };
    }
}
