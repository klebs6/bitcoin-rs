// ---------------- [ File: bitcoin-fuzz/src/fuzz_protocol.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/protocol.cpp]

#[fuzz_test] fn protocol() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const std::optional<CInv> inv = ConsumeDeserializable<CInv>(fuzzed_data_provider);
        if (!inv) {
            return;
        }
        try {
            (c_void)inv->GetCommand();
        } catch (const std::out_of_range&) {
        }
        (c_void)inv->ToString();
        const std::optional<CInv> another_inv = ConsumeDeserializable<CInv>(fuzzed_data_provider);
        if (!another_inv) {
            return;
        }
        (c_void)(*inv < *another_inv);

    */
}
