crate::ix!();

lazy_static!{
    pub static ref DEFAULT_BASE_PARAMS: BaseChainParams = *create_base_chain_params(&base_chain_params::MAIN).unwrap();
    pub static ref TESTNET_BASE_PARAMS: BaseChainParams = *create_base_chain_params(&base_chain_params::TESTNET).unwrap();
    pub static ref SIGNET_BASE_PARAMS:  BaseChainParams = *create_base_chain_params(&base_chain_params::SIGNET).unwrap();
    pub static ref REGTEST_BASE_PARAMS: BaseChainParams = *create_base_chain_params(&base_chain_params::REGTEST).unwrap();
}

pub const DEFAULT_RPCCONNECT:          &'static str = "127.0.0.1";
pub const DEFAULT_HTTP_CLIENT_TIMEOUT: i32   = 900;
pub const DEFAULT_WAIT_CLIENT_TIMEOUT: i32   = 0;
pub const DEFAULT_NAMED:               bool  = false;
pub const CONTINUE_EXECUTION:          i32   = -1;
pub const UNKNOWN_NETWORK:             i8    = -1;

/**
  | Default number of blocks to generate
  | for RPC generatetoaddress.
  |
  */
pub const DEFAULT_NBLOCKS: &'static str = "1";

/**
  | Default -color setting.
  |
  */
pub const DEFAULT_COLOR_SETTING: &'static str = "auto";

/**
  | Default max iterations to try in RPC
  | generatetodescriptor, generatetoaddress,
  | and generateblock.
  |
  */
pub const DEFAULT_MAX_TRIES: u64 = 1000000;
