// ---------------- [ File: bitcoin-aes/src/ctaes_setup.rs ]
crate::ix!();

/**
  | Expand the cipher key into the key schedule.
  | state must be a pointer to an array of
  | size nrounds + 1. key must be a pointer
  | to 4 * nkeywords bytes.
  | 
  | AES128 uses nkeywords = 4, nrounds =
  | 10
  | 
  | AES192 uses nkeywords = 6, nrounds =
  | 12
  | 
  | AES256 uses nkeywords = 8, nrounds =
  | 14
  |
  */
#[inline(always)]
pub fn aes_setup(
    rounds:     *mut AESState,
    key:        *const u8,
    nkeywords:  i32,
    nrounds:    i32,
) {
    tracing::info!(
        target: "aes",
        "aes_setup – entry; rounds = {:p}, key = {:p}, nkeywords = {}, nrounds = {}",
        rounds, key, nkeywords, nrounds
    );

    unsafe {
        zero_schedule(rounds, nrounds);
        populate_round_zero(rounds, key, nkeywords);
        expand_schedule(rounds, nkeywords, nrounds);
    }

    tracing::info!(target: "aes", "aes_setup – exit");
}

#[cfg(test)]
mod aes_setup_round_key_validation {
    use super::*;

    /// *Round‑0* of the expanded key must reproduce the caller‑supplied key
    /// **exactly** in **column‑major** order for AES‑128/192/256.
    #[traced_test]
    fn raw_key_is_preserved_in_round_zero() {
        let mut rng = thread_rng();

        #[derive(Clone, Copy)]
        struct Case { nkeywords: i32, nrounds: i32, key_len: usize }
        const CASES: &[Case] = &[
            Case { nkeywords: 4, nrounds: 10, key_len: 16 }, // AES‑128
            Case { nkeywords: 6, nrounds: 12, key_len: 24 }, // AES‑192
            Case { nkeywords: 8, nrounds: 14, key_len: 32 }, // AES‑256
        ];

        for case in CASES {
            // ----------- random key material ----------------------------
            let mut key = vec![0u8; case.key_len];
            rng.fill(&mut key[..]);

            // ----------- run key‑schedule -------------------------------
            let mut sched = vec![AESState::default(); (case.nrounds + 1) as usize];
            unsafe { aes_setup(sched.as_mut_ptr(), key.as_ptr(), case.nkeywords, case.nrounds) };

            // ----------- pull out round‑0 bytes (column‑major) ----------
            let mut extracted = Vec::with_capacity(case.key_len);
            for word in 0..case.nkeywords as usize {
                let round_idx = word >> 2;            // which AESState
                let col       = (word & 3) as i32;    // which column inside it
                for row in 0..4 {
                    let lane = (col * 4 + row) as u16;    // **column‑major**
                    let mut byte = 0u8;
                    for bit in 0..8 {
                        let v = ((sched[round_idx].slice()[bit] >> lane) & 1) as u8;
                        byte |= v << bit;
                    }
                    extracted.push(byte);
                }
            }

            debug!(target: "test",
                   algo = ?(case.key_len*8),
                   ?key,
                   ?extracted,
                   "raw‑key extraction comparison");
            assert_eq!(extracted, key,
                "round‑0 key mismatch for AES‑{}", case.key_len * 8);
        }

        info!(target: "test", "aes_setup raw‑key preservation ✓");
    }
}
