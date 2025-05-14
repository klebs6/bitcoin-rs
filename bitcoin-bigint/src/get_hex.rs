crate::ix!();

impl<const BITS: usize> BaseUInt<BITS>
where
    [(); BITS / 32]:,
{
    /// Returns this number as a big-endian hex string with no leading zeros (except that a
    /// value of zero is printed as `"0"`).  
    ///
    /// ### Method
    /// 1. Collect all limbs into a contiguous little-endian byte buffer (lowest limb first).  
    /// 2. Reverse that buffer to get big-endian order (highest significant byte first).  
    /// 3. Convert the entire buffer into hex **nibble-by-nibble**, skipping leading zero nibbles
    ///    until we find a nonzero nibble (or confirm the entire number is zero).  
    /// 4. Return the resulting lowercase hex string.
    ///
    /// This ensures that a single byte of `0x01` becomes `"1"` instead of `"01"`, etc.
    #[tracing::instrument(
        level = "trace",
        name = "get_hex",
        skip_all,
        fields(
            BITS = BITS,
            self_val = ?self
        )
    )]
    pub fn get_hex(&self) -> String {
        debug!("Generating big-endian hex string from BaseUInt<{BITS}> = {:?}", self);

        // (A) Gather all bytes in little-endian order
        let limb_count = BITS / 32;
        let total_bytes = BITS / 8;
        let mut le_bytes = Vec::with_capacity(total_bytes);

        for i in 0..limb_count {
            let limb = self.pn[i];
            let limb_bytes = limb.to_le_bytes();
            debug!("pn[{i}] = 0x{limb:08X} => limb_bytes (LE) = {limb_bytes:02X?}");
            le_bytes.extend_from_slice(&limb_bytes);
        }

        // (B) Reverse to get big-endian
        le_bytes.reverse();
        debug!("Reversed => big-endian bytes = {:02X?}", le_bytes);

        // (C) Now convert the big-endian bytes nibble by nibble:
        //     Each byte has two hex nibbles (hi, lo). We'll skip leading zero nibbles.
        let mut result = String::new();
        let mut found_nonzero_nibble = false;

        for &byte in &le_bytes {
            let hi = (byte >> 4) & 0xF;
            let lo = byte & 0xF;

            // High nibble
            if !found_nonzero_nibble {
                if hi != 0 {
                    found_nonzero_nibble = true;
                    result.push(core::char::from_digit(hi as u32, 16).unwrap());
                }
            } else {
                // Already found a nonzero nibble => always push
                result.push(core::char::from_digit(hi as u32, 16).unwrap());
            }

            // Low nibble
            if !found_nonzero_nibble {
                if lo != 0 {
                    found_nonzero_nibble = true;
                    result.push(core::char::from_digit(lo as u32, 16).unwrap());
                }
            } else {
                result.push(core::char::from_digit(lo as u32, 16).unwrap());
            }
        }

        // (D) If we never found any nonzero nibble => the entire number is zero
        if result.is_empty() {
            debug!("All limbs are zero => returning '0'.");
            return "0".to_string();
        }

        debug!("Final hex string => '{}'", result);
        result
    }
}
