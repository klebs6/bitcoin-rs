// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/verify_bits.rs ]
crate::ix!();

#[cfg(feature="secp256k1-verify")]
#[macro_export]
macro_rules! verify_bits {
    ($x:expr, $n:expr) => {
        verify_check(((($x) >> ($n)) == 0));
    }
}

#[cfg(not(feature="secp256k1-verify"))]
#[macro_export]
macro_rules! verify_bits {
    ($x:expr, $n:expr) => {
        {}
    }
}

#[cfg(test)]
mod verify_bits_rs_exhaustive_tests {
    use super::*;

    #[traced_test]
    fn verify_bits_macro_accepts_expr_form_and_succeeds_when_high_bits_are_zero() {
        tracing::info!("testing verify_bits! macro in non-failing configuration");

        let x0: u64 = 0;
        verify_bits!(x0, 1);

        let x1: u64 = 0x0FFF_FFFF_FFFF_FFFFu64;
        verify_bits!(x1, 60);

        let x2: u64 = 1u64 << 48;
        verify_bits!(x2, 49);
    }
}
