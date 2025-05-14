// ---------------- [ File: bitcoin-txmempool/src/txid.rs ]
crate::ix!();

/**
  | extracts a transaction hash from
  | 
  | TxMemPoolEntry or CTransactionRef
  |
  */
pub struct MemPoolEntryTxid {

}

pub mod mempool_entry_txid {
    use super::*;
    pub type ResultType = u256;
}

impl MemPoolEntryTxid {
    
    pub fn invoke_tx_mempool_entry(&self, entry: &TxMemPoolEntry) -> mempool_entry_txid::ResultType {
        
        todo!();
        /*
            return entry.GetTx().GetHash();
        */
    }
    
    pub fn invoke_txn(&self, tx: &TransactionRef) -> mempool_entry_txid::ResultType {
        
        todo!();
        /*
            return tx->GetHash();
        */
    }
}

/**
  | extracts a transaction witness-hash
  | from TxMemPoolEntry or TransactionRef
  |
  */
pub struct MemPoolEntryWTxid {

}

pub mod mempool_entry_wtxid {
    use super::*;
    pub type ResultType = u256;
}

impl MemPoolEntryWTxid {
    
    pub fn invoke_tx_mempool_entry(&self, entry: &TxMemPoolEntry) -> mempool_entry_wtxid::ResultType {
        
        todo!();
        /*
            return entry.GetTx().GetWitnessHash();
        */
    }
    
    pub fn invoke_txn(&self, tx: &TransactionRef) -> mempool_entry_wtxid::ResultType {
        
        todo!();
        /*
            return tx->GetWitnessHash();
        */
    }
}
