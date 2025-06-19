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

        /* ---------- bounds checks ---------- */
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

            /* ---------- derive per‑packet Poly1305 key ---------- */
            let mut poly_key = [0u8; POLY1305_KEYLEN];
            self.chacha_main_mut().setiv(seqnr_payload);
            self.chacha_main_mut().seek(0);
            self.chacha_main_mut()
                .keystream(poly_key.as_mut_ptr(), POLY1305_KEYLEN);

            /* ---------- verify tag when decrypting ---------- */
            if !is_encrypt {
                let tag_offset = src_len - POLY1305_TAGLEN;
                let provided_tag_ptr = src.add(tag_offset);

                let calc_tag =
                    poly1305(&poly_key, &src_slice[..tag_offset]);

                if timingsafe_bcmp(
                    calc_tag.as_ref().as_ptr(),
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

            /* ---------- AAD keystream cache ---------- */
            if *self.cached_aad_seqnr() != seqnr_aad {
                *self.cached_aad_seqnr_mut() = seqnr_aad;
                self.chacha_header_mut().setiv(seqnr_aad);
                self.chacha_header_mut().seek(0);
                self.chacha_header_mut().keystream(
                    self.aad_keystream_buffer_mut().as_mut_ptr(),
                    CHACHA20_ROUND_OUTPUT,
                );
            }

            /* ---------- crypt AAD (3 bytes) ---------- */
            for i in 0..CHACHA20_POLY1305_AEAD_AAD_LEN {
                dest_slice[i] =
                    src_slice[i]
                        ^ self.aad_keystream_buffer()
                            [(aad_pos as usize) + i];
            }

            /* ---------- crypt payload ---------- */
            self.chacha_main_mut().seek(1);
            self.chacha_main_mut().crypt(
                src.add(CHACHA20_POLY1305_AEAD_AAD_LEN),
                dest.add(CHACHA20_POLY1305_AEAD_AAD_LEN),
                src_len - CHACHA20_POLY1305_AEAD_AAD_LEN,
            );

            /* ---------- append tag when encrypting ---------- */
            if is_encrypt {
                let tag = poly1305(&poly_key, &dest_slice[..src_len]);
                dest_slice[src_len..src_len + POLY1305_TAGLEN]
                    .copy_from_slice(tag.as_ref());
            }

            /* ---------- cleanse key material ---------- */
            poly_key.zeroize();
            memory_cleanse(
                poly_key.as_mut_ptr() as *mut c_void,
                poly_key.len(),
            );
        }

        true
    }
}
