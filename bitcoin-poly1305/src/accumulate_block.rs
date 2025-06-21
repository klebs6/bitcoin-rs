// ---------------- [ File: bitcoin-poly1305/src/accumulate_block.rs ]
crate::ix!();

#[inline(always)]
pub fn accumulate_block(
    h: &mut LimbArr5,
    block: &[u8; 16],
    add_high_bit: bool,
) {
    let h_before = *h;
    tracing::trace!(
        h_before = ?h_before,
        block     = ?block,
        add_high_bit,
        "accumulate_block: start"
    );

    let t0 = read_le32(&block[0..]);
    let t1 = read_le32(&block[4..]);
    let t2 = read_le32(&block[8..]);
    let t3 = read_le32(&block[12..]);
    tracing::debug!(t0, t1, t2, t3, "accumulate_block: parsed little‑endian words");

    h[0] = h[0].wrapping_add(t0 & 0x3ffffff);
    h[1] = h[1].wrapping_add((((t1 as u64) << 32 | t0 as u64) >> 26) as u32 & 0x3ffffff);
    h[2] = h[2].wrapping_add((((t2 as u64) << 32 | t1 as u64) >> 20) as u32 & 0x3ffffff);
    h[3] = h[3].wrapping_add((((t3 as u64) << 32 | t2 as u64) >> 14) as u32 & 0x3ffffff);
    h[4] = h[4].wrapping_add((t3 >> 8) | if add_high_bit { 1 << 24 } else { 0 });

    let h_after = *h;
    tracing::trace!(h_after = ?h_after, "accumulate_block: finished");
}

#[cfg(test)]
mod tests_accumulate {
    use super::*;
    use proptest::prelude::*;

    /// Fixed check from the original suite.
    #[traced_test]
    fn high_bit_flag_sets_bit_24() {
        let mut h = [0u32; 5];
        let zero = [0u8; 16];

        accumulate_block(&mut h, &zero, true);
        assert_eq!(h[4] >> 24, 1, "high‑bit must be set");

        let mut h2 = [0u32; 5];
        accumulate_block(&mut h2, &zero, false);
        assert_eq!(h2[4] >> 24, 0, "high‑bit must remain clear");
    }

    /// Exhaustive random‑ised validation of the limb arithmetic.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(512))]

        #[traced_test]
        fn random_block_roundtrips(
            mut h in proptest::array::uniform5(0u32..(1u32 << 26)),
            block in proptest::array::uniform16(any::<u8>()),
            add_high_bit in any::<bool>(),
        ) {
            // “Reference” result computed in‑line (straight spec implementation).
            let mut expected = h;

            let t0 = u32::from_le_bytes([block[0], block[1], block[2], block[3]]);
            let t1 = u32::from_le_bytes([block[4], block[5], block[6], block[7]]);
            let t2 = u32::from_le_bytes([block[8], block[9], block[10], block[11]]);
            let t3 = u32::from_le_bytes([block[12], block[13], block[14], block[15]]);

            expected[0] = expected[0].wrapping_add(t0 & 0x3ffffff);
            expected[1] = expected[1].wrapping_add((((t1 as u64) << 32 | t0 as u64) >> 26) as u32 & 0x3ffffff);
            expected[2] = expected[2].wrapping_add((((t2 as u64) << 32 | t1 as u64) >> 20) as u32 & 0x3ffffff);
            expected[3] = expected[3].wrapping_add((((t3 as u64) << 32 | t2 as u64) >> 14) as u32 & 0x3ffffff);
            expected[4] = expected[4].wrapping_add((t3 >> 8) | if add_high_bit { 1 << 24 } else { 0 });

            accumulate_block(&mut h, &block, add_high_bit);

            prop_assert_eq!(h, expected, "accumulate_block must match reference arithmetic");
        }
    }
}
