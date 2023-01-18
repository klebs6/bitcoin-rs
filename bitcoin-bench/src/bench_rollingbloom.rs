crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/bench/rollingbloom.cpp]

#[bench] fn rolling_bloom(b: &mut Bencher)  {
    
    todo!();
        /*
            CRollingBloomFilter filter(120000, 0.000001);
        std::vector<unsigned char> data(32);
        uint32_t count = 0;
        bench.run([&] {
            count++;
            data[0] = count & 0xFF;
            data[1] = (count >> 8) & 0xFF;
            data[2] = (count >> 16) & 0xFF;
            data[3] = (count >> 24) & 0xFF;
            filter.insert(data);

            data[0] = (count >> 24) & 0xFF;
            data[1] = (count >> 16) & 0xFF;
            data[2] = (count >> 8) & 0xFF;
            data[3] = count & 0xFF;
            filter.contains(data);
        });
        */
}

#[bench] fn rolling_bloom_reset(b: &mut Bencher)  {
    
    todo!();
        /*
            CRollingBloomFilter filter(120000, 0.000001);
        bench.run([&] {
            filter.reset();
        });
        */
}
