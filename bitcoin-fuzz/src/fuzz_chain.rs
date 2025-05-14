// ---------------- [ File: bitcoin-fuzz/src/fuzz_chain.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/chain.cpp]

#[fuzz_test] fn chain() {
    todo!();
    /*
    
        FuzzedDataProvider fuzzed_data_provider(buffer.data(), buffer.size());
        std::optional<CDiskBlockIndex> disk_block_index = ConsumeDeserializable<CDiskBlockIndex>(fuzzed_data_provider);
        if (!disk_block_index) {
            return;
        }

        const uint256 zero{};
        disk_block_index->phashBlock = &zero;
        (c_void)disk_block_index->GetBlockHash();
        (c_void)disk_block_index->GetBlockPos();
        (c_void)disk_block_index->GetBlockTime();
        (c_void)disk_block_index->GetBlockTimeMax();
        (c_void)disk_block_index->GetMedianTimePast();
        (c_void)disk_block_index->GetUndoPos();
        (c_void)disk_block_index->HaveTxsDownloaded();
        (c_void)disk_block_index->IsValid();
        (c_void)disk_block_index->ToString();

        const CBlockHeader block_header = disk_block_index->GetBlockHeader();
        (c_void)CDiskBlockIndex{*disk_block_index};
        (c_void)disk_block_index->BuildSkip();

        while (fuzzed_data_provider.ConsumeBool()) {
            const BlockStatus block_status = fuzzed_data_provider.PickValueInArray({
                BlockStatus::BLOCK_VALID_UNKNOWN,
                BlockStatus::BLOCK_VALID_RESERVED,
                BlockStatus::BLOCK_VALID_TREE,
                BlockStatus::BLOCK_VALID_TRANSACTIONS,
                BlockStatus::BLOCK_VALID_CHAIN,
                BlockStatus::BLOCK_VALID_SCRIPTS,
                BlockStatus::BLOCK_VALID_MASK,
                BlockStatus::BLOCK_HAVE_DATA,
                BlockStatus::BLOCK_HAVE_UNDO,
                BlockStatus::BLOCK_HAVE_MASK,
                BlockStatus::BLOCK_FAILED_VALID,
                BlockStatus::BLOCK_FAILED_CHILD,
                BlockStatus::BLOCK_FAILED_MASK,
                BlockStatus::BLOCK_OPT_WITNESS,
            });
            if (block_status & ~BLOCK_VALID_MASK) {
                continue;
            }
            (c_void)disk_block_index->RaiseValidity(block_status);
        }

        CBlockIndex block_index{block_header};
        block_index.phashBlock = &zero;
        (c_void)block_index.GetBlockHash();
        (c_void)block_index.ToString();

    */
}
