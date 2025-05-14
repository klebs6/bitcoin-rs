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
    
    /**
      | Hash a 64-bit integer worth of data
      | 
      | It is treated as if this was the little-endian
      | interpretation of 8 bytes.
      | 
      | This function can only be used when a
      | multiple of 8 bytes have been written
      | so far.
      |
      */
    pub fn write_u64(&mut self, data: u64) -> &mut SipHasher {
        
        todo!();
        /*
            uint64_t v0 = v[0], v1 = v[1], v2 = v[2], v3 = v[3];

        assert(count % 8 == 0);

        v3 ^= data;
        SIPROUND;
        SIPROUND;
        v0 ^= data;

        v[0] = v0;
        v[1] = v1;
        v[2] = v2;
        v[3] = v3;

        count += 8;
        return *this;
        */
    }
    
    /**
      | Hash arbitrary bytes.
      |
      */
    pub fn write(&mut self, 
        data: *const u8,
        size: usize) -> &mut SipHasher {
        
        todo!();
        /*
            uint64_t v0 = v[0], v1 = v[1], v2 = v[2], v3 = v[3];
        uint64_t t = tmp;
        uint8_t c = count;

        while (size--) {
            t |= ((uint64_t)(*(data++))) << (8 * (c % 8));
            c++;
            if ((c & 7) == 0) {
                v3 ^= t;
                SIPROUND;
                SIPROUND;
                v0 ^= t;
                t = 0;
            }
        }

        v[0] = v0;
        v[1] = v1;
        v[2] = v2;
        v[3] = v3;
        count = c;
        tmp = t;

        return *this;
        */
    }
    
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

//-------------------------------------------[.cpp/bitcoin/src/crypto/siphash.cpp]

macro_rules! rotl {
    ($x:ident, $b:ident) => {
        /*
                (uint64_t)(((x) << (b)) | ((x) >> (64 - (b))))
        */
    }
}

macro_rules! sipround {
    () => {
        /*
                do { 
            v0 += v1; v1 = ROTL(v1, 13); v1 ^= v0; 
            v0 = ROTL(v0, 32); 
            v2 += v3; v3 = ROTL(v3, 16); v3 ^= v2; 
            v0 += v3; v3 = ROTL(v3, 21); v3 ^= v0; 
            v2 += v1; v1 = ROTL(v1, 17); v1 ^= v2; 
            v2 = ROTL(v2, 32); 
        } while (0)
        */
    }
}

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

pub fn sip_hash_uint_256extra(
        k0:    u64,
        k1:    u64,
        val:   &u256,
        extra: u32) -> u64 {
    
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
        d = (((uint64_t)36) << 56) | extra;
        v3 ^= d;
        SIPROUND;
        SIPROUND;
        v0 ^= d;
        v2 ^= 0xFF;
        SIPROUND;
        SIPROUND;
        SIPROUND;
        SIPROUND;
        return v0 ^ v1 ^ v2 ^ v3;
        */
}
