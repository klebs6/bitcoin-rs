crate::ix!();

#[test] fn arena_tests() {
    todo!();
    /*
    
        // Fake memory base address for testing
        // without actually using memory.
        c_void *synth_base = reinterpret_cast<c_void*>(0x08000000);
        const size_t synth_size = 1024*1024;
        Arena b(synth_base, synth_size, 16);
        c_void *chunk = b.alloc(1000);
    #ifdef ARENA_DEBUG
        b.walk();
    #endif
        BOOST_CHECK(chunk != nullptr);
        BOOST_CHECK(b.stats().used == 1008); // Aligned to 16
        BOOST_CHECK(b.stats().total == synth_size); // Nothing has disappeared?
        b.free(chunk);
    #ifdef ARENA_DEBUG
        b.walk();
    #endif
        BOOST_CHECK(b.stats().used == 0);
        BOOST_CHECK(b.stats().free == synth_size);
        try { // Test exception on double-free
            b.free(chunk);
            BOOST_CHECK(0);
        } catch(std::runtime_error &)
        {
        }

        c_void *a0 = b.alloc(128);
        c_void *a1 = b.alloc(256);
        c_void *a2 = b.alloc(512);
        BOOST_CHECK(b.stats().used == 896);
        BOOST_CHECK(b.stats().total == synth_size);
    #ifdef ARENA_DEBUG
        b.walk();
    #endif
        b.free(a0);
    #ifdef ARENA_DEBUG
        b.walk();
    #endif
        BOOST_CHECK(b.stats().used == 768);
        b.free(a1);
        BOOST_CHECK(b.stats().used == 512);
        c_void *a3 = b.alloc(128);
    #ifdef ARENA_DEBUG
        b.walk();
    #endif
        BOOST_CHECK(b.stats().used == 640);
        b.free(a2);
        BOOST_CHECK(b.stats().used == 128);
        b.free(a3);
        BOOST_CHECK(b.stats().used == 0);
        BOOST_CHECK_EQUAL(b.stats().chunks_used, 0U);
        BOOST_CHECK(b.stats().total == synth_size);
        BOOST_CHECK(b.stats().free == synth_size);
        BOOST_CHECK_EQUAL(b.stats().chunks_free, 1U);

        std::vector<c_void*> addr;
        BOOST_CHECK(b.alloc(0) == nullptr); // allocating 0 always returns nullptr
    #ifdef ARENA_DEBUG
        b.walk();
    #endif
        // Sweeping allocate all memory
        for (int x=0; x<1024; ++x)
            addr.push_back(b.alloc(1024));
        BOOST_CHECK(b.stats().free == 0);
        BOOST_CHECK(b.alloc(1024) == nullptr); // memory is full, this must return nullptr
        BOOST_CHECK(b.alloc(0) == nullptr);
        for (int x=0; x<1024; ++x)
            b.free(addr[x]);
        addr.clear();
        BOOST_CHECK(b.stats().total == synth_size);
        BOOST_CHECK(b.stats().free == synth_size);

        // Now in the other direction...
        for (int x=0; x<1024; ++x)
            addr.push_back(b.alloc(1024));
        for (int x=0; x<1024; ++x)
            b.free(addr[1023-x]);
        addr.clear();

        // Now allocate in smaller unequal chunks, then deallocate haphazardly
        // Not all the chunks will succeed allocating, but freeing nullptr is
        // allowed so that is no problem.
        for (int x=0; x<2048; ++x)
            addr.push_back(b.alloc(x+1));
        for (int x=0; x<2048; ++x)
            b.free(addr[((x*23)%2048)^242]);
        addr.clear();

        // Go entirely wild: free and alloc interleaved,
        // generate targets and sizes using pseudo-randomness.
        for (int x=0; x<2048; ++x)
            addr.push_back(nullptr);
        uint32_t s = 0x12345678;
        for (int x=0; x<5000; ++x) {
            int idx = s & (addr.size()-1);
            if (s & 0x80000000) {
                b.free(addr[idx]);
                addr[idx] = nullptr;
            } else if(!addr[idx]) {
                addr[idx] = b.alloc((s >> 16) & 2047);
            }
            bool lsb = s & 1;
            s >>= 1;
            if (lsb)
                s ^= 0xf00f00f0; // LFSR period 0xf7ffffe0
        }
        for (c_void *ptr: addr)
            b.free(ptr);
        addr.clear();

        BOOST_CHECK(b.stats().total == synth_size);
        BOOST_CHECK(b.stats().free == synth_size);

    */
}

