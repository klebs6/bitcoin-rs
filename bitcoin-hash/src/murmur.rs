crate::ix!();

pub fn murmur_hash3(
    n_hash_seed:  u32,
    data_to_hash: &[u8]) -> u32 {
    
    todo!();
        /*
            // The following is MurmurHash3 (x86_32), see https://code.google.com/p/smhasher/source/browse/trunk/MurmurHash3.cpp
        uint32_t h1 = nHashSeed;
        const uint32_t c1 = 0xcc9e2d51;
        const uint32_t c2 = 0x1b873593;

        const int nblocks = vDataToHash.size() / 4;

        //----------
        // body
        const uint8_t* blocks = vDataToHash.data();

        for (int i = 0; i < nblocks; ++i) {
            uint32_t k1 = ReadLE32(blocks + i*4);

            k1 *= c1;
            k1 = ROTL32(k1, 15);
            k1 *= c2;

            h1 ^= k1;
            h1 = ROTL32(h1, 13);
            h1 = h1 * 5 + 0xe6546b64;
        }

        //----------
        // tail
        const uint8_t* tail = vDataToHash.data() + nblocks * 4;

        uint32_t k1 = 0;

        switch (vDataToHash.size() & 3) {
            case 3:
                k1 ^= tail[2] << 16;
                [[fallthrough]];
            case 2:
                k1 ^= tail[1] << 8;
                [[fallthrough]];
            case 1:
                k1 ^= tail[0];
                k1 *= c1;
                k1 = ROTL32(k1, 15);
                k1 *= c2;
                h1 ^= k1;
        }

        //----------
        // finalization
        h1 ^= vDataToHash.size();
        h1 ^= h1 >> 16;
        h1 *= 0x85ebca6b;
        h1 ^= h1 >> 13;
        h1 *= 0xc2b2ae35;
        h1 ^= h1 >> 16;

        return h1;
        */
}
