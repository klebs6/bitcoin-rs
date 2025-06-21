crate::ix!();

#[inline(always)]
pub fn zero_schedule(rounds: *mut AESState, nrounds: i32) {
    tracing::trace!(
        target: "aes",
        "zero_schedule – rounds = {:p}, nrounds = {}",
        rounds,
        nrounds
    );
    unsafe {
        for i in 0..=(nrounds as usize) {
            (*rounds.add(i)).slice = [0u16; 8];
        }
    }
}

#[cfg(test)]
mod zero_schedule_validation {
    use super::*;

    #[traced_test]
    fn schedule_words_are_zeroed() {
        const CASES: &[i32] = &[10, 12, 14]; // AES‑128/192/256
        for &nrounds in CASES {
            let mut schedule = vec![AESState::from_slice([0xFFFFu16; 8]); (nrounds + 1) as usize];
            unsafe { super::zero_schedule(schedule.as_mut_ptr(), nrounds); }
            for (idx, st) in schedule.iter().enumerate() {
                assert_eq!(
                    st.slice(),
                    &[0u16; 8],
                    "non‑zero slice after zero_schedule (round {idx}, nrounds = {nrounds})"
                );
            }
        }
    }
}
