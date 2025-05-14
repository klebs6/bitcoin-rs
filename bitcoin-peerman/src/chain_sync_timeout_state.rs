// ---------------- [ File: bitcoin-peerman/src/chain_sync_timeout_state.rs ]
crate::ix!();

/**
  | State used to enforce CHAIN_SYNC_TIMEOUT
  | and EXTRA_PEER_CHECK_INTERVAL logic.
  | 
  | Both are only in effect for outbound,
  | non-manual, non-protected connections.
  | 
  | Any peer protected (m_protect = true)
  | is not chosen for eviction. A peer is
  | marked as protected if all of these are
  | true:
  | 
  | - its connection type is IsBlockOnlyConn() == false
  | 
  | - it gave us a valid connecting header
  | 
  | - we haven't reached MAX_OUTBOUND_PEERS_TO_PROTECT_FROM_DISCONNECT
  | yet
  | 
  | - its chain tip has at least as much work
  | as ours
  | 
  | CHAIN_SYNC_TIMEOUT: if a peer's best
  | known block has less work than our tip,
  | set a timeout CHAIN_SYNC_TIMEOUT seconds
  | in the future:
  | 
  | - If at timeout their best known block
  | now has more work than our tip when the
  | timeout was set, then either reset the
  | timeout or clear it (after comparing
  | against our current tip's work)
  | 
  | - If at timeout their best known block
  | still has less work than our tip did when
  | the timeout was set, then send a getheaders
  | message, and set a shorter timeout,
  | HEADERS_RESPONSE_TIME seconds in
  | future.
  | 
  | If their best known block is still behind
  | when that new timeout is reached, disconnect.
  | 
  | EXTRA_PEER_CHECK_INTERVAL: after
  | each interval, if we have too many outbound
  | peers, drop the outbound one that least
  | recently announced us a new block.
  |
  */
pub struct NodeStateChainSyncTimeoutState {

    /**
      | A timeout used for checking whether
      | our peer has sufficiently synced
      |
      */
    pub timeout:         Option<OffsetDateTime>,

    /**
      | A header with the work we require on our
      | peer's chain
      |
      */
    pub work_header:     Option<Arc<BlockIndex>>,

    /**
      | After timeout is reached, set to true
      | after sending getheaders
      |
      */
    pub sent_getheaders: bool,

    /**
      | Whether this peer is protected from
      | disconnection due to a bad/slow chain
      |
      */
    pub protect:         bool,
}

impl Default for NodeStateChainSyncTimeoutState {

    fn default() -> Self {
        Self {
            timeout:         None,
            work_header:     None,
            sent_getheaders: false,
            protect:         false,
        }
    }
}
