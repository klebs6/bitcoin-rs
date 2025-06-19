// ---------------- [ File: bitcoin-chacha/src/chacha_poly_aead_crypt.rs ]
crate::ix!();

impl ChaCha20Poly1305AEAD {

    /// Encrypts/decrypts a packet
    /// 
    /// -seqnr_payload, the message sequence number
    /// 
    /// -seqnr_aad, the messages AAD sequence number which allows reuse of the AAD keystream
    /// 
    /// -aad_pos, position to use in the AAD keystream to encrypt the AAD
    /// 
    /// -dest, output buffer, must be of a size equal or larger then CHACHA20_POLY1305_AEAD_AAD_LEN
    /// + payload (+ POLY1305_TAG_LEN in encryption) bytes
    /// 
    /// -destlen, length of the destination buffer
    /// 
    /// -src, the AAD+payload to encrypt or the AAD+payload+MAC to decrypt
    /// 
    /// -src_len, the length of the source buffer
    /// 
    /// -is_encrypt, set to true if we encrypt (creates and appends the MAC instead of verifying
    /// it)
    /// 
    #[allow(clippy::too_many_arguments)]
    pub fn crypt(
        &mut self,
        seqnr_payload: u64,
        seqnr_aad: u64,
        aad_pos: i32,
        dest: *mut u8,
        dest_len: usize,
        src: *const u8,
        mut src_len: usize,
        is_encrypt: bool,
    ) -> bool {
        debug!(
            seqnr_payload,
            seqnr_aad,
            aad_pos,
            src_len,
            dest_len,
            is_encrypt,
            "ChaCha20Poly1305AEAD::crypt"
        );

        /* ---------- 1. bounds checks ---------- */
        let need = CHACHA20_POLY1305_AEAD_AAD_LEN;
        if (is_encrypt && (src_len < need || dest_len < src_len + POLY1305_TAGLEN))
            || (!is_encrypt
                && (src_len < need + POLY1305_TAGLEN
                    || dest_len < src_len - POLY1305_TAGLEN))
        {
            return false;
        }

        unsafe {
            let src_slice = core::slice::from_raw_parts(src, src_len);
            let dest_slice =
                core::slice::from_raw_parts_mut(dest, dest_len);

            /* ---------- 2. derive per‑packet Poly1305 key ---------- */
            let mut poly_key = [0u8; POLY1305_KEYLEN];
            self.chacha_main_mut().setiv(seqnr_payload);
            self.chacha_main_mut().seek(0);
            self.chacha_main_mut()
                .keystream(poly_key.as_mut_ptr(), POLY1305_KEYLEN);

            /* ---------- 3. verify tag when decrypting ---------- */
            if !is_encrypt {
                let tag_offset = src_len - POLY1305_TAGLEN;
                let provided_tag_ptr = src.add(tag_offset);

                let calc_tag =
                    compute_poly1305_tag(&poly_key, &src_slice[..tag_offset]);

                if timingsafe_bcmp(
                    calc_tag.as_ptr(),
                    provided_tag_ptr,
                    POLY1305_TAGLEN,
                ) != 0
                {
                    poly_key.zeroize();
                    return false;
                }

                // exclude MAC from subsequent processing
                src_len -= POLY1305_TAGLEN;
            }

            /* ---------- 4. AAD keystream cache ---------- */
            if *self.cached_aad_seqnr() != seqnr_aad {
                *self.cached_aad_seqnr_mut() = seqnr_aad;

                // obtain raw pointer before borrowing `chacha_header`
                let buf_ptr = self.aad_keystream_buffer_mut().as_mut_ptr();

                {
                    let hdr = self.chacha_header_mut();
                    hdr.setiv(seqnr_aad);
                    hdr.seek(0);
                    hdr.keystream(buf_ptr, CHACHA20_ROUND_OUTPUT);
                }
            }

            /* ---------- 5. crypt AAD (3 bytes) ---------- */
            for i in 0..CHACHA20_POLY1305_AEAD_AAD_LEN {
                dest_slice[i] =
                    src_slice[i]
                        ^ self.aad_keystream_buffer()
                            [(aad_pos as usize) + i];
            }

            /* ---------- 6. crypt payload ---------- */
            self.chacha_main_mut().seek(1);
            self.chacha_main_mut().crypt(
                src.add(CHACHA20_POLY1305_AEAD_AAD_LEN),
                dest.add(CHACHA20_POLY1305_AEAD_AAD_LEN),
                src_len - CHACHA20_POLY1305_AEAD_AAD_LEN,
            );

            /* ---------- 7. append tag when encrypting ---------- */
            if is_encrypt {
                let tag =
                    compute_poly1305_tag(&poly_key, &dest_slice[..src_len]);
                dest_slice[src_len..src_len + POLY1305_TAGLEN]
                    .copy_from_slice(tag.as_slice());
            }

            /* ---------- 8. cleanse key material ---------- */
            poly_key.zeroize();
            memory_cleanse(
                poly_key.as_mut_ptr() as *mut c_void,
                poly_key.len(),
            );
        }

        true
    }
}

