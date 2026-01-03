// ---------------- [ File: bitcoinsecp256k1-scratch/src/ctz64_var.rs ]
crate::ix!();

/// Determine the number of trailing zero bits in a (non-zero) 64-bit x.
/// 
#[inline] pub fn ctz64_var(x: u64) -> i32 {
    
    trace!(target: "bitcoinsecp256k1_scratch::util", "ctz64_var");

    VERIFY_CHECK!{x != 0};

    x.trailing_zeros() as i32
}

#[cfg(test)]
mod ctz64_var_accuracy_test_suite {
    use super::*;

    fn reference_ctz64(mut x: u64) -> i32 {
        let mut n: i32 = 0;
        while (x & 1) == 0 {
            n += 1;
            x >>= 1;
        }
        n
    }

    #[traced_test]
    fn ctz64_var_matches_reference_for_all_single_bit_positions() {
        for i in 0..64u32 {
            let x = 1u64 << i;
            let got = ctz64_var(x);
            let want = i as i32;

            trace!(
                target: "bitcoinsecp256k1_scratch::tests::ctz64_var",
                i,
                x = x as usize,
                got,
                want,
                "single-bit trailing-zeros check"
            );

            assert_eq!(got, want);
        }
    }

    #[traced_test]
    fn ctz64_var_matches_reference_for_various_bit_patterns() {
        let cases: [u64; 14] = [
            1,
            3,
            4,
            6,
            8,
            12,
            16,
            24,
            0x8000_0000_0000_0000,
            0x0000_0001_0000_0000,
            0x0000_00F0_0000_0000,
            0x0000_0100_0000_0000,
            0xFFFF_FFFF_FFFF_FFFE,
            0x7FFF_FFFF_FFFF_FFF0,
        ];

        for &x in cases.iter() {
            assert_ne!(x, 0);
            let got = ctz64_var(x);
            let want = reference_ctz64(x);

            debug!(
                target: "bitcoinsecp256k1_scratch::tests::ctz64_var",
                x = x as usize,
                got,
                want,
                "pattern trailing-zeros check"
            );

            assert_eq!(got, want);
        }
    }
}
