// ---------------- [ File: bitcoin-indexed-chain/src/fill_block.rs ]
crate::ix!();

pub struct UniqueLock<T> { p: std::marker::PhantomData<T> }
pub type ReentrantMutexPlaceholder = i32;//TODO: find a suitable replacement

pub fn fill_block(
    index:  *const BlockIndex,
    block:  &FoundBlock,
    lock:   &mut UniqueLock<ReentrantMutexPlaceholder>,
    active: &Chain) -> bool {

    todo!();
        /*
            if (!index) return false;
        if (block.m_hash) *block.m_hash = index->GetBlockHash();
        if (block.m_height) *block.m_height = index->nHeight;
        if (block.m_time) *block.m_time = index->GetBlockTime();
        if (block.m_max_time) *block.m_max_time = index->GetBlockTimeMax();
        if (block.m_mtp_time) *block.m_mtp_time = index->GetMedianTimePast();
        if (block.m_in_active_chain) *block.m_in_active_chain = active[index->nHeight] == index;
        if (block.m_next_block) FillBlock(active[index->nHeight] == index ? active[index->nHeight + 1] : nullptr, *block.m_next_block, lock, active);
        if (block.m_data) {
            REVERSE_LOCK(lock);
            if (!ReadBlockFromDisk(*block.m_data, index, Params().GetConsensus())) block.m_data->SetNull();
        }
        block.found = true;
        return true;
        */
}
