// ---------------- [ File: bitcoin-siphash/src/finalize.rs ]
crate::ix!();

impl SipHasher {
    
    /**
      | Compute the 64-bit SipHash-2-4 of the
      | data written so far. The object remains
      | untouched.
      |
      */
    pub fn finalize(&self) -> u64 {
        
        todo!();
        /*
            uint64_t v0 = v[0], v1 = v[1], v2 = v[2], v3 = v[3];

        uint64_t t = tmp | (((uint64_t)count) << 56);

        v3 ^= t;
        SIPROUND;
        SIPROUND;
        v0 ^= t;
        v2 ^= 0xFF;
        SIPROUND;
        SIPROUND;
        SIPROUND;
        SIPROUND;
        return v0 ^ v1 ^ v2 ^ v3;
        */
    }
}
