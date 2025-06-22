// ---------------- [ File: bitcoin-aes/src/save_bytes.rs ]
crate::ix!();

/// Serialise an `AESState` back to 16 bytes (column‑major order).
#[inline(always)]
pub fn save_bytes(mut data16: *mut u8, state: *const AESState) {
    // SAFETY: caller guarantees the pointers are valid for 16 bytes.
    unsafe {
        // Same traversal order that `load_bytes` expects: c‑outer, r‑inner.
        for col in 0..4 {
            for row in 0..4 {
                let lane = (col * 4 + row) as u16;           // ← column‑major
                let mut byte = 0u8;
                for bit in 0..8 {
                    let bit_val = (((*state).slice[bit] >> lane) & 1) as u8;
                    byte |= bit_val << bit;
                }
                *data16 = byte;
                data16 = data16.add(1);
            }
        }
    }
}

#[cfg(test)]
mod save_bytes_spec {

    use super::*;

    /// `load_bytes` → `save_bytes` must be identity on the raw 16‑byte block.
    #[traced_test]
    fn load_then_save_is_identity() {
        let mut rng = thread_rng();

        for _ in 0..2_000 {
            let mut in_block = [0u8; 16];
            rng.fill(&mut in_block);

            let mut state = AESState::default();
            unsafe { load_bytes(&mut state as *mut _, in_block.as_ptr()) };

            let mut out_block = [0u8; 16];
            unsafe { save_bytes(out_block.as_mut_ptr(), &state as *const _) };

            assert_eq!(in_block, out_block, "round‑trip failed");
        }
    }

    /// Serialising a random bit‑sliced state and re‑loading it must recover the
    /// exact 8×u16 words.
    #[traced_test]
    fn save_then_load_restores_state() {
        let mut rng = thread_rng();

        for _ in 0..1_024 {
            let mut slice = [0u16; 8];
            for w in &mut slice {
                *w = rng.gen();
            }
            let original = AESState::from_slice(slice);

            // → bytes
            let mut bytes = [0u8; 16];
            unsafe { save_bytes(bytes.as_mut_ptr(), &original as *const _) };

            // → state again
            let mut restored = AESState::default();
            unsafe { load_bytes(&mut restored as *mut _, bytes.as_ptr()) };

            assert_eq!(
                original.slice(),
                restored.slice(),
                "state corruption after save/load"
            );
        }
    }
}
