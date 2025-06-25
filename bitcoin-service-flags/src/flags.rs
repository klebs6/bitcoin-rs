// ---------------- [ File: bitcoin-service-flags/src/flags.rs ]
crate::ix!();

/**
  | nServices flags
  |
  */
//#[repr(u64)]
bitflags!{

    #[derive(Serialize,Deserialize)]
    pub struct ServiceFlags: u64 {

        /**
          | @note
          | 
          | When adding here, be sure to update
          | serviceFlagToStr too Nothing
          |
          */
        const NODE_NONE = 0;

        /**
          | NODE_NETWORK means that the node is capable
          | of serving the complete block chain. It is
          | currently set by all Bitcoin Core non
          | pruned nodes, and is unset by SPV clients
          | or other light clients.
          */
        const NODE_NETWORK = (1 << 0);

        /**
          | NODE_BLOOM means the node is capable and
          | willing to handle bloom-filtered
          | connections.
          |
          | Bitcoin Core nodes used to support this by
          | default, without advertising this bit, but
          | no longer do as of protocol version 70011
          | (= NO_BLOOM_VERSION)
          */
        const NODE_BLOOM = 1 << 2;

        /**
          | NODE_WITNESS indicates that a node
          | can be asked for blocks and transactions
          | including witness data.
          |
          */
        const NODE_WITNESS = 1 << 3;

        /**
          | NODE_COMPACT_FILTERS means the node will
          | service basic block filter requests.
          |
          | See BIP157 and BIP158 for details on how
          | this is implemented.
          */
        const NODE_COMPACT_FILTERS = 1 << 6;

        /**
          | NODE_NETWORK_LIMITED means the same as
          | NODE_NETWORK with the limitation of only
          | serving the last 288 (2 day) blocks
          |
          | See BIP159 for details on how this is
          | implemented.
          */
        const NODE_NETWORK_LIMITED = 1 << 10;

        /*
          | Bits 24-31 are reserved for temporary
          | experiments. Just pick a bit that isn't
          | getting used, or one not being used much,
          | and notify the bitcoin-development mailing
          | list. Remember that service bits are just
          | unauthenticated advertisements, so your
          | code must be robust against collisions and
          | other cases where nodes may be advertising
          | a service they do not actually
          | support. Other service bits should be
          | allocated via the BIP process.
          */
    }
}

impl Default for ServiceFlags {
    fn default() -> Self {
        Self::NODE_NONE
    }
}

impl From<u64> for ServiceFlags {
    fn from(x: u64) -> Self {
        todo!();
    }
}
