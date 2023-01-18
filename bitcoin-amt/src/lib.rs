use bitcoin_derive::*;

#[macro_use] use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/consensus/consensus.h]

/**
  | The maximum allowed size for a serialized
  | block, in bytes (only for buffer size
  | limits)
  |
  */
pub const MAX_BLOCK_SERIALIZED_SIZE:           usize = 4000000;

/**
  | The maximum allowed weight for a block,
  | see BIP 141 (network rule)
  |
  */
pub const MAX_BLOCK_WEIGHT:                    usize = 4000000;

/**
  | The maximum allowed number of signature
  | check operations in a block (network
  | rule)
  |
  */
pub const MAX_BLOCK_SIGOPS_COST:               usize = 80000;

/**
  | Coinbase transaction outputs can only
  | be spent after this number of new blocks
  | (network rule)
  |
  */
pub const COINBASE_MATURITY:                   usize = 100;
pub const WITNESS_SCALE_FACTOR:                usize = 4;

/**
  | 60 is the lower bound for the size of a
  | valid serialized CTransaction
  |
  */
pub const MIN_TRANSACTION_WEIGHT:              usize = WITNESS_SCALE_FACTOR * 60;

/**
  | 10 is the lower bound for the size of a
  | serialized CTransaction
  |
  */
pub const MIN_SERIALIZABLE_TRANSACTION_WEIGHT: usize = WITNESS_SCALE_FACTOR * 10;


/* ---- Flags for nSequence and nLockTime locks  ---- */

/**
  | Interpret sequence numbers as relative
  | lock-time constraints.
  |
  */
pub const LOCKTIME_VERIFY_SEQUENCE: usize = 1 << 0;

/**
  | Use GetMedianTimePast() instead of
  | nTime for end point timestamp.
  |
  */
pub const LOCKTIME_MEDIAN_TIME_PAST: usize = 1 << 1;

//-------------------------------------------[.cpp/bitcoin/src/consensus/amount.h]

/**
  | Amount in satoshis (Can be negative)
  |
  */
#[Q_METATYPE]
pub type Amount = i64;

/**
  | The amount of satoshis in one BTC.
  |
  */
pub const COIN: Amount = 100000000;

/**
  | No amount larger than this (in satoshi)
  | is valid.
  | 
  | -----------
  | @note
  | 
  | this constant is *not* the total money
  | supply, which in Bitcoin currently
  | happens to be less than 21,000,000 BTC
  | for various reasons, but rather a sanity
  | check. As this sanity check is used by
  | consensus-critical validation code,
  | the exact value of the MAX_MONEY constant
  | is consensus critical; in unusual circumstances
  | like a(nother) overflow bug that allowed
  | for the creation of coins out of thin
  | air modification could lead to a fork.
  |
  */
pub const MAX_MONEY: Amount = 21000000 * COIN;

#[inline] pub fn money_range(n_value: &Amount) -> bool {
    *n_value >= 0 && *n_value <= MAX_MONEY
}

//-------------------------------------------[.cpp/bitcoin/src/wallet/ismine.h]

/**
  | IsMine() return codes, which depend
  | on ScriptPubKeyMan implementation.
  | 
  | Not every ScriptPubKeyMan covers all
  | types, please refer to https://github.com/bitcoin/bitcoin/blob/master/doc/release-notes/release-notes-0.21.0.md#ismine-semantics
  | for better understanding.
  | 
  | For LegacyScriptPubKeyMan,
  | 
  | ISMINE_NO: the scriptPubKey is not
  | in the wallet;
  | 
  | ISMINE_WATCH_ONLY: the scriptPubKey
  | has been imported into the wallet;
  | 
  | ISMINE_SPENDABLE: the scriptPubKey
  | corresponds to an address owned by the
  | wallet user (can spend with the private
  | key);
  | 
  | ISMINE_USED: the scriptPubKey corresponds
  | to a used address owned by the wallet
  | user;
  | 
  | ISMINE_ALL: all ISMINE flags except
  | for USED;
  | 
  | ISMINE_ALL_USED: all ISMINE flags
  | including USED;
  | 
  | ISMINE_ENUM_ELEMENTS: the number
  | of isminetype enum elements.
  | 
  | For DescriptorScriptPubKeyMan and
  | future ScriptPubKeyMan,
  | 
  | ISMINE_NO: the scriptPubKey is not
  | in the wallet;
  | 
  | ISMINE_SPENDABLE: the scriptPubKey
  | matches a scriptPubKey in the wallet.
  | 
  | ISMINE_USED: the scriptPubKey corresponds
  | to a used address owned by the wallet
  | user.
  |
  */
bitflags!{
    pub struct IsMineType: u32
    {
        const ISMINE_NO            = 0;
        const ISMINE_WATCH_ONLY    = 1 << 0;
        const ISMINE_SPENDABLE     = 1 << 1;
        const ISMINE_USED          = 1 << 2;
        const ISMINE_ALL           = Self::ISMINE_WATCH_ONLY.bits | Self::ISMINE_SPENDABLE.bits;
        const ISMINE_ALL_USED      = Self::ISMINE_ALL.bits        | Self::ISMINE_USED.bits;
        const ISMINE_ENUM_ELEMENTS = Self::ISMINE_ALL_USED.bits + 1;
    }
}

/**
  | used for bitflags of isminetype
  |
  */
pub type IsMineFilter = u8;

/**
  | Cachable amount subdivided into watchonly
  | and spendable parts.
  |
  */
pub struct CachableAmount {

    /**
      | NO and ALL are never (supposed to be)
      | cached
      |
      */
    //cached: BitSet<{IsMineType::ISMINE_ENUM_ELEMENTS as u32}>,
    cached: BitSet,
    value:  [Amount; IsMineType::ISMINE_ENUM_ELEMENTS.bits as usize],
}

impl CachableAmount {

    #[inline] pub fn reset(&mut self)  {
        self.cached.reset();
    }
    
    pub fn set(&mut self, 
        filter: IsMineFilter,
        value:  Amount)  {
        self.cached.set(filter.into(), true);
        self.value[filter as usize] = value;
    }
}
