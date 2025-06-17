// ---------------- [ File: bitcoin-log/src/flags.rs ]
crate::ix!();

#[repr(u32)]
#[derive(Clone,PartialEq,Eq,PartialOrd,Ord,Hash,Copy,Debug)]
pub enum LogFlags {
    NONE        = 0,
    NET         = 1 <<  0,
    TOR         = 1 <<  1,
    MEMPOOL     = 1 <<  2,
    HTTP        = 1 <<  3,
    BENCH       = 1 <<  4,
    ZMQ         = 1 <<  5,
    WALLETDB    = 1 <<  6,
    RPC         = 1 <<  7,
    ESTIMATEFEE = 1 <<  8,
    ADDRMAN     = 1 <<  9,
    SELECTCOINS = 1 << 10,
    REINDEX     = 1 << 11,
    CMPCTBLOCK  = 1 << 12,
    RAND        = 1 << 13,
    PRUNE       = 1 << 14,
    PROXY       = 1 << 15,
    MEMPOOLREJ  = 1 << 16,
    LIBEVENT    = 1 << 17,
    COINDB      = 1 << 18,
    QT          = 1 << 19,
    LEVELDB     = 1 << 20,
    VALIDATION  = 1 << 21,
    I2P         = 1 << 22,
    IPC         = 1 << 23,
    LOCK        = 1 << 24,
    UTIL        = 1 << 25,
    BLOCKSTORE  = 1 << 26,
    ALL         = 0xFFFFFFFF,
}
