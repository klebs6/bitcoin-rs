// ---------------- [ File: bitcoin-blob/src/hex.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_base_blob_hex {
    (
        $blob_ty:ident,
        $bits:expr,
        $bytes:expr
    ) => {

        impl $blob_ty {

            pub fn get_hex(&self) -> String {
                trace!(
                    "get_hex => converting BaseBlob<{}> to hex string",
                    $bits
                );
                // Traditional Bitcoin/crypto convention: display highest-order byte first
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
                trace!("set_hex => parsing hex string into BaseBlob<{}>", $bits);

                let c_str = unsafe { std::ffi::CStr::from_ptr(psz as *const i8) };
                let hex_str = c_str.to_string_lossy().to_string();
                let trimmed = hex_str.trim_start();
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
                self.data.fill(0);

                let max_hex_len = $bytes * 2;
                let full_hex = maybe_no_0x.trim_end();
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

                let mut byte_index = 0;
                let mut i = digit_count;
                let hex_chars: Vec<char> = full_hex.chars().collect();

                while i > 0 && byte_index < $bytes {
                    i -= 1;
                    let low_nibble_char = hex_chars[i];
                    let low_val = nibble_from_hexchar(low_nibble_char);
                    let mut val: u8 = low_val & 0xF;

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
                    "set_hex => final data (little-end)={:X?}",
                    self.data
                );
            }

            pub fn set_hex_from_str(&mut self, str_: &str) {
                trace!(
                    "set_hex_from_str => converting Rust &str to BaseBlob<{}>",
                    $bits
                );
                let cstring = std::ffi::CString::new(str_).expect("CString::new failed?");
                self.set_hex(cstring.as_ptr() as *const u8);
            }

            pub fn to_string(&self) -> String {
                trace!("to_string => same as get_hex for BaseBlob<{}>", $bits);
                self.get_hex()
            }
        }
    }
}

#[cfg(test)]
mod hex_exhaustive_tests {
    use super::*;
    use std::ffi::CString;
    use tracing::{info, debug};

    #[traced_test]
    fn test_nibble_from_hexchar() {
        info!("Testing nibble_from_hexchar with digits, letters, and invalid chars.");

        // digits 0..9
        for digit in b'0'..=b'9' {
            let ch = digit as char;
            let got = super::nibble_from_hexchar(ch);
            let expected = digit - b'0';
            assert_eq!(
                got, expected,
                "nibble_from_hexchar('{}') => expected {}, got {}",
                ch, expected, got
            );
        }

        // lowercase a..f
        for (ch, val) in [('a', 10), ('b', 11), ('c', 12), ('d', 13), ('e', 14), ('f', 15)] {
            let got = super::nibble_from_hexchar(ch);
            assert_eq!(
                got, val,
                "nibble_from_hexchar('{}') => expected {}, got {}",
                ch, val, got
            );
        }

        // uppercase A..F
        for (ch, val) in [('A', 10), ('B', 11), ('C', 12), ('D', 13), ('E', 14), ('F', 15)] {
            let got = super::nibble_from_hexchar(ch);
            assert_eq!(
                got, val,
                "nibble_from_hexchar('{}') => expected {}, got {}",
                ch, val, got
            );
        }

        // invalid chars => returns 0
        let invalid_chars = ['G', 'z', '!', ' ', '_', 'þ'];
        for ch in invalid_chars {
            let got = super::nibble_from_hexchar(ch);
            assert_eq!(
                got, 0,
                "nibble_from_hexchar('{}') => expected 0 for invalid char",
                ch
            );
        }

        info!("nibble_from_hexchar tests concluded successfully.");
    }

    #[traced_test]
    fn test_get_hex() {
        info!("Testing get_hex() for B=8, B=64, B=256...");
        test_get_hex_8();
        test_get_hex_64();
        test_get_hex_256();
        info!("get_hex tests concluded successfully.");
    }

    fn test_get_hex_8() {
        // all-zero
        let zero_blob = BaseBlob8::default();
        let zero_hex = zero_blob.get_hex();
        let expected_zero_hex = "00".repeat(1);
        assert_eq!(
            zero_hex, expected_zero_hex,
            "all-zero get_hex => expected all '0', B=8"
        );

        // all-ones => "ff"
        let mut ones_blob = BaseBlob8::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }
        let ones_hex = ones_blob.get_hex();
        let expected_ones_hex = "ff".repeat(1);
        assert_eq!(
            ones_hex, expected_ones_hex,
            "all-ones get_hex => expected 'ff', B=8"
        );

