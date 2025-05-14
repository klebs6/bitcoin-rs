// ---------------- [ File: bitcoin-txmempool/src/txmempool.rs ]
crate::ix!();


//-------------------------------------------[.cpp/bitcoin/src/txmempool.h]
//-------------------------------------------[.cpp/bitcoin/src/txmempool.cpp]

/**
  | Fake height value used in Coin to signify
  | they are only in the memory pool (since
  | 0.8)
  |
  */
pub const MEMPOOL_HEIGHT: u32 = 0x7FFFFFFF;

/**
  | Multi_index tag names
  |
  */
pub struct DescendantScore {}
pub struct EntryTime {}
pub struct AncestorScore {}
pub struct IndexByWTxid {}
pub struct TxIdIndex {}
pub struct InsertionOrder {}

/**
 | TxMemPool stores
 | valid-according-to-the-current-best-chain
 | transactions that may be included in the next
 | block.
 |
 | Transactions are added when they are seen on
 | the network (or created by the local node), but
 | not all transactions seen are added to the
 | pool. For example, the following new
 | transactions will not be added to the mempool:
 |
 | - a transaction which doesn't meet the minimum
 | fee requirements.
 |
 | - a new transaction that double-spends an input
 | of a transaction already in the pool where the
 | new transaction does not meet the
 | Replace-By-Fee requirements as defined in BIP
 | 125.
 |
 | - a non-standard transaction.
 |
 | TxMemPool::mapTx, and TxMemPoolEntry
 | bookkeeping:
 |
 | mapTx is a boost::multi_index that sorts the
 | mempool on 5 criteria:
 |
 | - transaction hash (txid)
 |
 | - witness-transaction hash (wtxid)
 |
 | - descendant feerate [we use max(feerate of tx,
 | feerate of tx with all descendants)]
 |
 | - time in mempool
 |
 | - ancestor feerate [we use min(feerate of tx,
 | feerate of tx with all unconfirmed ancestors)]
 |
 | Note: the term "descendant" refers to
 | in-mempool transactions that depend on this
 | one, while "ancestor" refers to in-mempool
 | transactions that a given transaction depends
 | on.
 |
 | In order for the feerate sort to remain
 | correct, we must update transactions in the
 | mempool when new descendants arrive.  To
 | facilitate this, we track the set of in-mempool
 | direct parents and direct children in mapLinks.
 | Within each TxMemPoolEntry, we track the size
 | and fees of all descendants.
 |
 | Usually when a new transaction is added to the
 | mempool, it has no in-mempool children (because
 | any such children would be an orphan).  So in
 | addUnchecked(), we:
 |
 | - update a new entry's setMemPoolParents to
 | include all in-mempool parents
 |
 | - update the new entry's direct parents to
 | include the new tx as a child
 |
 | - update all ancestors of the transaction to
 | include the new tx's size/fee
 |
 | When a transaction is removed from the mempool,
 | we must:
 |
 | - update all in-mempool parents to not track
 | the tx in setMemPoolChildren
 |
 | - update all ancestors to not include the tx's
 | size/fees in descendant state
 |
 | - update all in-mempool children to not include
 | it as a parent
 |
 | These happen in UpdateForRemoveFromMempool().
 | (Note that when removing a transaction along
 | with its descendants, we must calculate
 | that set of transactions to be removed
 | before doing the removal, or else the
 | mempool can be in an inconsistent state
 | where it's impossible to walk the ancestors
 | of a transaction.)
 |
 | In the event of a reorg, the assumption that
 | a newly added tx has no in-mempool children is
 | false.  
 |
 | In particular, the mempool is in an
 | inconsistent state while new transactions are
 | being added, because there may be descendant
 | transactions of a tx coming from a disconnected
 | block that are unreachable from just looking at
 | transactions in the mempool (the linking
 | transactions may also be in the
 | disconnected block, waiting to be added).
 |
 | Because of this, there's not much benefit in
 | trying to search for in-mempool children in
 | addUnchecked().  
 |
 | Instead, in the special case of transactions
 | being added from a disconnected block, we
 | require the caller to clean up the state, to
 | account for in-mempool, out-of-block
 | descendants for all the in-block transactions
 | by calling UpdateTransactionsFromBlock().  
 |
 | Note that until this is called, the mempool
 | state is not consistent, and in particular
 | mapLinks may not be correct (and therefore
 | functions like CalculateMemPoolAncestors()
 | and CalculateDescendants() that rely on
 | them to walk the mempool are not generally safe
 | to use).
 |
 | Computational limits:
 |
 | Updating all in-mempool ancestors of a newly
 | added transaction can be slow, if no bound
 | exists on how many in-mempool ancestors there
 | may be.
 |
 | CalculateMemPoolAncestors() takes configurable
 | limits that are designed to prevent these
 | calculations from being too CPU intensive.
 |
 */
pub struct TxMemPool {

    /**
      | Value n means that 1 times in n we check.
      |
      */
    pub check_ratio:                       i32,

    /**
      | Used by getblocktemplate to trigger
      | CreateNewBlock() invocation
      |
      */
    pub n_transactions_updated:            AtomicU32, // default = { 0 }

    pub miner_policy_estimator:            Arc<BlockPolicyEstimator>,

    /**
     | This mutex needs to be locked when
     | accessing `mapTx` or other members that are
     | guarded by it.
     |
     | @par Consistency guarantees
     |
     | By design, it is guaranteed that:
     |
     | 1. Locking both `cs_main` and
     |    `mempool.cs` will give a view of mempool
     |    that is consistent with current chain
     |    tip (`ActiveChain()` and `CoinsTip()`)
     |    and is fully populated. Fully populated
     |    means that if the current active chain
     |    is missing transactions that were
     |    present in a previously active chain,
     |    all the missing transactions will have
     |    been re-added to the mempool and should
     |    be present if they meet size and
     |    consistency constraints.
     |
     | 2. Locking `mempool.cs` without `cs_main`
     |    will give a view of a mempool consistent
     |    with some chain that was active since
     |    `cs_main` was last locked, and that is
     |    fully populated as described
     |    above. It is ok for code that only needs
     |    to query or remove transactions from the
     |    mempool to lock just `mempool.cs`
     |    without `cs_main`.
     |
     | To provide these guarantees, it is
     | necessary to lock both `cs_main` and
     | `mempool.cs` whenever adding transactions
     | to the mempool and whenever changing the
     | chain tip. It's necessary to keep both
     | mutexes locked until the mempool is
     | consistent with the new chain tip and fully
     | populated.
     */
    pub cs:        Arc<Mutex<TxMemPoolInner>>,
}

pub struct TxMemPoolInner {

    /**
      | sum of all mempool tx's virtual sizes.
      | Differs from serialized tx size since
      | witness data is discounted. Defined
      | in BIP 141.
      |
      */
    pub total_tx_size:                     u64,

    /**
      | sum of all mempool tx's fees (NOT modified
      | fee)
      |
      */
    pub total_fee:                         Amount,

    /**
      | sum of dynamic memory usage of all the
      | map elements (NOT the maps themselves)
      |
      */
    pub cached_inner_usage:                u64,

    pub last_rolling_fee_update:           Arc<Mutex<i64>>,

    pub block_since_last_rolling_fee_bump: AtomicBool,

    /**
      | minimum fee to get into the pool, decreases
      | exponentially
      |
      */
    pub rolling_minimum_fee_rate:          AtomicF64,

    pub epoch:                             Arc<Mutex<Epoch>>,

    /**
      | In-memory counter for external mempool
      | tracking purposes.
      |
      | This number is incremented once every time
      | a transaction is added or removed from the
      | mempool for any reason.
      */
    pub sequence_number:  RefCell<u64>, // default = { 1 }

    pub is_loaded:        bool, // default = { false }

    pub map_tx:           TxMemPoolIndexedTransactionSet,

    /**
      | All tx witness hashes/entries in mapTx,
      | in random order
      |
      */
    pub tx_hashes:        Vec<(u256,TxMemPoolTxIter)>,

    /**
      | Track locally submitted transactions
      | to periodically retry initial broadcast.
      |
      */
    pub unbroadcast_txids: HashSet<u256>,

    pub map_next_tx:       IndirectMap<OutPoint,Arc<Transaction>>,

    pub map_deltas:        HashMap<u256,Amount>,
}

pub type TxMemPoolSetEntries = HashSet<TxMemPoolTxIter,CompareIteratorByHash>;

//pub type TxIter            = IndexedTransactionSet::NthIndex<0>::ConstIterator;
pub type TxMemPoolIndexedTransactionSetNthIndex0ConstIterator = Broken;
pub type TxMemPoolTxIter     = TxMemPoolIndexedTransactionSetNthIndex0ConstIterator;

