use super::AddressableBits;

pub fn bg_mode_3(input: &[u8]) -> Vec<u8> {
    // Each color takes up two bytes
    debug_assert_eq!(input.len() % 2, 0);

    input
        .chunks_exact(2)
        .flat_map(|chunk| {
            let val = u16::from_le_bytes(chunk.try_into().unwrap());
            decode_color(val)
        })
        .collect()
}

pub fn bg_mode_4(input: &[u8], palette: &[u8]) -> Vec<u8> {
    input
        .iter()
        .flat_map(|&chunk| {
            let color_lo = palette[usize::from(2 * chunk)];
            let color_hi = palette[usize::from(2 * chunk + 1)];
            decode_color(u16::from_le_bytes([color_lo, color_hi]))
        })
        .collect()
}

pub fn decode_color(color: u16) -> [u8; 3] {
    let red = color.bits(0, 4) as u8;
    let green = color.bits(5, 9) as u8;
    let blue = color.bits(10, 14) as u8;

    [
        (red << 3) | (red >> 2),
        (green << 3) | (green >> 2),
        (blue << 3) | (blue >> 2),
    ]
}
