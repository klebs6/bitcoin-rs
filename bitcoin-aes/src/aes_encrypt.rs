// ---------------- [ File: bitcoin-aes/src/aes_encrypt.rs ]
crate::ix!();

/// Constant‑time AES encryption (forward rounds).
///
/// * `rounds` → pointer to the expanded key schedule  
/// * `nrounds` → 10/12/14 for AES‑128/192/256  
/// * `cipher16` → destination buffer (16 bytes)  
/// * `plain16` → source buffer (16 bytes)
#[inline(always)]
pub fn aes_encrypt(
    mut rounds: *const AESState,
    nrounds: i32,
    cipher16: *mut u8,
    plain16: *const u8,
) {
    tracing::trace!(
        target: "aes",
        "aes_encrypt – entry; rounds = {:p}, nrounds = {}, cipher = {:p}, plain = {:p}",
        rounds,
        nrounds,
        cipher16,
        plain16
    );

    /* --- working state -------------------------------------------------- */
    let mut s = AESState {
        slice: [0u16; 8],
    };

    /* --- round 0 -------------------------------------------------------- */
    load_bytes(&mut s as *mut _, plain16);
    add_round_key(&mut s as *mut _, rounds);
    rounds = unsafe { rounds.add(1) };

    /* --- middle rounds -------------------------------------------------- */
    for _round in 1..nrounds {
        // S‑box (forward)
        sub_bytes(unsafe { &mut *(&mut s as *mut AESState as *mut AESState) }, false);

        // ShiftRows
        shift_rows(&mut s as *mut _);

        // MixColumns (forward)
        unsafe { (&mut s).mix_columns(false) };

        // AddRoundKey
        add_round_key(&mut s as *mut _, rounds);
        rounds = unsafe { rounds.add(1) };
    }

    /* --- final round (no MixColumns) ------------------------------------ */
    sub_bytes(unsafe { &mut *(&mut s as *mut AESState as *mut AESState) }, false);
    shift_rows(&mut s as *mut _);
    add_round_key(&mut s as *mut _, rounds);

    /* --- write result --------------------------------------------------- */
    save_bytes(cipher16, &s as *const _);

    tracing::trace!(target: "aes", "aes_encrypt – exit");
}

#[cfg(test)]
mod aes_encrypt_correctness {
    use super::*;

    /// For randomly‑generated inputs across **all three key‑sizes**,
    /// `aes_encrypt` followed by `aes_decrypt` must yield the original
    /// plaintext exactly.
    #[traced_test]
    fn encrypt_then_decrypt_is_identity() {
        let mut rng = thread_rng();

        for _ in 0..10_000 {
            match rng.gen_range(0u8..=2) {
                /* ---------------- AES‑128 ---------------- */
                0 => {
                    let mut key = [0u8; 16];
                    rng.fill(&mut key);

                    let mut plain_in = [0u8; AES_BLOCKSIZE];
                    rng.fill(&mut plain_in);

                    let mut ctx = AES128_ctx::default();
                    unsafe { aes128_init(&mut ctx as *mut _, key.as_ptr()) };

                    let mut cipher    = [0u8; AES_BLOCKSIZE];
                    let mut plain_out = [0u8; AES_BLOCKSIZE];

                    unsafe {
                        aes_encrypt(
                            ctx.rk.as_ptr(),
                            10,
                            cipher.as_mut_ptr(),
                            plain_in.as_ptr(),
                        );
                        aes_decrypt(
                            ctx.rk.as_ptr(),
                            10,
                            plain_out.as_mut_ptr(),
                            cipher.as_ptr(),
                        );
                    }

                    info!(target: "test", ?key, ?plain_in, ?cipher, ?plain_out, "AES‑128 round‑trip");
                    assert_eq!(plain_out, plain_in, "AES‑128 round‑trip mismatch");
                }

                /* ---------------- AES‑192 ---------------- */
                1 => {
                    let mut key = [0u8; 24];
                    rng.fill(&mut key);

                    let mut plain_in = [0u8; AES_BLOCKSIZE];
                    rng.fill(&mut plain_in);

                    let mut ctx = AES192_ctx::default();
                    unsafe { aes192_init(&mut ctx as *mut _, key.as_ptr()) };

                    let mut cipher    = [0u8; AES_BLOCKSIZE];
                    let mut plain_out = [0u8; AES_BLOCKSIZE];

                    unsafe {
                        aes_encrypt(
                            ctx.rk.as_ptr(),
                            12,
                            cipher.as_mut_ptr(),
                            plain_in.as_ptr(),
                        );
                        aes_decrypt(
                            ctx.rk.as_ptr(),
                            12,
                            plain_out.as_mut_ptr(),
                            cipher.as_ptr(),
                        );
                    }

                    info!(target: "test", ?key, ?plain_in, ?cipher, ?plain_out, "AES‑192 round‑trip");
                    assert_eq!(plain_out, plain_in, "AES‑192 round‑trip mismatch");
                }

                /* ---------------- AES‑256 ---------------- */
                _ => {
                    let mut key = [0u8; 32];
                    rng.fill(&mut key);

                    let mut plain_in = [0u8; AES_BLOCKSIZE];
                    rng.fill(&mut plain_in);

                    let mut ctx = AES256_ctx::default();
                    unsafe { aes256_init(&mut ctx as *mut _, key.as_ptr()) };

                    let mut cipher    = [0u8; AES_BLOCKSIZE];
                    let mut plain_out = [0u8; AES_BLOCKSIZE];

                    unsafe {
                        aes_encrypt(
                            ctx.rk.as_ptr(),
                            14,
                            cipher.as_mut_ptr(),
                            plain_in.as_ptr(),
                        );
                        aes_decrypt(
                            ctx.rk.as_ptr(),
                            14,
                            plain_out.as_mut_ptr(),
                            cipher.as_ptr(),
                        );
                    }

                    info!(target: "test", ?key, ?plain_in, ?cipher, ?plain_out, "AES‑256 round‑trip");
                    assert_eq!(plain_out, plain_in, "AES‑256 round‑trip mismatch");
                }
            }
        }
    }
}
