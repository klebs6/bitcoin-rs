crate::ix!();

/**
  | Optimized SipHash-2-4 implementation
  | for uint256.
  | 
  | It is identical to:
  |   SipHasher(k0, k1)
  |     .Write(val.GetUint64(0))
  |     .Write(val.GetUint64(1))
  |     .Write(val.GetUint64(2))
  |     .Write(val.GetUint64(3))
  |     .Finalize()
  |
  */
pub fn sip_hash_uint256(
        k0:  u64,
        k1:  u64,
        val: &u256) -> u64 {
    
    todo!();
        /*
            /* Specialized implementation for efficiency */
        uint64_t d = val.GetUint64(0);

        uint64_t v0 = 0x736f6d6570736575ULL ^ k0;
        uint64_t v1 = 0x646f72616e646f6dULL ^ k1;
        uint64_t v2 = 0x6c7967656e657261ULL ^ k0;
        uint64_t v3 = 0x7465646279746573ULL ^ k1 ^ d;

        SIPROUND;
        SIPROUND;
        v0 ^= d;
        d = val.GetUint64(1);
        v3 ^= d;
        SIPROUND;
        SIPROUND;
        v0 ^= d;
        d = val.GetUint64(2);
        v3 ^= d;
        SIPROUND;
        SIPROUND;
        v0 ^= d;
        d = val.GetUint64(3);
        v3 ^= d;
        SIPROUND;
        SIPROUND;
        v0 ^= d;
        v3 ^= ((uint64_t)4) << 59;
        SIPROUND;
        SIPROUND;
        v0 ^= ((uint64_t)4) << 59;
        v2 ^= 0xFF;
        SIPROUND;
        SIPROUND;
        SIPROUND;
        SIPROUND;
        return v0 ^ v1 ^ v2 ^ v3;
        */
}
