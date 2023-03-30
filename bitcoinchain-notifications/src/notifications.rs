crate::ix!();

/**
  | Chain notifications.
  |
  */
pub trait ChainNotifications:
    TransactionAddedToMempool
    + TransactionRemovedFromMempool
    + BlockConnected
    + BlockDisconnected
    + UpdatedBlockTip
    + ChainStateFlushed
{ }

pub trait TransactionAddedToMempool {

    /**
      | Notifies listeners of a transaction
      | having been added to mempool.
      | 
      | Called on a background thread.
      |
      */
    fn transaction_added_to_mempool(&mut self, 
        tx:               &TransactionRef,
        mempool_sequence: u64) {}
}

pub trait TransactionRemovedFromMempool {

    /**
      | Notifies listeners of a transaction
      | leaving mempool.
      | 
      | This notification fires for transactions
      | that are removed from the mempool for
      | the following reasons:
      | 
      | - EXPIRY (expired from mempool after
      |   -mempoolexpiry hours)
      | 
      | - SIZELIMIT (removed in size limiting
      |   if the mempool exceeds -maxmempool
      |   megabytes)
      | 
      | - REORG (removed during a reorg)
      | 
      | - CONFLICT (removed because it conflicts
      |   with in-block transaction)
      | 
      | - REPLACED (removed due to RBF replacement)
      | 
      | This does not fire for transactions
      | that are removed from the mempool because
      | they have been included in a block. Any
      | client that is interested in transactions
      | removed from the mempool for inclusion
      | in a block can learn about those transactions
      | from the BlockConnected notification.
      | 
      | Transactions that are removed from
      | the mempool because they conflict with
      | a transaction in the new block will have
      | 
      | TransactionRemovedFromMempool events
      | fired *before* the BlockConnected
      | event is fired. If multiple blocks are
      | connected in one step, then the ordering
      | could be:
      | 
      | - TransactionRemovedFromMempool(tx1
      |   from block A)
      | 
      | - TransactionRemovedFromMempool(tx2
      |   from block A)
      | 
      | - TransactionRemovedFromMempool(tx1
      |   from block B)
      | 
      | - TransactionRemovedFromMempool(tx2
      |   from block B)
      | 
      | - BlockConnected(A)
      | 
      | - BlockConnected(B)
      | 
      | Called on a background thread.
      |
      */
    fn transaction_removed_from_mempool(&mut self, 
        tx:               &TransactionRef,
        reason:           MemPoolRemovalReason,
        mempool_sequence: u64) {}
}

