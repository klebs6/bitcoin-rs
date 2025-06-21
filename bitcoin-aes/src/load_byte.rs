crate::ix!();

#[inline(always)]
pub fn load_byte(state: &mut AESState, mut byte_in: u8, row: i32, col: i32) {
    // Lane index used everywhere else in the crate is **column‑major**:
    //   ┌───┬───┬───┬───┐        offset = col · 4 + row
    //   │ 0 │ 4 │ 8 │12 │
    //   │ 1 │ 5 │ 9 │13 │
    //   │ 2 │ 6 │10 │14 │
    //   │ 3 │ 7 │11 │15 │
    //   └───┴───┴───┴───┘
    let offset: u16 = (col * 4 + row) as u16;

    for slice_idx in 0..8 {
        let bit: u16 = (byte_in & 1) as u16;
        state.slice[slice_idx] |= bit << offset;
        byte_in >>= 1;
    }
}

#[cfg(test)]
mod load_byte_spec {

    use super::*;

    /// *Exhaustive* bit‑lane verification for every byte value.
    #[traced_test]
    fn exact_bit_lane_mapping() {
        for row in 0..4 {
            for col in 0..4 {
                for byte in 0u8..=u8::MAX {
                    let mut st = AESState::default();
                    load_byte(&mut st, byte, row, col);

                    let lane = (col * 4 + row) as u16; // column‑major mapping
                    for bit in 0..8 {
                        let want = ((byte >> bit) & 1) as u16;
                        let got = (st.slice()[bit] >> lane) & 1;
                        assert_eq!(
                            want, got,
                            "bit {bit} mismatch (row={row}, col={col}, byte=0x{byte:02x})"
                        );
                    }
                }
            }
        }
    }

    /// Writing into different (row,col) lanes must *accumulate* – not clobber –
    /// previously written data.
    #[traced_test]
    fn multiple_writes_accumulate() {
        let patterns = [
            (0, 0, 0b1010_1010u8),
            (1, 0, 0b0101_0101u8),
            (2, 0, 0b1111_0000u8),
            (3, 0, 0b0000_1111u8),
        ];

        let mut st = AESState::default();
        for &(r, c, b) in &patterns {
            load_byte(&mut st, b, r, c);
        }

        for &(r, _, b) in &patterns {
            let lane = r as u16; // still column 0
            for bit in 0..8 {
                let want = ((b >> bit) & 1) as u16;
                let got = (st.slice()[bit] >> lane) & 1;
                assert_eq!(want, got, "row {r}, bit {bit}");
            }
        }
    }

    /// Property: after inserting **all** 0x00‑FF byte values into the *same*
    /// lane, every slice bit in that lane must be 1.
    #[traced_test]
    fn full_byte_saturation() {
        let mut st = AESState::default();
        for b in 0u8..=u8::MAX {
            load_byte(&mut st, b, 0, 0);
        }
        for (idx, sl) in st.slice().iter().enumerate() {
            assert_eq!(sl & 1, 1, "slice {idx} not fully saturated");
        }
    }
}
