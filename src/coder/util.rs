pub const BITS_IN_BYTE: u8 = 8;

pub fn create_mask(bits: u8) -> u8 {
    1u8.checked_shl(bits as u32).unwrap_or(0).wrapping_sub(1)
}

mod tests {
    #[test]
    fn create_mask() {
        assert_eq!(super::create_mask(0), 0x00);
        assert_eq!(super::create_mask(1), 0x01);
        assert_eq!(super::create_mask(2), 0x03);
        assert_eq!(super::create_mask(4), 0x0F);
        assert_eq!(super::create_mask(8), 0xFF);
    }
}
