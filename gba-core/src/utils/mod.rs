mod addressable_bits;
pub mod js;
pub mod logging;

pub use addressable_bits::AddressableBits;

pub fn get_u32(slice: &[u8], index: usize) -> u32 {
    u32::from_le_bytes(slice[index..index + 4].try_into().unwrap())
}

pub fn set_u32(slice: &mut [u8], index: usize, value: u32) {
    slice[index..index + 4].copy_from_slice(&value.to_le_bytes());
}
