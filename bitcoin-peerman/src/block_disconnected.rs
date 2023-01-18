crate::ix!();

impl BlockDisconnected for PeerManager {

    fn block_disconnected(&mut self, 
        block:  Arc<Block>,
        pindex: Arc<BlockIndex>)  {
        
        //  To avoid relay problems with
        //  transactions that were previously
        //  confirmed, clear our filter of
        //  recently confirmed transactions
        //  whenever there's a reorg.
        //
        //  This means that in a 1-block reorg
        //  (where 1 block is disconnected and
        //  then another block reconnected), our
        //  filter will drop to having only one
        //  block's worth of transactions in it,
        //  but that should be fine, since
        //  presumably the most common case of
        //  relaying a confirmed transaction
        //  should be just after a new block
        //  containing it is found.
        self.recent_confirmed_transactions_mutex
            .get_mut()
            .recent_confirmed_transactions
            .reset();
    }
}
