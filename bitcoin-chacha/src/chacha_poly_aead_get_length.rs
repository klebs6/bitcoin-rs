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

        assert!(
            aad_pos >= 0
                && (aad_pos as usize)
                    < CHACHA20_ROUND_OUTPUT - CHACHA20_POLY1305_AEAD_AAD_LEN
        );

        if *self.cached_aad_seqnr() != seqnr_aad {
            *self.cached_aad_seqnr_mut() = seqnr_aad;

            // obtain raw pointer before borrowing header
            let buf_ptr = self.aad_keystream_buffer_mut().as_mut_ptr();

            {
                let hdr = self.chacha_header_mut();
                hdr.setiv(seqnr_aad);
                hdr.seek(0);
                hdr.keystream(buf_ptr, CHACHA20_ROUND_OUTPUT);
            }
        }

        unsafe {
            let c = core::slice::from_raw_parts(
                ciphertext,
                CHACHA20_POLY1305_AEAD_AAD_LEN,
            );

            let out = (c[0] ^ self.aad_keystream_buffer()[aad_pos as usize + 0]) as u32
                | ((c[1] ^ self.aad_keystream_buffer()[aad_pos as usize + 1]) as u32) << 8
                | ((c[2] ^ self.aad_keystream_buffer()[aad_pos as usize + 2]) as u32) << 16;

            std::ptr::write(len24_out, out);
        }

        true
    }
}

#[cfg(test)]
mod poly1305_tag_exhaustive_tests {
    use super::*;

    // RFC 8439 Poly1305 test vector
    const KEY: [u8; POLY1305_KEYLEN] = [
        0x85, 0xd6, 0xbe, 0x78, 0x57, 0x55, 0x6d, 0x33,
        0x7f, 0x44, 0x52, 0xfe, 0x42, 0xd5, 0x06, 0xa8,
        0x01, 0x03, 0x80, 0x8a, 0xfb, 0x0d, 0xb2, 0xfd,
        0x4a, 0xbf, 0xf6, 0xaf, 0x41, 0x49, 0xf5, 0x1b,
    ];

    const MSG: [u8; 34] = *b"Cryptographic Forum Research Group";
    const TAG: [u8; POLY1305_TAGLEN] = [
        0xa8,0x06,0x1d,0xc1,0x30,0x51,0x36,0xc6,
        0xc2,0x2b,0x8b,0xaf,0x0c,0x01,0x27,0xa9,
    ];

    #[traced_test]
    fn poly1305_matches_reference() {
        let t = compute_poly1305_tag(&KEY, &MSG);
        assert_eq!(t.as_slice(), &TAG, "Poly1305 tag must match RFC 8439");
    }
}
