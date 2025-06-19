// ---------------- [ File: bitcoin-chacha/src/chacha_poly_aead_get_length.rs ]
crate::ix!();

impl ChaCha20Poly1305AEAD {
    /**
      | decrypt the 3‑byte AAD length field
      | into `len24_out`
      */
    pub fn get_length(
        &mut self,
        len24_out: *mut u32,
        seqnr_aad: u64,
        aad_pos: i32,
        ciphertext: *const u8,
    ) -> bool {
        trace!(seqnr_aad, aad_pos, "ChaCha20Poly1305AEAD::get_length");

        // enforce valid aad position to avoid accessing outside of the 64byte keystream cache
        // (there is space for 21 times 3 bytes)
        assert!(
            aad_pos >= 0
                && (aad_pos as usize)
                    < CHACHA20_ROUND_OUTPUT - CHACHA20_POLY1305_AEAD_AAD_LEN
        );

        if *self.cached_aad_seqnr() != seqnr_aad {

            // we need to calculate the 64 keystream bytes since we reached a new aad sequence number
            *self.cached_aad_seqnr_mut() = seqnr_aad;

            // use LE for the nonce
            self.chacha_header_mut().setiv(seqnr_aad);

            // block counter 0
            self.chacha_header_mut().seek(0);

            // write keystream to the cache
            self.chacha_header_mut().keystream(
                self.aad_keystream_buffer_mut().as_mut_ptr(),
                CHACHA20_ROUND_OUTPUT,
            );
        }

        // decrypt the ciphertext length by XORing the right position of the 64byte keystream cache with the ciphertext
        unsafe {
            let c = core::slice::from_raw_parts(
                ciphertext,
                CHACHA20_POLY1305_AEAD_AAD_LEN,
            );
            let out = 
                   (c[0] ^ self.aad_keystream_buffer()[aad_pos as usize + 0]) as u32
                | ((c[1] ^ self.aad_keystream_buffer()[aad_pos as usize + 1]) as u32) << 8
                | ((c[2] ^ self.aad_keystream_buffer()[aad_pos as usize + 2]) as u32) << 16;

            std::ptr::write(len24_out, out);
        }

        true
    }
}
