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
