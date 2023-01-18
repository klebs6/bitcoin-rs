crate::ix!();

/**
  | Blocks that are in flight, and that are
  | in the queue to be downloaded.
  |
  */
#[derive(PartialEq)]
pub struct QueuedBlock {

    /**
      | BlockIndex. We must have this since
      | we only request blocks when we've already
      | validated the header.
      |
      */
    pub pindex:        Option<Arc<BlockIndex>>,


    /**
      | Optional, used for CMPCTBLOCK downloads
      |
      */
    pub partial_block: Amo<PartiallyDownloadedBlock>,
}

#[derive(Derivative)]
#[derivative(PartialEq)]
pub struct PartiallyDownloadedBlock {
    pub txn_available:   Vec<TransactionRef>,
    pub prefilled_count: usize, // default = 0
    pub mempool_count:   usize, // default = 0
    pub extra_count:     usize, // default = 0

    #[derivative(PartialEq="ignore")]
    pub pool:            Arc<TxMemPool>,

    pub header:          BlockHeader,
}

impl PartiallyDownloadedBlock {

    pub fn new(pool_in: Amo<TxMemPool>) -> Self {
    
        todo!();
        /*
        : pool(poolIn),
        */
    }

    /**
      | extra_txn is a list of extra transactions
      | to look at, in <witness hash, reference>
      | form
      */
    pub fn init_data(&mut self, 
        cmpctblock: &BlockHeaderAndShortTxIDs,
        extra_txn:  &Vec<Option<(u256,TransactionRef)>>) -> ReadStatus {
        
        if cmpctblock.header.is_null() 
        || (cmpctblock.shorttxids.is_empty() && cmpctblock.prefilledtxn.is_empty()) 
        {
            return ReadStatus::Invalid;
        }

        if cmpctblock.shorttxids.len() + cmpctblock.prefilledtxn.len() > MAX_BLOCK_WEIGHT / MIN_SERIALIZABLE_TRANSACTION_WEIGHT {
            return ReadStatus::Invalid;
        }

        assert!(self.header.is_null() && self.txn_available.is_empty());

        self.header = cmpctblock.header.clone();
        self.txn_available.resize(cmpctblock.block_tx_count(), Amo::<Transaction>::none());

        let mut lastprefilledindex: i32 = -1;

        for i in 0..cmpctblock.prefilledtxn.len() {

            if cmpctblock.prefilledtxn[i].tx.is_none() {
                return ReadStatus::Invalid;
            }

            // index is a uint16_t, so can't overflow here
            lastprefilledindex += cmpctblock.prefilledtxn[i].index as i32 + 1;

            if lastprefilledindex > u16::MAX.into() {
                return ReadStatus::Invalid;
            }

            if lastprefilledindex as u32 > (cmpctblock.shorttxids.len() + i).try_into().unwrap() {

                // If we are inserting a tx at an
                // index greater than our full
                // list of shorttxids plus the
                // number of prefilled txn we've
                // inserted, then we have txn for
                // which we have neither
                // a prefilled txn or a shorttxid!
                return ReadStatus::Invalid;
            }

            self.txn_available[usize::try_from(lastprefilledindex).unwrap()] = cmpctblock.prefilledtxn[i].tx.clone();
        }

        self.prefilled_count = cmpctblock.prefilledtxn.len();

        // Calculate map of txids -> positions and
        // check mempool to see what we have (or
        // don't)
        //
        // Because well-formed cmpctblock messages
        // will have a (relatively) uniform
        // distribution of short IDs, any
        // highly-uneven distribution of elements
        // can be safely treated as
        // a ReadStatus::Failed.
        let mut shorttxids: HashMap::<u64,u16> 
        = HashMap::<u64,u16>::with_capacity(cmpctblock.shorttxids.len());

        let mut index_offset: u16 = 0;

        for i in 0_usize..cmpctblock.shorttxids.len() {

            while self.txn_available[i + index_offset as usize].is_some() {
                index_offset += 1;
            }

            shorttxids
                .get_mut(&cmpctblock.shorttxids[i])
                .replace(&mut u16::try_from(i + index_offset as usize).unwrap());

            //  To determine the chance that the
            //  number of entries in a bucket
            //  exceeds N, we use the fact that
            //  the number of elements in a single
            //  bucket is binomially distributed
            //  (with n = the number of shorttxids
            //  S, and p =
            //  1 / the number of buckets), that
            //  in the worst case the number of
            //  buckets is equal to S (due to
            //  std::unordered_map having
            //  a default load factor of 1.0), and
            //  that the chance for any bucket to
            //  exceed N elements is at most
            //  buckets * (the chance that any
            //  given bucket is above N elements).
            //
            //  Thus: P(max_elements_per_bucket
            //  > N) <= S * (1
            //  - cdf(binomial(n=S,p=1/S), N)).
            //
            //  If we assume blocks of up to
            //  16000, allowing 12 elements per
            //  bucket should only fail once per
            //  ~1 million block transfers (per
            //  peer and connection).
            #[cfg(peerman_check_hashbucket_distribution)]
            if shorttxids.bucket_size(shorttxids.bucket(cmpctblock.shorttxids[i])) > 12 {
                return ReadStatus::Failed;
            }
        }

        //  TODO: in the shortid-collision case, we should instead request both transactions
        //  which collided. Falling back to full-block-request here is overkill.
        if shorttxids.len() != cmpctblock.shorttxids.len() {
            //  Short ID collision
            return ReadStatus::Failed;
        }

        let mut have_txn: Vec<bool> = Vec::<bool>::with_capacity(self.txn_available.len());

        {
            let mut guard = self.pool.cs.lock();

            for i in 0..guard.tx_hashes.len() {

                let shortid: u64 = cmpctblock.get_shortid(&guard.tx_hashes[i].0);

                let idit = shorttxids.get(&shortid);

                if let Some(idit) = idit {

                    let idit: usize = *idit as usize;

                    if !have_txn[idit] {

                        self.txn_available[idit] 
                            = (*guard.tx_hashes[i].1).get_shared_tx();

                        have_txn[idit] = true;

                        self.mempool_count += 1;

                    } else {

                        // If we find two mempool
                        // txn that match the
                        // short id, just request
                        // it.
                        //
                        // This should be rare
                        // enough that the extra
                        // bandwidth doesn't
                        // matter, but eating
                        // a round-trip due to
                        // FillBlock failure would
                        // be annoying
                        if self.txn_available[idit].is_some() {

                            self.txn_available[idit] = TransactionRef::none();

                            self.mempool_count -= 1;
                        }
                    }
                }

                // Though ideally we'd continue
                // scanning for the
                // two-txn-match-shortid case, the
                // performance win of an early
                // exit here is too good to pass
                // up and worth the extra risk.
                if self.mempool_count == shorttxids.len() {
                    break;
                }
            }
        }

        for i in 0..extra_txn.len() {

            let hash = &extra_txn[i].as_ref().unwrap().0;

            let shortid: u64 = cmpctblock.get_shortid(hash);

            let idit = shorttxids.get(&shortid);

            if let Some(idit) = idit {

                let idit: usize = *idit as usize;

                if !have_txn[idit] {

                    self.txn_available[idit] = extra_txn[i].as_ref().unwrap().1.clone();

                    have_txn[idit] = true;

                    self.mempool_count += 1;
                    self.extra_count   += 1;

                } else {

                    //  If we find two
                    //  mempool/extra txn that
                    //  match the short id, just
                    //  request it.
                    //
                    //  This should be rare enough
                    //  that the extra bandwidth
                    //  doesn't matter, but eating
                    //  a round-trip due to
                    //  FillBlock failure would be
                    //  annoying
                    //
                    //  Note that we don't want
                    //  duplication between
                    //  extra_txn and mempool to
                    //  trigger this case, so we
                    //  compare witness hashes
                    //  first
                    if self.txn_available[idit].is_some() 
                    && self.txn_available[idit].get().get_witness_hash() 
                        != extra_txn[i].as_ref().unwrap().1.get().get_witness_hash() {

                        self.txn_available[idit] = TransactionRef::none();

                        self.mempool_count -= 1;
                        self.extra_count -= 1;
                    }
                }
            }

            // Though ideally we'd continue
            // scanning for the
            // two-txn-match-shortid case, the
            // performance win of an early exit
            // here is too good to pass up and
            // worth the extra risk.
            if self.mempool_count == shorttxids.len() {
                break;
            }
        }

        log_print!(
            LogFlags::CMPCTBLOCK, 
            "Initialized PartiallyDownloadedBlock for block %s using a cmpctblock of size %lu\n", 
            cmpctblock.header().get_hash().to_string(), 
            get_serialize_size(cmpctblock,PROTOCOL_VERSION)
        );

        ReadStatus::Ok
    }
    
