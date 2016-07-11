pub fn extract_bit(byte: &u8, bit_to_extract: u8) -> u8 {
    return byte.clone() & (1<< (bit_to_extract - 1));
}
