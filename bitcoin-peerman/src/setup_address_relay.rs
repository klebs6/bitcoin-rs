// ---------------- [ File: bitcoin-peerman/src/setup_address_relay.rs ]
crate::ix!();

pub trait SetupAddressRelay {

    fn setup_address_relay(self: Arc<Self>, 
        node: &dyn NodeInterface,
        peer: &mut Peer) -> bool;
}

impl SetupAddressRelay for PeerManager {

    /**
      | Checks if address relay is permitted
      | with peer. If needed, initializes the
      | m_addr_known bloom filter and sets
      | m_addr_relay_enabled to true.
      | 
      | 
      | -----------
      | @return
      | 
      | True if address relay is enabled with
      | peer
      | 
      | False if address relay is disallowed
      |
      */
    fn setup_address_relay(self: Arc<Self>, 
        node: &dyn NodeInterface,
        peer: &mut Peer) -> bool {
        
        // We don't participate in addr relay with
        // outbound block-relay-only connections
        // to prevent providing adversaries with
        // the additional information of addr
        // traffic to infer the link.
        if node.is_block_only_conn() {
            return false;
        }

        if !peer.addr_relay_enabled.swap(true, atomic::Ordering::Relaxed) {

            // First addr message we have received
            // from the peer, initialize
            // m_addr_known
            peer.addr_known = Some(RollingBloomFilter::new(5000,0.001));
        }

        true
    }
}
