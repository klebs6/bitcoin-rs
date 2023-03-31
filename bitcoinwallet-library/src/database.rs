crate::ix!();

/**
  | Overview of wallet database classes:
  | 
  | - WalletBatch is an abstract modifier
  | object for the wallet database, and
  | encapsulates a database batch update
  | as well as methods to act on the database.
  | It should be agnostic to the database
  | implementation.
  | 
  | The following classes are implementation
  | specific:
  | 
  | - BerkeleyEnvironment is an environment
  | in which the database exists.
  | 
  | - BerkeleyDatabase represents a wallet
  | database.
  | 
  | - BerkeleyBatch is a low-level database
  | batch update.
  |
  */
pub const DEFAULT_FLUSHWALLET: bool = true;

/**
  | Callback for filtering key types to
  | deserialize in ReadKeyValue
  |
  */
pub type KeyFilterFn = fn(_0: &String) -> bool;

//-------------------------------------------[.cpp/bitcoin/src/wallet/walletdb.cpp]

pub mod db_keys {
    pub const ACENTRY:                 &'static str = "acentry";
    pub const ACTIVEEXTERNALSPK:       &'static str = "activeexternalspk";
    pub const ACTIVEINTERNALSPK:       &'static str = "activeinternalspk";
    pub const BESTBLOCK_NOMERKLE:      &'static str = "bestblock_nomerkle";
    pub const BESTBLOCK:               &'static str = "bestblock";
    pub const CRYPTED_KEY:             &'static str = "ckey";
    pub const CSCRIPT:                 &'static str = "cscript";
    pub const DEFAULTKEY:              &'static str = "defaultkey";
    pub const DESTDATA:                &'static str = "destdata";
    pub const FLAGS:                   &'static str = "flags";
    pub const HDCHAIN:                 &'static str = "hdchain";
    pub const KEYMETA:                 &'static str = "keymeta";
    pub const KEY:                     &'static str = "key";
    pub const LOCKED_UTXO:             &'static str = "lockedutxo";
    pub const MASTER_KEY:              &'static str = "mkey";
    pub const MINVERSION:              &'static str = "minversion";
    pub const NAME:                    &'static str = "name";
    pub const OLD_KEY:                 &'static str = "wkey";
    pub const ORDERPOSNEXT:            &'static str = "orderposnext";
    pub const POOL:                    &'static str = "pool";
    pub const PURPOSE:                 &'static str = "purpose";
    pub const SETTINGS:                &'static str = "settings";
    pub const TX:                      &'static str = "tx";
    pub const VERSION:                 &'static str = "version";
    pub const WALLETDESCRIPTOR:        &'static str = "walletdescriptor";
    pub const WALLETDESCRIPTORCACHE:   &'static str = "walletdescriptorcache";
    pub const WALLETDESCRIPTORLHCACHE: &'static str = "walletdescriptorlhcache";
    pub const WALLETDESCRIPTORCKEY:    &'static str = "walletdescriptorckey";
    pub const WALLETDESCRIPTORKEY:     &'static str = "walletdescriptorkey";
    pub const WATCHMETA:               &'static str = "watchmeta";
    pub const WATCHS:                  &'static str = "watchs";
}
