// ---------------- [ File: bitcoinsecp256k1-fe5x52/src/fe_storage.rs ]
crate::ix!();

pub struct FeStorage {
    n: [u64; 4],
}

impl FeStorage {
    #[inline] pub const fn new(n: [u64; 4]) -> Self {
        Self { n }
    }

    #[inline] pub const fn get_words(&self) -> [u32; 8] {
        [
            (self.n[3] >> 32) as u32, self.n[3] as u32,
            (self.n[2] >> 32) as u32, self.n[2] as u32,
            (self.n[1] >> 32) as u32, self.n[1] as u32,
            (self.n[0] >> 32) as u32, self.n[0] as u32,
        ]
    }
}

macro_rules! fe_storage_const {
    ($d7:expr,
     $d6:expr,
     $d5:expr,
     $d4:expr,
     $d3:expr,
     $d2:expr,
     $d1:expr,
     $d0:expr) => {
        FeStorage::new([
            ($d0 as u64) | (($d1 as u64) << 32),
            ($d2 as u64) | (($d3 as u64) << 32),
            ($d4 as u64) | (($d5 as u64) << 32),
            ($d6 as u64) | (($d7 as u64) << 32),
        ])
    }
}

macro_rules! fe_storage_const_get {
    ($d:expr) => {
        ($d).get_words()[0], ($d).get_words()[1],
        ($d).get_words()[2], ($d).get_words()[3],
        ($d).get_words()[4], ($d).get_words()[5],
        ($d).get_words()[6], ($d).get_words()[7]
    }
}

#[cfg(test)]
mod fe_storage_rs_exhaustive_tests {
    use super::*;

    #[traced_test]
    fn fe_storage_const_and_get_macros_roundtrip_words() {
        tracing::info!("testing fe_storage_const! and fe_storage_const_get! roundtrip behavior");

        let d7: u32 = 0x01234567;
        let d6: u32 = 0x89ABCDEF;
        let d5: u32 = 0x0BADBEEF;
        let d4: u32 = 0xDEADC0DE;
        let d3: u32 = 0xCAFEBABE;
        let d2: u32 = 0xFEEDFACE;
        let d1: u32 = 0x13579BDF;
        let d0: u32 = 0x2468ACE0;

        let s = fe_storage_const!(d7, d6, d5, d4, d3, d2, d1, d0);

        let (g7, g6, g5, g4, g3, g2, g1, g0) = fe_storage_const_get!(s);

        assert_eq!(g7, d7);
        assert_eq!(g6, d6);
        assert_eq!(g5, d5);
        assert_eq!(g4, d4);
        assert_eq!(g3, d3);
        assert_eq!(g2, d2);
        assert_eq!(g1, d1);
        assert_eq!(g0, d0);
    }
}
