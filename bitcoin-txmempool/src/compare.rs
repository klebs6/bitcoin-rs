// ---------------- [ File: bitcoin-txmempool/src/compare.rs ]
crate::ix!();

pub struct CompareInvMempoolOrder {
    mp:          Amo<TxMemPool>,
    wtxid_relay: bool,
}

impl CompareInvMempoolOrder {

    pub fn new(
        mempool:   Amo<TxMemPool>,
        use_wtxid: bool) -> Self {
    
        todo!();
        /*


            mp = _mempool;
            m_wtxid_relay = use_wtxid;
        */
    }
}

impl Comparator<u256> for CompareInvMempoolOrder {

    fn compare(&self, a: &u256, b: &u256) -> Ordering {

        todo!();
        /*
            /* As std::make_heap produces a max-heap, we want the entries with the
             * fewest ancestors/highest fee to sort later. */
            return mp->CompareDepthAndScore(*b, *a, m_wtxid_relay);
        */
    }
}
