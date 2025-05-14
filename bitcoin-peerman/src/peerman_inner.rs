// ---------------- [ File: bitcoin-peerman/src/peerman_inner.rs ]
crate::ix!();

pub struct PeerManagerInner {

    pub txrequest:                Arc<Mutex<TxRequestTracker>>,

    /**
      | Number of nodes with fSyncStarted.
      | 
      */
    pub n_sync_started:           i32, // default = 0

    /**
      | Sources of received blocks, saved to
      | be able punish them when processing
      | happens afterwards.
      | 
      | Set mapBlockSource[hash].second
      | to false if the node should not be punished
      | if the block is invalid.
      |
      */
    pub map_block_source:         HashMap<u256,(NodeId,bool)>,

    /**
      | Number of peers with wtxid relay.
      |
      */
    pub wtxid_relay_peers:        AtomicI32, // default = 0

    /**
      | Number of outbound peers with m_chain_sync.m_protect.
      |
      */
    pub outbound_peers_with_protect_from_disconnect: AtomicI32, // default = 0

    /**
      | Filter for transactions that were recently
      | rejected by
      | 
      | AcceptToMemoryPool. These are not
      | rerequested until the chain tip changes,
      | at which point the entire filter is reset.
      | 
      | Without this filter we'd be re-requesting
      | txs from each of our peers, increasing
      | bandwidth consumption considerably.
      | For instance, with 100 peers, half of
      | which relay a tx we don't accept, that
      | might be a 50x bandwidth increase. A
      | flooding attacker attempting to roll-over
      | the filter using minimum-sized, 60byte,
      | transactions might manage to send 1000/sec
      | if we have fast peers, so we pick 120,000
      | to give our peers a two minute window
      | to send invs to us.
      | 
      | Decreasing the false positive rate
      | is fairly cheap, so we pick one in a million
      | to make it highly unlikely for users
      | to have issues with this filter.
      | 
      | We typically only add wtxids to this
      | filter. For non-segwit transactions,
      | the txid == wtxid, so this only prevents
      | us from re-downloading non-segwit
      | transactions when communicating with
      | non-wtxidrelay peers -- which is important
      | for avoiding malleation attacks that
      | could otherwise interfere with transaction
      | relay from non-wtxidrelay peers. For
      | communicating with wtxidrelay peers,
      | having the reject filter store wtxids
      | is exactly what we want to avoid redownload
      | of a rejected transaction.
      | 
      | In cases where we can tell that a segwit
      | transaction will fail validation no
      | matter the witness, we may add the txid
      | of such transaction to the filter as
      | well. This can be helpful when communicating
      | with txid-relay peers or if we were to
      | otherwise fetch a transaction via txid
      | (eg in our orphan handling).
      | 
      | Memory used: 1.3 MB
      | 
      |
      */
    pub recent_rejects:                    RollingBloomFilter,

    pub hash_recent_rejects_chain_tip:     u256,

    pub map_blocks_in_flight:              Arc<Mutex<HashMap<u256,(NodeId, QueuedBlockIter)>>>,

    /**
      | Relay map (txid or wtxid -> CTransactionRef)
      |
      */
    pub map_relay:                         PeerManagerMapRelay,

    /**
      | Expiration-time ordered list of (expire
      | time, relay map entry) pairs.
      | 
      |
      */
    pub g_relay_expiration:                VecDeque<(OffsetDateTime /* micros */,PeerManagerMapRelayIterator)>,

    /**
      | Stack of nodes which we have set to announce
      | using compact blocks
      | 
      |
      */
    pub l_nodes_announcing_header_and_ids: VecDeque<NodeId>,

    /**
      | Number of peers from which we're downloading
      | blocks.
      | 
      |
      */
    pub peers_downloading_from:            AtomicI32, // default = 0
}
