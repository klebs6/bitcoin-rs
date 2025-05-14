// ---------------- [ File: bitcoin-txmempool/src/versionbits.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/versionbits.h]
//-------------------------------------------[.cpp/bitcoin/src/versionbits.cpp]

/**
  | What block version to use for new blocks
  | (pre versionbits)
  |
  */
pub const VERSIONBITS_LAST_OLD_BLOCK_VERSION: u32 = 4;

/**
  | What bits to set in version for versionbits
  | blocks
  |
  */
pub const VERSIONBITS_TOP_BITS: u32 = 0x20000000;

/**
  | What bitmask determines whether versionbits
  | is in use
  |
  */
pub const VERSIONBITS_TOP_MASK: u32 = 0xE0000000;

/**
  | Total bits available for versionbits
  |
  */
pub const VERSIONBITS_NUM_BITS: u32 = 29;

/**
  | BIP 9 defines a finite-state-machine
  | to deploy a softfork in multiple stages.
  | 
  | State transitions happen during retarget
  | period if conditions are met
  | 
  | In case of reorg, transitions can go
  | backward. Without transition, state
  | is inherited between periods. All blocks
  | of a period share the same state.
  |
  */
pub enum ThresholdState {

    /**
      | First state that each softfork starts
      | out as. The genesis block is by definition
      | in this state for each deployment.
      |
      */
    DEFINED,   

    /**
      | For blocks past the starttime.
      |
      */
    STARTED,   

    /**
      | For at least one retarget period after
      | the first retarget period with STARTED
      | blocks of which at least threshold have
      | the associated bit set in nVersion,
      | until min_activation_height is reached.
      |
      */
    LOCKED_IN, 

    /**
      | For all blocks after the LOCKED_IN retarget
      | period (final state)
      |
      */
    ACTIVE,    

    /**
      | For all blocks once the first retarget
      | period after the timeout time is hit,
      | if LOCKED_IN wasn't already reached
      | (final state)
      |
      */
    FAILED,    
}

/**
  | A map that gives the state for blocks whose
  | height is a multiple of Period().
  |
  | The map is indexed by the block's parent,
  | however, so all keys in the map will either be
  | nullptr or a block with (height + 1) % Period()
  | == 0.
  */
pub type ThresholdConditionCache = HashMap<*const BlockIndex,ThresholdState>;

/**
  | Display status of an in-progress BIP9
  | softfork
  |
  */
pub struct BIP9Stats {

    /**
      | Length of blocks of the BIP9 signalling
      | period
      |
      */
    period:    i32,

    /**
      | Number of blocks with the version bit
      | set required to activate the softfork
      |
      */
    threshold: i32,

    /**
      | Number of blocks elapsed since the beginning
      | of the current period
      |
      */
    elapsed:   i32,

    /**
      | Number of blocks with the version bit
      | set since the beginning of the current
      | period
      |
      */
    count:     i32,

    /**
      | False if there are not enough blocks
      | left in this period to pass activation
      | threshold
      |
      */
    possible:  bool,
}

/**
  | Abstract class that implements BIP9-style
  | threshold logic, and caches results.
  |
  */