    pub fn is_tx_available(&self, index: usize) -> bool {
        
        assert!(!self.header.is_null());
        assert!(index < self.txn_available.len());

        self.txn_available[index].is_some()
    }
    
    pub fn fill_block(&mut self, 
        block:       &mut Block,
        vtx_missing: &Vec<TransactionRef>) -> ReadStatus {
        
        assert!(!self.header.is_null());

        let hash: u256 = self.header.get_hash();

        block.header = self.header.clone();

        block.vtx.resize(self.txn_available.len(), TransactionRef::none());

        let mut tx_missing_offset: usize = 0;

        for i in 0..self.txn_available.len() {

            if self.txn_available[i].is_none() {

                if vtx_missing.len() <= tx_missing_offset {
                    return ReadStatus::Invalid;
                }

                block.vtx[i] = vtx_missing[{
                    let old = tx_missing_offset;
                    tx_missing_offset += 1;
                    old
                }].clone();

            } else {
                block.vtx[i] = self.txn_available.remove(i);
            }
        }

        // Make sure we can't call FillBlock again.
        self.header.set_null();

        self.txn_available.clear();

        if vtx_missing.len() != tx_missing_offset {
            return ReadStatus::Invalid;
        }

        let mut state = BlockValidationState::default();

        if !check_block(
            block,
            &mut state,
            &params().get_consensus(), 
            None, 
            None) 
        {
            // TODO: We really want to just check
            // merkle tree manually here, but that
            // is expensive, and CheckBlock caches
            // a block's "checked-status" (in the
            // CBlock?). CBlock should be able to
            // check its own merkle root and cache
            // that check.
            if state.get_result() == BlockValidationResult::BLOCK_MUTATED {
                //  Possible Short ID collision
                return ReadStatus::Failed;
            }

            return ReadStatus::CheckBlockFailed;
        }

        log_print!(
            BCLog::CMPCTBLOCK, 
            "Successfully reconstructed block %s with %lu txn prefilled, 
                %lu txn from mempool (incl at least %lu from extra pool) 
                and %lu txn requested\n", 
            hash.to_string(), 
            prefilled_count, 
            mempool_count, 
            extra_count, 
            vtx_missing.len()
        );

        if vtx_missing.len() < 5 {

            for tx in vtx_missing.iter() {

                log_print!(
                    BCLog::CMPCTBLOCK, 
                    "Reconstructed block %s required tx %s\n", 
                    hash.to_string(), 
                    (*tx).get_hash().to_string()
                );
            }
        }

        ReadStatus::Ok
    }
}
