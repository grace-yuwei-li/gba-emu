mod addressable_bits;
pub mod js;
pub mod logging;

pub use addressable_bits::AddressableBits;
use num_traits::{FromBytes, ToBytes};

pub fn get<T, const N: usize>(slice: &[u8], index: usize) -> T
where
    T: FromBytes<Bytes = [u8; N]>,
{
    let bytes: &[u8; N] = slice[index..index + N].try_into().unwrap();
    T::from_le_bytes(bytes)
}

pub fn set<T, const N: usize>(slice: &mut [u8], index: usize, value: T)
where
    T: ToBytes<Bytes = [u8; N]>,
{
    let bytes: &mut [u8] = &mut slice[index..index + N];
    bytes.copy_from_slice(&value.to_le_bytes());
}

pub fn get_u32(slice: &[u8], index: usize) -> u32 {
    u32::from_le_bytes(slice[index..index + 4].try_into().unwrap())
}

pub fn set_u32(slice: &mut [u8], index: usize, value: u32) {
    slice[index..index + 4].copy_from_slice(&value.to_le_bytes());
}

pub fn add_overflows(op1: u32, op2: u32, result: u32) -> bool {
    (op1.bit(31) == op2.bit(31)) && (op1.bit(31) != result.bit(31))
}

pub fn sub_overflows(op1: u32, op2: u32, result: u32) -> bool {
    (op1.bit(31) != op2.bit(31)) && (op1.bit(31) != result.bit(31))
}

pub fn reg_list(reg_list: u32, num_regs: u8) -> String {
    let mut regs: Vec<u8> = vec![];
    for i in 0u8..num_regs {
        if reg_list.bit(i.into()) == 1 {
            regs.push(i);
        }
    }

    if regs.is_empty() {
        return "".to_string();
    }

    let mut to_join: Vec<String> = vec![];
    let mut start: u8 = regs[0];
    let mut prev: u8 = regs[0];
    for reg in regs[1..].into_iter().copied() {
        if reg != prev + 1 {
            if start == prev {
                to_join.push(format!("r{}", start));
            } else {
                to_join.push(format!("r{}-r{}", start, prev));
            }
            start = reg;
            prev = reg;
        } else {
            prev += 1;
        }
    }
    if start == prev {
        to_join.push(format!("r{}", start));
    } else {
        to_join.push(format!("r{}-r{}", start, prev));
    }

    to_join.join(",")
}
