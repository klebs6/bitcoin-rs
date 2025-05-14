// ---------------- [ File: bitcoin-indexed-chain/src/node.rs ]
crate::ix!();

/**
  | Sort eviction candidates by network/localhost
  | and connection uptime.
  | 
  | Candidates near the beginning are more
  | likely to be evicted, and those near
  | the end are more likely to be protected,
  | e.g. less likely to be evicted.
  | 
  | - First, nodes that are not `is_local`
  | and that do not belong to `network`,
  | sorted by increasing uptime (from most
  | recently connected to connected longer).
  | 
  | - Then, nodes that are `is_local` or
  | belong to `network`, sorted by increasing
  | uptime.
  |
  */
pub struct CompareNodeNetworkTime {
    is_local: bool,
    network:  Network,
}

impl CompareNodeNetworkTime {

    pub fn new(
        is_local: bool,
        network:  Network) -> Self {
    
        todo!();
        /*
        : is_local(is_local),
        : network(network),

        
        */
    }
    
    pub fn invoke(&self, 
        a: &NodeEvictionCandidate,
        b: &NodeEvictionCandidate) -> bool {
        
        todo!();
        /*
            if (m_is_local && a.m_is_local != b.m_is_local) return b.m_is_local;
            if ((a.m_network == m_network) != (b.m_network == m_network)) return b.m_network == m_network;
            return a.nTimeConnected > b.nTimeConnected;
        }{
        */
    }
}

/**
  | Return implementation of Init interface for
  | the node process. 
  |
  | If the argv indicates that this is a child
  | process spawned to handle requests from
  | a parent process, this blocks and handles
  | requests, then returns null and a status code
  | to exit with. 
  |
  | If this returns non-null, the caller can start
  | up normally and use the Init object to spawn
  | and connect to other processes while it is
  | running.
  */
/*
pub fn make_node_init(
        node:        &mut NodeContext,
        argc:        i32,
        argv:        &[*mut u8],
        exit_status: &mut i32) -> Box<dyn Init> {
    
    todo!();
        /*
            return std::make_unique<init::BitcoindInit>(node);
        */
}
*/
pub fn make_node_init(
        node:        &mut NodeContext,
        argc:        i32,
        argv:        &[*mut u8],
        exit_status: &mut i32) -> Box<dyn Init> {
    
    todo!();
        /*
            auto init = std::make_unique<init::BitcoinNodeInit>(node, argc > 0 ? argv[0] : "");
            // Check if bitcoin-node is being invoked as an IPC server. If so, then
            // bypass normal execution and just respond to requests over the IPC
            // channel and return null.
            if (init->m_ipc->startSpawnedProcess(argc, argv, exit_status)) {
                return nullptr;
            }
            return init;
        */
}
