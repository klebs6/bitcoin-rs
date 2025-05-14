crate::ix!();

pub const BLOCK_INDEX_N_MEDIAN_TIME_SPAN: i32 = 11;

/**
  | Turn the lowest '1' bit in the binary
  | representation of a number into a '0'.
  |
  */
#[inline] pub fn invert_lowest_one(n: i32) -> i32 {
    
    n & (n - 1)
}

/**
  | Compute what height to jump back to with
  | the CBlockIndex::pskip pointer.
  |
  */
#[inline] pub fn get_skip_height(height: i32) -> i32 {
    
    if height < 2 {
        return 0;
    }

    // Determine which height to jump back to. Any
    // number strictly lower than height is
    // acceptable, but the following expression
    // seems to perform well in simulations (max
    // 110 steps to go back up to 2**18 blocks).
    match (height & 1) != 0 {
        true   => invert_lowest_one(invert_lowest_one(height - 1)) + 1,
        false  => invert_lowest_one(height)
    }
}

/**
  | The block chain is a tree shaped structure
  | starting with the genesis block at the
  | root, with each block potentially having
  | multiple candidates to be the next block.
  | A blockindex may have multiple pprev
  | pointing to it, but at most one of them
  | can be part of the currently active branch.
  |
  */
#[derive(PartialEq,Eq,Clone,Debug)]
pub struct BlockIndex {

    /**
      | pointer to the hash of the block, if any.
      | Memory is owned by this CBlockIndex
      |
      */
    pub phash_block:  Option<u256>, 

    /**
      | pointer to the index of the predecessor
      | of this block
      |
      */
    pub pprev:        Option<Arc<BlockIndex>>,  

    //note: it looks like pprev and pskip need to
    //stay as Option<Arc<BlockIndex>>
    //
    //In other words, we cant go to Amo

    /**
      | pointer to the index of some further
      | predecessor of this block
      |
      */
    pub pskip:        Option<Arc<BlockIndex>>, 

    /**
      | height of the entry in the chain. The
      | genesis block has height 0
      |
      */
    pub n_height:     i32, 

    /**
      | Which # file this block is stored in (blk?????.dat)
      |
      */
    pub n_file:       i32, 

    /**
      | Byte offset within blk?????.dat where
      | this block's data is stored
      |
      */
    pub n_data_pos:   u32, 

    /**
      | Byte offset within rev?????.dat where
      | this block's undo data is stored
      |
      */
    pub n_undo_pos:   u32, 

    /**
      | (memory only) Total amount of work (expected
      | number of hashes) in the chain up to and
      | including this block
      |
      */
    pub n_chain_work: ArithU256,

    /**
      | Number of transactions in this block.
      |
      | Note: in a potential headers-first mode,
      | this number cannot be relied upon
      |
      | Note: this value is faked during UTXO
      | snapshot load to ensure that
      | LoadBlockIndex() will load index entries
      | for blocks that we lack data for.
      |
      | @sa ActivateSnapshot
      */
    pub n_tx: u32, 

    /**
      | (memory only) Number of transactions in the
      | chain up to and including this block.
      |
      | This value will be non-zero only if and
      | only if transactions for this block and all
      | its parents are available.
      |
      | Change to 64-bit type before 2024 (assuming
      | worst case of 60 byte transactions).
      |
      | Note: this value is faked during use of
      | a UTXO snapshot because we don't have the
      | underlying block data available during
      | snapshot load.
      |
      | @sa AssumeutxoData @sa ActivateSnapshot
      */
    pub n_chain_tx:       u32, 

    /**
      | Verification status of this block. See enum
      | BlockStatus
      |
      | Note: this value is modified to show
      | BLOCK_OPT_WITNESS during UTXO snapshot load
      | to avoid the block index being spuriously
      | rewound.
      |
      | @sa NeedsRedownload @sa ActivateSnapshot
      */
    pub n_status:         u32, 

    /**
      | block header
      |
      */
    pub n_version:        i32, 
    pub hash_merkle_root: u256,
    pub n_time:           u32, 
    pub n_bits:           u32, 
    pub n_nonce:          u32, 

    /**
      | (memory only) Sequential id assigned
      | to distinguish order in which blocks
      | are received.
      |
      */
    pub n_sequence_id:    i32, 

    /**
      | (memory only) Maximum nTime in the chain
      | up to and including this block.
      |
      */
    pub n_time_max:       u32, 
}

unsafe impl Send for BlockIndex {}
unsafe impl Sync for BlockIndex {}

pub type BlockIndexRef = Option<Arc<BlockIndex>>;

impl Default for BlockIndex {
    fn default() -> Self {
        Self {
            phash_block:      None,
            pprev:            None,
            pskip:            None,
            n_height:         0,
            n_file:           0,
            n_data_pos:       0,
            n_undo_pos:       0,
            n_chain_work:     ArithU256::default(),
            n_tx:             0,
            n_chain_tx:       0,
            n_status:         0,
            n_version:        0,
            hash_merkle_root: u256::default(),
            n_time:           0,
            n_bits:           0,
            n_nonce:          0,
            n_sequence_id:    0,
            n_time_max:       0,
        }
    }
}

