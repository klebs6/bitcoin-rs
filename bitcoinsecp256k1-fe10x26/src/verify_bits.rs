// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/verify_bits.rs ]
crate::ix!();

#[cfg(not(feature="secp256k1-use-external-asm"))]
#[cfg(feature="secp256k1-verify")]
#[macro_export] macro_rules! verify_bits {
    ($x:expr, $n:expr) => {
        verify_check!((($x) >> ($n)) == 0);
    };
}

#[cfg(not(feature="secp256k1-use-external-asm"))]
#[cfg(not(feature="secp256k1-verify"))]
#[macro_export] macro_rules! verify_bits {
    ($x:expr, $n:expr) => { };
}

#[cfg(all(test, feature = "secp256k1-verify", not(feature = "secp256k1-use-external-asm")))]
mod verify_bits_macro_contract_suite {
    use super::*;
    use tracing::{debug, info};

    #[traced_test]
    fn verify_bits_allows_values_within_bit_budget() {
        info!("verify_bits should accept values whose high bits above n are zero");
        verify_bits!(0u32, 0);
        verify_bits!(0x3FFFFFFu32, 26);
        verify_bits!(0x03FFFFFu32, 22);
    }

    #[traced_test]
    fn verify_bits_rejects_values_exceeding_bit_budget() {
        info!("verify_bits should trip verify_check when bits exceed the requested width");
        let res = std::panic::catch_unwind(|| {
            verify_bits!(0x4000000u32, 26);
        });

        debug!(is_err = res.is_err(), "panic capture");
        assert!(res.is_err());
    }
}
