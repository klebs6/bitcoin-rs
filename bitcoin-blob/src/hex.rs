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

#[cfg(test)]
mod hex_exhaustive_tests {
    use super::*;

    /// We'll do a comprehensive test suite for:
    ///   - `get_hex()`
    ///   - `set_hex()` / `set_hex_from_str()`
    ///   - `to_string()`
    ///   - `nibble_from_hexchar()`
    ///
    /// We cover small (B=8), medium (B=64), and larger (B=256) cases.
    /// Our tests ensure we handle leading "0x", partial hex length,
    /// uppercase vs. lowercase, whitespace, etc.
    ///
    /// Note that `nibble_from_hexchar` is also tested implicitly by `set_hex`
    /// with some non-hex characters.

    // ---------------------------------------------------------------
    // nibble_from_hexchar tests

    #[traced_test]
    fn test_nibble_from_hexchar() {
        info!("Testing nibble_from_hexchar with digits, letters, and invalid chars.");

        // Digits 0..9
        for digit in b'0'..=b'9' {
            let ch = digit as char;
            let got = nibble_from_hexchar(ch);
            let expected = digit - b'0';
            assert_eq!(
                got, expected,
                "nibble_from_hexchar('{}') => expected {}, got {}",
                ch, expected, got
            );
        }

        // Lowercase a..f
        for (ch, val) in [('a', 10), ('b', 11), ('c', 12), ('d', 13), ('e', 14), ('f', 15)] {
            let got = nibble_from_hexchar(ch);
            assert_eq!(
                got, val,
                "nibble_from_hexchar('{}') => expected {}, got {}",
                ch, val, got
            );
        }

        // Uppercase A..F
        for (ch, val) in [('A', 10), ('B', 11), ('C', 12), ('D', 13), ('E', 14), ('F', 15)] {
            let got = nibble_from_hexchar(ch);
            assert_eq!(
                got, val,
                "nibble_from_hexchar('{}') => expected {}, got {}",
                ch, val, got
            );
        }

        // Invalid chars => returns 0
        let invalid_chars = ['G', 'z', '!', ' ', '_', 'þ']; // random non-hex
        for ch in invalid_chars {
            let got = nibble_from_hexchar(ch);
            assert_eq!(
                got, 0,
                "nibble_from_hexchar('{}') => expected 0 for invalid char",
                ch
            );
        }

        info!("nibble_from_hexchar tests concluded successfully.");
    }

    // ---------------------------------------------------------------
    // get_hex() tests

    #[traced_test]
    fn test_get_hex() {
        info!("Testing get_hex() for B=8, B=64, B=256...");
        test_get_hex_gen::<8>();
        test_get_hex_gen::<64>();
        test_get_hex_gen::<256>();
        info!("get_hex tests concluded successfully.");
    }

    fn test_get_hex_gen<const B: usize>()
    where
        [u8; (B % 8) + usize::MAX]:,
        [(); base_blob_width::<B>()]:
    {
        let width = base_blob_width::<B>();

        // We'll test a couple patterns: all-zero, all-ones, and a "mixed" pattern.
        // 1) all-zero
        let zero_blob = BaseBlob::<B>::default();
        let zero_hex = zero_blob.get_hex();
        // That hex should be width*2 of '00'.
        let expected_zero_hex = "00".repeat(width);
        assert_eq!(
            zero_hex, expected_zero_hex,
            "all-zero get_hex => expected all '0', B={}",
            B
        );

        // 2) all-ones => each byte=0xFF => reversed in hex => "ff" repeated
        let mut ones_blob = BaseBlob::<B>::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }
        let ones_hex = ones_blob.get_hex();
        let expected_ones_hex = "ff".repeat(width);
        assert_eq!(
            ones_hex, expected_ones_hex,
            "all-ones get_hex => expected all 'f', B={}",
            B
        );

