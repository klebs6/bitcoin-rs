// ---------------- [ File: bitcoin-sha256/src/compute.rs ]
crate::ix!();

pub trait ComputeSha256 {
    fn sha256(&self) -> u256;
}

/**
  | Single-SHA256 a 32-byte input (represented
  | as uint256).
  |
  */
pub fn sha256uint256(input: &u256) -> u256 {
    input.sha256()
}

impl ComputeSha256 for u256 {
    fn sha256(&self) -> u256 {
        let mut result = u256::zero();
        let mut sha = Sha256::new();
        sha.write(self.as_ref());
        sha.finalize(result.as_mut_slice_exact());
        result
    }
}
