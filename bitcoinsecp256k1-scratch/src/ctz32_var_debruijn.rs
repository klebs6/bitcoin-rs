// ---------------- [ File: bitcoinsecp256k1-scratch/src/ctz32_var_debruijn.rs ]
crate::ix!();

/// Determine the number of trailing zero bits in a (non-zero) 32-bit x.
/// 
/// This function is only intended to be used as fallback for ctz32_var, but permits it to be
/// tested separately.
/// 
#[inline] pub fn ctz32_var_debruijn(x: u32) -> i32 {

    const DEBRUIJN: [u8; 32] = [
        0x00, 0x01, 0x02, 0x18, 0x03, 0x13, 0x06, 0x19, 0x16, 0x04, 0x14, 0x0A,
        0x10, 0x07, 0x0C, 0x1A, 0x1F, 0x17, 0x12, 0x05, 0x15, 0x09, 0x0F, 0x0B,
        0x1E, 0x11, 0x08, 0x0E, 0x1D, 0x0D, 0x1C, 0x1B
    ];

    trace!(target: "bitcoinsecp256k1_scratch::util", "ctz32_var_debruijn");

    let idx: usize = ((x & x.wrapping_neg()).wrapping_mul(0x04D7651F) >> 27) as usize;
    DEBRUIJN[idx] as i32
}

#[cfg(test)]
mod ctz32_var_debruijn_correctness_test_suite {
    use super::*;

    #[traced_test]
    fn ctz32_var_debruijn_matches_builtin_for_all_single_bit_positions() {
        for i in 0..32u32 {
            let x = 1u32 << i;
            let got = ctz32_var_debruijn(x);
            let want = x.trailing_zeros() as i32;

            trace!(
                target: "bitcoinsecp256k1_scratch::tests::ctz32_debruijn",
                i,
                x,
                got,
                want,
                "single-bit debruijn trailing-zeros check"
            );

            assert_eq!(got, want);
        }
    }

    #[traced_test]
    fn ctz32_var_debruijn_matches_ctz32_var_for_selected_values() {
        let cases: [u32; 16] = [
            1,
            2,
            3,
            5,
            10,
            0x0000_0010,
            0x0000_0030,
            0x0000_0100,
            0x0000_8000,
            0x0001_0000,
            0x0010_0000,
            0x0100_0000,
            0x4000_0000,
            0x8000_0000,
            0xFFFF_FFFE,
            0x7FFF_FFF0,
        ];

        for &x in cases.iter() {
            assert_ne!(x, 0);
            let got = ctz32_var_debruijn(x);
            let want = ctz32_var(x);

            debug!(
                target: "bitcoinsecp256k1_scratch::tests::ctz32_debruijn",
                x,
                got,
                want,
                "cross-check against ctz32_var"
            );

            assert_eq!(got, want);
        }
    }
}
