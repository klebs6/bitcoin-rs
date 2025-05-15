// ---------------- [ File: bitcoin-blob/src/hex.rs ]
crate::ix!();

impl<const BITS: usize> BaseBlob<BITS> 
where [u8; (BITS % 8) + usize::MAX]: , [(); base_blob_width::<BITS>()]:
{    
    pub fn get_hex(&self) -> String {
        trace!(
            "get_hex => converting BaseBlob<{}> to hex string",
            BITS
        );
        // Traditional Bitcoin/crypto convention: display the highest-order byte first.
        // So we reverse the byte array and then hex-encode.
        let mut rev = self.data.clone();
        rev.reverse();
        let hex_str = rev
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();

        debug!("get_hex => {}", hex_str);
        hex_str
    }

    pub fn set_hex(&mut self, psz: *const u8) {
        trace!("set_hex => parsing hex string into BaseBlob<{}>", BITS);

        // Convert the raw pointer to a Rust slice/string safely (assuming it's valid UTF-8).
        // Typically in C++ we'd skip whitespace, 0x prefix, etc. 
        // Here we'll replicate that logic in Rust. We'll interpret psz as a null-terminated string.
        let c_str = unsafe { std::ffi::CStr::from_ptr(psz as *const i8) };
        let mut hex_str = c_str.to_string_lossy().to_string();

        // Trim leading whitespace:
        let trimmed = hex_str.trim_start();
        // If it starts with "0x" or "0X", skip that:
        let maybe_no_0x = if trimmed.len() >= 2 {
            let prefix = &trimmed[0..2];
            if prefix.eq_ignore_ascii_case("0x") {
                &trimmed[2..]
            } else {
                trimmed
            }
        } else {
            trimmed
        };

        // Clear self
        self.data.fill(0);

        // We only parse as many hex digits as fit in base_blob_width::<BITS>() * 2.
        let max_hex_len = base_blob_width::<BITS>() * 2;
        let full_hex = maybe_no_0x.trim_end();
        // Keep track of how many hex digits are actually present:
        let digit_count = full_hex
            .chars()
            .filter(|ch| ch.is_ascii_hexdigit())
            .count();
        debug!(
            "hex input='{}', digit_count={}, max_hex_len={}",
            full_hex,
            digit_count,
            max_hex_len
        );

        // We'll parse from the end (least significant nibble in the front of data).
        let mut byte_index = 0;
        let mut i = digit_count;
        let hex_chars: Vec<char> = full_hex.chars().collect();

        while i > 0 && byte_index < base_blob_width::<BITS>() {
            // Low nibble
            i -= 1;
            let low_nibble_char = hex_chars[i];
            let low_val = nibble_from_hexchar(low_nibble_char);
            let mut val: u8 = low_val & 0xF;

            // High nibble (if there's still a digit):
            if i > 0 {
                i -= 1;
                let high_nibble_char = hex_chars[i];
                let high_val = nibble_from_hexchar(high_nibble_char);
                val |= (high_val & 0xF) << 4;
            }

            self.data[byte_index] = val;
            byte_index += 1;
        }

        debug!(
            "set_hex => final data (little-end)={:X?}, reversing for big-end convention not needed internally",
            self.data
        );
    }

    pub fn set_hex_from_str(&mut self, str_: &str) {
        trace!(
            "set_hex_from_str => converting Rust &str to BaseBlob<{}>",
            BITS
        );

        // We can just do the same parsing but with a temporary CStr or direct method:
        // Easiest to re-use set_hex with a temporary null-terminated buffer.
        let cstring = std::ffi::CString::new(str_).expect("CString::new failed?");
        self.set_hex(cstring.as_ptr() as *const u8);
    }

    pub fn to_string(&self) -> String {
        trace!("to_string => same as get_hex for BaseBlob<{}>", BITS);
        self.get_hex()
    }
}

/// Helper: convert a single hex char to nibble
fn nibble_from_hexchar(ch: char) -> u8 {
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
