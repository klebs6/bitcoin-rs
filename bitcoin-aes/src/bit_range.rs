// ---------------- [ File: bitcoin-aes/src/bit_range.rs ]
crate::ix!();

/// Inclusive lower‑bound, exclusive upper‑bound bit‑range mask.
#[macro_export]
macro_rules! bit_range {
    ($from:expr, $to:expr) => {
        (((1u16 << ($to - $from)) - 1) << $from)
    };
}

/// Mask + left‑shift a sub‑range of bits.
#[macro_export]
macro_rules! bit_range_left {
    ($x:expr, $from:expr, $to:expr, $shift:expr) => {
        (($x & bit_range!($from, $to)) << $shift)
    };
}

/// Mask + right‑shift a sub‑range of bits.
#[macro_export]
macro_rules! bit_range_right {
    ($x:expr, $from:expr, $to:expr, $shift:expr) => {
        (($x & bit_range!($from, $to)) >> $shift)
    };
}

/// Rotate 4‑bit “nibbles” inside a 16‑bit word.
#[macro_export]
macro_rules! rot {
    ($x:expr, $b:expr) => {
        (($x >> ($b * 4)) | ($x << ((4 - $b) * 4)))
    };
}

#[cfg(test)]
mod bit_range_macro_validation {
    use super::*;

    /// Deterministic sanity checks for well‑known masks.
    #[traced_test]
    fn simple_masks_are_correct() {
        assert_eq!(bit_range!(0, 4), 0x000F, "low‑nibble");
        assert_eq!(bit_range!(4, 8), 0x00F0, "upper‑nibble");
        assert_eq!(bit_range!(7, 8), 0x0080, "single high bit");
    }

    /// Property test: for 1 000 random “from/to” pairs where
    /// `(to‑from) ≤ 15`, the macro must match the mathematical mask.
    #[traced_test]
    fn random_mask_matches_formula() {
        let mut rng = thread_rng();

        for _ in 0..1_000 {
            let from: u16 = rng.gen_range(0..15);
            let len : u16 = rng.gen_range(1..=15 - from); // ensures shift < 16
            let to   = from + len;

            let expected = (((1u16 << (to - from)) - 1) << from) as u16;
            let produced = bit_range!(from, to);

            debug!(target: "test", from, to, expected, produced, "mask comparison");
            assert_eq!(produced, expected, "bit_range!({from}, {to}) mismatch");
        }
    }

    /// `bit_range_left!` and `bit_range_right!` must equal their
    /// manual shift‑and‑mask counterparts.
    #[traced_test]
    fn left_right_helpers_match_manual() {
        let mut rng = thread_rng();

        for _ in 0..1_000 {
            let x: u16 = rng.gen();
            let from: usize = rng.gen_range(0..8);
            let len : usize = rng.gen_range(1..=15 - from);
            let to  = from + len;
            let shift: usize = rng.gen_range(0..4);

            let manual_left  = (x & (((1u16 << (to - from)) - 1) << from)) << shift;
            let macro_left   = bit_range_left!(x, from, to, shift);
            let manual_right = (x & (((1u16 << (to - from)) - 1) << from)) >> shift;
            let macro_right  = bit_range_right!(x, from, to, shift);

            trace!(target: "test", x, from, to, shift, manual_left, macro_left, manual_right, macro_right);
            assert_eq!(macro_left,  manual_left,  "bit_range_left! mismatch");
            assert_eq!(macro_right, manual_right, "bit_range_right! mismatch");
        }
    }
}
