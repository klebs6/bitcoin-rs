// ---------------- [ File: bitcoin-siphash/src/siphash.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/siphash.h]

/**
  | SipHash-2-4
  |
  */
pub struct SipHasher {
    v:     [u64; 4],
    tmp:   u64,

    /**
      | Only the low 8 bits of the input size matter.
      |
      */
    count: u8,
}

impl SipHasher {

    /**
      | Construct a SipHash calculator initialized
      | with 128-bit key (k0, k1)
      |
      */
    pub fn new(k0: u64, k1: u64) -> Self {
    
        todo!();
        /*

        v[0] = 0x736f6d6570736575ULL ^ k0;
        v[1] = 0x646f72616e646f6dULL ^ k1;
        v[2] = 0x6c7967656e657261ULL ^ k0;
        v[3] = 0x7465646279746573ULL ^ k1;
        count = 0;
        tmp = 0;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/crypto/siphash.cpp]
