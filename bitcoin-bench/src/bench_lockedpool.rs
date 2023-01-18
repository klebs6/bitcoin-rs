crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/lockedpool.cpp]

pub const ASIZE: usize = 2048;
pub const MSIZE: usize = 2048;

#[bench] fn bench_locked_pool(b: &mut Bencher)  {
    
    todo!();
        /*
            c_void *synth_base = reinterpret_cast<c_void*>(0x08000000);
        const size_t synth_size = 1024*1024;
        Arena b(synth_base, synth_size, 16);

        std::vector<c_void*> addr;
        for (int x=0; x<ASIZE; ++x)
            addr.push_back(nullptr);
        uint32_t s = 0x12345678;
        bench.run([&] {
            int idx = s & (addr.size() - 1);
            if (s & 0x80000000) {
                b.free(addr[idx]);
                addr[idx] = nullptr;
            } else if (!addr[idx]) {
                addr[idx] = b.alloc((s >> 16) & (MSIZE - 1));
            }
            bool lsb = s & 1;
            s >>= 1;
            if (lsb)
                s ^= 0xf00f00f0; // LFSR period 0xf7ffffe0
        });
        for (c_void *ptr: addr)
            b.free(ptr);
        addr.clear();
        */
}
