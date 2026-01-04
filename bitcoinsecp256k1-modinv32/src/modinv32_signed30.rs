// ---------------- [ File: bitcoinsecp256k1-modinv32/src/modinv32_signed30.rs ]
crate::ix!();

/// A signed 30-bit limb representation
/// of integers.
/// 
/// Its value is sum(v[i] * 2^(30*i), i=0..8).
/// 
#[derive(Debug,Copy,Clone)]
pub struct ModInv32Signed30 {
    pub v: [i32; 9],
}

#[cfg(test)]
mod modinv32_signed30_representation_validation {
    use super::*;

    #[traced_test]
    fn signed30_layout_matches_expected_size_and_alignment() {
        let size = core::mem::size_of::<ModInv32Signed30>();
        let align = core::mem::align_of::<ModInv32Signed30>();

        tracing::info!(size, align, "checking ModInv32Signed30 layout");

        assert!(size == 9 * 4);
        assert!(align == 4);
    }

    #[traced_test]
    fn signed30_low_limb_represents_small_integers_correctly() {
        let values: [i32; 10] = [
            -10,
            -2,
            -1,
            0,
            1,
            2,
            10,
            12345,
            -12345,
            support::M30_I32,
        ];

        for &v in values.iter() {
            let x = support::signed30_from_i32_low(v);
            let got = support::signed30_to_i128_horner(&x);
            tracing::debug!(v, got, "signed30 low limb roundtrip");
            assert!(got == v as i128);
        }
    }
}
