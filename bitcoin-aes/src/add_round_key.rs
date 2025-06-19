// ---------------- [ File: bitcoin-aes/src/add_round_key.rs ]
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

#[cfg(test)]
mod add_round_key_validation {
    use super::*;
    use rand::{thread_rng, Rng};

    /// Property‑based check that `add_round_key`
    /// performs an in‑place XOR of every word.
    #[traced_test]
    fn xor_is_applied_wordwise() {
        let mut rng = thread_rng();

        for _ in 0..10_000 {
            let mut state = AESState::random(&mut rng);
            let round_key = AESState::random(&mut rng);

            // Expected reference result (`state ⊕ round_key`).
            let mut expected = state.clone();
            for i in 0..8 {
                expected.slice[i] ^= round_key.slice()[i];
            }

            // Call function under test.
            unsafe { add_round_key(&mut state as *mut _, &round_key as *const _) };

            info!(?state.slice, ?expected.slice, "post‑xor state");
            assert_eq!(state.slice(), expected.slice(), "XOR mismatch");
        }
    }
}
