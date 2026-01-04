// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_const.rs ]
crate::ix!();

#[cfg(feature="widemul-int64")]
#[macro_export] macro_rules! scalar_const {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        Scalar {
            d: [
                $d0, 
                $d1, 
                $d2, 
                $d3, 
                $d4, 
                $d5, 
                $d6, 
                $d7
            ]
        }
    }
}

#[cfg(feature="widemul-int128")]
#[macro_export] macro_rules! scalar_const {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        Scalar {
            d: [
                ($d1 as u64) << 32 | $d0, 
                ($d3 as u64) << 32 | $d2, 
                ($d5 as u64) << 32 | $d4, 
                ($d7 as u64) << 32 | $d6
            ]
        }
    }
}

#[cfg(feature="exhaustive-test-order")]
#[macro_export] macro_rules! scalar_const {
    ($d7:ident, 
     $d6:ident, 
     $d5:ident, 
     $d4:ident, 
     $d3:ident, 
     $d2:ident, 
     $d1:ident, 
     $d0:ident) => {
        $d0
    }
}

#[cfg(test)]
mod scalar_const_macro_contracts {
    use super::*;
    use crate::scalar_test_support::*;
    use tracing::{debug, info};

    #[traced_test]
    fn scalar_const_macro_constructs_expected_byte_pattern() {
        info!("validating scalar_const! byte layout is consistent");

        let s: Scalar = scalar_const!(
            0x00000000,
            0x00000001,
            0x00000002,
            0x00000003,
            0x00000004,
            0x00000005,
            0x00000006,
            0x00000007
        );

        let be = scalar_to_be_bytes(&s);
        let expected: [u8; 32] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x03,
            0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x06, 0x00, 0x00, 0x00, 0x07,
        ];

        debug!(?be, ?expected, "constructed/expected");
        assert_eq!(be, expected);
        assert!(scalar_is_normalized_bytes(&be));
    }

}
