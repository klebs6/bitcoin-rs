// ---------------- [ File: bitcoin-aes/src/expand_schedule.rs ]
crate::ix!();

#[inline(always)]
pub fn expand_schedule(rounds: *mut AESState, nkeywords: i32, nrounds: i32) {
    tracing::trace!(
        target: "aes",
        "expand_schedule – rounds = {:p}, nkeywords = {}, nrounds = {}",
        rounds, nkeywords, nrounds
    );

    unsafe {
        let mut rcon = AESState { slice: [1, 0, 0, 0, 0, 0, 0, 0] };
        let mut column = AESState::default();
        get_one_column(
            &mut column as *mut _,
            rounds.add(((nkeywords - 1) >> 2) as usize),
            ((nkeywords - 1) & 3),
        );

        let mut pos = 0;
        let total_words = 4 * (nrounds + 1);

        for i in nkeywords..total_words {
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
}

#[cfg(test)]
mod expand_schedule_validation {
    use super::*;

    /// For random keys across all AES sizes, the decomposed pipeline
    /// (`zero_schedule` → `populate_round_zero` → `expand_schedule`) must
    /// produce **identical round‑keys** to `aes_setup`.
    #[traced_test]
    fn decomposed_vs_monolith_identical() {
        let mut rng = thread_rng();

        #[derive(Clone, Copy)]
        struct Case { nkeywords: i32, nrounds: i32, key_len: usize }
        const CASES: &[Case] = &[
            Case { nkeywords: 4, nrounds: 10, key_len: 16 },
            Case { nkeywords: 6, nrounds: 12, key_len: 24 },
            Case { nkeywords: 8, nrounds: 14, key_len: 32 },
        ];

        for case in CASES {
            let mut key = vec![0u8; case.key_len];
            rng.fill(&mut key[..]);

            // --- reference (monolithic) --------------------------------
            let mut ref_sched = vec![AESState::default(); (case.nrounds + 1) as usize];
            unsafe {
                aes_setup(
                    ref_sched.as_mut_ptr(),
                    key.as_ptr(),
                    case.nkeywords,
                    case.nrounds,
                );
            }

            // --- decomposed pipeline -----------------------------------
            let mut decomp_sched = vec![AESState::default(); (case.nrounds + 1) as usize];
            unsafe {
                zero_schedule(decomp_sched.as_mut_ptr(), case.nrounds);
                populate_round_zero(decomp_sched.as_mut_ptr(), key.as_ptr(), case.nkeywords);
                expand_schedule(decomp_sched.as_mut_ptr(), case.nkeywords, case.nrounds);
            }

            assert_eq!(
                decomp_sched, ref_sched,
                "round‑key schedule diverged ({}‑bit)",
                case.key_len * 8
            );
        }
    }
}
