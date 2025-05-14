// ---------------- [ File: bitcoin-bench/src/bench_hashpadding.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/hashpadding.cpp]

#[bench] fn pre_padded(b: &mut Bencher)  {
    
    todo!();
        /*
            CSHA256 hasher;

        // Setup the salted hasher
        uint256 nonce = GetRandHash();
        hasher.Write(nonce.begin(), 32);
        hasher.Write(nonce.begin(), 32);
        uint256 data = GetRandHash();
        bench.run([&] {
            unsigned char out[32];
            CSHA256 h = hasher;
            h.Write(data.begin(), 32);
            h.Finalize(out);
        });
        */
}

#[bench] fn regular_padded(b: &mut Bencher)  {
    
    todo!();
        /*
            CSHA256 hasher;

        // Setup the salted hasher
        uint256 nonce = GetRandHash();
        uint256 data = GetRandHash();
        bench.run([&] {
            unsigned char out[32];
            CSHA256 h = hasher;
            h.Write(nonce.begin(), 32);
            h.Write(data.begin(), 32);
            h.Finalize(out);
        });
        */
}
