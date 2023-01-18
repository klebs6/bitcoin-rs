/*!
  | A non-cryptographic RNG used only for
  | test infrastructure.
  |
  */

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/testrand.h]
//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/testrand_impl.h]

lazy_static!{
    /*
    static rfc6979_hmac_sha256 test_rng;
    static uint32_t test_rng_precomputed[8];
    static int test_rng_precomputed_used = 8;
    static uint64_t test_rng_integer;
    static int test_rng_integer_bits_left = 0;
    */
}

/**
  | Seed the pseudorandom number generator
  | for testing.
  |
  */
#[inline] pub fn testrand_seed(seed16: *const u8)  {
    
    todo!();
        /*
            rfc6979_hmac_sha256_initialize(&test_rng, seed16, 16);
        */
}

/**
  | Generate a pseudorandom number in the
  | range [0..2**32-1].
  |
  */
#[inline] pub fn testrand32() -> u32 {
    
    todo!();
        /*
            if (test_rng_precomputed_used == 8) {
            rfc6979_hmac_sha256_generate(&test_rng, (unsigned char*)(&test_rng_precomputed[0]), sizeof(test_rng_precomputed));
            test_rng_precomputed_used = 0;
        }
        return test_rng_precomputed[test_rng_precomputed_used++];
        */
}

/**
  | Generate a pseudorandom number in the
  | range [0..2**bits-1]. Bits must be
  | 1 or more.
  |
  */
pub fn testrand_bits(bits: i32) -> u32 {
    
    todo!();
        /*
            uint32_t ret;
        if (test_rng_integer_bits_left < bits) {
            test_rng_integer |= (((uint64_t)testrand32()) << test_rng_integer_bits_left);
            test_rng_integer_bits_left += 32;
        }
        ret = test_rng_integer;
        test_rng_integer >>= bits;
        test_rng_integer_bits_left -= bits;
        ret &= ((~((uint32_t)0)) >> (32 - bits));
        return ret;
        */
}

/**
  | Generate a pseudorandom number in the
  | range [0..range-1].
  |
  */
pub fn testrand_int(range: u32) -> u32 {
    
    todo!();
        /*
            /* We want a uniform integer between 0 and range-1, inclusive.
         * B is the smallest number such that range <= 2**B.
         * two mechanisms implemented here:
         * - generate B bits numbers until one below range is found, and return it
         * - find the largest multiple M of range that is <= 2**(B+A), generate B+A
         *   bits numbers until one below M is found, and return it modulo range
         * The second mechanism consumes A more bits of entropy in every iteration,
         * but may need fewer iterations due to M being closer to 2**(B+A) then
         * range is to 2**B. The array below (indexed by B) contains a 0 when the
         * first mechanism is to be used, and the number A otherwise.
         */
        static const int addbits[] = {0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 2, 1, 0};
        uint32_t trange, mult;
        int bits = 0;
        if (range <= 1) {
            return 0;
        }
        trange = range - 1;
        while (trange > 0) {
            trange >>= 1;
            bits++;
        }
        if (addbits[bits]) {
            bits = bits + addbits[bits];
            mult = ((~((uint32_t)0)) >> (32 - bits)) / range;
            trange = range * mult;
        } else {
            trange = range;
            mult = 1;
        }
        while(1) {
            uint32_t x = testrand_bits(bits);
            if (x < trange) {
                return (mult == 1) ? x : (x % range);
            }
        }
        */
}

/**
  | Generate a pseudorandom 32-byte array.
  |
  */
pub fn testrand256(b32: *mut u8)  {
    
    todo!();
        /*
            rfc6979_hmac_sha256_generate(&test_rng, b32, 32);
        */
}

/**
  | Generate pseudorandom bytes with long
  | sequences of zero and one bits.
  |
  */
pub fn testrand_bytes_test(
        bytes: *mut u8,
        len:   usize)  {
    
    todo!();
        /*
            size_t bits = 0;
        memset(bytes, 0, len);
        while (bits < len * 8) {
            int now;
            uint32_t val;
            now = 1 + (testrand_bits(6) * testrand_bits(5) + 16) / 31;
            val = testrand_bits(1);
            while (now > 0 && bits < len * 8) {
                bytes[bits / 8] |= val << (bits % 8);
                now--;
                bits++;
            }
        }
        */
}

/**
  | Generate a pseudorandom 32-byte array
  | with long sequences of zero and one bits.
  |
  */
pub fn testrand256_test(b32: *mut u8)  {
    
    todo!();
        /*
            testrand_bytes_test(b32, 32);
        */
}

/**
  | Flip a single random bit in a byte array
  |
  */
pub fn testrand_flip(
        b:   *mut u8,
        len: usize)  {
    
    todo!();
        /*
            b[testrand_int(len)] ^= (1 << testrand_int(8));
        */
}

/**
  | Initialize the test RNG using (hex encoded)
  | array up to 16 bytes, or randomly if hexseed
  | is NULL.
  |
  */
pub fn testrand_init(hexseed: *const u8)  {
    
    todo!();
        /*
            unsigned char seed16[16] = {0};
        if (hexseed && strlen(hexseed) != 0) {
            int pos = 0;
            while (pos < 16 && hexseed[0] != 0 && hexseed[1] != 0) {
                unsigned short sh;
                if ((sscanf(hexseed, "%2hx", &sh)) == 1) {
                    seed16[pos] = sh;
                } else {
                    break;
                }
                hexseed += 2;
                pos++;
            }
        } else {
            FILE *frand = fopen("/dev/urandom", "rb");
            if ((frand == NULL) || fread(&seed16, 1, sizeof(seed16), frand) != sizeof(seed16)) {
                uint64_t t = time(NULL) * (uint64_t)1337;
                fprintf(stderr, "WARNING: could not read 16 bytes from /dev/urandom; falling back to insecure PRNG\n");
                seed16[0] ^= t;
                seed16[1] ^= t >> 8;
                seed16[2] ^= t >> 16;
                seed16[3] ^= t >> 24;
                seed16[4] ^= t >> 32;
                seed16[5] ^= t >> 40;
                seed16[6] ^= t >> 48;
                seed16[7] ^= t >> 56;
            }
            if (frand) {
                fclose(frand);
            }
        }

        printf("random seed = %02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x\n", seed16[0], seed16[1], seed16[2], seed16[3], seed16[4], seed16[5], seed16[6], seed16[7], seed16[8], seed16[9], seed16[10], seed16[11], seed16[12], seed16[13], seed16[14], seed16[15]);
        testrand_seed(seed16);
        */
}

/**
  | Print final test information.
  |
  */
pub fn testrand_finish()  {
    
    todo!();
        /*
            unsigned char run32[32];
        testrand256(run32);
        printf("random run = %02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x\n", run32[0], run32[1], run32[2], run32[3], run32[4], run32[5], run32[6], run32[7], run32[8], run32[9], run32[10], run32[11], run32[12], run32[13], run32[14], run32[15]);
        */
}
