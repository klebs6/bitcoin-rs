// ---------------- [ File: bitcoin-peerman/src/txhashinfo.rs ]
crate::ix!();

/**
  | Per-txhash statistics object. Only
  | used for sanity checking.
  |
  */
pub struct TxHashInfo
{
    /**
      | Number of CANDIDATE_DELAYED announcements
      | for this txhash.
      |
      */
    pub candidate_delayed:             usize,

    /**
      | Number of CANDIDATE_READY announcements
      | for this txhash.
      |
      */
    pub candidate_ready:               usize,

    /**
      | Number of CANDIDATE_BEST announcements
      | for this txhash (at most one).
      |
      */
    pub candidate_best:                usize,

    /**
      | Number of REQUESTED announcements
      | for this txhash (at most one; mutually
      | exclusive with CANDIDATE_BEST).
      |
      */
    pub requested:                     usize,

    /**
      | The priority of the CANDIDATE_BEST
      | announcement if one exists, or max()
      | otherwise.
      |
      */
    pub priority_candidate_best:       Priority,

    /**
      | The highest priority of all CANDIDATE_READY
      | announcements (or min() if none exist).
      |
      */
    pub priority_best_candidate_ready: Priority,

    /**
      | All peers we have an announcement for
      | this txhash for.
      |
      */
    pub peers:                         Vec<NodeId>,
}

impl Default for TxHashInfo {

    fn default() -> Self {
        Self {
            candidate_delayed:             0,
            candidate_ready:               0,
            candidate_best:                0,
            requested:                     0,
            priority_candidate_best:       Priority::MAX,
            priority_best_candidate_ready: Priority::MIN,
            peers:                         vec![],
        }
    }
}
