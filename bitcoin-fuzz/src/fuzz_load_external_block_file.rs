// ---------------- [ File: bitcoin-fuzz/src/fuzz_load_external_block_file.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/load_external_block_file.cpp]

lazy_static!{
    /*
    const TestingSetup* g_setup;
    */
}

pub fn initialize_load_external_block_file()  {
    
    todo!();
        /*
            static const auto testing_setup = MakeNoLogFileContext<const TestingSetup>();
        g_setup = testing_setup.get();
        */
}

#[fuzz_test(initializer = "initialize_load_external_block_file")]
fn load_external_block_file() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        FuzzedFileProvider fuzzed_file_provider = ConsumeFile(fuzzed_data_provider);
        FILE* fuzzed_block_file = fuzzed_file_provider.open();
        if (fuzzed_block_file == nullptr) {
            return;
        }
        FlatFilePos flat_file_pos;
        g_setup->m_node.chainman->ActiveChainstate().LoadExternalBlockFile(fuzzed_block_file, fuzzed_data_provider.ConsumeBool() ? &flat_file_pos : nullptr);

    */
}
