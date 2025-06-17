crate::ix!();

/// AddRoundKey (state ⊕= round‑key)
#[inline(always)]
pub fn add_round_key(s: *mut AESState, round: *const AESState) {
    tracing::trace!(target: "aes", "add_round_key – entry {:p} ⊕ {:p}", s, round);

    unsafe {
        for b in 0..8 {
            (*s).slice[b] ^= (*round).slice[b];
        }
    }
}
