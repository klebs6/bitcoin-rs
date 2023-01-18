crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/signet.cpp]

pub fn initialize_signet()  {
    
    todo!();
        /*
            static const auto testing_setup = MakeNoLogFileContext<>(CBaseChainParams::SIGNET);
        */
}

#[fuzz_test(initializer = "initialize_signet")]
fn signet() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider{buffer.data(), buffer.size()};
        const std::optional<CBlock> block = ConsumeDeserializable<CBlock>(fuzzed_data_provider);
        if (!block) {
            return;
        }
        (c_void)CheckSignetBlockSolution(*block, Params().GetConsensus());
        (c_void)SignetTxs::Create(*block, ConsumeScript(fuzzed_data_provider));

    */
}
