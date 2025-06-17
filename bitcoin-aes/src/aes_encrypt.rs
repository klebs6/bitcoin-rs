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
