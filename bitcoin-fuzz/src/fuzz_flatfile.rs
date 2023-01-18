crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/flatfile.cpp]

#[fuzz_test] fn flatfile() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        std::optional<FlatFilePos> flat_file_pos = ConsumeDeserializable<FlatFilePos>(fuzzed_data_provider);
        if (!flat_file_pos) {
            return;
        }
        std::optional<FlatFilePos> another_flat_file_pos = ConsumeDeserializable<FlatFilePos>(fuzzed_data_provider);
        if (another_flat_file_pos) {
            assert((*flat_file_pos == *another_flat_file_pos) != (*flat_file_pos != *another_flat_file_pos));
        }
        (c_void)flat_file_pos->ToString();
        flat_file_pos->SetNull();
        assert(flat_file_pos->IsNull());

    */
}

