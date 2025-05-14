// ---------------- [ File: bitcoin-peerman/src/misbehavior.rs ]
crate::ix!();

#[derive(PartialEq,Eq)]
pub struct PeerMisbehavior {

    /**
      | Accumulated misbehavior score for
      | this peer
      | 
      |
      */
    pub score: i32,

    /**
      | Whether this peer should be disconnected
      | and marked as discouraged (unless it
      | has NetPermissionFlags::NoBan permission).
      | 
      |
      */
    pub should_discourage: bool,
}

impl Default for PeerMisbehavior {

    fn default() -> Self {
        Self {
            score: 0,
            should_discourage: false,
        }
    }
}

impl Misbehaving for PeerManager {

    fn misbehaving(&self,
        pnode:   NodeId,
        howmuch: i32,
        message: &str)  {
        
        assert!(howmuch > 0);

        let peer: Amo<Peer> = self.get_peer_ref(pnode);

        if peer.is_none() {
            return;
        }

        let peer = peer.get();

        let mut guard = peer.misbehavior.lock();

        let score_before: i32 = guard.score;

        guard.score += howmuch;

        let score_now: i32 = guard.score;

        let message_prefixed: String = match message.is_empty() {
            true   => "".to_string(),
            false  => format!(": {}", message)
        };

        let mut warning = String::default();

        if score_now >= DISCOURAGEMENT_THRESHOLD && score_before < DISCOURAGEMENT_THRESHOLD {

            warning = " DISCOURAGE THRESHOLD EXCEEDED".to_string();

            guard.should_discourage = true;
        }

        log_print!(
            LogFlags::NET, 
            "Misbehaving: peer=%d (%d -> %d)%s%s\n", 
            pnode, 
            score_before, 
            score_now, 
            warning, 
            message_prefixed
        );
    }
}
