// ---------------- [ File: bitcoin-chacha/src/chacha20_crypt.rs ]
crate::ix!();

impl ChaCha20 {

    /**
      | enciphers the message <input> of length
      | <bytes> and write the enciphered representation
      | into <output>
      | 
      | Used for encryption and decryption
      | (XOR)
      |
      */
    pub fn crypt(&mut self, m: *const u8, c: *mut u8, bytes: usize) {
        trace!(bytes, "ChaCha20::crypt");
        if bytes == 0 {
            return;
        }

        unsafe {
            // Produce keystream.
            let mut ks = vec![0u8; bytes];
            self.keystream(ks.as_mut_ptr(), bytes);

            // XOR keystream with `m` into `c`.
            let src = core::slice::from_raw_parts(m, bytes);
            let dst = core::slice::from_raw_parts_mut(c, bytes);
            for i in 0..bytes {
                dst[i] = src[i] ^ ks[i];
            }
        }
    }
}

#[cfg(test)]
mod crypt_exhaustive_small_sizes {
    use super::*;

    #[traced_test]
    fn crypt_zero_bytes_is_noop() {
        let mut c = ChaCha20::new([0u8; 32].as_ptr(), 32);
        c.setiv(77);
        let mut out = [123u8; 0];
        c.crypt(out.as_ptr(), out.as_mut_ptr(), 0);
        // nothing to assert, merely exercise branch
    }

    #[traced_test]
    fn encrypt_vs_keystream_xor() {
        let msg = b"hello world!";
        let mut cipher = ChaCha20::new([1u8; 32].as_ptr(), 32);
        cipher.setiv(9);

        let mut out = [0u8; 12];
        cipher.crypt(msg.as_ptr(), out.as_mut_ptr(), msg.len());

        // produce keystream separately and XOR
        let mut ks = [0u8; 12];
        let mut ks_gen = ChaCha20::new([1u8; 32].as_ptr(), 32);
        ks_gen.setiv(9);
        ks_gen.keystream(ks.as_mut_ptr(), ks.len());

        let mut manual = [0u8; 12];
        for i in 0..12 {
            manual[i] = msg[i] ^ ks[i];
        }
        assert_eq!(manual, out, "crypt == keystream XOR msg");
    }
}
