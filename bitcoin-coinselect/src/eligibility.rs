// ---------------- [ File: bitcoin-coinselect/src/eligibility.rs ]
crate::ix!();

/**
  | Parameters for filtering which OutputGroups
  | we may use in coin selection.
  | 
  | We start by being very selective and
  | requiring multiple confirmations
  | and then get more permissive if we cannot
  | fund the transaction.
  |
  */
pub struct CoinEligibilityFilter {

    /**
      | Minimum number of confirmations for
      | outputs that we sent to ourselves.
      | 
      | We may use unconfirmed UTXOs sent from
      | ourselves, e.g. change outputs.
      |
      */
    conf_mine:              i32,

    /**
      | Minimum number of confirmations for
      | outputs received from a different wallet.
      |
      */
    conf_theirs:            i32,

    /**
      | Maximum number of unconfirmed ancestors
      | aggregated across all UTXOs in an OutputGroup.
      |
      */
    max_ancestors:          u64,

    /**
      | Maximum number of descendants that
      | a single UTXO in the OutputGroup may
      | have.
      |
      */
    max_descendants:        u64,

    /**
      | When avoid_reuse=true and there are
      | full groups (OUTPUT_GROUP_MAX_ENTRIES),
      | whether or not to use any partial groups.
      |
      */
    include_partial_groups: bool, // default = { false }
}

impl CoinEligibilityFilter {

    pub fn new_with_max_descendants(
        conf_mine:     i32,
        conf_theirs:   i32,
        max_ancestors: u64) -> Self {
    
        todo!();
        /*
        : conf_mine(conf_mine),
        : conf_theirs(conf_theirs),
        : max_ancestors(max_ancestors),
        : max_descendants(max_ancestors),

        
        */
    }
    
    pub fn new(
        conf_mine:       i32,
        conf_theirs:     i32,
        max_ancestors:   u64,
        max_descendants: u64) -> Self {
    
        todo!();
        /*
        : conf_mine(conf_mine),
        : conf_theirs(conf_theirs),
        : max_ancestors(max_ancestors),
        : max_descendants(max_descendants),

        
        */
    }
    
    pub fn new_with_include_partial(
        conf_mine:       i32,
        conf_theirs:     i32,
        max_ancestors:   u64,
        max_descendants: u64,
        include_partial: bool) -> Self {
    
        todo!();
        /*
        : conf_mine(conf_mine),
        : conf_theirs(conf_theirs),
        : max_ancestors(max_ancestors),
        : max_descendants(max_descendants),
        : include_partial_groups(include_partial),

        
        */
    }
}
