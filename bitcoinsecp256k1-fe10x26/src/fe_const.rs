// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_const.rs ]
crate::ix!();

/**
  | Unpacks a constant into a overlapping
  | multi-limbed FE element.
  |
  */
#[macro_export] macro_rules! fe_const_inner {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        [
            ($d0) & 0x3FFFFFF, 
            (($d0 as u32) >> 26) | ((($d1 as u32) & 0xFFFFF) << 6), 
            (($d1 as u32) >> 20) | ((($d2 as u32) & 0x3FFF) << 12), 
            (($d2 as u32) >> 14) | ((($d3 as u32) & 0xFF) << 18), 
            (($d3 as u32) >>  8) | ((($d4 as u32) & 0x3) << 24), 
            (($d4 as u32) >> 2) & 0x3FFFFFF, 
            (($d4 as u32) >> 28) | ((($d5 as u32) & 0x3FFFFF) << 4), 
            (($d5 as u32) >> 22) | ((($d6 as u32) & 0xFFFF) << 10), 
            (($d6 as u32) >> 16) | ((($d7 as u32) & 0x3FF) << 16), 
            (($d7 as u32) >> 10) 
        ]
    }
}

#[cfg(feature="secp256k1-verify")]
#[macro_export] macro_rules! fe_const {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        Fe10x26 {
            n: fe_const_inner!{
                $d7, 
                $d6, 
                $d5, 
                $d4, 
                $d3, 
                $d2, 
                $d1, 
                $d0
            }, 
            magnitude:  1, 
            normalized: 1
        }
    }
}

#[cfg(not(feature="secp256k1-verify"))]
#[macro_export] macro_rules! fe_const {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        Fe10x26 {
            n: fe_const_inner!{
                $d7, 
                $d6, 
                $d5, 
                $d4, 
                $d3, 
                $d2, 
                $d1, 
                $d0
            }
        }
    }
}

#[cfg(test)]
mod fe_const_macro_contract_suite {
    use super::*;
    use crate::fe10x26_test_support::*;

    #[traced_test]
    fn fe_const_encodes_zero_correctly() {
        info!("fe_const! should encode 0");
        let mut z = fe_const!(0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        let out = fe_to_be_bytes_normalized(&mut z);
        debug!(?out, "zero bytes");
        assert_eq!(out, BYTES_ZERO);
    }

    #[traced_test]
    fn fe_const_encodes_one_correctly() {
        info!("fe_const! should encode 1");
        let mut o = fe_const!(0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32);
        let out = fe_to_be_bytes_normalized(&mut o);
        debug!(?out, "one bytes");
        assert_eq!(out, BYTES_ONE);
    }

    #[traced_test]
    fn fe_const_encodes_p_minus_one_correctly() {
        info!("fe_const! should encode p-1");
        let mut pm1 = fe_const!(
            0xFFFFFFFFu32,
            0xFFFFFFFFu32,
            0xFFFFFFFFu32,
            0xFFFFFFFFu32,
            0xFFFFFFFFu32,
            0xFFFFFFFFu32,
            0xFFFFFFFEu32,
            0xFFFFFC2Eu32
        );
        let out = fe_to_be_bytes_normalized(&mut pm1);
        debug!(?out, "p-1 bytes");
        assert_eq!(out, FIELD_PRIME_MINUS_ONE_BYTES_BE);
    }

    #[traced_test]
    fn fe_const_sets_verify_metadata_when_enabled() {
        info!("under secp256k1-verify, fe_const! should set magnitude=1 normalized=1");
        let a = fe_const!(0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 2u32);

        #[cfg(feature = "secp256k1-verify")]
        {
            assert_eq!(a.magnitude, 1);
            assert_eq!(a.normalized, 1);
        }
    }
}
