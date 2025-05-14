crate::ix!();

// Optionally, we can also provide a FromStr impl so `str.parse::<BaseUInt<BITS>>()` works:
impl<const BITS: usize> std::str::FromStr for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

impl<const BITS: usize> From<&str> for BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    fn from(s_in: &str) -> Self {
        trace!("Converting string '{s_in}' into BaseUInt<{BITS}> via From<&str>.");

        // 1) Trim and remove optional "0x"/"0X" prefix
        let mut s = s_in.trim();
        if s.len() > 2 {
            let prefix = &s[..2];
            if prefix.eq_ignore_ascii_case("0x") {
                s = &s[2..];
            }
        }
        s = s.trim();

        // 2) Collect valid hex digits, ignoring underscores. Stop at first truly invalid (non-hex, non-underscore).
        //    This accommodates strings like "0xFFFF_FFFF" or "deadbeefXYZ", etc.
        let mut hex_digits = Vec::with_capacity(s.len());
        for ch in s.chars() {
            match ch {
                '_' => {
                    // skip underscore entirely
                    continue;
                }
                '0'..='9' | 'a'..='f' | 'A'..='F' => {
                    // push uppercase nibble
                    hex_digits.push(ch.to_ascii_uppercase());
                }
                _ => {
                    // stop on first invalid
                    break;
                }
            }
        }

        // If no valid hex, return 0
        if hex_digits.is_empty() {
            debug!("No valid hex digits found; returning 0.");
            return Self::default();
        }

        debug!("Filtered/uppercase hex digits (underscores skipped): {:?}", hex_digits);

        // 3) Build a big-endian byte array from these hex digits, left→right => [0x12,0x34,0xAB,0xCD] for "1234ABCD".
        let mut be_bytes = Vec::with_capacity((hex_digits.len() + 1) / 2);
        let mut idx = 0usize;

        // If there's an odd number of digits, handle the first nibble alone:
        if (hex_digits.len() % 2) == 1 {
            let nib = hex_to_val(hex_digits[0]);
            be_bytes.push(nib as u8);
            idx = 1;
        }
        // Then handle pairs
        while idx + 1 < hex_digits.len() {
            let hi = hex_to_val(hex_digits[idx]);
            let lo = hex_to_val(hex_digits[idx + 1]);
            let byte_val = ((hi << 4) | lo) as u8;
            be_bytes.push(byte_val);
            idx += 2;
        }

        debug!(
            "Constructed big-endian byte array (length={}): {:02X?}",
            be_bytes.len(),
            be_bytes
        );

        // 4) Convert the big-endian byte array into our internal little-endian limbs:
        //    - We'll read from the right end of be_bytes to get the least-significant bytes first.
        //    - Each limb is up to 4 bytes in little-endian order (lowest byte in the lowest bits).
        //    - No extra "rotate" or "swap" is done. We just store the raw bytes in typical LE fashion.
        let mut result = Self::default();
        let limb_count = BITS / 32;

        let mut consumed = 0usize;
        let mut limb_idx = 0usize;

        while consumed < be_bytes.len() && limb_idx < limb_count {
            let mut limb_val = 0u32;
            // Take up to 4 bytes from the right end of be_bytes
            for sub_i in 0..4 {
                if consumed >= be_bytes.len() {
                    break;
                }
                // index from the end: 0 => last byte, 1 => second last, ...
                let be_idx = be_bytes.len() - 1 - consumed;
                let b = be_bytes[be_idx];
                limb_val |= (b as u32) << (8 * sub_i);
                consumed += 1;
            }

            debug!(
                "Storing limb {} => 0x{:08X} from the last {} bytes of be_bytes",
                limb_idx, limb_val, consumed
            );
            result.pn[limb_idx] = limb_val;
            limb_idx += 1;
        }

        info!("Finished parsing hex into BaseUInt<{BITS}>. Returning result.");
        result
    }
}

#[cfg(test)]
mod from_str_exhaustive_tests {
    use super::*;
    use std::str::FromStr;

    #[traced_test]
    fn test_from_str_empty_and_whitespace() {
        info!("Testing parsing of empty/whitespace strings => zero.");

        // 1) Empty
        let x32 = BaseUInt::<32>::from_str("").unwrap();
        let x64 = BaseUInt::<64>::from_str("").unwrap();
        let x256 = BaseUInt::<256>::from_str("").unwrap();
        // all should be zero
        assert_eq!(x32, BaseUInt::<32>::default());
        assert_eq!(x64, BaseUInt::<64>::default());
        assert_eq!(x256, BaseUInt::<256>::default());

        // 2) Whitespace only
        let w32 = BaseUInt::<32>::from_str("   ").unwrap();
        let w64 = BaseUInt::<64>::from_str("   ").unwrap();
        let w256 = BaseUInt::<256>::from_str("   ").unwrap();
        assert_eq!(w32, BaseUInt::<32>::default());
        assert_eq!(w64, BaseUInt::<64>::default());
        assert_eq!(w256, BaseUInt::<256>::default());

        // 3) "0x" with no digits
        let ox32 = BaseUInt::<32>::from_str("0x").unwrap();
        let ox64 = BaseUInt::<64>::from_str("0x").unwrap();
        let ox256 = BaseUInt::<256>::from_str("0x").unwrap();
        assert_eq!(ox32, BaseUInt::<32>::default());
        assert_eq!(ox64, BaseUInt::<64>::default());
        assert_eq!(ox256, BaseUInt::<256>::default());

        info!("Empty/whitespace/from_str => all zero tests passed.");
    }

