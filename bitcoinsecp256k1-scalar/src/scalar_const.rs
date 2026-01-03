// ---------------- [ File: bitcoinsecp256k1-scalar/src/scalar_const.rs ]
crate::ix!();

#[cfg(WIDEMUL_INT64)]
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

#[cfg(WIDEMUL_INT128)]
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

#[cfg(EXHAUSTIVE_TEST_ORDER)]
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


