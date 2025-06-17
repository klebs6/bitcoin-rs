// ---------------- [ File: bitcoin-bigint/src/from_limbs.rs ]
crate::ix!();

pub fn from_limbs_64(limbs: &[u32]) -> BaseUInt64 {
    let mut x = BaseUInt64::default();
    if !limbs.is_empty() {
        x.pn[0] = limbs[0];
    }
    if limbs.len() > 1 {
        x.pn[1] = limbs[1];
    }
    x
}

pub fn to_limbs_64(x: &BaseUInt64) -> Vec<u32> {
    vec![x.pn[0], x.pn[1]]
}

pub fn from_limbs_256(limbs: &[u32]) -> BaseUInt256 {
    let mut x = BaseUInt256::default();
    for (i, &val) in limbs.iter().enumerate().take(8) {
        x.pn[i] = val;
    }
    x
}

pub fn to_limbs_256(x: &BaseUInt256) -> Vec<u32> {
    x.pn.to_vec()
}
