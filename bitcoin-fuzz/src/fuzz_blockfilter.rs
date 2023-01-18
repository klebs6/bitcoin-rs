crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/blockfilter.cpp]

#[fuzz_test] fn blockfilter() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const std::optional<BlockFilter> block_filter = ConsumeDeserializable<BlockFilter>(fuzzed_data_provider);
        if (!block_filter) {
            return;
        }
        {
            (c_void)block_filter->ComputeHeader(ConsumeUInt256(fuzzed_data_provider));
            (c_void)block_filter->GetBlockHash();
            (c_void)block_filter->GetEncodedFilter();
            (c_void)block_filter->GetHash();
        }
        {
            const BlockFilterType block_filter_type = block_filter->GetFilterType();
            (c_void)BlockFilterTypeName(block_filter_type);
        }
        {
            const GCSFilter gcs_filter = block_filter->GetFilter();
            (c_void)gcs_filter.GetN();
            (c_void)gcs_filter.GetParams();
            (c_void)gcs_filter.GetEncoded();
            (c_void)gcs_filter.Match(ConsumeRandomLengthByteVector(fuzzed_data_provider));
            GCSFilter::ElementSet element_set;
            LIMITED_WHILE(fuzzed_data_provider.ConsumeBool(), 30000)
            {
                element_set.insert(ConsumeRandomLengthByteVector(fuzzed_data_provider));
            }
            gcs_filter.MatchAny(element_set);
        }

    */
}
