pub trait AddressableBits<T> {
    fn bit(&self, index: usize) -> T;
    fn bits(&self, start: usize, end_inclusive: usize) -> T;
}

impl AddressableBits<u32> for u32 {
    #[inline]
    fn bit(&self, index: usize) -> u32 {
        (self >> index) & 1
    }

    #[inline]
    fn bits(&self, start: usize, end_inclusive: usize) -> u32 {
        let len = end_inclusive - start + 1;
        (self >> start) & ((1 << len) - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits() {
        assert_eq!(0b00111100u32.bits(2, 5), 0b1111);
        assert_eq!(0b00000100u32.bits(2, 2), 1);
        assert_eq!(0b00001000u32.bits(2, 2), 0);
    }
}
