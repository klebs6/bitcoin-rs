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
    rounds: *mut AESState,
    key: *const u8,
    nkeywords: i32,
    nrounds: i32,
) {
    tracing::info!(
        target: "aes",
        "aes_setup – entry; rounds = {:p}, key = {:p}, nkeywords = {}, nrounds = {}",
        rounds,
        key,
        nkeywords,
        nrounds
    );

    unsafe {
        /* ---------- zero‑initialise the round‑key schedule ------------- */
        for i in 0..=(nrounds as usize) {
            (*rounds.add(i)).slice = [0u16; 8];
        }

        /* ---------- copy the raw key into the first `nkeywords` words -- */
        let mut key_ptr = key;
        for word in 0..(nkeywords as usize) {
            let round_idx = word >> 2;          // which AESState
            let col_idx = (word & 3) as i32;    // which column within that state
            for row in 0..4 {
                let byte = *key_ptr;
                key_ptr = key_ptr.add(1);
                load_byte(&mut *rounds.add(round_idx), byte, row as i32, col_idx);
            }
        }

        /* ---------- round‑constant (rcon) and working column ----------- */
        let mut rcon = AESState {
            slice: [1, 0, 0, 0, 0, 0, 0, 0],
        };
        let mut column = AESState::default();
        get_one_column(
            &mut column as *mut _,
            rounds.add(((nkeywords - 1) >> 2) as usize),
            ((nkeywords - 1) & 3),
        );

        /* ---------- expand remaining words ---------------------------- */
        let mut pos: i32 = 0;
        let total_words = 4 * (nrounds + 1);
        for i in nkeywords..total_words {
            /* transform once per full keyword group, and additionally for AES‑256 at pos==4 */
            if pos == 0 {
                sub_bytes(&mut column, false);
                key_setup_transform(&mut column as *mut _, &rcon as *const _);
                multx(&mut rcon as *mut _);
            } else if nkeywords > 6 && pos == 4 {
                sub_bytes(&mut column, false);
            }

            pos += 1;
            if pos == nkeywords {
                pos = 0;
            }

            key_setup_column_mix(
                &mut column as *mut _,
                rounds.add((i >> 2) as usize),
                rounds.add(((i - nkeywords) >> 2) as usize),
                (i & 3),
                ((i - nkeywords) & 3),
            );
        }
    }

    tracing::info!(target: "aes", "aes_setup – exit");
}

#[cfg(test)]
mod aes_setup_round_key_validation {
    use super::*;

    /// For AES‑128/192/256 the first *nkeywords* 32‑bit words of the expanded
    /// key **must exactly equal** the user‑supplied key material (in
    /// column‑major order) – this follows directly from FIPS‑197 §5.2 *Key
    /// Expansion*.  This test reconstructs those bytes and compares them
    /// byte‑for‑byte against the original random keys.
    #[traced_test]
    fn raw_key_is_preserved_in_round_zero() {
        let mut rng = thread_rng();

        #[derive(Clone, Copy)]
        struct Case {
            nkeywords: i32,
            nrounds:   i32,
            key_len:   usize,
        }
        const CASES: &[Case] = &[
            Case { nkeywords: 4, nrounds: 10, key_len: 16 }, // AES‑128
            Case { nkeywords: 6, nrounds: 12, key_len: 24 }, // AES‑192
            Case { nkeywords: 8, nrounds: 14, key_len: 32 }, // AES‑256
        ];

        for case in CASES {
            // ---------------- random key material ----------------------
            let mut key = vec![0u8; case.key_len];
            rng.fill(&mut key[..]);

            // ---------------- run key schedule ------------------------
            let mut schedule = vec![AESState::default(); (case.nrounds + 1) as usize];
            unsafe { aes_setup(schedule.as_mut_ptr(), key.as_ptr(), case.nkeywords, case.nrounds) };

            // ---------------- extract round‑0 bytes -------------------
            let mut extracted = Vec::with_capacity(case.key_len);
            for word in 0..case.nkeywords as usize {
                let round_idx = word >> 2;
                let col = (word & 3) as i32;
                for row in 0..4 {
                    // Reverse operation of `load_byte`
                    let mut byte = 0u8;
                    for bit in 0..8 {
                        let lane = (row * 4 + col) as u16;
                        let bit_val = ((schedule[round_idx].slice()[bit] >> lane) & 1) as u8;
                        byte |= bit_val << bit;
                    }
                    extracted.push(byte);
                }
            }

            debug!(target: "test", algo = ?(case.key_len*8), ?key, ?extracted);
            assert_eq!(extracted, key, "round‑0 key mismatch for AES‑{}", case.key_len*8);
        }
        info!(target: "test", "aes_setup raw‑key preservation ✓");
    }
}