        // "mixed" pattern => single byte => let's do 0xAB
        let mut mixed_blob = BaseBlob8::default();
        mixed_blob.data[0] = 0xAB;
        let mixed_hex = mixed_blob.get_hex();
        let expected_mixed_hex = "ab".to_string();
        assert_eq!(
            mixed_hex, expected_mixed_hex,
            "mixed pattern get_hex => mismatch, B=8"
        );
    }
    fn test_get_hex_64() {
        // B=64 => 8 bytes
        let mut zero_blob = BaseBlob64::default();
        let zero_hex = zero_blob.get_hex();
        let expected_zero_hex = "00".repeat(8);
        assert_eq!(
            zero_hex, expected_zero_hex,
            "all-zero get_hex => B=64"
        );

        let mut ones_blob = BaseBlob64::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }
        let ones_hex = ones_blob.get_hex();
        let expected_ones_hex = "ff".repeat(8);
        assert_eq!(
            ones_hex, expected_ones_hex,
            "all-ones get_hex => B=64"
        );

        let mut mixed_blob = BaseBlob64::default();
        for (i, b) in mixed_blob.data.iter_mut().enumerate() {
            *b = i as u8;
        }
        let mixed_hex = mixed_blob.get_hex();
        let mut rev_data = mixed_blob.data.clone();
        rev_data.reverse();
        let expected_mixed_hex = rev_data
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        assert_eq!(
            mixed_hex, expected_mixed_hex,
            "mixed pattern get_hex => mismatch, B=64"
        );
    }
    fn test_get_hex_256() {
        // B=256 => 32 bytes
        let mut zero_blob = BaseBlob256::default();
        let zero_hex = zero_blob.get_hex();
        let expected_zero_hex = "00".repeat(32);
        assert_eq!(
            zero_hex, expected_zero_hex,
            "all-zero get_hex => B=256"
        );

        let mut ones_blob = BaseBlob256::default();
        for b in ones_blob.data.iter_mut() {
            *b = 0xFF;
        }
        let ones_hex = ones_blob.get_hex();
        let expected_ones_hex = "ff".repeat(32);
        assert_eq!(
            ones_hex, expected_ones_hex,
            "all-ones get_hex => B=256"
        );

        let mut mixed_blob = BaseBlob256::default();
        for (i, b) in mixed_blob.data.iter_mut().enumerate() {
            *b = i as u8;
        }
        let mixed_hex = mixed_blob.get_hex();
        let mut rev_data = mixed_blob.data.clone();
        rev_data.reverse();
        let expected_mixed_hex = rev_data
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>();
        assert_eq!(
            mixed_hex, expected_mixed_hex,
            "mixed pattern get_hex => mismatch, B=256"
        );
    }

    #[traced_test]
    fn test_set_hex() {
        info!("Testing set_hex() for B=8, B=64, B=256...");
        test_set_hex_8();
        test_set_hex_64();
        test_set_hex_256();
        info!("set_hex tests concluded successfully.");
    }

    fn test_set_hex_8() {
        let all_zero_hex = "0".repeat(2);
        let mut zero_blob = BaseBlob8::default();
        let cstring = CString::new(all_zero_hex.clone()).unwrap();
        zero_blob.set_hex(cstring.as_ptr() as *const u8);
        assert!(zero_blob.is_null(), "set_hex(all zeros) => is_null() = true, B=8");
    }
    fn test_set_hex_64() {
        let width = 8;
        let all_zero_hex = "0".repeat(width*2);
        let mut zero_blob = BaseBlob64::default();
        let cstring = CString::new(all_zero_hex.clone()).unwrap();
        zero_blob.set_hex(cstring.as_ptr() as *const u8);
        assert!(zero_blob.is_null(), "set_hex(all zeros) => is_null() = true, B=64");
    }
    fn test_set_hex_256() {
        let width = 32;
        let all_zero_hex = "0".repeat(width*2);
        let mut zero_blob = BaseBlob256::default();
        let cstring = CString::new(all_zero_hex.clone()).unwrap();
        zero_blob.set_hex(cstring.as_ptr() as *const u8);
        assert!(zero_blob.is_null(), "set_hex(all zeros) => is_null() = true, B=256");
    }

    #[traced_test]
    fn test_set_hex_from_str() {
        info!("Testing set_hex_from_str() for B=8, B=64, B=256...");
        test_set_hex_from_str_8();
        test_set_hex_from_str_64();
        test_set_hex_from_str_256();
        info!("set_hex_from_str tests concluded successfully.");
    }

    fn test_set_hex_from_str_8() {
        let mut blob = BaseBlob8::default();
        let all_ff = "Ff".repeat(1);
        blob.set_hex_from_str(&all_ff);
        assert_eq!(blob.data[0], 0xFF, "after set_hex_from_str(all_ff), B=8 => data=0xFF");
    }
    fn test_set_hex_from_str_64() {
        let mut blob = BaseBlob64::default();
        let all_ff = "Ff".repeat(8);
        blob.set_hex_from_str(&all_ff);
        for &b in blob.data.iter() {
            assert_eq!(b, 0xFF, "B=64 => set_hex_from_str(all_ff) => all 0xFF");
        }
    }
    fn test_set_hex_from_str_256() {
        let mut blob = BaseBlob256::default();
        let all_ff = "Ff".repeat(32);
        blob.set_hex_from_str(&all_ff);
        for &b in blob.data.iter() {
            assert_eq!(b, 0xFF, "B=256 => set_hex_from_str(all_ff) => all 0xFF");
        }
    }

    #[traced_test]
    fn test_to_string() {
        info!("Testing to_string() => same as get_hex() for B=8, B=64, B=256...");
        test_to_string_8();
        test_to_string_64();
        test_to_string_256();
        info!("to_string tests concluded successfully.");
    }

    fn test_to_string_8() {
        let mut blob = BaseBlob8::default();
        blob.data[0] = 0x7A;
        let s1 = blob.to_string();
        let s2 = blob.get_hex();
        assert_eq!(s1, s2, "to_string() => same as get_hex(), B=8");
    }
    fn test_to_string_64() {
        let mut blob = BaseBlob64::default();
        for (i, b) in blob.data.iter_mut().enumerate() {
            *b = (i*7) as u8;
        }
        let s1 = blob.to_string();
        let s2 = blob.get_hex();
        assert_eq!(s1, s2, "to_string() => same as get_hex(), B=64");
    }
    fn test_to_string_256() {
        let mut blob = BaseBlob256::default();
        for (i, b) in blob.data.iter_mut().enumerate() {
            *b = (i*7) as u8;
        }
        let s1 = blob.to_string();
        let s2 = blob.get_hex();
        assert_eq!(s1, s2, "to_string() => same as get_hex(), B=256");
    }
}