impl BlockIndex {

    pub fn new(block: &BlockHeader) -> Self {
    
        Self {
            n_version:        block.n_version,
            hash_merkle_root: block.hash_merkle_root.clone(),
            n_time:           block.n_time,
            n_bits:           block.n_bits,
            n_nonce:          block.n_nonce,
            ..Default::default()
        }
    }
    
    pub fn get_block_pos(&self) -> FlatFilePos {
        
        let mut ret = FlatFilePos::default();

        if (self.n_status & BlockStatus::BLOCK_HAVE_DATA.bits()) != 0 {
            ret.n_file = self.n_file;
            ret.n_pos  = self.n_data_pos;
        }

        ret
    }
    
    pub fn get_undo_pos(&self) -> FlatFilePos {
        
        let mut ret = FlatFilePos::default();

        if (self.n_status & BlockStatus::BLOCK_HAVE_UNDO.bits()) != 0 {
            ret.n_file = self.n_file;
            ret.n_pos  = self.n_undo_pos;
        }

        ret
    }
    
    pub fn get_block_header(&self) -> BlockHeader {
        
        let mut block = BlockHeader::default();

        block.n_version = self.n_version;

        if self.pprev.is_some() {
            block.hash_prev_block = unsafe { 
                self.pprev.as_ref().unwrap().get_block_hash() 
            };
        }

        block.hash_merkle_root = self.hash_merkle_root.clone();
        block.n_time           = self.n_time;
        block.n_bits           = self.n_bits;
        block.n_nonce          = self.n_nonce;

        block
    }
    
    pub fn get_block_hash(&self) -> u256 {
        
        self.phash_block.as_ref().unwrap().clone()
    }

    /**
      | Check whether this block's and all previous
      | blocks' transactions have been downloaded
      | (and stored to disk) at some point.
      | 
      | Does not imply the transactions are
      | consensus-valid (ConnectTip might
      | fail)
      | 
      | Does not imply the transactions are
      | still stored on disk. (IsBlockPruned
      | might return true)
      |
      */
    pub fn have_txs_downloaded(&self) -> bool {
        
        self.n_chain_tx != 0
    }
    
    pub fn get_block_time(&self) -> i64 {
        
        self.n_time as i64
    }
    
    pub fn get_block_time_max(&self) -> i64 {
        
        self.n_time_max as i64
    }
    
    pub fn get_median_time_past(self: Arc<Self>) -> i64 {

        unsafe {

            let mut pmedian = [0_i64; BLOCK_INDEX_N_MEDIAN_TIME_SPAN as usize];

            let mut pbegin:  *mut i64 = pmedian.as_mut_ptr().offset(BLOCK_INDEX_N_MEDIAN_TIME_SPAN as isize);
            let pend:        *mut i64 = pmedian.as_mut_ptr().offset(BLOCK_INDEX_N_MEDIAN_TIME_SPAN as isize);

            let mut pindex: Option<Arc<BlockIndex>> = Some(self.clone());

            let mut i: i32 = 0;

            while i < BLOCK_INDEX_N_MEDIAN_TIME_SPAN && pindex.is_some() {

                pbegin = pbegin.offset(-1);

                *pbegin = pindex.as_ref().unwrap().get_block_time();

                i += 1;

                pindex = Some(pindex.as_ref().unwrap().pprev.clone().unwrap());
            }

            let len: usize = pend.offset_from(pbegin).try_into().unwrap();

            let mut slice = std::slice::from_raw_parts_mut(pbegin, len);

            slice.sort();

            let idx: usize = ((pend.offset_from(pbegin)) / 2)
                .try_into()
                .unwrap();

            slice[idx]
        }
    }
    
    pub fn to_string(&self) -> String {
        
        format!(
            "BlockIndex(pprev={:?}, nHeight={:?}, merkle={:?}, hashBlock={:?})",
            self.pprev,
            self.n_height,
            self.hash_merkle_root,
            self.get_block_hash()
        )
    }

    /**
      | Check whether this block index entry
      | is valid up to the passed validity level.
      |
      */
    pub fn is_valid(&self, n_up_to: Option<BlockStatus>) -> bool {

        let n_up_to: BlockStatus 
        = n_up_to.unwrap_or(BlockStatus::BLOCK_VALID_TRANSACTIONS);

        //  Only validity flags allowed.
        assert!((n_up_to.bits() & !BlockStatus::BLOCK_VALID_MASK.bits()) == 0);

        if (self.n_status & BlockStatus::BLOCK_FAILED_MASK.bits()) != 0 {
            return false;
        }

        ((self.n_status & BlockStatus::BLOCK_VALID_MASK.bits()) >= n_up_to.bits())
    }

