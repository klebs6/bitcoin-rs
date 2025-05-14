// ---------------- [ File: bitcoin-bench/src/bench_chacha_poly_aead.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/chacha_poly_aead.cpp]

/**
  | Number of bytes to process per iteration
  |
  */
pub const BUFFER_SIZE_TINY:  u64 = 64;
pub const BUFFER_SIZE_SMALL: u64 = 256;
pub const BUFFER_SIZE_LARGE: u64 = 1024 * 1024;

pub const K1: [u8; 32] = [0; 32];
pub const K2: [u8; 32] = [0; 32];

lazy_static!{
    /*
    static ChaCha20Poly1305AEAD aead(k1, 32, k2, 32);
    */
}

pub fn chacha20_poly1305_aead(
        bench:              &mut Bencher,
        buffersize:         usize,
        include_decryption: bool)  {
    
    todo!();
        /*
        std::vector<unsigned char> in(buffersize + CHACHA20_POLY1305_AEAD_AAD_LEN + POLY1305_TAGLEN, 0);
        std::vector<unsigned char> out(buffersize + CHACHA20_POLY1305_AEAD_AAD_LEN + POLY1305_TAGLEN, 0);
        uint64_t seqnr_payload = 0;
        uint64_t seqnr_aad = 0;
        int aad_pos = 0;
        uint32_t len = 0;
        bench.batch(buffersize).unit("byte").run([&] {
            // encrypt or decrypt the buffer with a static key
            const bool crypt_ok_1 = aead.Crypt(seqnr_payload, seqnr_aad, aad_pos, out.data(), out.size(), in.data(), buffersize, true);
            assert(crypt_ok_1);

            if (include_decryption) {
                // if we decrypt, include the GetLength
                const bool get_length_ok = aead.GetLength(&len, seqnr_aad, aad_pos, in.data());
                assert(get_length_ok);
                const bool crypt_ok_2 = aead.Crypt(seqnr_payload, seqnr_aad, aad_pos, out.data(), out.size(), in.data(), buffersize, true);
                assert(crypt_ok_2);
            }

            // increase main sequence number
            seqnr_payload++;
            // increase aad position (position in AAD keystream)
            aad_pos += CHACHA20_POLY1305_AEAD_AAD_LEN;
            if (aad_pos + CHACHA20_POLY1305_AEAD_AAD_LEN > CHACHA20_ROUND_OUTPUT) {
                aad_pos = 0;
                seqnr_aad++;
            }
            if (seqnr_payload + 1 == std::numeric_limits<uint64_t>::max()) {
                // reuse of nonce+key is okay while benchmarking.
                seqnr_payload = 0;
                seqnr_aad = 0;
                aad_pos = 0;
            }
        });
        */
}

#[bench] fn chacha20_poly1305_aead_64bytes_only_encrypt(b: &mut Bencher)  {
    
    todo!();
        /*
            CHACHA20_POLY1305_AEAD(bench, BUFFER_SIZE_TINY, false);
        */
}

#[bench] fn chacha20_poly1305_aead_256bytes_only_encrypt(b: &mut Bencher)  {
    
    todo!();
        /*
            CHACHA20_POLY1305_AEAD(bench, BUFFER_SIZE_SMALL, false);
        */
}

#[bench] fn chacha20_poly1305_aead_1mb_only_encrypt(b: &mut Bencher)  {
    
    todo!();
        /*
            CHACHA20_POLY1305_AEAD(bench, BUFFER_SIZE_LARGE, false);
        */
}

#[bench] fn chacha20_poly1305_aead_64bytes_encrypt_decrypt(b: &mut Bencher)  {
    
    todo!();
        /*
            CHACHA20_POLY1305_AEAD(bench, BUFFER_SIZE_TINY, true);
        */
}

#[bench] fn chacha20_poly1305_aead_256bytes_encrypt_decrypt(b: &mut Bencher)  {
    
    todo!();
        /*
            CHACHA20_POLY1305_AEAD(bench, BUFFER_SIZE_SMALL, true);
        */
}

#[bench] fn chacha20_poly1305_aead_1mb_encrypt_decrypt(b: &mut Bencher)  {
    
    todo!();
        /*
            CHACHA20_POLY1305_AEAD(bench, BUFFER_SIZE_LARGE, true);
        */
}

/* - * Add Hash() (dbl-sha256) bench for comparison  - */

pub fn hash(
    bench:      &mut Bencher,
    buffersize: usize)  {

    todo!();
        /*
        uint8_t hash[CHash256::OUTPUT_SIZE];
        std::vector<uint8_t> in(buffersize,0);
        bench.batch(in.size()).unit("byte").run([&] {
            CHash256().Write(in).Finalize(hash);
        });
        */
}

#[bench] fn hash_64bytes(b: &mut Bencher)  {
    
    todo!();
        /*
            HASH(bench, BUFFER_SIZE_TINY);
        */
}

#[bench] fn hash_256bytes(b: &mut Bencher)  {
    
    todo!();
        /*
            HASH(bench, BUFFER_SIZE_SMALL);
        */
}

#[bench] fn hash_1mb(b: &mut Bencher)  {
    
    todo!();
        /*
            HASH(bench, BUFFER_SIZE_LARGE);
        */
}
