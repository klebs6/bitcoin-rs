// ---------------- [ File: bitcoin-peerman/src/remove_peer.rs ]
crate::ix!();

pub trait RemovePeer {
    fn remove_peer(&mut self, id: NodeId) -> Amo<Peer>;
}

impl RemovePeer for PeerManager {

    /**
      | Get a shared pointer to the Peer object
      | and remove it from m_peer_map.
      | 
      | May return an empty shared_ptr if the
      | Peer object can't be found.
      |
      */
    fn remove_peer(&mut self, id: NodeId) -> Amo<Peer> {

        let mut ret: Amo<Peer> = amo_none();

        let mut peer_map = self.peer_map.get_mut();

        if let Some(it) = peer_map.get(&id) {

            ret = it.clone();

            peer_map.remove(&id);
        }

        ret
    }
}
