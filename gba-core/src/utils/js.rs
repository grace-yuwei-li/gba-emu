use std::iter::FlatMap;

use js_sys::Uint8ClampedArray;
use wasm_bindgen::prelude::wasm_bindgen;

use super::AddressableBits;

fn decode_color(color: u16) -> [u8; 4] {
    let red = color.bits(0, 4) as u8;
    let green = color.bits(5, 9) as u8;
    let blue = color.bits(10, 14) as u8;

    [(red << 3) | (red >> 2), (green << 3) | (green >> 2), (blue << 3) | (blue >> 2), 255]
}

pub fn to_canvas_data(input: &[u8]) -> Uint8ClampedArray {
    // Each color takes up two bytes
    assert_eq!(input.len() % 2, 0);
    
    let canvas_vec: Vec<u8> = input
        .chunks_exact(2)
        .flat_map(|chunk| {
            let val = u16::from_le_bytes(chunk.try_into().unwrap());
            decode_color(val)
        })
        .collect();

    let output = Uint8ClampedArray::new_with_length(canvas_vec.len() as u32);
    output.copy_from(&canvas_vec);
    output
}

fn decode_byte(byte: u8) -> Vec<u8> {
    (0 .. 8)
        .into_iter()
        .flat_map(|i| {
            let bit = byte.bit(i);
            [255 * bit, 255 * bit, 255 * bit, 255]
        })
        .collect()
}

#[wasm_bindgen]
pub fn to_canvas_binary_data(input: &[u8]) -> Uint8ClampedArray {
    let canvas_vec: Vec<u8> = input
        .into_iter()
        .flat_map(|&chunk| {
            decode_byte(chunk)
        })
        .collect();

    let output = Uint8ClampedArray::new_with_length(canvas_vec.len() as u32);
    output.copy_from(&canvas_vec);
    output
}