#[cfg(test)]
mod crypt_exhaustive_tests {
    use super::*;

    const K1: [u8; CHACHA20_POLY1305_AEAD_KEY_LEN] = [0x55; CHACHA20_POLY1305_AEAD_KEY_LEN];
    const K2: [u8; CHACHA20_POLY1305_AEAD_KEY_LEN] = [0xAA; CHACHA20_POLY1305_AEAD_KEY_LEN];

    fn build_packet(payload: &[u8]) -> Vec<u8> {
        let mut v = Vec::with_capacity(3 + payload.len());
        v.extend_from_slice(&[(payload.len() as u8), 0, 0]); // 24‑bit length
        v.extend_from_slice(payload);
        v
    }

    #[traced_test]
    fn roundtrip_various_aad_pos_and_seq() {
        let payload = b"PAYLOAD_DATA_1234567890";
        let src     = build_packet(payload);

        for (seq_aad, aad_pos) in &[(0u64, 0i32), (1, 3), (7, 60)] {
            // Encrypt
            let mut enc = vec![0u8; src.len() + POLY1305_TAGLEN];
            let mut aead = ChaCha20Poly1305AEAD::new(
                K1.as_ptr(), K1.len(), K2.as_ptr(), K2.len(),
            );
            assert!(
                aead.crypt(
                    42,                // seqnr_payload
                    *seq_aad,          // seqnr_aad
                    *aad_pos,          // aad position
                    enc.as_mut_ptr(),
                    enc.len(),
                    src.as_ptr(),
                    src.len(),
                    true,              // encrypt
                ),
                "encryption must succeed (seq_aad {}, pos {})",
                seq_aad, aad_pos
            );

            // Decrypt
            let mut out = vec![0u8; src.len()];
            let mut aead2 = ChaCha20Poly1305AEAD::new(
                K1.as_ptr(), K1.len(), K2.as_ptr(), K2.len(),
            );
            assert!(
                aead2.crypt(
                    42,
                    *seq_aad,
                    *aad_pos,
                    out.as_mut_ptr(),
                    out.len(),
                    enc.as_ptr(),
                    enc.len(),
                    false,             // decrypt
                ),
                "decryption must succeed (seq_aad {}, pos {})",
                seq_aad, aad_pos
            );
            assert_eq!(out, src, "round‑trip mismatch");
        }
    }

    #[traced_test]
    fn tampered_tag_detected() {
        let src = build_packet(b"1234");
        let mut enc = vec![0u8; src.len() + POLY1305_TAGLEN];
        let mut aead = ChaCha20Poly1305AEAD::new(K1.as_ptr(), 32, K2.as_ptr(), 32);
        assert!(aead.crypt(0, 0, 0, enc.as_mut_ptr(), enc.len(), src.as_ptr(), src.len(), true));

        // Flip a bit in the tag
        let last = enc.len() - 1;
        enc[last] ^= 0x80;

        let mut out = vec![0u8; enc.len() - POLY1305_TAGLEN];
        let mut aead2 = ChaCha20Poly1305AEAD::new(K1.as_ptr(), 32, K2.as_ptr(), 32);
        assert!(
            !aead2.crypt(0, 0, 0, out.as_mut_ptr(), out.len(), enc.as_ptr(), enc.len(), false),
            "tampered tag must fail authentication"
        );
    }

    #[traced_test]
    fn invalid_buffer_lengths_rejected() {
        let src = build_packet(b"abcd");

        let mut small_dest = vec![0u8; src.len()]; // too small for tag
        let mut aead = ChaCha20Poly1305AEAD::new(K1.as_ptr(), 32, K2.as_ptr(), 32);
        assert!(
            !aead.crypt(
                0, 0, 0,
                small_dest.as_mut_ptr(),
                small_dest.len(),
                src.as_ptr(),
                src.len(),
                true
            ),
            "destination too small must be rejected"
        );

        // source too short (missing MAC)
        let mut dest = vec![0u8; src.len()];
        assert!(
            !aead.crypt(
                0, 0, 0,
                dest.as_mut_ptr(),
                dest.len(),
                src.as_ptr(),
                CHACHA20_POLY1305_AEAD_AAD_LEN + 1, // insufficient
                false
            ),
            "source lacking MAC must be rejected"
        );
    }
}
