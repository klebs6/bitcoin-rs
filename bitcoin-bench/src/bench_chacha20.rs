// ---------------- [ File: bitcoin-bench/src/bench_chacha20.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/chacha20.cpp]

/**
  | Number of bytes to process per iteration
  |
  */
pub const buffer_size_tiny:  u64 = 64;
pub const buffer_size_small: u64 = 256;
pub const buffer_size_large: u64 = 1024*1024;

pub fn chacha20(
        bench:      &mut Bencher,
        buffersize: usize)  {
    
    todo!();
        /*
        std::vector<uint8_t> key(32,0);
        ChaCha20 ctx(key.data(), key.size());
        ctx.SetIV(0);
        ctx.Seek(0);
        std::vector<uint8_t> in(buffersize,0);
        std::vector<uint8_t> out(buffersize,0);
        bench.batch(in.size()).unit("byte").run([&] {
            ctx.Crypt(in.data(), out.data(), in.size());
        });
        */
}

#[bench] fn chacha20_64bytes(b: &mut Bencher)  {
    
    todo!();
        /*
            CHACHA20(bench, BUFFER_SIZE_TINY);
        */
}

#[bench] fn chacha20_256bytes(b: &mut Bencher)  {
    
    todo!();
        /*
            CHACHA20(bench, BUFFER_SIZE_SMALL);
        */
}

#[bench] fn chacha20_1mb(b: &mut Bencher)  {
    
    todo!();
        /*
            CHACHA20(bench, BUFFER_SIZE_LARGE);
        */
}