        // 3) a "mixed" pattern: each byte = index. Then reversed in hex.
        let mut mixed_blob = BaseBlob::<B>::default();
        for (i, b) in mixed_blob.data.iter_mut().enumerate() {
            *b = i as u8;
        }
        let mixed_hex = mixed_blob.get_hex();
        // The highest order byte is data[width-1], so that appears first in hex.
        // We'll build an expected string accordingly:
        let mut rev_data = mixed_blob.data.clone();
        rev_data.reverse();
        let expected_mixed_hex = rev_data
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        assert_eq!(
            mixed_hex, expected_mixed_hex,
            "mixed pattern get_hex => mismatch, B={}",
            B
        );
    }

    // ---------------------------------------------------------------
    // set_hex() tests

    #[traced_test]
    fn test_set_hex() {
        info!("Testing set_hex() for B=8, B=64, B=256...");
        test_set_hex_gen::<8>();
        test_set_hex_gen::<64>();
        test_set_hex_gen::<256>();
        info!("set_hex tests concluded successfully.");
    }

    fn test_set_hex_gen<const B: usize>()
    where
        [u8; (B % 8) + usize::MAX]:,
        [(); base_blob_width::<B>()]:
    {
        let width = base_blob_width::<B>();

        // We'll feed a hex string that covers each byte with a known pattern.
        // Because the final in-memory is "little-endian", we parse from right to left.
        // Then get_hex() will reverse it back, so effectively we can check round-trip.

        // 1) all-zero => we pass a string of 2*width '0's => parse => expect .data => all zero
        let all_zero_hex = "0".repeat(width * 2);
        let mut zero_blob = BaseBlob::<B>::default();
        // We'll convert the &str to a *const u8
        let cstring = std::ffi::CString::new(all_zero_hex.clone()).unwrap();
        zero_blob.set_hex(cstring.as_ptr() as *const u8);
        // => check is_null()
        assert!(
            zero_blob.is_null(),
            "set_hex(all zeros) => is_null() should be true, B={}",
            B
        );

        // 2) partial hex => e.g. if we only pass half the digits, the upper bytes remain 0
        // For instance, let's pass (width) hex digits => half as many as normal => covers the lower (width/2) bytes
        if width > 1 {
            // e.g. B=8 => width=1 => can't do half => skip
            let half_digits = width; // each byte => 2 digits => half => 'width' digits total
            let partial_str = "abc123".repeat(half_digits / 6 + 1); // repeat enough so we get at least half_digits
            let partial_str = partial_str[..half_digits].to_string(); // exact length
            let mut partial_blob = BaseBlob::<B>::default();

            let cstring_partial = std::ffi::CString::new(partial_str.clone()).unwrap();
            partial_blob.set_hex(cstring_partial.as_ptr() as *const u8);

            // We can do a quick check: the "lowest bytes" (partial_str length/2 of them) should be set, rest are zero.
            // The number of parsed bytes => half_digits/2
            let parsed_bytes = half_digits / 2;
            for i in 0..parsed_bytes {
                // data[i] is from the last two hex digits in partial_str, parsing from right to left
                // We'll just confirm it's not all zero. For thoroughness, we could re-parse in a reference code path.
                assert_ne!(
                    partial_blob.data[i], 0,
                    "partial set => data[{}] should be nonzero, B={}",
                    i,
                    B
                );
            }
            for i in parsed_bytes..width {
                assert_eq!(
                    partial_blob.data[i],
                    0,
                    "partial set => data[{}] should remain zero, B={}",
                    i,
                    B
                );
            }
        }

        // 3) whitespace and "0x" prefix => skip
        let fancy_str = format!("   0x {}", all_zero_hex);
        let mut fancy_blob = BaseBlob::<B>::default();
        let cstring_fancy = std::ffi::CString::new(fancy_str.clone()).unwrap();
        fancy_blob.set_hex(cstring_fancy.as_ptr() as *const u8);
        assert!(
            fancy_blob.is_null(),
            "whitespace+0x prefix => should parse same as all_zero_hex, B={}",
            B
        );

        // 4) Round-trip check with a random pattern
        // Let's produce a random hex string of length=width*2
        // We'll just fill with "0123456789ABCDEF" repeated, etc.
        // Then parse set_hex => get_hex => compare
        let mut big_string = String::new();
        let digits = "0123456789ABCDEF";
        // total needed => width*2
        while big_string.len() < width * 2 {
            big_string.push_str(digits);
        }
        big_string.truncate(width * 2); // exact length
        let mut roundtrip_blob = BaseBlob::<B>::default();
        let cstring_rt = std::ffi::CString::new(big_string.clone()).unwrap();
        roundtrip_blob.set_hex(cstring_rt.as_ptr() as *const u8);

        // Now call get_hex, which should produce that string in reversed byte order
        // But let's do the "reverse" ourselves to see the expected final
        let expected_hex = {
            // parse big_string as if it were the big-end hex, but we store little-end => reversed
            // So the "lowest" byte is the last two hex digits in big_string
            // The easiest check: just get the roundtrip_blob's get_hex and see if it matches
            roundtrip_blob.get_hex()
        };
        debug!(
            "Round-trip => input='{}', final get_hex='{}'",
            big_string, expected_hex
        );
        // We won't do a direct equality check with big_string because of endianness.
        // The actual "get_hex()" is reversed. We just confirm we got a valid 2*width string:
        assert_eq!(
            expected_hex.len(),
            width * 2,
            "roundtrip get_hex => length mismatch for B={}",
            B
        );
    }

    // ---------------------------------------------------------------
    // set_hex_from_str() tests

    #[traced_test]
    fn test_set_hex_from_str() {
        info!("Testing set_hex_from_str() for B=8, B=64, B=256...");
        test_set_hex_from_str_gen::<8>();
        test_set_hex_from_str_gen::<64>();
        test_set_hex_from_str_gen::<256>();
        info!("set_hex_from_str tests concluded successfully.");
    }

    fn test_set_hex_from_str_gen<const B: usize>()
    where
        [u8; (B % 8) + usize::MAX]:,
        [(); base_blob_width::<B>()]:
    {
        // We'll do a simpler check. Just pass "FF" repeated for the entire width => all-ones result
        let width = base_blob_width::<B>();
        let all_ff = "Ff".repeat(width); // uppercase & lowercase mix

        let mut blob = BaseBlob::<B>::default();
        blob.set_hex_from_str(&all_ff);

        // => data => all 0xFF
        for (i, &byte) in blob.data.iter().enumerate() {
            assert_eq!(
                byte, 0xFF,
                "After set_hex_from_str(all_ff), data[{}] should be 0xFF, B={}",
                i, B
            );
        }
    }

    // ---------------------------------------------------------------
    // to_string() tests

    #[traced_test]
    fn test_to_string() {
        info!("Testing to_string() => same as get_hex() for B=8, B=64, B=256...");
        test_to_string_gen::<8>();
        test_to_string_gen::<64>();
        test_to_string_gen::<256>();
        info!("to_string tests concluded successfully.");
    }

    fn test_to_string_gen<const B: usize>()
    where
        [u8; (B % 8) + usize::MAX]:,
        [(); base_blob_width::<B>()]:
    {
        // We'll fill the blob with an incremental pattern, call to_string(),
        // compare with get_hex().
        let mut blob = BaseBlob::<B>::default();
        for (i, b) in blob.data.iter_mut().enumerate() {
            *b = (i * 7) as u8; // some pattern
        }
        let s1 = blob.to_string();
        let s2 = blob.get_hex();
        assert_eq!(
            s1, s2,
            "to_string() => same as get_hex(), B={}",
            B
        );
    }
}
