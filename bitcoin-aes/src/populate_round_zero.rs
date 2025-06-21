crate::ix!();

#[inline(always)]
pub fn populate_round_zero(rounds: *mut AESState, key: *const u8, nkeywords: i32) {
    tracing::trace!(
        target: "aes",
        "populate_round_zero – rounds = {:p}, key = {:p}, nkeywords = {}",
        rounds,
        key,
        nkeywords
    );

    /* ------------------------------------------------------------------
     * Required byte order
     * -------------------
     * For a caller‑supplied key byte‑stream K = k₀‥k_{4·Nk‑1}, FIPS‑197
     * defines the first Nk 32‑bit words W[0‥Nk‑1] of the expanded key as
     *
     *   W[0] = {k₀,k₁,k₂,k₃}
     *   W[1] = {k₄,k₅,k₆,k₇}
     *   ...
     *
     * Re‑expressed in our **column‑major** lane layout, this means that
     * byte‑index *i* maps to (row = i mod 4, col = i / 4).
     *
     * Therefore we must iterate columns (*words*) outer‑most and rows
     * inner‑most so that the reconstructed byte‑stream matches the
     * caller’s original order when extracted by the validation tests.
     * ------------------------------------------------------------------ */

    unsafe {
        let mut key_ptr = key;
        for word in 0..(nkeywords as usize) {          // ← column‑major outer loop
            for row in 0..4 {
                let byte      = *key_ptr;
                key_ptr       = key_ptr.add(1);
                let round_idx = word >> 2;             // which AESState within round‑0
                let col_idx   = (word & 3) as i32;     // column within that AESState
                load_byte(&mut *rounds.add(round_idx), byte, row as i32, col_idx);
            }
        }
    }
}

#[cfg(test)]
mod populate_round_zero_validation {
    use super::*;

    #[traced_test]
    fn caller_key_is_preserved_exactly() {
        let mut rng = thread_rng();

        #[derive(Clone, Copy)]
        struct Case { nkeywords: i32, nrounds: i32, key_len: usize }
        const CASES: &[Case] = &[
            Case { nkeywords: 4, nrounds: 10, key_len: 16 }, // AES‑128
            Case { nkeywords: 6, nrounds: 12, key_len: 24 }, // AES‑192
            Case { nkeywords: 8, nrounds: 14, key_len: 32 }, // AES‑256
        ];

        for case in CASES {
            let mut key = vec![0u8; case.key_len];
            rng.fill(&mut key[..]);

            let mut schedule = vec![AESState::default(); (case.nrounds + 1) as usize];
            unsafe {
                zero_schedule(schedule.as_mut_ptr(), case.nrounds);
                populate_round_zero(schedule.as_mut_ptr(), key.as_ptr(), case.nkeywords);
            }

            // Re‑extract round‑0 bytes in column‑major order
            let mut extracted = Vec::with_capacity(case.key_len);
            for word in 0..case.nkeywords as usize {
                let round_idx = word >> 2;
                let col = (word & 3) as i32;
                for row in 0..4 {
                    let mut byte = 0u8;
                    for bit in 0..8 {
                        let lane = (col * 4 + row) as u16;
                        let v = ((schedule[round_idx].slice()[bit] >> lane) & 1) as u8;
                        byte |= v << bit;
                    }
                    extracted.push(byte);
                }
            }

            assert_eq!(
                extracted, key,
                "round‑0 key mismatch ({}‑bit)",
                case.key_len * 8
            );
        }
    }
}
