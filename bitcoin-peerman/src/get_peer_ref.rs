// ---------------- [ File: bitcoin-peerman/src/get_peer_ref.rs ]
crate::ix!();

pub trait GetPeerRef {
    fn get_peer_ref(&self, id: NodeId) -> Amo<Peer>;
}

impl GetPeerRef for PeerManager {

    /**
      | Get a shared pointer to the Peer object.
      | 
      | May return an empty shared_ptr if the
      | Peer object can't be found.
      |
      */
    fn get_peer_ref(&self, id: NodeId) -> Amo<Peer> {
        
        match self.peer_map.get().get(&id) {
            Some(maybe_peer) => maybe_peer.clone(),
            None             => amo_none(),
        }
    }
}