pub type TxMemPoolCacheMap   = HashMap<TxMemPoolTxIter,TxMemPoolSetEntries,CompareIteratorByHash>;

pub const ROLLING_FEE_HALFLIFE: i32 = 60 * 60 * 12; // public only for testing

pub type TxMemPoolIndexedTransactionSet = Broken;//TODO

pub type TxMemPoolIndexedTransactionSetConstIterator = Broken;
pub type TxMemPoolIndexedTransactionSetIterator = Broken;

/*
 | typedef boost::multi_index_container<
 |         TxMemPoolEntry,
 |         boost::multi_index::indexed_by<
 |
 |             // sorted by txid
 |             boost::multi_index::hashed_unique<mempoolentry_txid, SaltedTxidHasher>,
 |
 |             // sorted by wtxid
 |             boost::multi_index::hashed_unique<
 |                 boost::multi_index::tag<index_by_wtxid>,
 |                 mempoolentry_wtxid,
 |                 SaltedTxidHasher
 |             >,
 |
 |             // sorted by fee rate
 |             boost::multi_index::ordered_non_unique<
 |                 boost::multi_index::tag<descendant_score>,
 |                 boost::multi_index::identity<TxMemPoolEntry>,
 |                 CompareTxMemPoolEntryByDescendantScore
 |             >,
 |
 |             // sorted by entry time
 |             boost::multi_index::ordered_non_unique<
 |                 boost::multi_index::tag<entry_time>,
 |                 boost::multi_index::identity<TxMemPoolEntry>,
 |                 CompareTxMemPoolEntryByEntryTime
 |             >,
 |
 |             // sorted by fee rate with ancestors
 |             boost::multi_index::ordered_non_unique<
 |                 boost::multi_index::tag<ancestor_score>,
 |                 boost::multi_index::identity<TxMemPoolEntry>,
 |                 CompareTxMemPoolEntryByAncestorFee
 |             >
 |         >
 |     > indexed_transaction_set;
 */
multidex!{

    name => TxMemPoolIndexedTransactionSet,
    item => TxMemPoolEntry,

    hashed_unique => [

        // sorted by txid
        (IndexByTxid,  MempoolEntryTxId,  SaltedTxidHasher),

        // sorted by wtxid
        (IndexByWtxId, MempoolEntryWTxId, SaltedTxidHasher)
    ]

    ordered_nonunique => [

        // sorted by fee rate
        (DescendantScore, TxMemPoolEntry, CompareTxMemPoolEntryByDescendantScore),

        // sorted by entry time
        (EntryTime,       TxMemPoolEntry, CompareTxMemPoolEntryByEntryTime),

        // sorted by fee rate with ancestors
        (AncestorScore,   TxMemPoolEntry, CompareTxMemPoolEntryByAncestorFee)
    ]
}

/**
  | we will need to see which interfaces
  | we need upon uncommenting
  |
  */
pub trait ITxMemPool {} 

impl ITxMemPool for TxMemPool {}

impl TxMemPool {

    pub fn size(&self) -> u64 {
        
        todo!();
        /*
            LOCK(cs);
            return mapTx.size();
        */
    }

    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn get_total_tx_size(&self) -> u64 {
        
        todo!();
        /*
            AssertLockHeld(cs);
            return totalTxSize;
        */
    }

    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn get_total_fee(&self) -> Amount {
        
        todo!();
        /*
            AssertLockHeld(cs);
            return m_total_fee;
        */
    }
    
    pub fn exists(&self, gtxid: &GenTxId) -> bool {
        
        todo!();
        /*
            LOCK(cs);
            if (gtxid.IsWtxid()) {
                return (mapTx.get<index_by_wtxid>().count(gtxid.GetHash()) != 0);
            }
            return (mapTx.count(gtxid.GetHash()) != 0);
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn get_iter_from_wtxid(&self, wtxid: &u256) -> TxMemPoolTxIter {
        
        todo!();
        /*
            AssertLockHeld(cs);
            return mapTx.project<0>(mapTx.get<index_by_wtxid>().find(wtxid));
        */
    }
    
    /**
      | Adds a transaction to the unbroadcast
      | set
      |
      */
    pub fn add_unbroadcast_tx(&mut self, txid: &u256)  {
        
        todo!();
        /*
            LOCK(cs);
            // Sanity check the transaction is in the mempool & insert into
            // unbroadcast set.
            if (exists(GenTxId::Txid(txid))) m_unbroadcast_txids.insert(txid);
        }{
        */
    }

    /**
      | Returns transactions in unbroadcast
      | set
      |
      */
    pub fn get_unbroadcast_txs(&self) -> HashSet<u256> {
        
        todo!();
        /*
            LOCK(cs);
            return m_unbroadcast_txids;
        */
    }

