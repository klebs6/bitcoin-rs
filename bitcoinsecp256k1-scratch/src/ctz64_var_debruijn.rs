// ---------------- [ File: bitcoinsecp256k1-scratch/src/ctz64_var_debruijn.rs ]
crate::ix!();

/// Determine the number of trailing zero bits in a (non-zero) 64-bit x.
/// 
/// This function is only intended to be used as fallback for ctz64_var, but permits it to be
/// tested separately.
/// 
#[inline] pub fn ctz64_var_debruijn(x: u64) -> i32 {
    
    const DEBRUIJN: [u8; 64] = [
        0, 1, 2, 53, 3, 7, 54, 27, 4, 38, 41, 8, 34, 55, 48, 28,
        62, 5, 39, 46, 44, 42, 22, 9, 24, 35, 59, 56, 49, 18, 29, 11,
        63, 52, 6, 26, 37, 40, 33, 47, 61, 45, 43, 21, 23, 58, 17, 10,
        51, 25, 36, 32, 60, 20, 57, 16, 50, 31, 19, 15, 30, 14, 13, 12
    ];

    trace!(target: "bitcoinsecp256k1_scratch::util", "ctz64_var_debruijn");

    let idx: usize = ((x & x.wrapping_neg()).wrapping_mul(0x022FDD63CC95386D) >> 58) as usize;
    DEBRUIJN[idx] as i32
}

#[cfg(test)]
mod ctz64_var_debruijn_correctness_test_suite {
    use super::*;

    #[traced_test]
    fn ctz64_var_debruijn_matches_builtin_for_all_single_bit_positions() {
        for i in 0..64u32 {
            let x = 1u64 << i;
            let got = ctz64_var_debruijn(x);
            let want = x.trailing_zeros() as i32;

            trace!(
                target: "bitcoinsecp256k1_scratch::tests::ctz64_debruijn",
                i,
                x = x as usize,
                got,
                want,
                "single-bit debruijn trailing-zeros check"
            );

            assert_eq!(got, want);
        }
    }

    #[traced_test]
    fn ctz64_var_debruijn_matches_ctz64_var_for_selected_values() {
        let cases: [u64; 16] = [
            1,
            2,
            3,
            5,
            10,
            0x0000_0000_0000_0010,
            0x0000_0000_0000_0030,
            0x0000_0000_0000_0100,
            0x0000_0000_0000_8000,
            0x0000_0001_0000_0000,
            0x0000_0010_0000_0000,
            0x0000_0100_0000_0000,
            0x4000_0000_0000_0000,
            0x8000_0000_0000_0000,
            0xFFFF_FFFF_FFFF_FFFE,
            0x7FFF_FFFF_FFFF_FFF0,
        ];

        for &x in cases.iter() {
            assert_ne!(x, 0);
            let got = ctz64_var_debruijn(x);
            let want = ctz64_var(x);

            debug!(
                target: "bitcoinsecp256k1_scratch::tests::ctz64_debruijn",
                x = x as usize,
                got,
                want,
                "cross-check against ctz64_var"
            );

            assert_eq!(got, want);
        }
    }
}
