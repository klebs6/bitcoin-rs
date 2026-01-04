// ---------------- [ File: bitcoinsecp256k1-fe10x26/src/fe_storage.rs ]
crate::ix!();

pub struct FeStorage {
    pub n: [u32; 8],
}

#[macro_export] macro_rules! fe_storage_const {
    ($d7:expr,
     $d6:expr,
     $d5:expr,
     $d4:expr,
     $d3:expr,
     $d2:expr,
     $d1:expr,
     $d0:expr) => {
        FeStorage { n: [($d0) as u32, ($d1) as u32, ($d2) as u32, ($d3) as u32, ($d4) as u32, ($d5) as u32, ($d6) as u32, ($d7) as u32] }
    };
}

#[macro_export] macro_rules! fe_storage_const_get {
    ($d:expr) => {
        (($d).n[7], ($d).n[6], ($d).n[5], ($d).n[4], ($d).n[3], ($d).n[2], ($d).n[1], ($d).n[0])
    };
}

#[cfg(test)]
mod fe_storage_macro_contract_suite {
    use super::*;
    use tracing::{debug, info};

    #[traced_test]
    fn fe_storage_const_orders_words_as_expected() {
        info!("fe_storage_const!(d7..d0) stores as [d0..d7]");
        let s = fe_storage_const!(
            0x07060504u32,
            0x06050403u32,
            0x05040302u32,
            0x04030201u32,
            0x03020100u32,
            0x221100FFu32,
            0xAABBCCDDu32,
            0x11223344u32
        );

        debug!(?s.n, "storage words");
        assert_eq!(s.n[0], 0x11223344u32);
        assert_eq!(s.n[1], 0xAABBCCDDu32);
        assert_eq!(s.n[2], 0x221100FFu32);
        assert_eq!(s.n[3], 0x03020100u32);
        assert_eq!(s.n[4], 0x04030201u32);
        assert_eq!(s.n[5], 0x05040302u32);
        assert_eq!(s.n[6], 0x06050403u32);
        assert_eq!(s.n[7], 0x07060504u32);
    }

    #[traced_test]
    fn fe_storage_const_get_returns_words_in_big_endian_tuple_order() {
        info!("fe_storage_const_get!(s) returns (d7..d0)");
        let s = fe_storage_const!(
            0xDEADBEEFu32,
            0xCAFEBABEu32,
            0xFEEDFACEu32,
            0x01234567u32,
            0x89ABCDEFu32,
            0x0BADF00Du32,
            0x13579BDFu32,
            0x2468ACE0u32
        );

        let (d7, d6, d5, d4, d3, d2, d1, d0) = fe_storage_const_get!(s);
        assert_eq!(d7, 0xDEADBEEFu32);
        assert_eq!(d6, 0xCAFEBABEu32);
        assert_eq!(d5, 0xFEEDFACEu32);
        assert_eq!(d4, 0x01234567u32);
        assert_eq!(d3, 0x89ABCDEFu32);
        assert_eq!(d2, 0x0BADF00Du32);
        assert_eq!(d1, 0x13579BDFu32);
        assert_eq!(d0, 0x2468ACE0u32);
    }
}
