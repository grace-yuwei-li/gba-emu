use super::AddressableBits;

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
