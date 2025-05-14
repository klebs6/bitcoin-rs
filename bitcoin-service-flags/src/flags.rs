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

/**
  | A shortcut for (services & GetDesirableServiceFlags(services))
  | == GetDesirableServiceFlags(services),
  | ie determines whether the given set
  | of service flags are sufficient for
  | a peer to be "relevant".
  |
  */
#[inline] pub fn has_all_desirable_service_flags(services: ServiceFlags) -> bool {
    
    todo!();
        /*
            return !(GetDesirableServiceFlags(services) & (~services));
        */
}

/**
  | Checks if a peer with the given service
  | flags may be capable of having a robust
  | address-storage DB.
  |
  */
#[inline] pub fn may_have_useful_addressdb(services: ServiceFlags) -> bool {
    
    todo!();
        /*
            return (services & NODE_NETWORK) || (services & NODE_NETWORK_LIMITED);
        */
}

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


/**
  | Convert a service flag (NODE_*) to a
  | human readable string.
  | 
  | It supports unknown service flags which
  | will be returned as "UNKNOWN[...]".
  | 
  | -----------
  | @param[in] bit
  | 
  | the service flag is calculated as (1
  | << bit)
  |
  */
pub fn service_flag_to_str(bit: usize) -> String {
    
    todo!();
        /*
            const uint64_t service_flag = 1ULL << bit;
        switch ((ServiceFlags)service_flag) {
        case NODE_NONE: abort();  // impossible
        case NODE_NETWORK:         return "NETWORK";
        case NODE_BLOOM:           return "BLOOM";
        case NODE_WITNESS:         return "WITNESS";
        case NODE_COMPACT_FILTERS: return "COMPACT_FILTERS";
        case NODE_NETWORK_LIMITED: return "NETWORK_LIMITED";
        // Not using default, so we get warned when a case is missing
        }

        std::ostringstream stream;
        stream.imbue(std::locale::classic());
        stream << "UNKNOWN[";
        stream << "2^" << bit;
        stream << "]";
        return stream.str();
        */
}

/**
  | Convert service flags (a bitmask of
  | NODE_*) to human readable strings.
  | 
  | It supports unknown service flags which
  | will be returned as "UNKNOWN[...]".
  | 
  | -----------
  | @param[in] flags
  | 
  | multiple NODE_* bitwise-OR-ed together
  |
  */
pub fn service_flags_to_str(flags: u64) -> Vec<String> {
    
    todo!();
        /*
            std::vector<std::string> str_flags;

        for (size_t i = 0; i < sizeof(flags) * 8; ++i) {
            if (flags & (1ULL << i)) {
                str_flags.emplace_back(serviceFlagToStr(i));
            }
        }

        return str_flags;
        */
}
