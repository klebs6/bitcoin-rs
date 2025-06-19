// ---------------- [ File: bitcoin-aes/src/load_bytes.rs ]
crate::ix!();

/// Insert one plaintext byte (`byte_in`) into the bit‑sliced AES state `state`.
///
/// Each of the eight slices represents one bit‑plane over the 4 × 4 byte
/// state matrix.  The target bit‑position for the *least‑significant* bit of
/// `byte_in` is determined by `row` (`0‥3`) and `col` (`0‥3`) as
/// `row * 4 + col`.  Higher‑order bits follow in the same slice, so the entire
/// byte occupies a vertical “lane” across the eight 16‑bit words.
///
/// This replacement fixes a **debug‑build panic** that arose from shifting an
/// `u8` by more than 7 bits.  The bit to be inserted is now widened to `u16`
/// *before* the shift, ensuring all shift counts (up to 15) are within range.
#[inline(always)]
pub fn load_byte(
    state: &mut AESState,
    mut byte_in: u8,
    row: i32,
    col: i32,
) {
    //------------------------------------------------------------------------
    // Diagnostics
    //------------------------------------------------------------------------
    tracing::trace!(
        target = "aes",
        "load_byte – entry byte = 0x{byte_in:02x}, row = {row}, col = {col}"
    );

    debug_assert!((0..4).contains(&row), "row must be 0‥3");
    debug_assert!((0..4).contains(&col), "col must be 0‥3");

    //-----------------------------------------------------------------------
    // Compute the 0‑based bit offset inside each 16‑bit slice.
    //-----------------------------------------------------------------------
    let offset: u16 = (row * 4 + col) as u16;

    //-----------------------------------------------------------------------
    // Scatter the 8 bits of `byte_in` (LSB first) across the 8 slices.
    //-----------------------------------------------------------------------
    for slice_idx in 0..8 {
        let lsb: u16 = (byte_in & 1) as u16;      // extract current LSB
        state.slice[slice_idx] |= lsb << offset;  // safe: `lsb` is u16

        byte_in >>= 1;                            // prepare next bit
    }

    tracing::trace!(target = "aes", "load_byte – exit");
}

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
        for c in 0..4 {
            for r in 0..4 {
                let byte = *data16;
                data16 = data16.add(1);
                load_byte(&mut *state, byte, r as i32, c as i32);
            }
        }
    }

    tracing::trace!(target: "aes", "load_bytes – exit");
}

/// Convert a bit‑sliced `AESState` back into its canonical 16‑byte form.
#[inline(always)]
pub fn save_bytes(mut data16: *mut u8, state: *const AESState) {
    tracing::trace!(
        target: "aes",
        "save_bytes – entry; state_ptr = {:p}, data_ptr = {:p}",
        state,
        data16
    );

    // Safety: caller guarantees both pointers are valid for the required size.
    unsafe {
        for c in 0..4 {
            for r in 0..4 {
                let mut v: u8 = 0;
                for b in 0..8 {
                    let bit = (((*state).slice[b] >> (r * 4 + c)) & 1) as u8;
                    v |= bit << b;
                }
                *data16 = v;
                data16 = data16.add(1);
            }
        }
    }

    tracing::trace!(target: "aes", "save_bytes – exit");
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
                    let lane = (row * 4 + col) as u16;
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
}
