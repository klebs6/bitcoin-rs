// ---------------- [ File: bitcoin-txmempoolentry/src/entry.rs ]
crate::ix!();

pub struct LockPoints {

    /**
      | Will be set to the blockchain height and
      | median time past values that would be
      | necessary to satisfy all relative locktime
      | constraints (BIP68) of this tx given our
      | view of block chain history
      */
    height:          i32, // default = { 0 }

    time:            i64, // default = { 0 }

    /**
      | As long as the current chain descends from
      | the highest height block containing one of
      | the inputs used in the calculation, then
      | the cached values are still valid even
      | after a reorg.
      */
    max_input_block: *mut BlockIndex, // default = { nullptr }
}

/**
  | \class TxMemPoolEntry
  | 
  | TxMemPoolEntry stores data about
  | the corresponding transaction, as
  | well as data about all in-mempool transactions
  | that depend on the transaction ("descendant"
  | transactions).
  | 
  | When a new entry is added to the mempool,
  | we update the descendant state (nCountWithDescendants,
  | nSizeWithDescendants, and nModFeesWithDescendants)
  | for all ancestors of the newly added
  | transaction.
  |
  */
pub struct TxMemPoolEntry {

    tx:              TransactionRef,
    parents:         RefCell<TxMemPoolEntryParents>,
    children:        RefCell<TxMemPoolEntryChildren>,

    /**
      | Cached to avoid expensive parent-transaction
      | lookups
      |
      */
    n_fee:           Amount,

    /**
      | ... and avoid recomputing tx weight
      | (also used for GetTxSize())
      |
      */
    n_tx_weight:     usize,

    /**
      | ... and total memory usage
      |
      */
    n_usage_size:    usize,

    /**
      | Local time when entering the mempool
      |
      */
    n_time:          OffsetDateTime,

    /**
      | Chain height when entering the mempool
      |
      */
    entry_height:    u32,

    /**
      | keep track of transactions that spend
      | a coinbase
      |
      */
    spends_coinbase: bool,

    /**
      | Total sigop cost
      |
      */
    sig_op_cost:     i64,

    /**
      | Used for determining the priority of
      | the transaction for mining in a block
      |
      */
    fee_delta:       i64, // default = { 0 }

    /**
      | Track the height and time at which tx
      | was final
      |
      */
    lock_points:     LockPoints,

    /*
      | Information about descendants of this
      | transaction that are in the mempool; if we
      | remove this transaction we must remove all
      | of these descendants as well.
      */

    /**
      | number of descendant transactions
      |
      */
    n_count_with_descendants:    u64, // default = { 1 }

    /**
      | ... and size
      |
      */
    n_size_with_descendants:     u64,

    /**
      | ... and total fees (all including us)
      |
      */
    n_mod_fees_with_descendants: Amount,

    /**
      | Analogous statistics for ancestor
      | transactions
      |
      */
    n_count_with_ancestors:       u64, // default = { 1 }
    n_size_with_ancestors:        u64,
    n_mod_fees_with_ancestors:    Amount,
    n_sig_op_cost_with_ancestors: i64,

    /**
      | Index in mempool's vTxHashes
      |
      */
    tx_hashes_idx: RefCell<usize>,

    /**
      | epoch when last touched, useful for
      | graph algorithms
      |
      */
    epoch_marker:  RefCell<EpochMarker>,
}

pub type TxMemPoolEntryRef = Amo<TxMemPoolEntry>;

/**
  | two aliases, should the types ever diverge
  |
  */
pub type TxMemPoolEntryParents  = HashSet<TxMemPoolEntryRef,CompareIteratorByHash>;
pub type TxMemPoolEntryChildren = HashSet<TxMemPoolEntryRef,CompareIteratorByHash>;

impl TxMemPoolEntry {
    
    pub fn get_tx(&self) -> TransactionRef {
        self.tx.clone()
    }
    
    pub fn get_fee(&self) -> &Amount {
        &self.n_fee
    }
    
    pub fn get_tx_weight(&self) -> usize {
        self.n_tx_weight
    }
    
    pub fn get_time(&self) -> OffsetDateTime {
        self.n_time
    }

    pub fn get_height(&self) -> u32 {
        self.entry_height
    }
    
    pub fn get_sig_op_cost(&self) -> i64 {
        self.sig_op_cost
    }
    
    pub fn get_modified_fee(&self) -> i64 {
        self.n_fee + self.fee_delta
    }
    
    pub fn dynamic_memory_usage(&self) -> usize {
        self.n_usage_size
    }
    
    pub fn get_lock_points(&self) -> &LockPoints {
        &self.lock_points
    }

    pub fn get_count_with_descendants(&self) -> u64 {
        self.n_count_with_descendants
    }
    
