// ---------------- [ File: bitcoin-peerman/src/send_block_transactions.rs ]
crate::ix!();

pub trait SendBlockTransactions {

    fn send_block_transactions(self: Arc<Self>, 
        pfrom: &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        block: &Block,
        req:   &BlockTransactionsRequest);
}

impl SendBlockTransactions for PeerManager {

    fn send_block_transactions(
        self:  Arc<Self>, 
        pfrom: &mut AmoWriteGuard<Box<dyn NodeInterface>>,
        block: &Block,
        req:   &BlockTransactionsRequest)  {
        
        let mut resp: BlockTransactions = BlockTransactions::new(req);

        for i in 0..req.indexes.len() {

            if req.indexes[i] as usize >= block.vtx.len() {
                self.misbehaving(pfrom.get_id(), 100, "getblocktxn with out-of-bounds tx indices");
                return;
            }

            let idx: usize = req.indexes[i].try_into().unwrap();

            resp.txn[i] = block.vtx[idx].clone(); //TODO: do we want to remove it?
        }

        let mut guard = CS_MAIN.lock();

        let msg_maker: NetMsgMaker = NetMsgMaker::new(pfrom.get_common_version());

        let n_send_flags: i32 = match create_state(pfrom.get_id()).get().wants_cmpct_witness.load(atomic::Ordering::Relaxed) {
            true   => 0,
            false  => SERIALIZE_TRANSACTION_NO_WITNESS
        };

        self.connman.get_mut().push_message(
            &mut *pfrom, 
            msg_maker.make_with_flags(
                n_send_flags, 
                NetMsgType::BLOCKTXN, 
                &[
                    &resp
                ]
            )
        );
    }
}
