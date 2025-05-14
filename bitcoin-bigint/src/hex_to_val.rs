crate::ix!();

// A small helper used by From<&str> for nibble decoding
pub fn hex_to_val(ch: char) -> u8 {
    match ch {
        '0'..='9' => (ch as u8) - b'0',
        'A'..='F' => (ch as u8) - b'A' + 10,
        _ => 0,
    }
}
