crate::ix!();

#[derive(Default)]
pub struct BlockTransactions {

    /**
      | A BlockTransactions message
      |
      */
    pub blockhash: u256,

    pub txn:       Vec<TransactionRef>,

}

lazy_static!{
    /*
    SERIALIZE_METHODS(BlockTransactions, obj)
        {
            READWRITE(obj.blockhash, Using<VectorFormatter<TransactionCompression>>(obj.txn));
        }
    */
}

impl BlockTransactions {

    pub fn new(req: &BlockTransactionsRequest) -> Self {
    
        todo!();
        /*
        : blockhash(req.blockhash),
        : txn(req.indexes.size()),

        
        */
    }
}
