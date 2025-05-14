// ---------------- [ File: bitcoin-net/src/connection_type.rs ]
crate::ix!();

/**
  | Different types of connections to a
  | peer. This enum encapsulates the information
  | we have available at the time of opening
  | or accepting the connection. Aside
  | from INBOUND, all types are initiated
  | by us.
  | 
  | If adding or removing types, please
  | update CONNECTION_TYPE_DOC in src/rpc/net.cpp
  | and src/qt/rpcconsole.cpp, as well
  | as the descriptions in src/qt/guiutil.cpp
  | and src/bitcoin-cli.cpp::NetinfoRequestHandler.
  |
  */
#[derive(Copy,Clone,PartialEq,Eq)]
pub enum ConnectionType {

    /**
      | Inbound connections are those initiated
      | by a peer. This is the only property we
      | know at the time of connection, until
      | 
      | P2P messages are exchanged.
      |
      */
    INBOUND,

    /**
      | These are the default connections that
      | we use to connect with the network. There
      | is no restriction on what is relayed;
      | by default we relay blocks, addresses
      | & transactions. We automatically attempt
      | to open
      | 
      | MAX_OUTBOUND_FULL_RELAY_CONNECTIONS
      | using addresses from our AddrMan.
      |
      */
    OUTBOUND_FULL_RELAY,

    /**
      | We open manual connections to addresses
      | that users explicitly requested via
      | the addnode RPC or the -addnode/-connect
      | configuration options. Even if a manual
      | connection is misbehaving, we do not
      | automatically disconnect or add it
      | to our discouragement filter.
      |
      */
    MANUAL,

    /**
      | Feeler connections are short-lived
      | connections made to check that a node
      | is alive. They can be useful for:
      | 
      | - test-before-evict: if one of the peers
      | is considered for eviction from our
      | AddrMan because another peer is mapped
      | to the same slot in the tried table, evict
      | only if this longer-known peer is offline.
      | 
      | - move node addresses from New to Tried
      | table, so that we have more connectable
      | addresses in our AddrMan.
      | 
      | -----------
      | @note
      | 
      | in the literature ("Eclipse Attacks
      | on Bitcoin’s Peer-to-Peer Network")
      | only the latter feature is referred
      | to as "feeler connections", although
      | in our codebase feeler connections
      | encompass test-before-evict as well.
      | 
      | We make these connections approximately
      | every FEELER_INTERVAL: first we resolve
      | previously found collisions if they
      | exist (test-before-evict), otherwise
      | we connect to a node from the new table.
      |
      */
    FEELER,

    /** 
      | We use block-relay-only connections to
      | help prevent against partition attacks.
      |
      | By not relaying transactions or addresses,
      | these connections are harder to detect by
      | a third party, thus helping obfuscate the
      | network topology. 
      |
      | We automatically attempt to open
      | MAX_BLOCK_RELAY_ONLY_ANCHORS using
      | addresses from our anchors.dat. 
      |
      | Then addresses from our AddrMan if
      | MAX_BLOCK_RELAY_ONLY_CONNECTIONS isn't
      | reached yet.
      |
      */
    BLOCK_RELAY,

    /**
      | AddrFetch connections are short lived
      | connections used to solicit addresses
      | from peers. These are initiated to addresses
      | submitted via the -seednode command line 
      | argument, or under certain conditions when the
      | AddrMan is empty.
      |
      */
    ADDR_FETCH,
}

impl Default for ConnectionType {

    fn default() -> Self {
        todo!();
    }
}

/**
  | Convert ConnectionType enum to a string
  | value
  |
  */
pub fn connection_type_as_string(conn_type: ConnectionType) -> String {
    
    todo!();
        /*
            switch (conn_type) {
        case ConnectionType::INBOUND:
            return "inbound";
        case ConnectionType::MANUAL:
            return "manual";
        case ConnectionType::FEELER:
            return "feeler";
        case ConnectionType::OUTBOUND_FULL_RELAY:
            return "outbound-full-relay";
        case ConnectionType::BLOCK_RELAY:
            return "block-relay-only";
        case ConnectionType::ADDR_FETCH:
            return "addr-fetch";
        } // no default case, so the compiler can warn about missing cases

        assert(false);
        */
}
