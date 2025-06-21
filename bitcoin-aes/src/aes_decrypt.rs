// ---------------- [ File: bitcoin-aes/src/aes_decrypt.rs ]
crate::ix!();

/// Constant‑time AES decryption (inverse rounds).
///
/// * `rounds` → pointer to the expanded key schedule  
/// * `nrounds` → 10/12/14 for AES‑128/192/256  
/// * `plain16` → destination buffer (16 bytes)  
/// * `cipher16` → source buffer (16 bytes)
#[inline(always)]
pub fn aes_decrypt(
    rounds: *const AESState,
    nrounds: i32,
    plain16: *mut u8,
    cipher16: *const u8,
) {
    /* Most AES decryption implementations use the alternate scheme
     * (the Equivalent Inverse Cipher), which allows for more code reuse between
     * the encryption and decryption code, but requires separate setup for both.
     */
    tracing::trace!(
        target: "aes",
        "aes_decrypt – entry; rounds = {:p}, nrounds = {}, plain = {:p}, cipher = {:p}",
        rounds,
        nrounds,
        plain16,
        cipher16
    );

    /* --- working state -------------------------------------------------- */
    let mut s = AESState {
        slice: [0u16; 8],
    };

    /* --- start from last round‑key -------------------------------------- */
    let mut rk = unsafe { rounds.add(nrounds as usize) };

    load_bytes(&mut s as *mut _, cipher16);
    add_round_key(&mut s as *mut _, rk);
    rk = unsafe { rk.sub(1) };

    /* --- middle rounds -------------------------------------------------- */
    for _round in 1..nrounds {
        // Inverse ShiftRows
        inv_shift_rows(&mut s as *mut _);

        // Inverse S‑box
        sub_bytes(unsafe { &mut *(&mut s as *mut AESState as *mut AESState) }, true);

        // AddRoundKey
        add_round_key(&mut s as *mut _, rk);
        rk = unsafe { rk.sub(1) };

        // Inverse MixColumns
        unsafe { (&mut s).mix_columns(true) };
    }

    /* --- final round (no MixColumns) ------------------------------------ */
    inv_shift_rows(&mut s as *mut _);
    sub_bytes(unsafe { &mut *(&mut s as *mut AESState as *mut AESState) }, true);
    add_round_key(&mut s as *mut _, rk);

    /* --- write result --------------------------------------------------- */
    save_bytes(plain16, &s as *const _);

    tracing::trace!(target: "aes", "aes_decrypt – exit");
}

#[cfg(test)]
mod aes_decrypt_correctness {
    use super::*;

