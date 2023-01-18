crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/node/context.h]
//-------------------------------------------[.cpp/bitcoin/src/node/context.cpp]

/**
  | NodeContext struct containing references to
  | chain state and connection state.
  |
  | This is used by init, rpc, and test code to
  | pass object references around without needing
  | to declare the same variables and parameters
  | repeatedly, or to use globals.
  |
  | More variables could be added to this struct
  | (particularly references to validation
  | objects) to eliminate use of globals and make
  | code more modular and testable. The struct
  | isn't intended to have any member functions.
  |
  | It should just be a collection of references
  | that can be used without pulling in unwanted
  | dependencies or functionality.
  */
pub struct NodeContext {

    /**
      | Init interface for initializing current
      | process and connecting to other processes.
      |
      */
    init:                   Rc<RefCell<dyn Init>>, // default = { nullptr }

    addrman:                Box<AddrMan>,
    connman:                Box<Connman>,
    mempool:                Box<TxMemPool>,
    fee_estimator:          Box<BlockPolicyEstimator>,
    peerman:                Box<PeerManager>,
    chainman:               Box<ChainstateManager>,
    banman:                 Box<BanMan>,

    /**
      | Currently a raw pointer because the
      | memory is not managed by this struct
      |
      */
    args:                   *mut ArgsManager, // default = { nullptr }

    chain:                  Box<dyn ChainInterface>,

    /**
      | List of all chain clients (wallet processes
      | or other client) connected to node.
      |
      */
    chain_clients:          Vec<Box<dyn ChainClient>>,

    /**
      | Reference to chain client that should
      | used to load or create wallets opened
      | by the gui.
      |
      */
    wallet_client:          Rc<RefCell<dyn WalletClient>>, // default = { nullptr }

    scheduler:              Box<Scheduler>,
    rpc_interruption_point: fn() -> (), // default = noop
}

impl NodeContextInterface for NodeContext {}
