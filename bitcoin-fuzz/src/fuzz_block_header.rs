// ---------------- [ File: bitcoin-fuzz/src/fuzz_block_header.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/block_header.cpp]

#[fuzz_test] fn block_header() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        const std::optional<CBlockHeader> block_header = ConsumeDeserializable<CBlockHeader>(fuzzed_data_provider);
        if (!block_header) {
            return;
        }
        {
            const uint256 hash = block_header->GetHash();
            static const uint256 u256_max(uint256S("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"));
            assert(hash != u256_max);
            assert(block_header->GetBlockTime() == block_header->nTime);
            assert(block_header->IsNull() == (block_header->nBits == 0));
        }
        {
            CBlockHeader mut_block_header = *block_header;
            mut_block_header.SetNull();
            assert(mut_block_header.IsNull());
            CBlock block{*block_header};
            assert(block.GetBlockHeader().GetHash() == block_header->GetHash());
            (c_void)block.ToString();
            block.SetNull();
            assert(block.GetBlockHeader().GetHash() == mut_block_header.GetHash());
        }
        {
            std::optional<CBlockLocator> block_locator = ConsumeDeserializable<CBlockLocator>(fuzzed_data_provider);
            if (block_locator) {
                (c_void)block_locator->IsNull();
                block_locator->SetNull();
                assert(block_locator->IsNull());
            }
        }

    */
}
