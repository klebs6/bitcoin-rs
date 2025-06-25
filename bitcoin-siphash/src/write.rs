crate::ix!();

impl SipHasher {
    
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
}
