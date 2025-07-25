// ---------------- [ File: bitcoin-chacha/src/chacha20_set_key.rs ]
crate::ix!();

impl ChaCha20 {
    /// Set key with flexible key‑length (128‑ or 256‑bit; 256 bit recommended)
    pub fn set_key(&mut self, k: *const u8, keylen: usize) {
        debug!(keylen, "ChaCha20::set_key");
        assert!(
            keylen == 16 || keylen == 32,
            "ChaCha20 key must be 128 or 256 bits"
        );

        // SAFETY: caller guarantees `k` is valid for `keylen` bytes.
        let key = unsafe { core::slice::from_raw_parts(k, keylen) };
        let (constants, k_tail) = if keylen == 32 {
            (crate::chacha20::SIGMA_BYTES, &key[16..])
        } else {
            (crate::chacha20::TAU_BYTES, key)
        };

        self.input_mut()[4]  = read_le32(&key[0..]);
        self.input_mut()[5]  = read_le32(&key[4..]);
        self.input_mut()[6]  = read_le32(&key[8..]);
        self.input_mut()[7]  = read_le32(&key[12..]);
        self.input_mut()[8]  = read_le32(&k_tail[0..]);
        self.input_mut()[9]  = read_le32(&k_tail[4..]);
        self.input_mut()[10] = read_le32(&k_tail[8..]);
        self.input_mut()[11] = read_le32(&k_tail[12..]);

        self.input_mut()[0] = read_le32(&constants[0..]);
        self.input_mut()[1] = read_le32(&constants[4..]);
        self.input_mut()[2] = read_le32(&constants[8..]);
        self.input_mut()[3] = read_le32(&constants[12..]);

        self.input_mut()[12] = 0;
        self.input_mut()[13] = 0;
        self.input_mut()[14] = 0;
        self.input_mut()[15] = 0;
    }
}

#[cfg(test)]
mod chacha20_set_key_exhaustive_tests {
    use super::*;

    #[traced_test]
    fn sets_correct_constants_256bit() {
        let key = [0xAAu8; 32];
        let c = ChaCha20::new(key.as_ptr(), key.len());
        assert_eq!(
            c.input()[0..4],
            [
                read_le32(&SIGMA_BYTES[0..]),
                read_le32(&SIGMA_BYTES[4..]),
                read_le32(&SIGMA_BYTES[8..]),
                read_le32(&SIGMA_BYTES[12..])
            ],
            "σ constants must be loaded for 256‑bit key"
        );
    }

    #[traced_test]
    fn sets_correct_constants_128bit() {
        let key = [0xBBu8; 16];
        let c = ChaCha20::new(key.as_ptr(), key.len());
        assert_eq!(
            c.input()[0..4],
            [
                read_le32(&TAU_BYTES[0..]),
                read_le32(&TAU_BYTES[4..]),
                read_le32(&TAU_BYTES[8..]),
                read_le32(&TAU_BYTES[12..])
            ],
            "τ constants must be loaded for 128‑bit key"
        );
    }
}
