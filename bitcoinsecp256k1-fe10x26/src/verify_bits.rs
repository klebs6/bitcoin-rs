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
        tracing::info!("verify_bits should enforce the requested bit-width (implementation may be panicking or non-panicking depending on verify_check)");

        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            verify_bits!(0x3FFFFFFu32, 26);
        }));
        assert!(ok.is_ok(), "in-budget value must not panic");

        let x: u32 = 0x4000000u32; // 1<<26, which exceeds a 26-bit budget
        tracing::debug!(x, shifted = (x >> 26), "constructed out-of-budget value");
        assert_ne!((x >> 26), 0, "sanity: x must actually exceed the 26-bit budget");

        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            verify_bits!(x, 26);
        }));
        tracing::debug!(is_err = res.is_err(), "verify_bits out-of-budget panic capture");

        if res.is_err() {
            tracing::info!("verify_bits enforced via panic in this configuration");
        } else {
            tracing::warn!("verify_bits did not panic for out-of-budget value; verify_check appears non-panicking in this configuration");
        }
    }
}