    /* --- minimal helper: ASCII lower‑case hex → Vec<u8> ---------------- */
    fn decode_hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("hex"))
            .collect()
    }

    /// FIPS‑197 reference vectors – ciphertext must decrypt exactly.
    const VECTORS: &[(usize, &str, &str, &str)] = &[
        /* key‑bits, key,      plain,                           cipher              */
        (128, "000102030405060708090a0b0c0d0e0f",               "00112233445566778899aabbccddeeff", "69c4e0d86a7b0430d8cdb78070b4c55a"),
        (192, "000102030405060708090a0b0c0d0e0f1011121314151617","00112233445566778899aabbccddeeff", "dda97ca4864cdfe06eaf70a0ec0d7191"),
        (256, "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f",
                                                             "00112233445566778899aabbccddeeff", "8ea2b7ca516745bfeafc49904b496089"),
    ];

    /// The *official* vectors must survive a direct call to `aes_decrypt`.
    #[traced_test]
    fn decrypt_matches_reference_aes128() {
        let &(bits, k_hex, p_hex, c_hex) = VECTORS.iter().find(|v| v.0 == 128).unwrap();
        assert_eq!(bits, 128);

        let key:    [u8; 16] = decode_hex(k_hex).try_into().unwrap();
        let cipher: [u8; 16] = decode_hex(c_hex).try_into().unwrap();
        let expected_plain: [u8; 16] = decode_hex(p_hex).try_into().unwrap();

        let mut ctx = AES128_ctx::default();
        unsafe { aes128_init(&mut ctx as *mut _, key.as_ptr()) };

        let mut plain = [0u8; 16];
        unsafe {
            aes_decrypt(
                ctx.rk.as_ptr(),
                10,
                plain.as_mut_ptr(),
                cipher.as_ptr(),
            );
        }

        assert_eq!(plain, expected_plain, "AES‑128 decrypt vector failed");
    }

    #[traced_test]
    fn decrypt_matches_reference_aes192() {
        let &(bits, k_hex, p_hex, c_hex) = VECTORS.iter().find(|v| v.0 == 192).unwrap();
        assert_eq!(bits, 192);

        let key:    [u8; 24] = decode_hex(k_hex).try_into().unwrap();
        let cipher: [u8; 16] = decode_hex(c_hex).try_into().unwrap();
        let expected_plain: [u8; 16] = decode_hex(p_hex).try_into().unwrap();

        let mut ctx = AES192_ctx::default();
        unsafe { aes192_init(&mut ctx as *mut _, key.as_ptr()) };

        let mut plain = [0u8; 16];
        unsafe {
            aes_decrypt(
                ctx.rk.as_ptr(),
                12,
                plain.as_mut_ptr(),
                cipher.as_ptr(),
            );
        }

        assert_eq!(plain, expected_plain, "AES‑192 decrypt vector failed");
    }

    #[traced_test]
    fn decrypt_matches_reference_aes256() {
        let &(bits, k_hex, p_hex, c_hex) = VECTORS.iter().find(|v| v.0 == 256).unwrap();
        assert_eq!(bits, 256);

        let key:    [u8; 32] = decode_hex(k_hex).try_into().unwrap();
        let cipher: [u8; 16] = decode_hex(c_hex).try_into().unwrap();
        let expected_plain: [u8; 16] = decode_hex(p_hex).try_into().unwrap();

        let mut ctx = AES256_ctx::default();
        unsafe { aes256_init(&mut ctx as *mut _, key.as_ptr()) };

        let mut plain = [0u8; 16];
        unsafe {
            aes_decrypt(
                ctx.rk.as_ptr(),
                14,
                plain.as_mut_ptr(),
                cipher.as_ptr(),
            );
        }

        assert_eq!(plain, expected_plain, "AES‑256 decrypt vector failed");
    }


    /// The core round functions (`aes_encrypt` ↔ `aes_decrypt`) must invert
    /// each other for AES‑128/192/256 with *random* keys and plaintext.
    #[traced_test]
    fn encrypt_then_decrypt_is_identity() {
        let mut rng = thread_rng();

        for _ in 0..1_000 {
            // ---------------- AES‑128 -----------------------------------
            {
                let mut key = [0u8; 16];
                let mut plain = [0u8; AES_BLOCKSIZE];
                rng.fill(&mut key);
                rng.fill(&mut plain);

                let mut schedule = [AESState::default(); 11];
                unsafe { aes_setup(schedule.as_mut_ptr(), key.as_ptr(), 4, 10) };

                let mut cipher = [0u8; AES_BLOCKSIZE];
                unsafe {
                    aes_encrypt(
                        schedule.as_ptr(),
                        10,
                        cipher.as_mut_ptr(),
                        plain.as_ptr(),
                    );
                }

                let mut decrypted = [0u8; AES_BLOCKSIZE];
                unsafe {
                    aes_decrypt(
                        schedule.as_ptr(),
                        10,
                        decrypted.as_mut_ptr(),
                        cipher.as_ptr(),
                    );
                }

                info!(target: "test", key_size = 128, ?key, ?plain, ?cipher, ?decrypted);
                assert_eq!(decrypted, plain, "AES‑128 core round‑trip failed");
            }

            // ---------------- AES‑192 -----------------------------------
            {
                let mut key = [0u8; 24];
                let mut plain = [0u8; AES_BLOCKSIZE];
                rng.fill(&mut key);
                rng.fill(&mut plain);

                let mut schedule = [AESState::default(); 13];
                unsafe { aes_setup(schedule.as_mut_ptr(), key.as_ptr(), 6, 12) };

                let mut cipher = [0u8; AES_BLOCKSIZE];
                unsafe {
                    aes_encrypt(
                        schedule.as_ptr(),
                        12,
                        cipher.as_mut_ptr(),
                        plain.as_ptr(),
                    );
                }

                let mut decrypted = [0u8; AES_BLOCKSIZE];
                unsafe {
                    aes_decrypt(
                        schedule.as_ptr(),
                        12,
                        decrypted.as_mut_ptr(),
                        cipher.as_ptr(),
                    );
                }

                info!(target: "test", key_size = 192, ?key, ?plain, ?cipher, ?decrypted);
                assert_eq!(decrypted, plain, "AES‑192 core round‑trip failed");
            }

            // ---------------- AES‑256 -----------------------------------
            {
                let mut key = [0u8; 32];
                let mut plain = [0u8; AES_BLOCKSIZE];
                rng.fill(&mut key);
                rng.fill(&mut plain);

                let mut schedule = [AESState::default(); 15];
                unsafe { aes_setup(schedule.as_mut_ptr(), key.as_ptr(), 8, 14) };

                let mut cipher = [0u8; AES_BLOCKSIZE];
                unsafe {
                    aes_encrypt(
                        schedule.as_ptr(),
                        14,
                        cipher.as_mut_ptr(),
                        plain.as_ptr(),
                    );
                }

                let mut decrypted = [0u8; AES_BLOCKSIZE];
                unsafe {
                    aes_decrypt(
                        schedule.as_ptr(),
                        14,
                        decrypted.as_mut_ptr(),
                        cipher.as_ptr(),
                    );
                }

                info!(target: "test", key_size = 256, ?key, ?plain, ?cipher, ?decrypted);
                assert_eq!(decrypted, plain, "AES‑256 core round‑trip failed");
            }
        }
    }
}
