// ---------------- [ File: bitcoin-fuzz/src/fuzz_validation_load_mempool.rs ]
crate::ix!();

lazy_static!{
    /*
    const TestingSetup* g_setup;
    */
}

pub fn initialize_validation_load_mempool()  {
    
    todo!();
        /*
            static const auto testing_setup = MakeNoLogFileContext<const TestingSetup>();
        g_setup = testing_setup.get();
        */
}

#[fuzz_test(initializer = "initialize_validation_load_mempool")]
fn validation_load_mempool() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        SetMockTime(ConsumeTime(fuzzed_data_provider));
        FuzzedFileProvider fuzzed_file_provider = ConsumeFile(fuzzed_data_provider);

        CTxMemPool pool{};
        auto fuzzed_fopen = [&](const fs::path&, const char*) {
            return fuzzed_file_provider.open();
        };
        (c_void)LoadMempool(pool, g_setup->m_node.chainman->ActiveChainstate(), fuzzed_fopen);
        (c_void)DumpMempool(pool, fuzzed_fopen, true);

    */
}