    /**
      | @returns true if the block is
      |   assumed-valid; this means it is queued
      |   to be validated by a background
      |   chainstate.
      */
    pub fn is_assumed_valid(&self) -> bool {
        
        (self.n_status & BlockStatus::BLOCK_ASSUMED_VALID.bits()) != 0
    }

    /**
      | Raise the validity level of this block
      | index entry.
      |
      | Returns true if the validity was changed.
      */
    pub fn raise_validity(&mut self, n_up_to: BlockStatus) -> bool {

        //  Only validity flags allowed.
        assert!((n_up_to & !BlockStatus::BLOCK_VALID_MASK).bits() == 0);

        if (self.get_n_status() & BlockStatus::BLOCK_FAILED_MASK).bits() != 0 {
            return false;
        }

        if (self.get_n_status() & BlockStatus::BLOCK_VALID_MASK) < n_up_to {

            // If this block had been marked
            // assumed-valid and we're raising its
            // validity to a certain point, there
            // is no longer an assumption.
            if (self.n_status & BlockStatus::BLOCK_ASSUMED_VALID.bits()) != 0 
            && n_up_to >= BlockStatus::BLOCK_VALID_SCRIPTS {
                self.n_status &= !BlockStatus::BLOCK_ASSUMED_VALID.bits();
            }

            self.n_status = 
                ((self.get_n_status() & !BlockStatus::BLOCK_VALID_MASK) | n_up_to).bits();

            return true;
        }

        false
    }

    pub fn get_n_status(&self) -> BlockStatus {
        BlockStatus::from_bits(self.n_status).unwrap()
    }

    /**
      | Efficiently find an ancestor of this
      | block.
      |
      */
    pub fn get_ancestor(self: Arc<Self>, height: i32) -> Option<Arc<BlockIndex>> {

        if height > self.n_height || height < 0 {
            return None;
        }
        
        let mut pindex_walk: Option<Arc<BlockIndex>> = Some(self.clone());

        let mut height_walk: i32 = self.n_height;

        while height_walk > height{

            let height_skip:      i32 = get_skip_height(height_walk);
            let height_skip_prev: i32 = get_skip_height(height_walk - 1);

            unsafe {

                if pindex_walk.as_ref().unwrap().pskip.is_some()
                && (
                    height_skip == height 
                    || (height_skip > height && !(height_skip_prev < height_skip - 2 && height_skip_prev >= height))
                ) {

                    // Only follow pskip if
                    // pprev->pskip isn't better
                    // than pskip->pprev.
                    pindex_walk = Some(
                        pindex_walk.as_ref().unwrap().pskip.as_ref().unwrap().clone()
                    );

                    height_walk = height_skip;

                } else {

                    assert!(pindex_walk.as_ref().unwrap().pprev.is_some());

                    let pprev = pindex_walk.as_ref().unwrap().pprev.clone();

                    pindex_walk = Some(pprev.unwrap().clone());

                    {
                        let old = height_walk;
                        height_walk -= 1;
                        old
                    };
                }
            }
        }

        pindex_walk
    }
    
    /**
      | Build the skiplist pointer for this
      | entry.
      |
      */
    pub fn build_skip(&mut self)  {
        
        if let Some(ref pprev) = self.pprev {

            let pprev = pprev.clone();

            let skip_height = get_skip_height(self.n_height);

            self.pskip = pprev.get_ancestor(skip_height);
        }
    }
}

pub fn get_block_proof(block: &BlockIndex) -> ArithU256 {
    
    todo!();
        /*
            ArithU256 bnTarget;
        bool fNegative;
        bool fOverflow;
        bnTarget.SetCompact(block.nBits, &fNegative, &fOverflow);
        if (fNegative || fOverflow || bnTarget == 0)
            return 0;
        // We need to compute 2**256 / (bnTarget+1), but we can't represent 2**256
        // as it's too large for an ArithU256. However, as 2**256 is at least as large
        // as bnTarget+1, it is equal to ((2**256 - bnTarget - 1) / (bnTarget+1)) + 1,
        // or ~bnTarget / (bnTarget+1) + 1.
        return (~bnTarget / (bnTarget + 1)) + 1;
        */
}

/**
  | Find the last common ancestor two blocks
  | have.
  | 
  | Both pa and pb must be non-nullptr.
  |
  | ie, find the forking point between two chain
  | tips.
  |
  */
pub fn last_common_ancestor(
    pa: Option<Arc<BlockIndex>>,
    pb: Option<Arc<BlockIndex>>

) -> Option<Arc<BlockIndex>> {
    
    todo!();
        /*
            if (pa->nHeight > pb->nHeight) {
            pa = pa->GetAncestor(pb->nHeight);
        } else if (pb->nHeight > pa->nHeight) {
            pb = pb->GetAncestor(pa->nHeight);
        }

        while (pa != pb && pa && pb) {
            pa = pa->pprev;
            pb = pb->pprev;
        }

        // Eventually all chain branches meet at the genesis block.
        assert(pa == pb);
        return pa;
        */
}
