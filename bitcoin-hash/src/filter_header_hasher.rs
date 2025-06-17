// ---------------- [ File: bitcoin-hash/src/filter_header_hasher.rs ]
crate::ix!();

pub struct FilterHeaderHasher { }

impl FilterHeaderHasher {
    #[instrument(level = "debug", skip(self, hash))]
    pub fn invoke(&self, hash: &u256) -> usize {
        read_le64(hash.as_ref()) as usize
    }
}

/// Little‑endian helper – identical to Bitcoin Core’s `ReadLE64`.
#[inline]
pub fn read_le64(bytes: &[u8]) -> u64 {
    let mut tmp = [0u8; 8];
    tmp.copy_from_slice(&bytes[..8]);
    u64::from_le_bytes(tmp)
}
