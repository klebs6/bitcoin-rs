crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/policy_estimator_io.cpp]

pub fn initialize_policy_estimator_io()  {
    
    todo!();
        /*
            static const auto testing_setup = MakeNoLogFileContext<>();
        */
}

#[fuzz_test(initializer = "initialize_policy_estimator_io")]
fn policy_estimator_io() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        FuzzedAutoFileProvider fuzzed_auto_file_provider = ConsumeAutoFile(fuzzed_data_provider);
        CAutoFile fuzzed_auto_file = fuzzed_auto_file_provider.open();
        // Re-using block_policy_estimator across runs to avoid costly creation of CBlockPolicyEstimator object.
        static CBlockPolicyEstimator block_policy_estimator;
        if (block_policy_estimator.Read(fuzzed_auto_file)) {
            block_policy_estimator.Write(fuzzed_auto_file);
        }

    */
}