pub trait AbstractThresholdConditionChecker: 
abstract_threshold_condition_checker::Interface { 

    /**
      | Returns the state for pindex A based
      | on parent pindexPrev B. Applies any
      | state transition if conditions are
      | present.
      | 
      | Caches state from first block of period.
      |
      */
    fn get_state_for(&self, 
        pindex_prev: *const BlockIndex,
        params:      &ChainConsensusParams,
        cache:       &mut ThresholdConditionCache) -> ThresholdState {
        
        todo!();
        /*
            int nPeriod = Period(params);
        int nThreshold = Threshold(params);
        int min_activation_height = MinActivationHeight(params);
        int64_t nTimeStart = BeginTime(params);
        int64_t nTimeTimeout = EndTime(params);

        // Check if this deployment is always active.
        if (nTimeStart == consensus::BIP9Deployment::ALWAYS_ACTIVE) {
            return ThresholdState::ACTIVE;
        }

        // Check if this deployment is never active.
        if (nTimeStart == consensus::BIP9Deployment::NEVER_ACTIVE) {
            return ThresholdState::FAILED;
        }

        // A block's state is always the same as that of the first of its period, so it is computed based on a pindexPrev whose height equals a multiple of nPeriod - 1.
        if (pindexPrev != nullptr) {
            pindexPrev = pindexPrev->GetAncestor(pindexPrev->nHeight - ((pindexPrev->nHeight + 1) % nPeriod));
        }

        // Walk backwards in steps of nPeriod to find a pindexPrev whose information is known
        std::vector<const CBlockIndex*> vToCompute;
        while (cache.count(pindexPrev) == 0) {
            if (pindexPrev == nullptr) {
                // The genesis block is by definition defined.
                cache[pindexPrev] = ThresholdState::DEFINED;
                break;
            }
            if (pindexPrev->GetMedianTimePast() < nTimeStart) {
                // Optimization: don't recompute down further, as we know every earlier block will be before the start time
                cache[pindexPrev] = ThresholdState::DEFINED;
                break;
            }
            vToCompute.push_back(pindexPrev);
            pindexPrev = pindexPrev->GetAncestor(pindexPrev->nHeight - nPeriod);
        }

        // At this point, cache[pindexPrev] is known
        assert(cache.count(pindexPrev));
        ThresholdState state = cache[pindexPrev];

        // Now walk forward and compute the state of descendants of pindexPrev
        while (!vToCompute.empty()) {
            ThresholdState stateNext = state;
            pindexPrev = vToCompute.back();
            vToCompute.pop_back();

            switch (state) {
                case ThresholdState::DEFINED: {
                    if (pindexPrev->GetMedianTimePast() >= nTimeStart) {
                        stateNext = ThresholdState::STARTED;
                    }
                    break;
                }
                case ThresholdState::STARTED: {
                    // We need to count
                    const CBlockIndex* pindexCount = pindexPrev;
                    int count = 0;
                    for (int i = 0; i < nPeriod; i++) {
                        if (Condition(pindexCount, params)) {
                            count++;
                        }
                        pindexCount = pindexCount->pprev;
                    }
                    if (count >= nThreshold) {
                        stateNext = ThresholdState::LOCKED_IN;
                    } else if (pindexPrev->GetMedianTimePast() >= nTimeTimeout) {
                        stateNext = ThresholdState::FAILED;
                    }
                    break;
                }
                case ThresholdState::LOCKED_IN: {
                    // Progresses into ACTIVE provided activation height will have been reached.
                    if (pindexPrev->nHeight + 1 >= min_activation_height) {
                        stateNext = ThresholdState::ACTIVE;
                    }
                    break;
                }
                case ThresholdState::FAILED:
                case ThresholdState::ACTIVE: {
                    // Nothing happens, these are terminal states.
                    break;
                }
            }
            cache[pindexPrev] = state = stateNext;
        }

        return state;
        */
    }
    
    /**
      | Returns the numerical statistics of
      | an in-progress BIP9 softfork in the
      | current period
      |
      */
    fn get_state_statistics_for(&self, 
        pindex: *const BlockIndex,
        params: &ChainConsensusParams) -> BIP9Stats {
        
        todo!();
        /*
            BIP9Stats stats = {};

        stats.period = Period(params);
        stats.threshold = Threshold(params);

        if (pindex == nullptr)
            return stats;

        // Find beginning of period
        const CBlockIndex* pindexEndOfPrevPeriod = pindex->GetAncestor(pindex->nHeight - ((pindex->nHeight + 1) % stats.period));
        stats.elapsed = pindex->nHeight - pindexEndOfPrevPeriod->nHeight;

        // Count from current block to beginning of period
        int count = 0;
        const CBlockIndex* currentIndex = pindex;
        while (pindexEndOfPrevPeriod->nHeight != currentIndex->nHeight){
            if (Condition(currentIndex, params))
                count++;
            currentIndex = currentIndex->pprev;
        }

        stats.count = count;
        stats.possible = (stats.period - stats.threshold ) >= (stats.elapsed - count);

        return stats;
        */
    }
    
    /**
      | Returns the height since when the ThresholdState
      | has started for pindex A based on parent
      | pindexPrev B, all blocks of a period
      | share the same
      |
      */
    fn get_state_since_height_for(&self, 
        pindex_prev: *const BlockIndex,
        params:      &ChainConsensusParams,
        cache:       &mut ThresholdConditionCache) -> i32 {
        
        todo!();
        /*
            int64_t start_time = BeginTime(params);
        if (start_time == consensus::BIP9Deployment::ALWAYS_ACTIVE || start_time == consensus::BIP9Deployment::NEVER_ACTIVE) {
            return 0;
        }

        const ThresholdState initialState = GetStateFor(pindexPrev, params, cache);

        // BIP 9 about state DEFINED: "The genesis block is by definition in this state for each deployment."
        if (initialState == ThresholdState::DEFINED) {
            return 0;
        }

        const int nPeriod = Period(params);

        // A block's state is always the same as that of the first of its period, so it is computed based on a pindexPrev whose height equals a multiple of nPeriod - 1.
        // To ease understanding of the following height calculation, it helps to remember that
        // right now pindexPrev points to the block prior to the block that we are computing for, thus:
        // if we are computing for the last block of a period, then pindexPrev points to the second to last block of the period, and
        // if we are computing for the first block of a period, then pindexPrev points to the last block of the previous period.
        // The parent of the genesis block is represented by nullptr.
        pindexPrev = pindexPrev->GetAncestor(pindexPrev->nHeight - ((pindexPrev->nHeight + 1) % nPeriod));

        const CBlockIndex* previousPeriodParent = pindexPrev->GetAncestor(pindexPrev->nHeight - nPeriod);

        while (previousPeriodParent != nullptr && GetStateFor(previousPeriodParent, params, cache) == initialState) {
            pindexPrev = previousPeriodParent;
            previousPeriodParent = pindexPrev->GetAncestor(pindexPrev->nHeight - nPeriod);
        }

        // Adjust the result because right now we point to the parent block.
        return pindexPrev->nHeight + 1;
        */
    }
}

