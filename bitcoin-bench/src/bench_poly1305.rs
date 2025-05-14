// ---------------- [ File: bitcoin-bench/src/bench_poly1305.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/bench/poly1305.cpp]

/**
  | Number of bytes to process per iteration
  |
  */
pub const buffer_size_tiny:  u64 = 64;
pub const buffer_size_small: u64 = 256;
pub const buffer_size_large: u64 = 1024*1024;

pub fn poly1305(
        bench:      &mut Bencher,
        buffersize: usize)  {
    
    todo!();
        /*
            std::vector<unsigned char> tag(POLY1305_TAGLEN, 0);
        std::vector<unsigned char> key(POLY1305_KEYLEN, 0);
        std::vector<unsigned char> in(buffersize, 0);
        bench.batch(in.size()).unit("byte").run([&] {
            poly1305_auth(tag.data(), in.data(), in.size(), key.data());
        });
        */
}

#[bench] fn poly1305_64bytes(b: &mut Bencher)  {
    
    todo!();
        /*
            POLY1305(bench, BUFFER_SIZE_TINY);
        */
}

#[bench] fn poly1305_256bytes(b: &mut Bencher)  {
    
    todo!();
        /*
            POLY1305(bench, BUFFER_SIZE_SMALL);
        */
}

#[bench] fn poly1305_1mb(b: &mut Bencher)  {
    
    todo!();
        /*
            POLY1305(bench, BUFFER_SIZE_LARGE);
        */
}
