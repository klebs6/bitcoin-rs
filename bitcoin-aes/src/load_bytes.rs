// ---------------- [ File: bitcoin-aes/src/load_bytes.rs ]
crate::ix!();

/// Load 16 bytes into their bit‑sliced representation (8× u16 words ≈ AES state).
#[inline(always)]
pub fn load_bytes(state: *mut AESState, mut data16: *const u8) {
    tracing::trace!(
        target: "aes",
        "load_bytes – entry; state_ptr = {:p}, data_ptr = {:p}",
        state,
        data16
    );

    // Safety: caller guarantees both pointers are valid for the required size.
    unsafe {
        // Column‑major loader (matches FIPS‑197 little‑endian reference
        // implementation and the lane diagram above).
        for col in 0..4 {
            for row in 0..4 {
                let byte = *data16;
                data16 = data16.add(1);
                load_byte(&mut *state, byte, row as i32, col as i32);
            }
        }
    }

    tracing::trace!(target: "aes", "load_bytes – exit");
}

// ---------------- [ File: bitcoin-aes/src/load_bytes.rs ] (new)

#[cfg(test)]
mod load_byte_validation {
    use super::*;

    /// Exhaustively verify that `load_byte` places *every* bit of the input
    /// byte at the correct lane (`row * 4 + col`) of the corresponding slice.
    #[traced_test]
    fn bit_lane_mapping_is_exact() {
        for row in 0..4 {
            for col in 0..4 {
                for byte in 0u8..=255 {
                    let mut state = AESState::default();
                    load_byte(&mut state, byte, row, col);

                    // Build reference bit‑slices.
                    let lane = (col * 4 + row) as u16;      // ← updated

                    let mut expected = [0u16; 8];
                    for bit in 0..8 {
                        if (byte >> bit) & 1 == 1 {
                            expected[bit] = 1u16 << lane;
                        }
                    }

                    debug!(row, col, byte, ?expected, "checking bit mapping for slice");
                    assert_eq!(
                        state.slice(),
                        &expected,
                        "Incorrect bit mapping for row={row}, col={col}, byte=0x{byte:02x}"
                    );
                }
            }
        }
    }

    /// Round‑trip identity: `load_bytes` followed by `save_bytes`
    /// must reproduce the original 16‑byte block for *random* inputs.
    #[traced_test]
    fn load_then_save_is_identity() {
        let mut rng = thread_rng();

        for _ in 0..2_000 {
            let mut input = [0u8; 16];
            rng.fill(&mut input);

            let mut state = AESState::default();
            unsafe { load_bytes(&mut state as *mut _, input.as_ptr()) };

            let mut output = [0u8; 16];
            unsafe { save_bytes(output.as_mut_ptr(), &state as *const _) };

            info!(target: "test", ?input, ?output);
            assert_eq!(output, input, "load/save round‑trip failed");
        }
    }

    /// Consistency: `load_bytes` must yield the same state as sixteen
    /// successive calls to `load_byte`.
    #[traced_test]
    fn bulk_and_scalar_loading_match() {
        let mut rng = thread_rng();

        for _ in 0..1_024 {
            let mut bytes = [0u8; 16];
            rng.fill(&mut bytes);

            // Bulk path
            let mut bulk_state = AESState::default();
            unsafe { load_bytes(&mut bulk_state as *mut _, bytes.as_ptr()) };

            // Scalar path
            let mut scalar_state = AESState::default();
            let mut idx = 0;
            for col in 0..4 {
                for row in 0..4 {
                    load_byte(&mut scalar_state, bytes[idx], row, col);
                    idx += 1;
                }
            }

            assert_eq!(
                bulk_state.slice(),
                scalar_state.slice(),
                "Mismatch between load_bytes and repeated load_byte"
            );
        }
    }

    /// `save_bytes` must produce a byte array that, when re‑loaded through
    /// `load_bytes`, re‑creates the original `AESState` **exactly**.
    #[traced_test]
    fn save_then_load_restores_state() {
        let mut rng = thread_rng();

        for _ in 0..1_024 {
            // Random bit‑sliced state.
            let mut slice = [0u16; 8];
            for w in &mut slice {
                *w = rng.gen();
            }
            let original = AESState::from_slice(slice);

            // Serialise to bytes.
            let mut bytes = [0u8; 16];
            unsafe { save_bytes(bytes.as_mut_ptr(), &original as *const _) };

            // Re‑load into a fresh state.
            let mut restored = AESState::default();
            unsafe { load_bytes(&mut restored as *mut _, bytes.as_ptr()) };

            debug!(?slice, ?bytes);
            assert_eq!(
                restored.slice(),
                original.slice(),
                "save/load cycle corrupted the state"
            );
        }
    }

    /// Bulk loader must equal sixteen single‑byte loads (property‑test).
    #[traced_test]
    fn bulk_equals_scalar() {
        let mut rng = thread_rng();

        for _ in 0..500 {
            let mut block = [0u8; 16];
            rng.fill(&mut block);

            // bulk path
            let mut bulk = AESState::default();
            unsafe { load_bytes(&mut bulk as *mut _, block.as_ptr()) };

            // scalar reference path
            let mut scalar = AESState::default();
            for (idx, byte) in block.into_iter().enumerate() {
                let row = (idx % 4) as i32;
                let col = (idx / 4) as i32; // column‑major
                load_byte(&mut scalar, byte, row, col);
            }

            assert_eq!(
                bulk.slice(),
                scalar.slice(),
                "bulk vs scalar mismatch"
            );
        }
    }

    /// Edge vectors: all‑zero, all‑FF, and a single walking‑1 bit across the
    /// entire 128‑bit block.
    #[traced_test]
    fn edge_case_vectors() {
        let mut vectors = Vec::new();
        vectors.push([0u8; 16]);
        vectors.push([0xFFu8; 16]);

        // Walking bit
        for i in 0..128 {
            let mut v = [0u8; 16];
            v[i / 8] = 1u8 << (i % 8);
            vectors.push(v);
        }

        for v in vectors {
            let mut st = AESState::default();
            unsafe { load_bytes(&mut st as *mut _, v.as_ptr()) };

            let pop_state: u32 = st.slice().iter().map(|w| w.count_ones()).sum();
            let pop_block: u32 = v.iter().map(|b| b.count_ones()).sum();
            assert_eq!(pop_state, pop_block, "population count mismatch for {v:?}");
        }
    }
}
