crate::ix!();

/// Helper: convert a single hex char to nibble
pub fn nibble_from_hexchar(ch: char) -> u8 {
    match ch {
        '0'..='9' => ch as u8 - b'0',
        'a'..='f' => ch as u8 - b'a' + 10,
        'A'..='F' => ch as u8 - b'A' + 10,
        _ => {
            warn!("nibble_from_hexchar => non-hex input char={}", ch);
            0
        }
    }
}
