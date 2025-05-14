// ---------------- [ File: bitcoin-fuzz/src/fuzz_cuckoocache.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/cuckoocache.cpp]

lazy_static!{
    /*
    FuzzedDataProvider* fuzzed_data_provider_ptr = nullptr;
    */
}

pub struct RandomHasher {

}

impl RandomHasher {
    
    pub fn invoke<uint8_t>(&self, unused: &bool) -> u32 {
    
        todo!();
        /*
            assert(fuzzed_data_provider_ptr != nullptr);
            return fuzzed_data_provider_ptr->ConsumeIntegral<uint32_t>();
        */
    }
}

#[fuzz_test] fn cuckoocache() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        fuzzed_data_provider_ptr = &fuzzed_data_provider;
        CuckooCache::cache<int, RandomHasher> cuckoo_cache{};
        if (fuzzed_data_provider.ConsumeBool()) {
            const size_t megabytes = fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, 16);
            cuckoo_cache.setup_bytes(megabytes << 20);
        } else {
            cuckoo_cache.setup(fuzzed_data_provider.ConsumeIntegralInRange<uint32_t>(0, 4096));
        }
        while (fuzzed_data_provider.ConsumeBool()) {
            if (fuzzed_data_provider.ConsumeBool()) {
                cuckoo_cache.insert(fuzzed_data_provider.ConsumeBool());
            } else {
                cuckoo_cache.contains(fuzzed_data_provider.ConsumeBool(), fuzzed_data_provider.ConsumeBool());
            }
        }
        fuzzed_data_provider_ptr = nullptr;

    */
}
