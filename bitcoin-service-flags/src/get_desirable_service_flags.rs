// ---------------- [ File: bitcoin-service-flags/src/get_desirable_service_flags.rs ]
crate::ix!();

/**
  | Gets the set of service flags which are
  | "desirable" for a given peer.
  | 
  | These are the flags which are required
  | for a peer to support for them to be "interesting"
  | to us, ie for us to wish to use one of our
  | few outbound connection slots for or
  | for us to wish to prioritize keeping
  | their connection around.
  | 
  | Relevant service flags may be peer-
  | and state-specific in that the version
  | of the peer may determine which flags
  | are required (eg in the case of NODE_NETWORK_LIMITED
  | where we seek out NODE_NETWORK peers
  | unless they set NODE_NETWORK_LIMITED
  | and we are out of IBD, in which case NODE_NETWORK_LIMITED
  | suffices).
  | 
  | Thus, generally, avoid calling with
  | peerServices == NODE_NONE, unless
  | state-specific flags must absolutely
  | be avoided. When called with peerServices
  | == NODE_NONE, the returned desirable
  | service flags are guaranteed to not
  | change dependent on state - ie they are
  | suitable for use when describing peers
  | which we know to be desirable, but for
  | which we do not have a confirmed set of
  | service flags.
  | 
  | If the NODE_NONE return value is changed,
  | contrib/seeds/makeseeds.py should
  | be updated appropriately to filter
  | for the same nodes.
  |
  */
pub fn get_desirable_service_flags(services: ServiceFlags) -> ServiceFlags {
    
    todo!();
        /*
            if ((services & NODE_NETWORK_LIMITED) && g_initial_block_download_completed) {
            return ServiceFlags(NODE_NETWORK_LIMITED | NODE_WITNESS);
        }
        return ServiceFlags(NODE_NETWORK | NODE_WITNESS);
        */
}
