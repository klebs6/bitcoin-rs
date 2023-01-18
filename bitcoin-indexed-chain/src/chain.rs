crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/chain.h]

//-------------------------------------------[.cpp/bitcoin/src/interfaces/chain.h]

/**
  | An in-memory indexed chain of blocks.
  |
  */
pub struct Chain {
    chain: Vec<Option<Arc<BlockIndex>>>,
}

impl Index<i32> for Chain {
    type Output = Option<Arc<BlockIndex>>;
    
    /**
      | Returns the index entry at a particular
      | height in this chain, or nullptr if no
      | such height exists.
      |
      */
    #[inline] fn index(&self, n_height: i32) -> &Self::Output {
        todo!();
        /*
            if (nHeight < 0 || nHeight >= (int)vChain.size())
                return nullptr;
            return vChain[nHeight];
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/chain.cpp]
impl Tip for Chain {

    fn tip(&self) -> Option<Arc<BlockIndex>> {
        
        todo!();
        /*
            return vChain.size() > 0 ? vChain[vChain.size() - 1] : nullptr;
        */
    }
}

impl Contains<Arc<BlockIndex>> for Chain {

    /**
      | Efficiently check whether a block is
      | present in this chain.
      |
      */
    fn contains(&self, pindex: Option<Arc<BlockIndex>>) -> bool {
        
        todo!();
        /*
            return (*this)[pindex->nHeight] == pindex;
        */
    }
}

impl GetLocator<Arc<BlockIndex>> for Chain {

    type LocatorType = BlockLocator;

    /**
      | Return a CBlockLocator that refers
      | to a block in this chain (by default the
      | tip).
      |
      */
    fn get_locator(&self, pindex: Option<Arc<BlockIndex>>) -> Self::LocatorType {
        
        todo!();
        /*
            int nStep = 1;
        std::vector<uint256> vHave;
        vHave.reserve(32);

        if (!pindex)
            pindex = Tip();
        while (pindex) {
            vHave.push_back(pindex->GetBlockHash());
            // Stop when we have added the genesis block.
            if (pindex->nHeight == 0)
                break;
            // Exponentially larger steps back, plus the genesis block.
            int nHeight = std::max(pindex->nHeight - nStep, 0);
            if (Contains(pindex)) {
                // Use O(1) CChain index if possible.
                pindex = (*this)[nHeight];
            } else {
                // Otherwise, use O(log n) skiplist.
                pindex = pindex->GetAncestor(nHeight);
            }
            if (vHave.size() > 10)
                nStep *= 2;
        }

        return CBlockLocator(vHave);
        */
    }
}

impl ChainHeight for Chain {

    /**
      | Return the maximal height in the chain.
      | Is equal to chain.Tip() ? chain.Tip()->nHeight
      | : -1.
      |
      */
    fn height(&self) -> Option<usize> {
        
        todo!();
        /*
            return vChain.size() - 1;
        */
    }
}

impl ChainNext for Chain {

    /**
      | Find the successor of a block in this
      | chain, or nullptr if the given index
      | is not found or is the tip.
      |
      */
    fn next(&self, pindex: Option<Arc<BlockIndex>>) -> Option<Arc<BlockIndex>> {
        
        todo!();
        /*
            if (Contains(pindex))
                return (*this)[pindex->nHeight + 1];
            else
                return nullptr;
        */
    }
}

impl Chain {

    /**
      | Returns the index entry for the genesis
      | block of this chain, or nullptr if none.
      |
      */
    pub fn genesis(&self) -> Option<Arc<BlockIndex>> {
        
        todo!();
        /*
            return vChain.size() > 0 ? vChain[0] : nullptr;
        */
    }

    /**
      | Set/initialize a chain with a given
      | tip.
      |
      */
    pub fn set_tip(&mut self, pindex: Option<Arc<BlockIndex>>)  {
        
        todo!();
        /*
            if (pindex == nullptr) {
            vChain.clear();
            return;
        }
        vChain.resize(pindex->nHeight + 1);
        while (pindex && vChain[pindex->nHeight] != pindex) {
            vChain[pindex->nHeight] = pindex;
            pindex = pindex->pprev;
        }
        */
    }
    
    
    /**
      | Find the last common block between this
      | chain and a block index entry.
      |
      */
    pub fn find_fork(&self, pindex: Option<Arc<BlockIndex>>) -> Option<Arc<BlockIndex>> {
        
        todo!();
        /*
            if (pindex == nullptr) {
            return nullptr;
        }
        if (pindex->nHeight > Height())
            pindex = pindex->GetAncestor(Height());
        while (pindex && !Contains(pindex))
            pindex = pindex->pprev;
        return pindex;
        */
    }
    
    /**
      | Find the earliest block with timestamp
      | equal or greater than the given time
      | and height equal or greater than the
      | given height.
      |
      */
    pub fn find_earliest_at_least(&self, 
        n_time: i64,
        height: i32) -> Option<Arc<BlockIndex>> {
        
        todo!();
        /*
            std::pair<int64_t, int> blockparams = std::make_pair(nTime, height);
        std::vector<CBlockIndex*>::const_iterator lower = std::lower_bound(vChain.begin(), vChain.end(), blockparams,
            [](CBlockIndex* pBlock, const std::pair<int64_t, int>& blockparams) -> bool { return pBlock->GetBlockTimeMax() < blockparams.first || pBlock->nHeight < blockparams.second; });
        return (lower == vChain.end() ? nullptr : *lower);
        */
    }
}

#[EXCLUSIVE_LOCKS_REQUIRED(cs_main)]
pub fn next_sync_block(
        pindex_prev: *const BlockIndex,
        chain:       &mut Chain) -> *const BlockIndex {
    
    todo!();
        /*
            AssertLockHeld(cs_main);

        if (!pindex_prev) {
            return chain.Genesis();
        }

        const CBlockIndex* pindex = chain.Next(pindex_prev);
        if (pindex) {
            return pindex;
        }

        return chain.Next(chain.FindFork(pindex_prev));
        */
}

/**
  | Return implementation of Chain interface.
  |
  */
pub fn make_chain(context: &mut NodeContext) -> Box<Chain> {
    
    todo!();
        /*
            return std::make_unique<ChainImpl>(context);
        */
}
