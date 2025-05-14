// ---------------- [ File: bitcoin-peerman/src/addr_send_times.rs ]
crate::ix!();

pub struct PeerAddrSendTimes {

    /**
      | Time point to send the next ADDR message
      | to this peer.
      | 
      |
      */
    pub next_addr_send:        Option<OffsetDateTime>, // microseconds

    /**
      | Time point to possibly re-announce
      | our local address to this peer.
      | 
      |
      */
    pub next_local_addr_send:   Option<OffsetDateTime>, // microseconds
}

impl Default for PeerAddrSendTimes {
    fn default() -> Self {
        Self {
            next_addr_send:       None,
            next_local_addr_send: None,
        }
    }
}
