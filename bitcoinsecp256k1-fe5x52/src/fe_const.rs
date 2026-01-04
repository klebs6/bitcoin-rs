// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_const.rs ]
crate::ix!();

/**
  | Unpacks a constant into a overlapping
  | multi-limbed FE element.
  |
  */
macro_rules! fe_const_inner {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        [ 
            ($d0) | ((($d1 as u64) & 0xFFFFF) << 32), 
            (($d1 as u64) >> 20) | (($d2 as u64) << 12) | ((($d3 as u64) & 0xFF) << 44), 
            (($d3 as u64) >> 8)  | ((($d4 as u64) & 0xFFFFFFF) << 24), 
            (($d4 as u64) >> 28) | (($d5 as u64) << 4) | ((($d6 as u64) & 0xFFFF) << 36), 
            (($d6 as u64) >> 16) | (($d7 as u64) << 16) 
        ]
    }
}

#[cfg(feature="secp256k1-verify")]
macro_rules! fe_const {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        Fe5x52 {
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
macro_rules! fe_const {
    ($d7:expr, 
     $d6:expr, 
     $d5:expr, 
     $d4:expr, 
     $d3:expr, 
     $d2:expr, 
     $d1:expr, 
     $d0:expr) => {
        Fe5x52 {
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
mod fe_const_rs_exhaustive_tests {
    use super::*;

    const SAMPLE_B32: [u8; 32] = [
        0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0xCD, 0xEF,
        0xFE, 0xDC, 0xBA, 0x98, 0x76, 0x54, 0x32, 0x10,
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0x01,
    ];

    const FIELD_P_MINUS_1_B32: [u8; 32] = [
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
        0xFF, 0xFF, 0xFF, 0xFE, 0xFF, 0xFF, 0xFC, 0x2E,
    ];

    fn words_from_be32(bytes: &[u8; 32]) -> [u32; 8] {
        [
            u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
            u32::from_be_bytes([bytes[4], bytes[5], bytes[6], bytes[7]]),
            u32::from_be_bytes([bytes[8], bytes[9], bytes[10], bytes[11]]),
            u32::from_be_bytes([bytes[12], bytes[13], bytes[14], bytes[15]]),
            u32::from_be_bytes([bytes[16], bytes[17], bytes[18], bytes[19]]),
            u32::from_be_bytes([bytes[20], bytes[21], bytes[22], bytes[23]]),
            u32::from_be_bytes([bytes[24], bytes[25], bytes[26], bytes[27]]),
            u32::from_be_bytes([bytes[28], bytes[29], bytes[30], bytes[31]]),
        ]
    }

    unsafe fn fe_get_b32_normalized(a: &Fe5x52) -> [u8; 32] {
        let mut tmp = *a;
        crate::fe_normalize(&mut tmp as *mut Fe5x52);
        let mut out = [0u8; 32];
        crate::fe_get_b32(out.as_mut_ptr(), &tmp as *const Fe5x52);
        out
    }

    #[traced_test]
    fn fe_const_macro_roundtrips_to_expected_big_endian_bytes_for_samples() {
        tracing::info!("testing fe_const!/fe_const_inner packing via fe_get_b32");

        unsafe {
            let w = words_from_be32(&SAMPLE_B32);
            let a = fe_const!(w[0], w[1], w[2], w[3], w[4], w[5], w[6], w[7]);
            let got = fe_get_b32_normalized(&a);
            assert_eq!(got, SAMPLE_B32);

            let w2 = words_from_be32(&FIELD_P_MINUS_1_B32);
            let b = fe_const!(w2[0], w2[1], w2[2], w2[3], w2[4], w2[5], w2[6], w2[7]);
            let got2 = fe_get_b32_normalized(&b);
            assert_eq!(got2, FIELD_P_MINUS_1_B32);
        }
    }
}

