// ---------------- [ File: bitcoin-siphash/src/write.rs ]
crate::ix!();

impl BitcoinSipHasher {

    /// Hash a 64-bit integer worth of data
    /// 
    /// It is treated as if this was the little-endian interpretation of 8 bytes.
    /// 
    /// This function can only be used when a multiple of 8 bytes have been written so far.
    ///
    /// Append exactly 8 bytes (as a little‑endian `u64`) to the stream.
    ///
    #[inline]
    pub fn write_u64(&mut self, data: u64) -> &mut Self {
        debug!("write_u64 {:016x}", data);
        assert_eq!(self.count() % 8, 0, "write_u64 misaligned");

        let (mut v0, mut v1, mut v2, mut v3) = {
            let v = self.v();
            (v[0], v[1], v[2], v[3])
        };

        v3 ^= data;
        sipround!(v0, v1, v2, v3);
        sipround!(v0, v1, v2, v3);
        v0 ^= data;

        *self.v_mut() = [v0, v1, v2, v3];
        *self.count_mut() = self.count().wrapping_add(8);
        *self.tmp_mut() = 0;
        self
    }

    /// Hash arbitrary bytes.
    ///
    /// Append an arbitrary byte slice to the stream.
    #[inline]
    pub fn write(&mut self, data: &[u8]) -> &mut Self {
        let (mut v0, mut v1, mut v2, mut v3) = {
            let v = self.v();
            (v[0], v[1], v[2], v[3])
        };
        let mut t = self.tmp();
        let mut c = self.count();

        for &byte in data {
            t |= (byte as u64) << (8 * (c as u64 & 7));
            c = c.wrapping_add(1);
            if (c & 7) == 0 {
                v3 ^= t;
                sipround!(v0, v1, v2, v3);
                sipround!(v0, v1, v2, v3);
                v0 ^= t;
                t = 0;
            }
        }

        *self.v_mut()     = [v0, v1, v2, v3];
        *self.count_mut() = c;
        *self.tmp_mut()   = t;
        self
    }
}

#[cfg(test)]
mod write_tests {
    use super::*;

    /// Confirm that byte‑wise and word‑wise writes yield identical hashes.
    #[traced_test]
    fn write_paths_converge() {
        let k0 = 0xabcd_ef01_2345_6789;
        let k1 = 0x9876_5432_10fe_dcba;

        let bytes: Vec<u8> = (0u8..16).collect();

        let mut hasher_bytes = BitcoinSipHasher::new(k0, k1);
        hasher_bytes.write(&bytes);

        let mut hasher_words = BitcoinSipHasher::new(k0, k1);
        hasher_words.write_u64(u64::from_le_bytes(bytes[0..8].try_into().unwrap()));
        hasher_words.write_u64(u64::from_le_bytes(bytes[8..16].try_into().unwrap()));

        assert_eq!(
            hasher_bytes.finalize(),
            hasher_words.finalize(),
            "byte‑wise and word‑wise paths must agree"
        );
    }
}
