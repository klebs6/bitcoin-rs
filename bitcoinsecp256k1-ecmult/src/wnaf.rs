// ---------------- [ File: bitcoinsecp256k1-ecmult/src/wnaf.rs ]
crate::ix!();

pub const WNAF_BITS: usize = 128;

#[macro_export]
macro_rules! wnaf_size_bits {
    ($bits:expr, $w:expr) => {
        ((($bits) + (($w) as usize) - 1usize) / (($w) as usize))
    };
}

#[macro_export]
macro_rules! wnaf_size {
    ($w:expr) => {
        wnaf_size_bits!(WNAF_BITS, ($w))
    };
}

#[cfg(test)]
mod wnaf_macro_contract_suite {
    use super::*;

    #[traced_test]
    fn wnaf_size_macros_match_manual_ceiling_division() {
        tracing::info!(
            target: "secp256k1::ecmult::tests",
            "wnaf_size_macros_match_manual_ceiling_division"
        );

        assert_eq!(WNAF_BITS, 128);

        for w in 2usize..=16usize {
            let got = wnaf_size_bits!(WNAF_BITS, w);
            let expected = (WNAF_BITS + w - 1) / w;

            tracing::debug!(
                target: "secp256k1::ecmult::tests",
                w = w,
                got = got,
                expected = expected,
                "wnaf_size_bits!(WNAF_BITS, w)"
            );

            assert_eq!(got, expected);
        }

        assert_eq!(wnaf_size!(4), 32);
        assert_eq!(wnaf_size!(8), 16);
    }
}
