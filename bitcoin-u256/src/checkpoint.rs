// ---------------- [ File: bitcoin-u256/src/checkpoint.rs ]
crate::ix!();

/// We'll define the map:
pub type MapCheckpoints = HashMap<i32, u256>;

/// Data holding checkpoints keyed by block height.
#[derive(Default)]
pub struct CheckpointData {
    /// map_checkpoints: (height -> block hash in `u256`)
    pub(crate) map_checkpoints: MapCheckpoints,
}

impl CheckpointData {
    /// Return the highest (final) checkpoint height. 
    /// In C++: `mapCheckpoints.rbegin()->first`
    pub fn get_height(&self) -> i32 {
        // If empty => return 0 (or some sentinel).
        if self.map_checkpoints.is_empty() {
            0
        } else {
            // Return the max key
            *self.map_checkpoints.keys().max().unwrap()
        }
    }
}

#[cfg(test)]
mod checkpoint_tests {
    use super::*;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[test]
    fn test_checkpointdata_get_height() {
        let mut cd = CheckpointData::default();
        // empty => returns 0
        assert_eq!(cd.get_height(), 0, "empty => height=0");

        // insert a few random heights
        cd.map_checkpoints.insert(100, u256::default());
        cd.map_checkpoints.insert(200, u256::default());
        cd.map_checkpoints.insert(150, u256::default());

        // => highest is 200
        assert_eq!(cd.get_height(), 200, "highest=200");
        // add an even bigger
        cd.map_checkpoints.insert(99999, u256::default());
        assert_eq!(cd.get_height(), 99999);
    }

    #[test]
    fn test_arith_to_uint256() {
        // We'll manually set limbs in ArithU256 => call arith_to_uint256 => check the raw bytes in LE
        let mut a = ArithU256::default();
        // Suppose we do limb_count=8. We'll set each limb = i
        for i in 0..8 {
            a.base.set_limb(i, i as u32);
        }
        let out = arith_to_uint256(&a);
        let out_bytes = out.as_slice();

        // Then out_bytes[0..4] => limb0 in little-end => 0 => all zero
        // out_bytes[4..8] => limb1 => 1 => 0x01 00 00 00 
        // etc
        for i in 0..8 {
            let expected = i as u32;
            let le = &out_bytes[(i * 4)..(i * 4 + 4)];
            let val = u32::from_le_bytes(le.try_into().unwrap());
            assert_eq!(val, expected, "mismatch at limb {}", i);
        }
    }

    #[test]
    fn test_uint_to_arith256() {
        // We'll prepare a u256 with each 4-byte chunk = i in LE, then parse into ArithU256 => check limbs
        let mut x = u256::default();
        for i in 0..8 {
            let le = (i as u32).to_le_bytes();
            x.as_slice_mut()[(i * 4)..(i * 4 + 4)].copy_from_slice(&le);
        }

        let got = uint_to_arith256(&x);

        // each limb i => i
        for i in 0..8 {
            let limb = got.base.get_limb(i);
            assert_eq!(limb, i as u32, "limb[{}] mismatch", i);
        }
    }

    #[test]
    fn test_arith_uint256_roundtrip() {
        // We'll do random limbs in ArithU256 => arith_to_uint256 => uint_to_arith256 => compare
        let mut rng = 0xDEAD_BEEF_9999_0000u64;

        for _ in 0..10 {
            let mut a = ArithU256::default();
            // fill random limbs
            for i in 0..8 {
                rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
                a.base.set_limb(i, rng as u32);
            }

            let converted = arith_to_uint256(&a);
            let round = uint_to_arith256(&converted);
            assert_eq!(a, round, "round trip mismatch with random limbs");
        }
    }
}
