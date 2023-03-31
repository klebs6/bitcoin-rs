crate::ix!();

/**
  | -paytxfee default
  |
  */
pub const DEFAULT_PAY_TX_FEE: Amount = 0;

/**
  | -fallbackfee default
  |
  */
pub const DEFAULT_FALLBACK_FEE: Amount = 0;

/**
  | -discardfee default
  |
  */
pub const DEFAULT_DISCARD_FEE: Amount = 10000;

/**
  | -mintxfee default
  |
  */
pub const DEFAULT_TRANSACTION_MINFEE: Amount = 1000;

/**
  | -consolidatefeerate default
  |
  */
pub const DEFAULT_CONSOLIDATE_FEERATE: Amount = 10000; // 10 sat/vbyte

/**
  | maximum fee increase allowed to do partial
  | spend avoidance, even for nodes with
  | this feature disabled by default
  | 
  | A value of -1 disables this feature completely.
  | 
  | A value of 0 (current default) means
  | to attempt to do partial spend avoidance,
  | and use its results if the fees remain
  | *unchanged*
  | 
  | A value > 0 means to do partial spend avoidance
  | if the fee difference against a regular
  | coin selection instance is in the range
  | [0..value].
  |
  */
pub const DEFAULT_MAX_AVOIDPARTIALSPEND_FEE: Amount = 0;

/**
  | discourage APS fee higher than this
  | amount
  |
  */
pub const HIGH_APS_FEE: Amount = {COIN / 10000};

/**
  | minimum recommended increment for
  | BIP 125 replacement txs
  |
  */
pub const WALLET_INCREMENTAL_RELAY_FEE: Amount = 5000;

/**
  | Default for -spendzeroconfchange
  |
  */
pub const DEFAULT_SPEND_ZEROCONF_CHANGE: bool = true;

/**
  | Default for -walletrejectlongchains
  |
  */
pub const DEFAULT_WALLET_REJECT_LONG_CHAINS: bool = false;

/**
  | -txconfirmtarget default
  |
  */
pub const DEFAULT_TX_CONFIRM_TARGET: u32 = 6;

/**
  | -walletrbf default
  |
  */
pub const DEFAULT_WALLET_RBF:      bool = false;
pub const DEFAULT_WALLETBROADCAST: bool = true;
pub const DEFAULT_DISABLE_WALLET:  bool = false;

/**
   -maxtxfee default
  */
pub const DEFAULT_TRANSACTION_MAXFEE: Amount = COIN / 10;

/**
  | Discourage users to set fees higher
  | than this amount (in satoshis) per kB
  |
  */
pub const HIGH_TX_FEE_PER_KB: Amount = COIN / 100;

/**
  | -maxtxfee will warn if called with a
  | higher fee than this amount (in satoshis)
  |
  */
pub const HIGH_MAX_TX_FEE: Amount = 100 * HIGH_TX_FEE_PER_KB;

/**
  | Pre-calculated constants for input
  | size estimation in *virtual size*
  |
  */
pub const DUMMY_NESTED_P2WPKH_INPUT_SIZE: usize = 91;

/**
  | Default for -addresstype
  |
  */
pub const DEFAULT_ADDRESS_TYPE: OutputType = OutputType::BECH32;

