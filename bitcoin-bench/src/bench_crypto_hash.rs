crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/crypto_hash.cpp]

/**
  | Number of bytes to hash per iteration
  |
  */
pub const buffer_size: u64 = 1000*1000;

#[bench] fn ripemd160(b: &mut Bencher)  {
    
    todo!();
        /*
        uint8_t hash[CRIPEMD160::OUTPUT_SIZE];
        std::vector<uint8_t> in(BUFFER_SIZE,0);
        bench.batch(in.size()).unit("byte").run([&] {
            CRIPEMD160().Write(in.data(), in.size()).Finalize(hash);
        });
        */
}

#[bench] fn sha1(b: &mut Bencher)  {
    
    todo!();
        /*
        uint8_t hash[CSHA1::OUTPUT_SIZE];
        std::vector<uint8_t> in(BUFFER_SIZE,0);
        bench.batch(in.size()).unit("byte").run([&] {
            CSHA1().Write(in.data(), in.size()).Finalize(hash);
        });
        */
}

#[bench] fn sha256(b: &mut Bencher)  {
    
    todo!();
        /*
        uint8_t hash[CSHA256::OUTPUT_SIZE];
        std::vector<uint8_t> in(BUFFER_SIZE,0);
        bench.batch(in.size()).unit("byte").run([&] {
            CSHA256().Write(in.data(), in.size()).Finalize(hash);
        });
        */
}

#[bench] fn sha3_256_1m(b: &mut Bencher)  {
    
    todo!();
        /*
        uint8_t hash[SHA3_256::OUTPUT_SIZE];
        std::vector<uint8_t> in(BUFFER_SIZE,0);
        bench.batch(in.size()).unit("byte").run([&] {
            SHA3_256().Write(in).Finalize(hash);
        });
        */
}

#[bench] fn sha256_32b(b: &mut Bencher)  {
    
    todo!();
        /*
        std::vector<uint8_t> in(32,0);
        bench.batch(in.size()).unit("byte").run([&] {
            CSHA256()
                .Write(in.data(), in.size())
                .Finalize(in.data());
        });
        */
}

#[bench] fn sha256d64_1024(b: &mut Bencher)  {
    
    todo!();
        /*
        std::vector<uint8_t> in(64 * 1024, 0);
        bench.batch(in.size()).unit("byte").run([&] {
            SHA256D64(in.data(), in.data(), 1024);
        });
        */
}

#[bench] fn sha512(b: &mut Bencher)  {
    
    todo!();
        /*
        uint8_t hash[CSHA512::OUTPUT_SIZE];
        std::vector<uint8_t> in(BUFFER_SIZE,0);
        bench.batch(in.size()).unit("byte").run([&] {
            CSHA512().Write(in.data(), in.size()).Finalize(hash);
        });
        */
}

#[bench] fn sip_hash_32b(b: &mut Bencher)  {
    
    todo!();
        /*
        uint256 x;
        uint64_t k1 = 0;
        bench.run([&] {
            *((uint64_t*)x.begin()) = SipHashUint256(0, ++k1, x);
        });
        */
}

#[bench] fn fast_random_32bit(b: &mut Bencher)  {
    
    todo!();
        /*
        FastRandomContext rng(true);
        bench.run([&] {
            rng.rand32();
        });
        */
}

#[bench] fn fast_random_1bit(b: &mut Bencher)  {
    
    todo!();
        /*
        FastRandomContext rng(true);
        bench.run([&] {
            rng.randbool();
        });
        */
}

#[bench] fn mu_hash(b: &mut Bencher)  {
    
    todo!();
        /*
        MuHash3072 acc;
        unsigned char key[32] = {0};
        uint32_t i = 0;
        bench.run([&] {
            key[0] = ++i & 0xFF;
            acc *= MuHash3072(key);
        });
        */
}

#[bench] fn mu_hash_mul(b: &mut Bencher)  {
    
    todo!();
        /*
        MuHash3072 acc;
        FastRandomContext rng(true);
        MuHash3072 muhash{rng.randbytes(32)};

        bench.run([&] {
            acc *= muhash;
        });
        */
}

#[bench] fn mu_hash_div(b: &mut Bencher)  {
    
    todo!();
        /*
        MuHash3072 acc;
        FastRandomContext rng(true);
        MuHash3072 muhash{rng.randbytes(32)};

        bench.run([&] {
            acc /= muhash;
        });
        */
}

#[bench] fn mu_hash_precompute(b: &mut Bencher)  {
    
    todo!();
        /*
        MuHash3072 acc;
        FastRandomContext rng(true);
        std::vector<unsigned char> key{rng.randbytes(32)};

        bench.run([&] {
            MuHash3072{key};
        });
        */
}
