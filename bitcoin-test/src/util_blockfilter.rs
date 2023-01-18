crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/util/blockfilter.h]
//-------------------------------------------[.cpp/bitcoin/src/test/util/blockfilter.cpp]

pub fn compute_filter(
        filter_type: BlockFilterType,
        block_index: *const BlockIndex,
        filter:      &mut BlockFilter) -> bool {
    
    todo!();
        /*
            CBlock block;
        if (!ReadBlockFromDisk(block, block_index->GetBlockPos(), Params().GetConsensus())) {
            return false;
        }

        CBlockUndo block_undo;
        if (block_index->nHeight > 0 && !UndoReadFromDisk(block_undo, block_index)) {
            return false;
        }

        filter = BlockFilter(filter_type, block, block_undo);
        return true;
        */
}
