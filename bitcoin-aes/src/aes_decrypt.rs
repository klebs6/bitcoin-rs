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
