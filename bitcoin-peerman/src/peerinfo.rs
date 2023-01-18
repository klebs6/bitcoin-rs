crate::ix!();

/**
  | Per-peer statistics object.
  |
  */
pub struct PeerInfo {

    /**
      | Total number of announcements for this
      | peer.
      |
      */
    pub total:     usize,

    /**
      | Number of COMPLETED announcements
      | for this peer.
      |
      */
    pub completed: usize,

    /**
      | Number of REQUESTED announcements
      | for this peer.
      |
      */
    pub requested: usize,
}

impl Default for PeerInfo {

    fn default() -> Self {
        Self {
            total:     0,
            completed: 0,
            requested: 0,
        }
    }
}


impl PartialEq<PeerInfo> for PeerInfo {
    
    /**
      | Compare two PeerInfo objects. Only
      | used for sanity checking.
      |
      */
    #[inline] fn eq(&self, other: &PeerInfo) -> bool {
        (self.total,self.completed,self.requested) 
        == (other.total,other.completed,other.requested)
    }
}

impl Eq for PeerInfo {}

/**
  | (Re)compute the PeerInfo map from the
  | index. Only used for sanity checking.
  |
  */
pub fn recompute_peer_info(index: Arc<AnnouncementIndex>) -> HashMap<NodeId,PeerInfo> {
    
    let mut ret = HashMap::<NodeId,PeerInfo>::default();

    for ann in index.get_by_peer() {

        if let Some(info) = ret.get_mut(&ann.peer) {

            info.total += 1;

            info.requested += match ann.get_state() == State::REQUESTED { true => 1, false => 0 };
            info.completed += match ann.get_state() == State::COMPLETED { true => 1, false => 0 };
        }
    }

    ret
}
