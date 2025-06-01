// ---------------- [ File: bitcoin-bigint/src/from_str.rs ]
crate::ix!();

#[macro_export]
macro_rules! define_baseuint_fromstr {

    ($uint_type:ident, $bits:expr, $limbs:expr) => {

        impl FromStr for $uint_type {
            type Err = &'static str;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self::from(s))
            }
        }

        impl From<&str> for $uint_type {
            fn from(s_in: &str) -> Self {
                trace!("Converting string '{}' into {} via From<&str>, BITS={}.", s_in, stringify!($uint_type), $bits);

                let mut s = s_in.trim();
                if s.len() > 2 {
                    let prefix = &s[..2];
                    if prefix.eq_ignore_ascii_case("0x") {
                        s = &s[2..];
                    }
                }
                s = s.trim();

                let mut hex_digits = Vec::with_capacity(s.len());
                for ch in s.chars() {
                    match ch {
                        '_' => { continue; }
                        '0'..='9' | 'a'..='f' | 'A'..='F' => {
                            hex_digits.push(ch.to_ascii_uppercase());
                        }
                        _ => {
                            break;
                        }
                    }
                }
                if hex_digits.is_empty() {
                    debug!("No valid hex digits found => returning 0.");
                    return Self::default();
                }

                let mut be_bytes = Vec::with_capacity((hex_digits.len()+1)/2);
                let mut idx = 0usize;

                if (hex_digits.len() % 2) == 1 {
                    let nib = hex_to_val(hex_digits[0]);
                    be_bytes.push(nib as u8);
                    idx = 1;
                }

                while idx+1 < hex_digits.len() {
                    let hi = hex_to_val(hex_digits[idx]);
                    let lo = hex_to_val(hex_digits[idx+1]);
                    let byte_val = ((hi<<4) | lo) as u8;
                    be_bytes.push(byte_val);
                    idx += 2;
                }

                let mut result = Self::default();
                let mut consumed = 0usize;
                let mut limb_idx = 0usize;
                while consumed < be_bytes.len() && limb_idx < $limbs {
                    let mut limb_val = 0u32;
                    for sub_i in 0..4 {
                        if consumed >= be_bytes.len() {
                            break;
                        }
                        let be_idx = be_bytes.len()-1-consumed;
                        let b = be_bytes[be_idx];
                        limb_val |= (b as u32) << (8*sub_i);
                        consumed+=1;
                    }
                    result.pn[limb_idx] = limb_val;
                    limb_idx+=1;
                }

                result
            }
        }
    }
}

#[cfg(test)]
mod from_str_exhaustive_tests {
    use super::*;

    #[traced_test]
    fn test_from_str_empty_and_whitespace() {
        info!("Testing parsing of empty/whitespace strings => zero.");

        // 1) Empty
        let x32 = BaseUInt32::from_str("").unwrap();
        let x64 = BaseUInt64::from_str("").unwrap();
        let x256 = BaseUInt256::from_str("").unwrap();
        // all should be zero
        assert_eq!(x32, BaseUInt32::default());
        assert_eq!(x64, BaseUInt64::default());
        assert_eq!(x256, BaseUInt256::default());

        // 2) Whitespace only
        let w32 = BaseUInt32::from_str("   ").unwrap();
        let w64 = BaseUInt64::from_str("   ").unwrap();
        let w256 = BaseUInt256::from_str("   ").unwrap();
        assert_eq!(w32, BaseUInt32::default());
        assert_eq!(w64, BaseUInt64::default());
        assert_eq!(w256, BaseUInt256::default());

        // 3) "0x" with no digits
        let ox32 = BaseUInt32::from_str("0x").unwrap();
        let ox64 = BaseUInt64::from_str("0x").unwrap();
        let ox256 = BaseUInt256::from_str("0x").unwrap();
        assert_eq!(ox32, BaseUInt32::default());
        assert_eq!(ox64, BaseUInt64::default());
        assert_eq!(ox256, BaseUInt256::default());

        info!("Empty/whitespace/from_str => all zero tests passed.");
    }

    #[traced_test]
    fn test_from_str_partial_stoppage() {
        info!("Testing partial parse stoppage on invalid characters.");

        // We'll pick 64 bits for demonstration. The logic is identical for other bit widths.
        type U64 = BaseUInt64;

        // "deadbeefXYZ" => parse only 'deadbeef' => 0xDEADBEEF
        let partial = U64::from_str("deadbeefXYZ").unwrap();
        let expected = U64::from(0xDEAD_BEEF_u64);
        assert_eq!(partial, expected, "Should parse up to invalid 'X' => 0xDEADBEEF");

        // "1234_5678_G0" => parse up to 'G' => ignoring underscore => "12345678" => 0x12345678
        let partial2 = U64::from_str("1234_5678_G0").unwrap();
        let expected2 = U64::from(0x12345678_u64);
        assert_eq!(partial2, expected2);

        info!("Partial parse stoppage test passed.");
    }

