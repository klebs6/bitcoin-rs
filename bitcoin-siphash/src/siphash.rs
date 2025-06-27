// ---------------- [ File: bitcoin-siphash/src/siphash.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/siphash.h]

/// SipHash-2-4
#[derive(Debug,Clone,CopyGetters,MutGetters,Getters)]
pub struct BitcoinSipHasher {

    #[getset(get = "pub(crate)", get_mut = "pub(crate)")]
    v:     [u64; 4],

    #[getset(get_copy = "pub(crate)", get_mut = "pub(crate)")]
    tmp:   u64,

    /// Only the low 8 bits of the input size matter.
    #[getset(get_copy = "pub(crate)", get_mut = "pub(crate)")]
    count: u8,
}

impl BitcoinSipHasher {

    /// Initialise a SipHash‑2‑4 state with the 128‑bit key `(k0,k1)`.
    #[inline]
    pub fn new(k0: u64, k1: u64) -> Self {
        debug!("BitcoinSipHasher::new(k0={:016x}, k1={:016x})", k0, k1);
        Self {
            v: [
                0x736f6d6570736575u64 ^ k0,
                0x646f72616e646f6du64 ^ k1,
                0x6c7967656e657261u64 ^ k0,
                0x7465646279746573u64 ^ k1,
            ],
            tmp:   0,
            count: 0,
        }
    }
}
