// ---------------- [ File: bitcoinsecp256k1-scratch/src/ctz32_var.rs ]
crate::ix!();

/// Determine the number of trailing zero bits in a (non-zero) 32-bit x.
///
#[inline] pub fn ctz32_var(x: u32) -> i32 {
    
    trace!(target: "bitcoinsecp256k1_scratch::util", "ctz32_var");

    verify_check!{x != 0};

    x.trailing_zeros() as i32
}

#[cfg(test)]
mod ctz32_var_accuracy_test_suite {
    use super::*;

    fn reference_ctz32(mut x: u32) -> i32 {
        let mut n: i32 = 0;
        while (x & 1) == 0 {
            n += 1;
            x >>= 1;
        }
        n
    }

    #[traced_test]
    fn ctz32_var_matches_reference_for_all_single_bit_positions() {
        for i in 0..32u32 {
            let x = 1u32 << i;
            let got = ctz32_var(x);
            let want = i as i32;

            trace!(
                target: "bitcoinsecp256k1_scratch::tests::ctz32_var",
                i,
                x,
                got,
                want,
                "single-bit trailing-zeros check"
            );

            assert_eq!(got, want);
        }
    }

    #[traced_test]
    fn ctz32_var_matches_reference_for_various_bit_patterns() {
        let cases: [u32; 14] = [
            1,
            3,
            4,
            6,
            8,
            12,
            16,
            24,
            0x8000_0000,
            0x0001_0000,
            0x00F0_0000,
            0x0100_0000,
            0xFFFF_FFFE,
            0x7FFF_FFF0,
        ];

        for &x in cases.iter() {
            assert_ne!(x, 0);
            let got = ctz32_var(x);
            let want = reference_ctz32(x);

            debug!(
                target: "bitcoinsecp256k1_scratch::tests::ctz32_var",
                x,
                got,
                want,
                "pattern trailing-zeros check"
            );

            assert_eq!(got, want);
        }
    }
}