    pub fn get_size_with_descendants(&self) -> u64 {
        self.n_size_with_descendants
    }
    
    pub fn get_mod_fees_with_descendants(&self) -> Amount {
        self.n_mod_fees_with_descendants
    }
    
    pub fn get_spends_coinbase(&self) -> bool {
        self.spends_coinbase
    }
    
    pub fn get_count_with_ancestors(&self) -> u64 {
        self.n_count_with_ancestors
    }
    
    pub fn get_size_with_ancestors(&self) -> u64 {
        self.n_size_with_ancestors
    }
    
    pub fn get_mod_fees_with_ancestors(&self) -> Amount {
        self.n_mod_fees_with_ancestors
    }
    
    pub fn get_sig_op_cost_with_ancestors(&self) -> i64 {
        self.n_sig_op_cost_with_ancestors
    }
    
    pub fn get_mem_pool_parents_const(&self) -> &TxMemPoolEntryParents {
        todo!();
        /*
        self.parents
        */
    }
    
    pub fn get_mem_pool_children_const(&self) -> &TxMemPoolEntryChildren {
        todo!();
        /*
        self.children
        */
    }
    
    pub fn get_mem_pool_parents(&self) -> &mut TxMemPoolEntryParents {
        todo!();
        /*
        self.parents
        */
    }
    
    pub fn get_mem_pool_children(&self) -> &mut TxMemPoolEntryChildren {
        todo!();
        /*
        self.children
        */
    }
    
    /**
      | Adjusts the descendant state.
      |
      */
    pub fn update_descendant_state(&mut self, 
        modify_size:  i64,
        modify_fee:   Amount,
        modify_count: i64)  {
        
        todo!();
        /*
            nSizeWithDescendants += modifySize;
        assert(int64_t(nSizeWithDescendants) > 0);
        nModFeesWithDescendants += modifyFee;
        nCountWithDescendants += modifyCount;
        assert(int64_t(nCountWithDescendants) > 0);
        */
    }
    
    /**
      | Adjusts the ancestor state
      |
      */
    pub fn update_ancestor_state(&mut self, 
        modify_size:    i64,
        modify_fee:     Amount,
        modify_count:   i64,
        modify_sig_ops: i64)  {
        
        todo!();
        /*
            nSizeWithAncestors += modifySize;
        assert(int64_t(nSizeWithAncestors) > 0);
        nModFeesWithAncestors += modifyFee;
        nCountWithAncestors += modifyCount;
        assert(int64_t(nCountWithAncestors) > 0);
        nSigOpCostWithAncestors += modifySigOps;
        assert(int(nSigOpCostWithAncestors) >= 0);
        */
    }
    
    pub fn new(
        tx:              &TransactionRef,
        fee:             Amount,
        time:            i64,
        entry_height:    u32,
        spends_coinbase: bool,
        sigops_cost:     i64,
        lp:              LockPoints) -> Self {
    
        todo!();
        /*


            : tx{tx},
          nFee{fee},
          nTxWeight(GetTransactionWeight(*tx)),
          nUsageSize{RecursiveDynamicUsage(tx)},
          nTime{time},
          entryHeight{entry_height},
          spendsCoinbase{spends_coinbase},
          sigOpCost{sigops_cost},
          lockPoints{lp},
          nSizeWithDescendants{GetTxSize()},
          nModFeesWithDescendants{nFee},
          nSizeWithAncestors{GetTxSize()},
          nModFeesWithAncestors{nFee},
          nSigOpCostWithAncestors{sigOpCost}
        */
    }
    
    /**
      | Updates the fee delta used for mining
      | priority score, and the modified fees
      | with descendants.
      |
      */
    pub fn update_fee_delta(&mut self, new_fee_delta: i64)  {
        
        todo!();
        /*
           nModFeesWithDescendants += newFeeDelta - feeDelta;
           nModFeesWithAncestors += newFeeDelta - feeDelta;
           feeDelta = newFeeDelta;
           */
    }
    
    /**
      | Update the LockPoints after a reorg
      |
      */
    pub fn update_lock_points(&mut self, lp: LockPoints)  {
        self.lock_points = lp;
    }
    
    pub fn get_tx_size(&self) -> usize {

        get_virtual_transaction_size(
            self.n_tx_weight.try_into().unwrap(), 
            self.sig_op_cost
        ).try_into().unwrap()
    }
}

#[inline] pub fn get_virtual_transaction_size(
        weight:     i64,
        sigop_cost: i64) -> i64 {
    
    todo!();
        /*
            return GetVirtualTransactionSize(weight, sigop_cost, ::nBytesPerSigOp);
        */
}