pub mod abstract_threshold_condition_checker {

    use super::*;

    pub trait Condition {
        fn condition(&self, 
            pindex: *const BlockIndex,
            params: &ChainConsensusParams) -> bool;
    }

    pub trait BeginTime {
        fn begin_time(&self, params: &ChainConsensusParams) -> i64;
    }

    pub trait EndTime {
        fn end_time(&self, params: &ChainConsensusParams) -> i64;
    }

    pub trait MinActivationHeight {
        fn min_activation_height(&self, params: &ChainConsensusParams) -> i32 { 0 }
    }

    pub trait Period {
        fn period(&self, params: &ChainConsensusParams) -> i32;
    }

    pub trait Threshold {
        fn threshold(&self, params: &ChainConsensusParams) -> i32;
    }

    pub trait Interface:
        Condition
        + BeginTime
        + EndTime
        + MinActivationHeight
        + Period
        + Threshold { }
}

/**
  | BIP 9 allows multiple softforks to be
  | deployed in parallel. We cache per-period
  | state for every one of them.
  |
  */
pub struct VersionBitsCache {
    mutex:  std::sync::Mutex<version_bits_cache::Inner>,
}

pub mod version_bits_cache {

    use super::*;

    pub struct Inner {
        caches: [ThresholdConditionCache; ConsensusDeploymentPos::MAX_VERSION_BITS_DEPLOYMENTS as usize],
    }
}

/**
  | Class to implement versionbits logic.
  |
  */
pub struct VersionBitsConditionChecker {
    id:   ConsensusDeploymentPos,
}

impl AbstractThresholdConditionChecker for VersionBitsConditionChecker {

}

impl abstract_threshold_condition_checker::Interface for VersionBitsConditionChecker {}

impl abstract_threshold_condition_checker::BeginTime for VersionBitsConditionChecker {
    fn begin_time(&self, params: &ChainConsensusParams) -> i64 {
        
        todo!();
        /*
            return params.vDeployments[id].nStartTime;
        */
    }
}
    
impl abstract_threshold_condition_checker::EndTime for VersionBitsConditionChecker {
    fn end_time(&self, params: &ChainConsensusParams) -> i64 {
        
        todo!();
        /*
            return params.vDeployments[id].nTimeout;
        */
    }
}
    
impl abstract_threshold_condition_checker::MinActivationHeight for VersionBitsConditionChecker {
    fn min_activation_height(&self, params: &ChainConsensusParams) -> i32 {
        
        todo!();
        /*
            return params.vDeployments[id].min_activation_height;
        */
    }
}
    
