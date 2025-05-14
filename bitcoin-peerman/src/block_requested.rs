// ---------------- [ File: bitcoin-peerman/src/block_requested.rs ]
crate::ix!();

/**
  | Return the time it would take to redo
  | the work difference between from and
  | to, assuming the current hashrate corresponds
  | to the difficulty at tip, in seconds.
  |
  */
pub fn get_block_proof_equivalent_time(
    to:     Option<Arc<BlockIndex>>,
    from:   Option<Arc<BlockIndex>>,
    tip:    Option<Arc<BlockIndex>>,
    params: &ChainConsensusParams) -> Duration {
    
    todo!();
        /*
            ArithU256 r;
        int sign = 1;
        if (to.nChainWork > from.nChainWork) {
            r = to.nChainWork - from.nChainWork;
        } else {
            r = from.nChainWork - to.nChainWork;
            sign = -1;
        }
        r = r * ArithU256(params.nPowTargetSpacing) / GetBlockProof(tip);
        if (r.bits() > 63) {
            return sign * std::numeric_limits<int64_t>::max();
        }
        return sign * r.GetLow64();
        */
}


pub type CppIter<T: Iterator> = Peekable<Enumerate<Box<T>>>;

pub trait BlockRequested {

    fn block_requested(
        self:   Arc<Self>, 
        nodeid: NodeId,
        block:  Option<Arc<BlockIndex>>,
        pit:    Amo<QueuedBlockIter>) -> bool;
}

impl BlockRequested for PeerManager {
    
    /**
      | Mark a block as in flight
      | 
      | Returns false, still setting pit, if
      | the block was already in flight from
      | the same peer pit will only be valid as
      | long as the same CS_MAIN lock is being
      | held
      |
      */
    #[EXCLUSIVE_LOCKS_REQUIRED(CS_MAIN)]
    fn block_requested(
        self:   Arc<Self>, 
        nodeid: NodeId,
        block:  Option<Arc<BlockIndex>>,
        pit:    Amo<QueuedBlockIter>) -> bool {

        assert!(block.is_some());

        let hash: &u256 = &block.as_ref().unwrap().get_block_hash();

        let state: Amo<NodeState> = create_state(nodeid);

        assert!(state.is_some());

        let mut inner = self.inner.lock();

        let mut mbif = inner.map_blocks_in_flight.lock();

        // Short-circuit most stuff in case it is
        // from the same node
        let mut it_in_flight = mbif.get(hash);

        if it_in_flight.is_some()
        && it_in_flight.unwrap().0 == nodeid {

            if pit.is_some() {
                pit.replace(&it_in_flight.unwrap().1);
            }

            return false;
        }

        // Make sure it's not listed somewhere
        // already.
        inner.remove_block_request(hash);

        let partial_block = match pit.is_some() {
            true  => Amo::from(PartiallyDownloadedBlock::new(amo_none())),
            false => amo_none(),
        };

        state.get_mut().blocks_in_flight.push(
            QueuedBlock {
                pindex: block.clone(),
                partial_block
            }
        );

        let len = state.get().blocks_in_flight.len();

        let mut it = state.get_mut().blocks_in_flight_iter();;

        it.advance_by(len - 1);

        state.get_mut().n_blocks_in_flight.fetch_add(1,atomic::Ordering::Relaxed);

        if state.get().n_blocks_in_flight.load(atomic::Ordering::Relaxed) == 1 {

            // We're starting a block download (batch) from this peer.
            state.get_mut().downloading_since = get_datetime();

            inner.peers_downloading_from.fetch_add(1, atomic::Ordering::Relaxed);
        }

        let it_in_flight = mbif.insert(hash.clone(), (nodeid,it));

        if pit.is_some() {
            *pit.get_mut() = it_in_flight.unwrap().1;
        }

        true
    }
}