    /**
      | Returns whether a txid is in the unbroadcast
      | set
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn is_unbroadcast_tx(&self, txid: &u256) -> bool {
        
        todo!();
        /*
            AssertLockHeld(cs);
            return m_unbroadcast_txids.count(txid) != 0;
        */
    }

    /**
      | Guards this internal counter for external
      | reporting
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn get_and_increment_sequence(&self) -> u64 {
        
        todo!();
        /*
            return m_sequence_number++;
        */
    }

    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn get_sequence(&self) -> u64 {
        
        todo!();
        /*
            return m_sequence_number;
        */
    }

    /**
      | visited marks a TxMemPoolEntry as
      | having been traversed during the lifetime
      | of the most recently created Epoch::Guard
      | and returns false if we are the first
      | visitor, true otherwise.
      | 
      | An Epoch::Guard must be held when visited
      | is called or an assert will be triggered.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs, m_epoch)]
    fn visited_impl(&self, it: TxMemPoolTxIter) -> bool {
        
        todo!();
        /*
            return m_epoch.visited(it->m_epoch_marker);
        */
    }

    #[EXCLUSIVE_LOCKS_REQUIRED(cs, m_epoch)]
    pub fn visited(&self, it: Option<TxMemPoolTxIter>) -> bool {
        
        todo!();
        /*
            assert(m_epoch.guarded()); // verify guard even when it==nullopt
            return !it || visited(*it);
        */
    }
    
    /**
      | Update the given tx for any in-mempool
      | descendants.
      |
      | Assumes that TxMemPool::m_children is correct
      | for the given tx and all descendants.
      |
      | UpdateForDescendants is used by
      | UpdateTransactionsFromBlock to update the
      | descendants for a single transaction that
      | has been added to the mempool but may have
      | child transactions in the mempool, eg
      | during a chain reorg.  setExclude is the
      | set of descendant transactions in the
      | mempool that must not be accounted for
      | (because any descendants in setExclude were
      | added to the mempool after the transaction
      | being updated and hence their state is
      | already reflected in the parent state).
      |
      | cachedDescendants will be updated with the
      | descendants of the transaction being
      | updated, so that future invocations don't
      | need to walk the same transaction again,
      | if encountered in another transaction
      | chain.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn update_for_descendants(&mut self, 
        update_it:          TxMemPoolTxIter,
        cached_descendants: &mut TxMemPoolCacheMap,
        set_exclude:        &HashSet<u256>)  {
        
        todo!();
        /*
            TxMemPoolEntry::Children stageEntries, descendants;
        stageEntries = updateIt->GetMemPoolChildrenConst();

        while (!stageEntries.empty()) {
            const TxMemPoolEntry& descendant = *stageEntries.begin();
            descendants.insert(descendant);
            stageEntries.erase(descendant);
            const TxMemPoolEntry::Children& children = descendant.GetMemPoolChildrenConst();
            for (const TxMemPoolEntry& childEntry : children) {
                cacheMap::iterator cacheIt = cachedDescendants.find(mapTx.iterator_to(childEntry));
                if (cacheIt != cachedDescendants.end()) {
                    // We've already calculated this one, just add the entries for this set
                    // but don't traverse again.
                    for (txiter cacheEntry : cacheIt->second) {
                        descendants.insert(*cacheEntry);
                    }
                } else if (!descendants.count(childEntry)) {
                    // Schedule for later processing
                    stageEntries.insert(childEntry);
                }
            }
        }
        // descendants now contains all in-mempool descendants of updateIt.
        // Update and add to cached descendant map
        int64_t modifySize = 0;
        CAmount modifyFee = 0;
        int64_t modifyCount = 0;
        for (const TxMemPoolEntry& descendant : descendants) {
            if (!setExclude.count(descendant.GetTx().GetHash())) {
                modifySize += descendant.GetTxSize();
                modifyFee += descendant.GetModifiedFee();
                modifyCount++;
                cachedDescendants[updateIt].insert(mapTx.iterator_to(descendant));
                // Update ancestor state for each descendant
                mapTx.modify(mapTx.iterator_to(descendant), update_ancestor_state(updateIt->GetTxSize(), updateIt->GetModifiedFee(), 1, updateIt->GetSigOpCost()));
            }
        }
        mapTx.modify(updateIt, update_descendant_state(modifySize, modifyFee, modifyCount));
        */
    }

    /**
      | When adding transactions from a disconnected
      | block back to the mempool, new mempool
      | entries may have children in the mempool
      | (which is generally not the case when
      | otherwise adding transactions).
      | 
      | UpdateTransactionsFromBlock() will
      | find child transactions and update
      | the descendant state for each transaction
      | in vHashesToUpdate (excluding any
      | child transactions present in vHashesToUpdate,
      | which are already accounted for). Note:
      | vHashesToUpdate should be the set of
      | transactions from the disconnected
      | block that have been accepted back into
      | the mempool.
      |
      | vHashesToUpdate is the set of transaction
      | hashes from a disconnected block which has been
      | re-added to the mempool.
      |
      | for each entry, look for descendants that are
      | outside vHashesToUpdate, and add fee/size
      | information for such descendants to the parent.
      |
      | for each such descendant, also update the
      | ancestor state to include the parent.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs, cs_main)]
    #[LOCKS_EXCLUDED(m_epoch)]
    pub fn update_transactions_from_block(&mut self, hashes_to_update: &Vec<u256>)  {
        
        todo!();
        /*
            AssertLockHeld(cs);
        // For each entry in vHashesToUpdate, store the set of in-mempool, but not
        // in-vHashesToUpdate transactions, so that we don't have to recalculate
        // descendants when we come across a previously seen entry.
        cacheMap mapMemPoolDescendantsToUpdate;

        // Use a set for lookups into vHashesToUpdate (these entries are already
        // accounted for in the state of their ancestors)
        std::set<uint256> setAlreadyIncluded(vHashesToUpdate.begin(), vHashesToUpdate.end());

        // Iterate in reverse, so that whenever we are looking at a transaction
        // we are sure that all in-mempool descendants have already been processed.
        // This maximizes the benefit of the descendant cache and guarantees that
        // TxMemPool::m_children will be updated, an assumption made in
        // UpdateForDescendants.
        for (const uint256 &hash : reverse_iterate(vHashesToUpdate)) {
            // calculate children from mapNextTx
            txiter it = mapTx.find(hash);
            if (it == mapTx.end()) {
                continue;
            }
            auto iter = mapNextTx.lower_bound(OutPoint(hash, 0));
            // First calculate the children, and update TxMemPool::m_children to
            // include them, and update their TxMemPoolEntry::m_parents to include this tx.
            // we cache the in-mempool children to avoid duplicate updates
            {
                WITH_FRESH_EPOCH(m_epoch);
                for (; iter != mapNextTx.end() && iter->first->hash == hash; ++iter) {
                    const uint256 &childHash = iter->second->GetHash();
                    txiter childIter = mapTx.find(childHash);
                    assert(childIter != mapTx.end());
                    // We can skip updating entries we've encountered before or that
                    // are in the block (which are already accounted for).
                    if (!visited(childIter) && !setAlreadyIncluded.count(childHash)) {
                        UpdateChild(it, childIter, true);
                        UpdateParent(childIter, it, true);
                    }
                }
            } // release epoch guard for UpdateForDescendants
            UpdateForDescendants(it, mapMemPoolDescendantsToUpdate, setAlreadyIncluded);
        }
        */
    }
    
    /**
      | Helper function to calculate all in-mempool
      | ancestors of staged_ancestors and
      | apply ancestor and descendant limits
      | (including staged_ancestors thsemselves,
      | entry_size and entry_count).
      | 
      | -----------
      | @param[in] entry_size
      | 
      | Virtual size to include in the limits.
      | ----------
      | @param[in] entry_count
      | 
      | How many entries to include in the limits.
      | ----------
      | @param[in] staged_ancestors
      | 
      | Should contain entries in the mempool.
      | ----------
      | @param[out] setAncestors
      | 
      | Will be populated with all mempool ancestors.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn calculate_ancestors_and_check_limits(&self, 
        entry_size:             usize,
        entry_count:            usize,
        set_ancestors:          &mut TxMemPoolSetEntries,
        staged_ancestors:       &mut TxMemPoolEntryParents,
        limit_ancestor_count:   u64,
        limit_ancestor_size:    u64,
        limit_descendant_count: u64,
        limit_descendant_size:  u64,
        err_string:             &mut String) -> bool {
        
        todo!();
        /*
            size_t totalSizeWithAncestors = entry_size;

        while (!staged_ancestors.empty()) {
            const TxMemPoolEntry& stage = staged_ancestors.begin()->get();
            txiter stageit = mapTx.iterator_to(stage);

            setAncestors.insert(stageit);
            staged_ancestors.erase(stage);
            totalSizeWithAncestors += stageit->GetTxSize();

            if (stageit->GetSizeWithDescendants() + entry_size > limitDescendantSize) {
                errString = strprintf("exceeds descendant size limit for tx %s [limit: %u]", stageit->GetTx().GetHash().ToString(), limitDescendantSize);
                return false;
            } else if (stageit->GetCountWithDescendants() + entry_count > limitDescendantCount) {
                errString = strprintf("too many descendants for tx %s [limit: %u]", stageit->GetTx().GetHash().ToString(), limitDescendantCount);
                return false;
            } else if (totalSizeWithAncestors > limitAncestorSize) {
                errString = strprintf("exceeds ancestor size limit [limit: %u]", limitAncestorSize);
                return false;
            }

            const TxMemPoolEntry::Parents& parents = stageit->GetMemPoolParentsConst();
            for (const TxMemPoolEntry& parent : parents) {
                txiter parent_it = mapTx.iterator_to(parent);

                // If this is a new ancestor, add it.
                if (setAncestors.count(parent_it) == 0) {
                    staged_ancestors.insert(parent);
                }
                if (staged_ancestors.size() + setAncestors.size() + entry_count > limitAncestorCount) {
                    errString = strprintf("too many unconfirmed ancestors [limit: %u]", limitAncestorCount);
                    return false;
                }
            }
        }

        return true;
        */
    }
    
    /**
      | Calculate all in-mempool ancestors
      | of a set of transactions not already
      | in the mempool and check ancestor and
      | descendant limits. Heuristics are
      | used to estimate the ancestor and descendant
      | count of all entries if the package were
      | to be added to the mempool. The limits
      | are applied to the union of all package
      | transactions. For example, if the package
      | has 3 transactions and limitAncestorCount
      | = 25, the union of all 3 sets of ancestors
      | (including the transactions themselves)
      | must be <= 22.
      | 
      | -----------
      | @param[in] package
      | 
      | Transaction package being evaluated
      | for acceptance to mempool. The transactions
      | need not be direct ancestors/descendants
      | of each other.
      | ----------
      | @param[in] limitAncestorCount
      | 
      | Max number of txns including ancestors.
      | ----------
      | @param[in] limitAncestorSize
      | 
      | Max virtual size including ancestors.
      | ----------
      | @param[in] limitDescendantCount
      | 
      | Max number of txns including descendants.
      | ----------
      | @param[in] limitDescendantSize
      | 
      | Max virtual size including descendants.
      | ----------
      | @param[out] errString
      | 
      | Populated with error reason if a limit
      | is hit.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn check_package_limits(&self, 
        package:                &Package,
        limit_ancestor_count:   u64,
        limit_ancestor_size:    u64,
        limit_descendant_count: u64,
        limit_descendant_size:  u64,
        err_string:             &mut String) -> bool {
        
        todo!();
        /*
            TxMemPoolEntry::Parents staged_ancestors;
        size_t total_size = 0;
        for (const auto& tx : package) {
            total_size += GetVirtualTransactionSize(*tx);
            for (const auto& input : tx->vin) {
                std::optional<txiter> piter = GetIter(input.prevout.hash);
                if (piter) {
                    staged_ancestors.insert(**piter);
                    if (staged_ancestors.size() + package.size() > limitAncestorCount) {
                        errString = strprintf("too many unconfirmed parents [limit: %u]", limitAncestorCount);
                        return false;
                    }
                }
            }
        }
        // When multiple transactions are passed in, the ancestors and descendants of all transactions
        // considered together must be within limits even if they are not interdependent. This may be
        // stricter than the limits for each individual transaction.
        setEntries setAncestors;
        const auto ret = CalculateAncestorsAndCheckLimits(total_size, package.size(),
                                                          setAncestors, staged_ancestors,
                                                          limitAncestorCount, limitAncestorSize,
                                                          limitDescendantCount, limitDescendantSize, errString);
        // It's possible to overestimate the ancestor/descendant totals.
        if (!ret) errString.insert(0, "possibly ");
        return ret;
        */
    }
    
    /**
      | Try to calculate all in-mempool ancestors
      | of entry. (these are all calculated
      | including the tx itself)
      | 
      | - limitAncestorCount = max number of
      | ancestors
      | 
      | - limitAncestorSize = max size of ancestors
      | 
      | - limitDescendantCount = max number
      | of descendants any ancestor can have
      | 
      | - limitDescendantSize = max size of
      | descendants any ancestor can have
      | 
      | - errString = populated with error reason
      | if any limits are hit
      | 
      | - fSearchForParents = whether to search
      | a tx's vin for in-mempool parents, or
      | look up parents from mapLinks. Must
      | be true for entries not in the mempool
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn calculate_mem_pool_ancestors(&self, 
        entry:                  &TxMemPoolEntry,
        set_ancestors:          &mut TxMemPoolSetEntries,
        limit_ancestor_count:   u64,
        limit_ancestor_size:    u64,
        limit_descendant_count: u64,
        limit_descendant_size:  u64,
        err_string:             &mut String,
        search_for_parents:     Option<bool>) -> bool {
        let search_for_parents: bool = search_for_parents.unwrap_or(true);

        todo!();
        /*
            TxMemPoolEntry::Parents staged_ancestors;
        const CTransaction &tx = entry.GetTx();

        if (fSearchForParents) {
            // Get parents of this transaction that are in the mempool
            // GetMemPoolParents() is only valid for entries in the mempool, so we
            // iterate mapTx to find parents.
            for (unsigned int i = 0; i < tx.vin.size(); i++) {
                std::optional<txiter> piter = GetIter(tx.vin[i].prevout.hash);
                if (piter) {
                    staged_ancestors.insert(**piter);
                    if (staged_ancestors.size() + 1 > limitAncestorCount) {
                        errString = strprintf("too many unconfirmed parents [limit: %u]", limitAncestorCount);
                        return false;
                    }
                }
            }
        } else {
            // If we're not searching for parents, we require this to already be an
            // entry in the mempool and use the entry's cached parents.
            txiter it = mapTx.iterator_to(entry);
            staged_ancestors = it->GetMemPoolParentsConst();
        }

        return CalculateAncestorsAndCheckLimits(entry.GetTxSize(), /* entry_count */ 1,
                                                setAncestors, staged_ancestors,
                                                limitAncestorCount, limitAncestorSize,
                                                limitDescendantCount, limitDescendantSize, errString);
        */
    }
    
    /**
      | Update ancestors of hash to add/remove
      | it as a descendant transaction.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn update_ancestors_of(&mut self, 
        add:           bool,
        it:            TxMemPoolTxIter,
        set_ancestors: &mut TxMemPoolSetEntries)  {
        
        todo!();
        /*
            TxMemPoolEntry::Parents parents = it->GetMemPoolParents();
        // add or remove this tx as a child of each parent
        for (const TxMemPoolEntry& parent : parents) {
            UpdateChild(mapTx.iterator_to(parent), it, add);
        }
        const int64_t updateCount = (add ? 1 : -1);
        const int64_t updateSize = updateCount * it->GetTxSize();
        const CAmount updateFee = updateCount * it->GetModifiedFee();
        for (txiter ancestorIt : setAncestors) {
            mapTx.modify(ancestorIt, update_descendant_state(updateSize, updateFee, updateCount));
        }
        */
    }
    
    /**
      | Set ancestor state for an entry
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn update_entry_for_ancestors(&mut self, 
        it:            TxMemPoolTxIter,
        set_ancestors: &TxMemPoolSetEntries)  {
        
        todo!();
        /*
            int64_t updateCount = setAncestors.size();
        int64_t updateSize = 0;
        CAmount updateFee = 0;
        int64_t updateSigOpsCost = 0;
        for (txiter ancestorIt : setAncestors) {
            updateSize += ancestorIt->GetTxSize();
            updateFee += ancestorIt->GetModifiedFee();
            updateSigOpsCost += ancestorIt->GetSigOpCost();
        }
        mapTx.modify(it, update_ancestor_state(updateSize, updateFee, updateCount, updateSigOpsCost));
        */
    }
    
    /**
      | Sever link between specified transaction
      | and direct children.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn update_children_for_removal(&mut self, it: TxMemPoolTxIter)  {
        
        todo!();
        /*
            const TxMemPoolEntry::Children& children = it->GetMemPoolChildrenConst();
        for (const TxMemPoolEntry& updateIt : children) {
            UpdateParent(mapTx.iterator_to(updateIt), it, false);
        }
        */
    }
    
    /**
      | For each transaction being removed,
      | update ancestors and any direct children.
      | 
      | If updateDescendants is true, then
      | also update in-mempool descendants'
      | ancestor state.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn update_for_remove_from_mempool(&mut self, 
        entries_to_remove:  &TxMemPoolSetEntries,
        update_descendants: bool)  {
        
        todo!();
        /*
            // For each entry, walk back all ancestors and decrement size associated with this
        // transaction
        const uint64_t nNoLimit = std::numeric_limits<uint64_t>::max();
        if (updateDescendants) {
            // updateDescendants should be true whenever we're not recursively
            // removing a tx and all its descendants, eg when a transaction is
            // confirmed in a block.
            // Here we only update statistics and not data in TxMemPool::Parents
            // and TxMemPoolEntry::Children (which we need to preserve until we're
            // finished with all operations that need to traverse the mempool).
            for (txiter removeIt : entriesToRemove) {
                setEntries setDescendants;
                CalculateDescendants(removeIt, setDescendants);
                setDescendants.erase(removeIt); // don't update state for self
                int64_t modifySize = -((int64_t)removeIt->GetTxSize());
                CAmount modifyFee = -removeIt->GetModifiedFee();
                int modifySigOps = -removeIt->GetSigOpCost();
                for (txiter dit : setDescendants) {
                    mapTx.modify(dit, update_ancestor_state(modifySize, modifyFee, -1, modifySigOps));
                }
            }
        }
        for (txiter removeIt : entriesToRemove) {
            setEntries setAncestors;
            const TxMemPoolEntry &entry = *removeIt;
            std::string dummy;
            // Since this is a tx that is already in the mempool, we can call CMPA
            // with fSearchForParents = false.  If the mempool is in a consistent
            // state, then using true or false should both be correct, though false
            // should be a bit faster.
            // However, if we happen to be in the middle of processing a reorg, then
            // the mempool can be in an inconsistent state.  In this case, the set
            // of ancestors reachable via GetMemPoolParents()/GetMemPoolChildren()
            // will be the same as the set of ancestors whose packages include this
            // transaction, because when we add a new transaction to the mempool in
            // addUnchecked(), we assume it has no children, and in the case of a
            // reorg where that assumption is false, the in-mempool children aren't
            // linked to the in-block tx's until UpdateTransactionsFromBlock() is
            // called.
            // So if we're being called during a reorg, ie before
            // UpdateTransactionsFromBlock() has been called, then
            // GetMemPoolParents()/GetMemPoolChildren() will differ from the set of
            // mempool parents we'd calculate by searching, and it's important that
            // we use the cached notion of ancestor transactions as the set of
            // things to update for removal.
            CalculateMemPoolAncestors(entry, setAncestors, nNoLimit, nNoLimit, nNoLimit, nNoLimit, dummy, false);
            // Note that UpdateAncestorsOf severs the child links that point to
            // removeIt in the entries for the parents of removeIt.
            UpdateAncestorsOf(false, removeIt, setAncestors);
        }
        // After updating all the ancestor sizes, we can now sever the link between each
        // transaction being removed and any mempool children (ie, update TxMemPoolEntry::m_parents
        // for each direct child of a transaction being removed).
        for (txiter removeIt : entriesToRemove) {
            UpdateChildrenForRemoval(removeIt);
        }
        */
    }
    
    /**
      | Create a new TxMemPool.
      | 
      | Sanity checks will be off by default
      | for performance, because otherwise
      | accepting transactions becomes O(N^2)
      | where N is the number of transactions
      | in the pool.
      | 
      | -----------
      | @param[in] estimator
      | 
      | is used to estimate appropriate transaction
      | fees.
      | ----------
      | @param[in] check_ratio
      | 
      | is the ratio used to determine how often
      | sanity checks will run.
      |
      */
    pub fn new(
        estimator:   *mut BlockPolicyEstimator,
        check_ratio: Option<i32>) -> Self {

        let check_ratio: i32 = check_ratio.unwrap_or(0);
    
        todo!();
        /*


            : m_check_ratio(check_ratio), minerPolicyEstimator(estimator)
        _clear(); //lock free clear
        */
    }
    
    pub fn is_spent(&self, outpoint: &OutPoint) -> bool {
        
        todo!();
        /*
            LOCK(cs);
        return mapNextTx.count(outpoint);
        */
    }
    
    pub fn get_transactions_updated(&self) -> u32 {
        
        todo!();
        /*
            return nTransactionsUpdated;
        */
    }
    
    pub fn add_transactions_updated(&mut self, n: u32)  {
        
        todo!();
        /*
            nTransactionsUpdated += n;
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs, cs_main)]
    pub fn add_unchecked_with_set_ancestors(&mut self, 
        entry:              &TxMemPoolEntry,
        set_ancestors:      &mut TxMemPoolSetEntries,
        valid_fee_estimate: Option<bool>)  {

        let valid_fee_estimate: bool = valid_fee_estimate.unwrap_or(true);
        
        todo!();
        /*
            // Add to memory pool without checking anything.
        // Used by AcceptToMemoryPool(), which DOES do
        // all the appropriate checks.
        IndexedTransactionSet::Iterator newit = mapTx.insert(entry).first;

        // Update transaction for any feeDelta created by PrioritiseTransaction
        // TODO: refactor so that the fee delta is calculated before inserting
        // into mapTx.
        CAmount delta{0};
        ApplyDelta(entry.GetTx().GetHash(), delta);
        if (delta) {
                mapTx.modify(newit, update_fee_delta(delta));
        }

        // Update cachedInnerUsage to include contained transaction's usage.
        // (When we update the entry for in-mempool parents, memory usage will be
        // further updated.)
        cachedInnerUsage += entry.DynamicMemoryUsage();

        const CTransaction& tx = newit->GetTx();
        std::set<uint256> setParentTransactions;
        for (unsigned int i = 0; i < tx.vin.size(); i++) {
            mapNextTx.insert(std::make_pair(&tx.vin[i].prevout, &tx));
            setParentTransactions.insert(tx.vin[i].prevout.hash);
        }
        // Don't bother worrying about child transactions of this one.
        // Normal case of a new transaction arriving is that there can't be any
        // children, because such children would be orphans.
        // An exception to that is if a transaction enters that used to be in a block.
        // In that case, our disconnect block logic will call UpdateTransactionsFromBlock
        // to clean up the mess we're leaving here.

        // Update ancestors with information about this tx
        for (const auto& pit : GetIterSet(setParentTransactions)) {
                UpdateParent(newit, pit, true);
        }
        UpdateAncestorsOf(true, newit, setAncestors);
        UpdateEntryForAncestors(newit, setAncestors);

        nTransactionsUpdated++;
        totalTxSize += entry.GetTxSize();
        m_total_fee += entry.GetFee();
        if (minerPolicyEstimator) {
            minerPolicyEstimator->processTransaction(entry, validFeeEstimate);
        }

        vTxHashes.emplace_back(tx.GetWitnessHash(), newit);
        newit->vTxHashesIdx = vTxHashes.size() - 1;
        */
    }
    
    /**
      | Before calling removeUnchecked for
      | a given transaction,
      | 
      | UpdateForRemoveFromMempool must
      | be called on the entire (dependent)
      | set of transactions being removed at
      | the same time. We use each
      | 
      | TxMemPoolEntry's setMemPoolParents
      | in order to walk ancestors of a given
      | transaction that is removed, so we can't
      | remove intermediate transactions
      | in a chain before we've updated all the
      | state for the removal.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn remove_unchecked(&mut self, 
        it:     TxMemPoolTxIter,
        reason: MemPoolRemovalReason)  {
        
        todo!();
        /*
            // We increment mempool sequence value no matter removal reason
        // even if not directly reported below.
        uint64_t mempool_sequence = GetAndIncrementSequence();

        if (reason != MemPoolRemovalReason::BLOCK) {
            // Notify clients that a transaction has been removed from the mempool
            // for any reason except being included in a block. Clients interested
            // in transactions included in blocks can subscribe to the BlockConnected
            // notification.
            GetMainSignals().TransactionRemovedFromMempool(it->GetSharedTx(), reason, mempool_sequence);
        }

        const uint256 hash = it->GetTx().GetHash();
        for (const CTxIn& txin : it->GetTx().vin)
            mapNextTx.erase(txin.prevout);

        RemoveUnbroadcastTx(hash, true /* add logging because unchecked */ );

        if (vTxHashes.size() > 1) {
            vTxHashes[it->vTxHashesIdx] = std::move(vTxHashes.back());
            vTxHashes[it->vTxHashesIdx].second->vTxHashesIdx = it->vTxHashesIdx;
            vTxHashes.pop_back();
            if (vTxHashes.size() * 2 < vTxHashes.capacity())
                vTxHashes.shrink_to_fit();
        } else
            vTxHashes.clear();

        totalTxSize -= it->GetTxSize();
        m_total_fee -= it->GetFee();
        cachedInnerUsage -= it->DynamicMemoryUsage();
        cachedInnerUsage -= memusage::DynamicUsage(it->GetMemPoolParentsConst()) + memusage::DynamicUsage(it->GetMemPoolChildrenConst());
        mapTx.erase(it);
        nTransactionsUpdated++;
        if (minerPolicyEstimator) {minerPolicyEstimator->removeTx(hash, false);}
        */
    }

    /**
      | Populate setDescendants with all in-mempool
      | descendants of hash.
      | 
      | Assumes that setDescendants includes
      | all in-mempool descendants of anything
      | already in it.
      |
      | Calculates descendants of entry that are not
      | already in setDescendants, and adds to
      | setDescendants. Assumes entryit is already a tx
      | in the mempool and TxMemPoolEntry::m_children
      | is correct for tx and all descendants.
      |
      | Also assumes that if an entry is in
      | setDescendants already, then all in-mempool
      | descendants of it are already in setDescendants
      | as well, so that we can save time by not
      | iterating over those entries.
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn calculate_descendants(&self, 
        entryit:         TxMemPoolTxIter,
        set_descendants: &mut TxMemPoolSetEntries)  {
        
        todo!();
        /*
            setEntries stage;
        if (setDescendants.count(entryit) == 0) {
            stage.insert(entryit);
        }
        // Traverse down the children of entry, only adding children that are not
        // accounted for in setDescendants already (because those children have either
        // already been walked, or will be walked in this iteration).
        while (!stage.empty()) {
            txiter it = *stage.begin();
            setDescendants.insert(it);
            stage.erase(it);

            const TxMemPoolEntry::Children& children = it->GetMemPoolChildrenConst();
            for (const TxMemPoolEntry& child : children) {
                txiter childiter = mapTx.iterator_to(child);
                if (!setDescendants.count(childiter)) {
                    stage.insert(childiter);
                }
            }
        }
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn remove_recursive(&mut self, 
        orig_tx: &Transaction,
        reason:  MemPoolRemovalReason)  {
        
        todo!();
        /*
            // Remove transaction from memory pool
        AssertLockHeld(cs);
            setEntries txToRemove;
            txiter origit = mapTx.find(origTx.GetHash());
            if (origit != mapTx.end()) {
                txToRemove.insert(origit);
            } else {
                // When recursively removing but origTx isn't in the mempool
                // be sure to remove any children that are in the pool. This can
                // happen during chain re-orgs if origTx isn't re-accepted into
                // the mempool for any reason.
                for (unsigned int i = 0; i < origTx.vout.size(); i++) {
                    auto it = mapNextTx.find(OutPoint(origTx.GetHash(), i));
                    if (it == mapNextTx.end())
                        continue;
                    txiter nextit = mapTx.find(it->second->GetHash());
                    assert(nextit != mapTx.end());
                    txToRemove.insert(nextit);
                }
            }
            setEntries setAllRemoves;
            for (txiter it : txToRemove) {
                CalculateDescendants(it, setAllRemoves);
            }

            RemoveStaged(setAllRemoves, false, reason);
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn remove_conflicts(&mut self, tx: &Transaction)  {
        
        todo!();
        /*
            // Remove transactions which depend on inputs of tx, recursively
        AssertLockHeld(cs);
        for (const CTxIn &txin : tx.vin) {
            auto it = mapNextTx.find(txin.prevout);
            if (it != mapNextTx.end()) {
                const CTransaction &txConflict = *it->second;
                if (txConflict != tx)
                {
                    ClearPrioritisation(txConflict.GetHash());
                    removeRecursive(txConflict, MemPoolRemovalReason::CONFLICT);
                }
            }
        }
        */
    }

    /**
      | Called when a block is connected. Removes
      | from mempool and updates the miner fee
      | estimator.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn remove_for_block(&mut self, 
        vtx:            &Vec<TransactionRef>,
        n_block_height: u32)  {
        
        todo!();
        /*
            AssertLockHeld(cs);
        std::vector<const TxMemPoolEntry*> entries;
        for (const auto& tx : vtx)
        {
            uint256 hash = tx->GetHash();

            IndexedTransactionSet::Iterator i = mapTx.find(hash);
            if (i != mapTx.end())
                entries.push_back(&*i);
        }
        // Before the txs in the new block have been removed from the mempool, update policy estimates
        if (minerPolicyEstimator) {minerPolicyEstimator->processBlock(nBlockHeight, entries);}
        for (const auto& tx : vtx)
        {
            txiter it = mapTx.find(tx->GetHash());
            if (it != mapTx.end()) {
                setEntries stage;
                stage.insert(it);
                RemoveStaged(stage, true, MemPoolRemovalReason::BLOCK);
            }
            removeConflicts(*tx);
            ClearPrioritisation(tx->GetHash());
        }
        lastRollingFeeUpdate = GetTime();
        blockSinceLastRollingFeeBump = true;
        */
    }
    
    /**
      | lock free
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    fn clear_impl(&mut self)  {
        
        todo!();
        /*
            mapTx.clear();
        mapNextTx.clear();
        totalTxSize = 0;
        m_total_fee = 0;
        cachedInnerUsage = 0;
        lastRollingFeeUpdate = GetTime();
        blockSinceLastRollingFeeBump = false;
        rollingMinimumFeeRate = 0;
        ++nTransactionsUpdated;
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            LOCK(cs);
        _clear();
        */
    }
    
    /**
      | If sanity-checking is turned on, check
      | makes sure the pool is consistent (does
      | not contain two transactions that spend
      | the same inputs, all inputs are in the
      | mapNextTx array). If sanity-checking
      | is turned off, check does nothing.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(::cs_main)]
    pub fn check(&self, 
        active_coins_tip: &CoinsViewCache,
        spendheight:      i64) 
    {
        todo!();

        /*
            if (m_check_ratio == 0) return;

        if (GetRand(m_check_ratio) >= 1) return;

        AssertLockHeld(::cs_main);
        LOCK(cs);
        LogPrint(BCLog::MEMPOOL, "Checking mempool with %u transactions and %u inputs\n", (unsigned int)mapTx.size(), (unsigned int)mapNextTx.size());

        uint64_t checkTotal = 0;
        CAmount check_total_fee{0};
        uint64_t innerUsage = 0;
        uint64_t prev_ancestor_count{0};

        CCoinsViewCache mempoolDuplicate(const_cast<CCoinsViewCache*>(&active_coins_tip));

        for (const auto& it : GetSortedDepthAndScore()) {
            checkTotal += it->GetTxSize();
            check_total_fee += it->GetFee();
            innerUsage += it->DynamicMemoryUsage();
            const CTransaction& tx = it->GetTx();
            innerUsage += memusage::DynamicUsage(it->GetMemPoolParentsConst()) + memusage::DynamicUsage(it->GetMemPoolChildrenConst());
            TxMemPoolEntry::Parents setParentCheck;
            for (const CTxIn &txin : tx.vin) {
                // Check that every mempool transaction's inputs refer to available coins, or other mempool tx's.
                IndexedTransactionSetConstIterator it2 = mapTx.find(txin.prevout.hash);
                if (it2 != mapTx.end()) {
                    const CTransaction& tx2 = it2->GetTx();
                    assert(tx2.vout.size() > txin.prevout.n && !tx2.vout[txin.prevout.n].IsNull());
                    setParentCheck.insert(*it2);
                }
                // We are iterating through the mempool entries sorted in order by ancestor count.
                // All parents must have been checked before their children and their coins added to
                // the mempoolDuplicate coins cache.
                assert(mempoolDuplicate.HaveCoin(txin.prevout));
                // Check whether its inputs are marked in mapNextTx.
                auto it3 = mapNextTx.find(txin.prevout);
                assert(it3 != mapNextTx.end());
                assert(it3->first == &txin.prevout);
                assert(it3->second == &tx);
            }
            auto comp = [](const TxMemPoolEntry& a, const TxMemPoolEntry& b) -> bool {
                return a.GetTx().GetHash() == b.GetTx().GetHash();
            };
            assert(setParentCheck.size() == it->GetMemPoolParentsConst().size());
            assert(std::equal(setParentCheck.begin(), setParentCheck.end(), it->GetMemPoolParentsConst().begin(), comp));
            // Verify ancestor state is correct.
            setEntries setAncestors;
            uint64_t nNoLimit = std::numeric_limits<uint64_t>::max();
            std::string dummy;
            CalculateMemPoolAncestors(*it, setAncestors, nNoLimit, nNoLimit, nNoLimit, nNoLimit, dummy);
            uint64_t nCountCheck = setAncestors.size() + 1;
            uint64_t nSizeCheck = it->GetTxSize();
            CAmount nFeesCheck = it->GetModifiedFee();
            int64_t nSigOpCheck = it->GetSigOpCost();

            for (txiter ancestorIt : setAncestors) {
                nSizeCheck += ancestorIt->GetTxSize();
                nFeesCheck += ancestorIt->GetModifiedFee();
                nSigOpCheck += ancestorIt->GetSigOpCost();
            }

            assert(it->GetCountWithAncestors() == nCountCheck);
            assert(it->GetSizeWithAncestors() == nSizeCheck);
            assert(it->GetSigOpCostWithAncestors() == nSigOpCheck);
            assert(it->GetModFeesWithAncestors() == nFeesCheck);
            // Sanity check: we are walking in ascending ancestor count order.
            assert(prev_ancestor_count <= it->GetCountWithAncestors());
            prev_ancestor_count = it->GetCountWithAncestors();

            // Check children against mapNextTx
            TxMemPoolEntry::Children setChildrenCheck;
            auto iter = mapNextTx.lower_bound(OutPoint(it->GetTx().GetHash(), 0));
            uint64_t child_sizes = 0;
            for (; iter != mapNextTx.end() && iter->first->hash == it->GetTx().GetHash(); ++iter) {
                txiter childit = mapTx.find(iter->second->GetHash());
                assert(childit != mapTx.end()); // mapNextTx points to in-mempool transactions
                if (setChildrenCheck.insert(*childit).second) {
                    child_sizes += childit->GetTxSize();
                }
            }
            assert(setChildrenCheck.size() == it->GetMemPoolChildrenConst().size());
            assert(std::equal(setChildrenCheck.begin(), setChildrenCheck.end(), it->GetMemPoolChildrenConst().begin(), comp));
            // Also check to make sure size is greater than sum with immediate children.
            // just a sanity check, not definitive that this calc is correct...
            assert(it->GetSizeWithDescendants() >= child_sizes + it->GetTxSize());

            TxValidationState dummy_state; // Not used. CheckTxInputs() should always pass
            CAmount txfee = 0;
            assert(!tx.IsCoinBase());
            assert(consensus::CheckTxInputs(tx, dummy_state, mempoolDuplicate, spendheight, txfee));
            for (const auto& input: tx.vin) mempoolDuplicate.SpendCoin(input.prevout);
            AddCoins(mempoolDuplicate, tx, std::numeric_limits<int>::max());
        }
        for (auto it = mapNextTx.cbegin(); it != mapNextTx.cend(); it++) {
            uint256 hash = it->second->GetHash();
            IndexedTransactionSetConstIterator it2 = mapTx.find(hash);
            const CTransaction& tx = it2->GetTx();
            assert(it2 != mapTx.end());
            assert(&tx == it->second);
        }

        assert(totalTxSize == checkTotal);
        assert(m_total_fee == check_total_fee);
        assert(innerUsage == cachedInnerUsage);
        */
    }
    
    pub fn compare_depth_and_score(&mut self, 
        hasha: &u256,
        hashb: &u256,
        wtxid: Option<bool>) -> bool {

        let wtxid: bool = wtxid.unwrap_or(false);
        
        todo!();
        /*
            LOCK(cs);
        IndexedTransactionSetConstIterator i = wtxid ? get_iter_from_wtxid(hasha) : mapTx.find(hasha);
        if (i == mapTx.end()) return false;
        IndexedTransactionSetConstIterator j = wtxid ? get_iter_from_wtxid(hashb) : mapTx.find(hashb);
        if (j == mapTx.end()) return true;
        uint64_t counta = i->GetCountWithAncestors();
        uint64_t countb = j->GetCountWithAncestors();
        if (counta == countb) {
            return CompareTxMemPoolEntryByScore()(*i, *j);
        }
        return counta < countb;
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn get_sorted_depth_and_score(&self) -> Vec<TxMemPoolIndexedTransactionSetConstIterator> {
        
        todo!();
        /*
            std::vector<IndexedTransactionSetConstIterator> iters;
        AssertLockHeld(cs);

        iters.reserve(mapTx.size());

        for (IndexedTransactionSet::Iterator mi = mapTx.begin(); mi != mapTx.end(); ++mi) {
            iters.push_back(mi);
        }
        std::sort(iters.begin(), iters.end(), DepthAndScoreComparator());
        return iters;
        */
    }
    
    pub fn query_hashes(&self, vtxid: &mut Vec<u256>)  {
        
        todo!();
        /*
            LOCK(cs);
        auto iters = GetSortedDepthAndScore();

        vtxid.clear();
        vtxid.reserve(mapTx.size());

        for (auto it : iters) {
            vtxid.push_back(it->GetTx().GetHash());
        }
        */
    }
    
    pub fn info_all(&self) -> Vec<TxMemPoolInfo> {
        
        todo!();
        /*
            LOCK(cs);
        auto iters = GetSortedDepthAndScore();

        std::vector<TxMemPoolInfo> ret;
        ret.reserve(mapTx.size());
        for (auto it : iters) {
            ret.push_back(GetInfo(it));
        }

        return ret;
        */
    }
    
    pub fn get(&self, hash: &u256) -> TransactionRef {
        
        todo!();
        /*
            LOCK(cs);
        IndexedTransactionSetConstIterator i = mapTx.find(hash);
        if (i == mapTx.end())
            return nullptr;
        return i->GetSharedTx();
        */
    }
    
    pub fn info(&self, gtxid: &GenTxId) -> TxMemPoolInfo {
        
        todo!();
        /*
            LOCK(cs);
        IndexedTransactionSetConstIterator i = (gtxid.IsWtxid() ? get_iter_from_wtxid(gtxid.GetHash()) : mapTx.find(gtxid.GetHash()));
        if (i == mapTx.end())
            return TxMemPoolInfo();
        return GetInfo(i);
        */
    }
    
    /**
      | Affect CreateNewBlock prioritisation
      | of transactions
      |
      */
    pub fn prioritise_transaction(&mut self, 
        hash:        &u256,
        n_fee_delta: &Amount)  {
        
        todo!();
        /*
            {
            LOCK(cs);
            CAmount &delta = mapDeltas[hash];
            delta += nFeeDelta;
            txiter it = mapTx.find(hash);
            if (it != mapTx.end()) {
                mapTx.modify(it, update_fee_delta(delta));
                // Now update all ancestors' modified fees with descendants
                setEntries setAncestors;
                uint64_t nNoLimit = std::numeric_limits<uint64_t>::max();
                std::string dummy;
                CalculateMemPoolAncestors(*it, setAncestors, nNoLimit, nNoLimit, nNoLimit, nNoLimit, dummy, false);
                for (txiter ancestorIt : setAncestors) {
                    mapTx.modify(ancestorIt, update_descendant_state(0, nFeeDelta, 0));
                }
                // Now update all descendants' modified fees with ancestors
                setEntries setDescendants;
                CalculateDescendants(it, setDescendants);
                setDescendants.erase(it);
                for (txiter descendantIt : setDescendants) {
                    mapTx.modify(descendantIt, update_ancestor_state(0, nFeeDelta, 0, 0));
                }
                ++nTransactionsUpdated;
            }
        }
        LogPrintf("PrioritiseTransaction: %s fee += %s\n", hash.ToString(), FormatMoney(nFeeDelta));
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn apply_delta(&self, 
        hash:        &u256,
        n_fee_delta: &mut Amount)  {
        
        todo!();
        /*
            AssertLockHeld(cs);
        std::map<uint256, CAmount>::const_iterator pos = mapDeltas.find(hash);
        if (pos == mapDeltas.end())
            return;
        const CAmount &delta = pos->second;
        nFeeDelta += delta;
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn clear_prioritisation(&mut self, hash: &u256)  {
        
        todo!();
        /*
            AssertLockHeld(cs);
        mapDeltas.erase(hash);
        */
    }
    
    /**
      | Get the transaction in the pool that
      | spends the same prevout
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn get_conflict_tx(&self, prevout: &OutPoint) -> Arc<Transaction> {
        
        todo!();
        /*
            const auto it = mapNextTx.find(prevout);
        return it == mapNextTx.end() ? nullptr : it->second;
        */
    }
    
    /**
      | Returns an iterator to the given hash,
      | if found
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn get_iter(&self, txid: &u256) -> Option<TxMemPoolTxIter> {
        
        todo!();
        /*
            auto it = mapTx.find(txid);
        if (it != mapTx.end()) return it;
        return std::nullopt;
        */
    }
    
    /**
      | Translate a set of hashes into a set of
      | pool iterators to avoid repeated lookups
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn get_iter_set(&self, hashes: &HashSet<u256>) -> TxMemPoolSetEntries {
        
        todo!();
        /*
            TxMemPool::setEntries ret;
        for (const auto& h : hashes) {
            const auto mi = GetIter(h);
            if (mi) ret.insert(*mi);
        }
        return ret;
        */
    }
    
    /**
      | Check that none of this transactions
      | inputs are in the mempool, and thus the
      | tx is not dependent on other mempool
      | transactions to be included in a block.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn has_no_inputs_of(&self, tx: &Transaction) -> bool {
        
        todo!();
        /*
            for (unsigned int i = 0; i < tx.vin.size(); i++)
            if (exists(GenTxId::Txid(tx.vin[i].prevout.hash)))
                return false;
        return true;
        */
    }
    
    pub fn dynamic_memory_usage(&self) -> usize {
        
        todo!();
        /*
            LOCK(cs);
        // Estimate the overhead of mapTx to be 15 pointers + an allocation, as no exact formula for boost::multi_index_contained is implemented.
        return memusage::MallocUsage(sizeof(TxMemPoolEntry) + 15 * sizeof(c_void*)) * mapTx.size() + memusage::DynamicUsage(mapNextTx) + memusage::DynamicUsage(mapDeltas) + memusage::DynamicUsage(vTxHashes) + cachedInnerUsage;
        */
    }
    
    /**
      | Removes a transaction from the unbroadcast
      | set
      |
      */
    pub fn remove_unbroadcast_tx(&mut self, 
        txid:      &u256,
        unchecked: Option<bool>)  {

        let unchecked: bool = unchecked.unwrap_or(false);
        
        todo!();
        /*
            LOCK(cs);

        if (m_unbroadcast_txids.erase(txid))
        {
            LogPrint(BCLog::MEMPOOL, "Removed %i from set of unbroadcast txns%s\n", txid.GetHex(), (unchecked ? " before confirmation that txn was sent out" : ""));
        }
        */
    }
    
    /**
      | Remove a set of transactions from the
      | mempool.
      | 
      | If a transaction is in this set, then
      | all in-mempool descendants must also
      | be in the set, unless this transaction
      | is being removed for being in a block.
      | 
      | Set updateDescendants to true when
      | removing a tx that was in a block, so that
      | any in-mempool descendants have their
      | ancestor state updated.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn remove_staged(&mut self, 
        stage:              &mut TxMemPoolSetEntries,
        update_descendants: bool,
        reason:             MemPoolRemovalReason)  {
        
        todo!();
        /*
            AssertLockHeld(cs);
        UpdateForRemoveFromMempool(stage, updateDescendants);
        for (txiter it : stage) {
            removeUnchecked(it, reason);
        }
        */
    }
    
    /**
      | Expire all transaction (and their dependencies)
      | in the mempool older than time. Return
      | the number of removed transactions.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn expire(&mut self, time: Instant /* seconds */) -> i32 {
        
        todo!();
        /*
            AssertLockHeld(cs);
        IndexedTransactionSet::Index<EntryTime>::Iterator it = mapTx.get<entry_time>().begin();
        setEntries toremove;
        while (it != mapTx.get<entry_time>().end() && it->GetTime() < time) {
            toremove.insert(mapTx.project<0>(it));
            it++;
        }
        setEntries stage;
        for (txiter removeit : toremove) {
            CalculateDescendants(removeit, stage);
        }
        RemoveStaged(stage, false, MemPoolRemovalReason::EXPIRY);
        return stage.size();
        */
    }
    
    /**
      | addUnchecked must updated state for all
      | ancestors of a given transaction, to track
      | size/count of descendant transactions.
      | First version of addUnchecked can be used
      | to have it call
      | CalculateMemPoolAncestors(), and then
      | invoke the second version.
      |
      | Note that addUnchecked is ONLY called from
      | ATMP outside of tests and any other callers
      | may break wallet's in-mempool tracking (due
      | to lack of
      | CValidationInterface::TransactionAddedToMempool
      | callbacks).
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs, cs_main)]
    pub fn add_unchecked(&mut self, 
        entry:              &TxMemPoolEntry,
        valid_fee_estimate: Option<bool>)  {

        let valid_fee_estimate: bool = valid_fee_estimate.unwrap_or(true);
        
        todo!();
        /*
            setEntries setAncestors;
        uint64_t nNoLimit = std::numeric_limits<uint64_t>::max();
        std::string dummy;
        CalculateMemPoolAncestors(entry, setAncestors, nNoLimit, nNoLimit, nNoLimit, nNoLimit, dummy);
        return addUnchecked(entry, setAncestors, validFeeEstimate);
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn update_child(&mut self, 
        entry: TxMemPoolTxIter,
        child: TxMemPoolTxIter,
        add:   bool)  {
        
        todo!();
        /*
            AssertLockHeld(cs);
        TxMemPoolEntry::Children s;
        if (add && entry->GetMemPoolChildren().insert(*child).second) {
            cachedInnerUsage += memusage::IncrementalDynamicUsage(s);
        } else if (!add && entry->GetMemPoolChildren().erase(*child)) {
            cachedInnerUsage -= memusage::IncrementalDynamicUsage(s);
        }
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn update_parent(&mut self, 
        entry:  TxMemPoolTxIter,
        parent: TxMemPoolTxIter,
        add:    bool)  {
        
        todo!();
        /*
            AssertLockHeld(cs);
        TxMemPoolEntry::Parents s;
        if (add && entry->GetMemPoolParents().insert(*parent).second) {
            cachedInnerUsage += memusage::IncrementalDynamicUsage(s);
        } else if (!add && entry->GetMemPoolParents().erase(*parent)) {
            cachedInnerUsage -= memusage::IncrementalDynamicUsage(s);
        }
        */
    }
    
    /**
      | The minimum fee to get into the mempool,
      | which may itself not be enough for larger-sized
      | transactions.
      | 
      | The incrementalRelayFee policy variable
      | is used to bound the time it takes the
      | fee rate to go back down all the way to
      | 0. When the feerate would otherwise
      | be half of this, it is set to 0 instead.
      |
      */
    pub fn get_min_fee(&self, sizelimit: usize) -> FeeRate {
        
        todo!();
        /*
            LOCK(cs);
        if (!blockSinceLastRollingFeeBump || rollingMinimumFeeRate == 0)
            return CFeeRate(llround(rollingMinimumFeeRate));

        int64_t time = GetTime();
        if (time > lastRollingFeeUpdate + 10) {
            double halflife = ROLLING_FEE_HALFLIFE;
            if (DynamicMemoryUsage() < sizelimit / 4)
                halflife /= 4;
            else if (DynamicMemoryUsage() < sizelimit / 2)
                halflife /= 2;

            rollingMinimumFeeRate = rollingMinimumFeeRate / pow(2.0, (time - lastRollingFeeUpdate) / halflife);
            lastRollingFeeUpdate = time;

            if (rollingMinimumFeeRate < (double)incrementalRelayFee.GetFeePerK() / 2) {
                rollingMinimumFeeRate = 0;
                return CFeeRate(0);
            }
        }
        return std::max(CFeeRate(llround(rollingMinimumFeeRate)), incrementalRelayFee);
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn track_package_removed(&mut self, rate: &FeeRate)  {
        
        todo!();
        /*
            AssertLockHeld(cs);
        if (rate.GetFeePerK() > rollingMinimumFeeRate) {
            rollingMinimumFeeRate = rate.GetFeePerK();
            blockSinceLastRollingFeeBump = false;
        }
        */
    }
    
    /**
      | Remove transactions from the mempool
      | until its dynamic size is <= sizelimit.
      | pvNoSpendsRemaining, if set, will
      | be populated with the list of outpoints
      | which are not in mempool which no longer
      | have any spends in this mempool.
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn trim_to_size(&mut self, 
        sizelimit:              usize,
        pv_no_spends_remaining: Arc<Mutex<Vec<OutPoint>>>)  {
        
        todo!();
        /*
            AssertLockHeld(cs);

        unsigned nTxnRemoved = 0;
        CFeeRate maxFeeRateRemoved(0);
        while (!mapTx.empty() && DynamicMemoryUsage() > sizelimit) {
            IndexedTransactionSet::Index<DescendantScore>::Iterator it = mapTx.get<descendant_score>().begin();

            // We set the new mempool min fee to the feerate of the removed set, plus the
            // "minimum reasonable fee rate" (ie some value under which we consider txn
            // to have 0 fee). This way, we don't allow txn to enter mempool with feerate
            // equal to txn which were removed with no block in between.
            CFeeRate removed(it->GetModFeesWithDescendants(), it->GetSizeWithDescendants());
            removed += incrementalRelayFee;
            trackPackageRemoved(removed);
            maxFeeRateRemoved = std::max(maxFeeRateRemoved, removed);

            setEntries stage;
            CalculateDescendants(mapTx.project<0>(it), stage);
            nTxnRemoved += stage.size();

            std::vector<CTransaction> txn;
            if (pvNoSpendsRemaining) {
                txn.reserve(stage.size());
                for (txiter iter : stage)
                    txn.push_back(iter->GetTx());
            }
            RemoveStaged(stage, false, MemPoolRemovalReason::SIZELIMIT);
            if (pvNoSpendsRemaining) {
                for (const CTransaction& tx : txn) {
                    for (const CTxIn& txin : tx.vin) {
                        if (exists(GenTxId::Txid(txin.prevout.hash))) continue;
                        pvNoSpendsRemaining->push_back(txin.prevout);
                    }
                }
            }
        }

        if (maxFeeRateRemoved > CFeeRate(0)) {
            LogPrint(BCLog::MEMPOOL, "Removed %u txn, rolling minimum fee bumped to %s\n", nTxnRemoved, maxFeeRateRemoved.ToString());
        }
        */
    }
    
    #[EXCLUSIVE_LOCKS_REQUIRED(cs)]
    pub fn calculate_descendant_maximum(&self, entry: TxMemPoolTxIter) -> u64 {
        
        todo!();
        /*
            // find parent with highest descendant count
        std::vector<txiter> candidates;
        setEntries counted;
        candidates.push_back(entry);
        uint64_t maximum = 0;
        while (candidates.size()) {
            txiter candidate = candidates.back();
            candidates.pop_back();
            if (!counted.insert(candidate).second) continue;
            const TxMemPoolEntry::Parents& parents = candidate->GetMemPoolParentsConst();
            if (parents.size() == 0) {
                maximum = std::max(maximum, candidate->GetCountWithDescendants());
            } else {
                for (const TxMemPoolEntry& i : parents) {
                    candidates.push_back(mapTx.iterator_to(i));
                }
            }
        }
        return maximum;
        */
    }
    
    /**
      | Calculate the ancestor and descendant
      | count for the given transaction.
      | 
      | The counts include the transaction
      | itself.
      | 
      | When ancestors is non-zero (ie, the
      | transaction itself is in the mempool),
      | ancestorsize and ancestorfees will
      | also be set to the appropriate values.
      |
      */
    pub fn get_transaction_ancestry(&self, 
        txid:         &u256,
        ancestors:    &mut usize,
        descendants:  &mut usize,
        ancestorsize: *mut usize,
        ancestorfees: *mut Amount)  {
        
        todo!();
        /*
            LOCK(cs);
        auto it = mapTx.find(txid);
        ancestors = descendants = 0;
        if (it != mapTx.end()) {
            ancestors = it->GetCountWithAncestors();
            if (ancestorsize) *ancestorsize = it->GetSizeWithAncestors();
            if (ancestorfees) *ancestorfees = it->GetModFeesWithAncestors();
            descendants = CalculateDescendantMaximum(it);
        }
        */
    }
    
    /**
      | @return
      | 
      | true if the mempool is fully loaded
      |
      */
    pub fn is_loaded(&self) -> bool {
        
        todo!();
        /*
            LOCK(cs);
        return m_is_loaded;
        */
    }
    
    /**
      | Sets the current loaded state
      |
      */
    pub fn set_is_loaded(&mut self, loaded: bool)  {
        
        todo!();
        /*
            LOCK(cs);
        m_is_loaded = loaded;
        */
    }
}

pub fn get_info(it: TxMemPoolIndexedTransactionSetConstIterator) -> TxMemPoolInfo {
    
    todo!();
        /*
            return TxMemPoolInfo{it->GetSharedTx(), it->GetTime(), it->GetFee(), it->GetTxSize(), it->GetModifiedFee() - it->GetFee()};
        */
}