impl abstract_threshold_condition_checker::Period for VersionBitsConditionChecker {
    fn period(&self, params: &ChainConsensusParams) -> i32 {
        
        todo!();
        /*
            return params.nMinerConfirmationWindow;
        */
    }
}
    
impl abstract_threshold_condition_checker::Threshold for VersionBitsConditionChecker {
    fn threshold(&self, params: &ChainConsensusParams) -> i32 {
        
        todo!();
        /*
            return params.nRuleChangeActivationThreshold;
        */
    }
}
    
impl abstract_threshold_condition_checker::Condition for VersionBitsConditionChecker {
    fn condition(&self, 
        pindex: *const BlockIndex,
        params: &ChainConsensusParams) -> bool {
        
        todo!();
        /*
            return (((pindex->nVersion & VERSIONBITS_TOP_MASK) == VERSIONBITS_TOP_BITS) && (pindex->nVersion & Mask(params)) != 0);
        */
    }
}
    
impl VersionBitsConditionChecker {

    pub fn mask(&self, params: &ChainConsensusParams) -> u32 {
        
        todo!();
        /*
            return ((uint32_t)1) << params.vDeployments[id].bit;
        */
    }
    
    pub fn new(id: ConsensusDeploymentPos) -> Self {
    
        todo!();
        /*
        : id(id_),
        */
    }
}

impl VersionBitsCache {
    
    /**
      | Get the BIP9 state for a given deployment
      | for the block after pindexPrev.
      |
      */
    pub fn state(&mut self, 
        pindex_prev: *const BlockIndex,
        params:      &ChainConsensusParams,
        pos:         ConsensusDeploymentPos) -> ThresholdState {
        
        todo!();
        /*
            LOCK(m_mutex);
        return VersionBitsConditionChecker(pos).GetStateFor(pindexPrev, params, m_caches[pos]);
        */
    }
    
    /**
      | Get the numerical statistics for a given
      | deployment for the signalling period
      | that includes the block after pindexPrev.
      |
      */
    pub fn statistics(&mut self, 
        pindex_prev: *const BlockIndex,
        params:      &ChainConsensusParams,
        pos:         ConsensusDeploymentPos) -> BIP9Stats {
        
        todo!();
        /*
            return VersionBitsConditionChecker(pos).GetStateStatisticsFor(pindexPrev, params);
        */
    }
    
    /**
      | Get the block height at which the BIP9
      | deployment switched into the state
      | for the block after pindexPrev.
      |
      */
    pub fn state_since_height(&mut self, 
        pindex_prev: *const BlockIndex,
        params:      &ChainConsensusParams,
        pos:         ConsensusDeploymentPos) -> i32 {
        
        todo!();
        /*
            LOCK(m_mutex);
        return VersionBitsConditionChecker(pos).GetStateSinceHeightFor(pindexPrev, params, m_caches[pos]);
        */
    }
    
    pub fn mask(&mut self, 
        params: &ChainConsensusParams,
        pos:    ConsensusDeploymentPos) -> u32 {
        
        todo!();
        /*
            return VersionBitsConditionChecker(pos).Mask(params);
        */
    }
    
    /**
      | Determine what nVersion a new block
      | should use
      |
      */
    pub fn compute_block_version(&mut self, 
        pindex_prev: *const BlockIndex,
        params:      &ChainConsensusParams) -> i32 {
        
        todo!();
        /*
            LOCK(m_mutex);
        int32_t nVersion = VERSIONBITS_TOP_BITS;

        for (int i = 0; i < (int)consensus::MAX_VERSION_BITS_DEPLOYMENTS; i++) {
            consensus::DeploymentPos pos = static_cast<consensus::DeploymentPos>(i);
            ThresholdState state = VersionBitsConditionChecker(pos).GetStateFor(pindexPrev, params, m_caches[pos]);
            if (state == ThresholdState::LOCKED_IN || state == ThresholdState::STARTED) {
                nVersion |= Mask(params, pos);
            }
        }

        return nVersion;
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            LOCK(m_mutex);
        for (unsigned int d = 0; d < consensus::MAX_VERSION_BITS_DEPLOYMENTS; d++) {
            m_caches[d].clear();
        }
        */
    }
}
