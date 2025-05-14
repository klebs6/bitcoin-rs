// ---------------- [ File: bitcoin-argsman/src/options_category.rs ]
crate::ix!();

#[derive(PartialEq,Eq,PartialOrd,Ord,Hash)]
pub enum OptionsCategory {
    OPTIONS,
    CONNECTION,
    WALLET,
    WALLET_DEBUG_TEST,
    ZMQ,
    DEBUG_TEST,
    CHAINPARAMS,
    NODE_RELAY,
    BLOCK_CREATION,
    RPC,
    GUI,
    COMMANDS,
    REGISTER_COMMANDS,

    /**
      | Always the last option to avoid printing
      | these in the help
      |
      */
    HIDDEN, 
}