    #[traced_test]
    fn test_from_str_underscores_and_case() {
        info!("Testing underscores removal and mixed-case acceptance.");

        type U64 = BaseUInt64;

        // "0xFFFF_FFFF" => ignoring underscores => "FFFFFFFF" => 0xFFFFFFFF
        let val1 = U64::from_str("0xFFFF_FFFF").unwrap();
        assert_eq!(val1.low64(), 0xFFFF_FFFF);

        // "AbCd_Ef" => ignoring underscore => "ABCDEF" => 0xABCDEF
        let val2 = U64::from_str("AbCd_Ef").unwrap();
        assert_eq!(val2.low64(), 0xABCDEF);

        // "FfFf__Ff" => ignoring underscores => "FFFFFf" => 6 hex nibbles => 0xFFFF_FF
        // => decimal 16777215 => which is 0xFFFFFF.
        // So the correct expected result is 0xFFFFFF (NOT 0xFFFFF).
        let val3 = U64::from_str("FfFf__Ff").unwrap();
        assert_eq!(val3.low64(), 0xFF_FFFF, "Should yield 0xFFFFFF for 'FfFf__Ff'");

        info!("Underscore skipping and case-insensitivity test passed.");
    }

    #[traced_test]
    fn test_from_str_truncation_32_64_256() {
        info!("Testing that large hex strings truncate for smaller bit widths.");

        // A big hex => 24 hex digits => 96 bits => "FFFFFFFFFFFFFFFFFFFFFFFF"
        let big_hex = "FFFFFFFFFFFFFFFFFFFFFFFF";

        // 32 bits => only keep lowest 8 hex digits => 0xFFFFFFFF
        let x32 = BaseUInt32::from_str(big_hex).unwrap();
        assert_eq!(x32.pn[0], 0xFFFF_FFFF);

        // 64 bits => keep 16 hex digits => 0xFFFF_FFFF_FFFF_FFFF
        let x64 = BaseUInt64::from_str(big_hex).unwrap();
        assert_eq!(x64.pn[0], 0xFFFF_FFFF);
        assert_eq!(x64.pn[1], 0xFFFF_FFFF);

        // 256 bits => 96 bits is smaller than 256 => that should fit in 3 limbs => each 0xFFFF_FFFF
        let x256 = BaseUInt256::from_str(big_hex).unwrap();
        assert_eq!(x256.pn[0], 0xFFFF_FFFF);
        assert_eq!(x256.pn[1], 0xFFFF_FFFF);
        assert_eq!(x256.pn[2], 0xFFFF_FFFF);
        for i in 3..8 {
            assert_eq!(x256.pn[i], 0);
        }

        info!("Truncation tests for 32/64/256 bits passed.");
    }

    #[traced_test]
    fn test_from_str_random() {
        info!("Testing random hex generation => parse => compare lower bits in 64 & 256 widths.");

        let mut rng = SimpleLCG::new(0x1234_5678_ABCD_9876);

        for i in 0..30 {
            // random hex up to 48 digits (a bit larger to test truncation)
            let hex_str = random_hex_string(&mut rng, 48);

            // We'll parse for 64 bits, parse for 256 bits
            let parsed64 = BaseUInt64::from_str(&hex_str).unwrap();
            let parsed256 = BaseUInt256::from_str(&hex_str).unwrap();

            // Convert them to the same truncated 64 bits => compare with a "best-effort" approach in normal Rust
            // to handle large hex. If the string is more than 32 nibbles, standard u128 parse can fail/overflow.
            // We'll only parse the *lowest* 32 hex digits in a standard library approach, to match our code's
            // truncation logic.

            let clean_hex: String = hex_str
                .chars()
                .filter(|c| c.is_ascii_hexdigit())
                .collect();
            if clean_hex.is_empty() {
                // both should parse to zero
                assert_eq!(
                    parsed64,
                    BaseUInt64::default(),
                    "Expected parse=0 for empty after cleanup"
                );
                assert_eq!(
                    parsed256,
                    BaseUInt256::default(),
                    "Expected parse=0 for empty after cleanup"
                );
                continue;
            }

            // Keep only the final 32 hex digits, because any extra leading digits
            // would be truncated away in the 64-bit interpretation anyway.
            let truncated = if clean_hex.len() > 32 {
                &clean_hex[clean_hex.len() - 32..]
            } else {
                &clean_hex
            };

            // Now parse that truncated string as u128 (which can hold up to 32 hex digits).
            let val_128 = u128::from_str_radix(truncated, 16).unwrap_or(0);
            let reference_val_64 = (val_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64;

            let got64 = parsed64.low64();
            assert_eq!(
                got64, reference_val_64,
                "Mismatch in 64-bit parse from random hex '{}'",
                hex_str
            );

            // For 256 => also check its low64
            let got256_low64 = parsed256.low64();
            assert_eq!(
                got256_low64, reference_val_64,
                "Mismatch in low64 portion for 256-bit parse from '{}'",
                hex_str
            );
        }

        info!("Random from_str tests for 64 & 256 passed.");
    }

    #[traced_test]
    fn test_std_from_str_trait() {
        info!("Testing that `str.parse::<BaseUInt<BITS>>()` works identically to `From<&str>`.");

        let s1 = "0x1234abcd";
        let parsed64 = "0x1234abcd".parse::<BaseUInt64>().unwrap();
        let direct64 = BaseUInt64::from(s1);
        assert_eq!(parsed64, direct64, "parse::<BaseUInt64>() mismatch");

        let s2 = "ffff_ffff_ffff";
        let parsed256 = s2.parse::<BaseUInt256>().unwrap();
        let direct256 = BaseUInt256::from(s2);
        assert_eq!(parsed256, direct256, "parse::<BaseUInt256>() mismatch");

        // Edge: parse an empty => zero
        let empty32 = "".parse::<BaseUInt32>().unwrap();
        assert_eq!(empty32, BaseUInt32::default());

        info!("std::str::FromStr trait integration tests passed.");
    }
}