    #[traced_test]
    fn test_from_str_partial_stoppage() {
        info!("Testing partial parse stoppage on invalid characters.");

        // We'll pick 64 bits for demonstration. The logic is the same for other bit widths.
        type U64 = BaseUInt<64>;

        // "deadbeefXYZ" => parse only 'deadbeef' => 0xDEADBEEF
        let partial = U64::from_str("deadbeefXYZ").unwrap();
        let expected = U64::from(0xDEAD_BEEFu64);
        assert_eq!(partial, expected, "Should parse up to invalid 'X' => 0xDEADBEEF");

        // "1234_5678_G0" => parse up to 'G' => ignoring underscore => "12345678" => 0x12345678
        let partial2 = U64::from_str("1234_5678_G0").unwrap();
        let expected2 = U64::from(0x12345678u64);
        assert_eq!(partial2, expected2);

        info!("Partial parse stoppage test passed.");
    }

    #[traced_test]
    fn test_from_str_underscores_and_case() {
        info!("Testing underscores removal and mixed-case acceptance.");

        type U64 = BaseUInt<64>;
        // "0xFFFF_FFFF" => 0xFFFFFFFF
        let val1 = U64::from_str("0xFFFF_FFFF").unwrap();
        assert_eq!(val1.low64(), 0xFFFF_FFFF);

        // "AbCd_Ef" => parse => 0xABCDEF
        let val2 = U64::from_str("AbCd_Ef").unwrap();
        assert_eq!(val2.low64(), 0xABCDEF);

        // "FfFf__Ff" => => 0xFFFFF
        let val3 = U64::from_str("FfFf__Ff").unwrap();
        assert_eq!(val3.low64(), 0xFFFFF);

        info!("Underscore skipping and case-insensitivity test passed.");
    }

    #[traced_test]
    fn test_from_str_truncation_32_64_256() {
        info!("Testing that large hex strings truncate for smaller bit widths.");

        // A big hex => 24 hex digits => 96 bits => "FFFFFFFFFFFFFFFFFFFFFFFF"
        let big_hex = "FFFFFFFFFFFFFFFFFFFFFFFF";

        // 32 bits => only keep lowest 8 hex digits => 0xFFFFFFFF
        let x32 = BaseUInt::<32>::from_str(big_hex).unwrap();
        assert_eq!(x32.pn[0], 0xFFFF_FFFF);

        // 64 bits => keep 16 hex digits => 0xFFFF_FFFF_FFFF_FFFF
        let x64 = BaseUInt::<64>::from_str(big_hex).unwrap();
        assert_eq!(x64.pn[0], 0xFFFF_FFFF);
        assert_eq!(x64.pn[1], 0xFFFF_FFFF);

        // 256 bits => 96 bits is smaller than 256 => all 3 limbs become 0xFFFF_FFFF, rest zero
        let x256 = BaseUInt::<256>::from_str(big_hex).unwrap();
        // 24 nibble digits => 12 bytes => 3 limbs of 32 bits => all 0xFFFF_FFFF
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
            // random hex up to 40 digits
            let hex_str = random_hex_string(&mut rng, 40);

            // We'll parse for 64 bits, parse for 256 bits
            let parsed64 = BaseUInt::<64>::from_str(&hex_str).unwrap();
            let parsed256 = BaseUInt::<256>::from_str(&hex_str).unwrap();

            // Convert them to the same truncated 64 bits => compare with a direct BigInt parse in standard Rust if we want
            // For simplicity, we can replicate the same parsing logic in 128 or large, but let's do a quick approach:
            let reference_val_64 = {
                // We'll do a best-effort parse in Rust standard library ignoring underscores
                let clean_hex: String = hex_str
                    .chars()
                    .filter(|c| c.is_ascii_hexdigit())
                    .collect();
                if clean_hex.is_empty() {
                    0
                } else {
                    // parse as u128, then truncate
                    let val_128 = u128::from_str_radix(&clean_hex, 16).unwrap_or(0);
                    (val_128 & 0xFFFF_FFFF_FFFF_FFFF) as u64
                }
            };

            let got64 = parsed64.low64();
            assert_eq!(
                got64, reference_val_64,
                "Mismatch in 64-bit parse from random hex '{}'",
                hex_str
            );

            // For 256 => also check its low64
            let got256_low64 = parsed256.low64();
            // That should match the same truncated value
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
        let parsed64 = "0x1234abcd".parse::<BaseUInt<64>>().unwrap();
        let direct64 = BaseUInt::<64>::from(s1);

        assert_eq!(parsed64, direct64, "parse::<BaseUInt<64>>() mismatch");

        let s2 = "ffff_ffff_ffff";
        let parsed256 = s2.parse::<BaseUInt<256>>().unwrap();
        let direct256 = BaseUInt::<256>::from(s2);
        assert_eq!(parsed256, direct256, "parse::<BaseUInt<256>>() mismatch");

        // Edge: parse an empty => zero
        let empty32 = "".parse::<BaseUInt<32>>().unwrap();
        assert_eq!(empty32, BaseUInt::<32>::default());

        info!("std::str::FromStr trait integration tests passed.");
    }
}
