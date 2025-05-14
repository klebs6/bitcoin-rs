// ---------------- [ File: bitcoin-txmempool/src/txdb.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/txdb.h]
//-------------------------------------------[.cpp/bitcoin/src/txdb.cpp]

/**
  | -dbcache default (MiB)
  |
  */
pub const N_DEFAULT_DB_CACHE: usize = 450;

/**
  | -dbbatchsize default (bytes)
  |
  */
pub const N_DEFAULT_DB_BATCH_SIZE: usize = 16 << 20;

/**
  | max. -dbcache (MiB)
  |
  */
pub const N_MAX_DB_CACHE: usize = ternary!{
    size_of::<*mut c_void>() > 4, 
    16384, 
    1024
};

/**
  | min. -dbcache (MiB)
  |
  */
pub const N_MIN_DB_CACHE: usize = 4;

/**
  | Max memory allocated to block tree DB
  | specific cache, if no -txindex (MiB)
  |
  */
pub const N_MAX_BLOCK_DB_CACHE: usize = 2;

/**
  | Max memory allocated to block tree DB specific
  | cache, if -txindex (MiB)
  |
  | Unlike for the UTXO database, for the txindex
  | scenario the leveldb cache make a meaningful
  | difference:
  | https://github.com/bitcoin/bitcoin/pull/8273#issuecomment-229601991
  */
pub const N_MAX_TX_INDEX_CACHE: usize = 1024;

/**
  | Max memory allocated to all block filter
  | index caches combined in MiB.
  |
  */
pub const MAX_FILTER_INDEX_CACHE: usize = 1024;

/**
  | Max memory allocated to coin DB specific
  | cache (MiB)
  |
  */
pub const N_MAX_COINS_DB_CACHE: usize = 8;

/**
  | Actually declared in validation.cpp;
  | can't include because of circular dependency.
  |
  */
lazy_static!{
    /*
    extern RecursiveMutex cs_main;
    */
}
