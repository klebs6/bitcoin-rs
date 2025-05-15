// ---------------- [ File: bitcoin-peerman/src/compute_tx_hashinfo.rs ]
crate::ix!();

/**
  | Compute the TxHashInfo map. Only used
  | for sanity checking.
  |
  */
pub fn compute_tx_hash_info(
    index:    &AnnouncementIndex,
    computer: &PriorityComputer) -> HashMap<u256,TxHashInfo> {
    
    let mut ret = HashMap::<u256,TxHashInfo>::default();

    for ann in index.get_by_txhash() {

        let info: &mut TxHashInfo = ret.get_mut(&ann.txhash).unwrap();

        // Classify how many announcements of each
        // state we have for this txhash.
        info.candidate_delayed += match ann.get_state() == State::CANDIDATE_DELAYED { true => 1, false => 0 };
        info.candidate_ready   += match ann.get_state() == State::CANDIDATE_READY   { true => 1, false => 0 };
        info.candidate_best    += match ann.get_state() == State::CANDIDATE_BEST    { true => 1, false => 0 };
        info.requested         += match ann.get_state() == State::REQUESTED         { true => 1, false => 0 };

        // And track the priority of the best
        // CANDIDATE_READY/CANDIDATE_BEST
        // announcements.
        if ann.get_state() == State::CANDIDATE_BEST {
            info.priority_candidate_best = computer.invoke_announcement(&ann);
        }

        if ann.get_state() == State::CANDIDATE_READY {
            info.priority_best_candidate_ready = max(
                info.priority_best_candidate_ready,
                computer.invoke_announcement(&ann)
            );
        }

        // Also keep track of which peers this
        // txhash has an announcement for (so we
        // can detect duplicates).
        info.peers.push(ann.peer);
    }

    ret
}
