// ---------------- [ File: bitcoin-log/src/category.rs ]
crate::ix!();

#[repr(u32)]
#[derive(Clone,Debug)]
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

pub struct LogCategory {
    category: &'static str,
    active:   bool,
}

pub struct LogCategoryDesc {
    flag:     LogFlags,
    category: &'static str,
}

/**
  | Return true if log accepts specified
  | category
  |
  */
#[inline] pub fn log_accept_category(category: LogFlags) -> bool {
    log_instance().will_log_category(category)
}

pub const LOG_CATEGORIES: &[LogCategoryDesc] = &[
        LogCategoryDesc {flag: LogFlags::NONE,        category: "0"},
        LogCategoryDesc {flag: LogFlags::NONE,        category: "none"},
        LogCategoryDesc {flag: LogFlags::NET,         category: "net"},
        LogCategoryDesc {flag: LogFlags::TOR,         category: "tor"},
        LogCategoryDesc {flag: LogFlags::MEMPOOL,     category: "mempool"},
        LogCategoryDesc {flag: LogFlags::HTTP,        category: "http"},
        LogCategoryDesc {flag: LogFlags::BENCH,       category: "bench"},
        LogCategoryDesc {flag: LogFlags::ZMQ,         category: "zmq"},
        LogCategoryDesc {flag: LogFlags::WALLETDB,    category: "walletdb"},
        LogCategoryDesc {flag: LogFlags::RPC,         category: "rpc"},
        LogCategoryDesc {flag: LogFlags::ESTIMATEFEE, category: "estimatefee"},
        LogCategoryDesc {flag: LogFlags::ADDRMAN,     category: "addrman"},
        LogCategoryDesc {flag: LogFlags::SELECTCOINS, category: "selectcoins"},
        LogCategoryDesc {flag: LogFlags::REINDEX,     category: "reindex"},
        LogCategoryDesc {flag: LogFlags::CMPCTBLOCK,  category: "cmpctblock"},
        LogCategoryDesc {flag: LogFlags::RAND,        category: "rand"},
        LogCategoryDesc {flag: LogFlags::PRUNE,       category: "prune"},
        LogCategoryDesc {flag: LogFlags::PROXY,       category: "proxy"},
        LogCategoryDesc {flag: LogFlags::MEMPOOLREJ,  category: "mempoolrej"},
        LogCategoryDesc {flag: LogFlags::LIBEVENT,    category: "libevent"},
        LogCategoryDesc {flag: LogFlags::COINDB,      category: "coindb"},
        LogCategoryDesc {flag: LogFlags::QT,          category: "qt"},
        LogCategoryDesc {flag: LogFlags::LEVELDB,     category: "leveldb"},
        LogCategoryDesc {flag: LogFlags::VALIDATION,  category: "validation"},
        LogCategoryDesc {flag: LogFlags::I2P,         category: "i2p"},
        LogCategoryDesc {flag: LogFlags::IPC,         category: "ipc"},
        LogCategoryDesc {flag: LogFlags::LOCK,        category: "lock"},
        LogCategoryDesc {flag: LogFlags::UTIL,        category: "util"},
        LogCategoryDesc {flag: LogFlags::BLOCKSTORE,  category: "blockstorage"},
        LogCategoryDesc {flag: LogFlags::ALL,         category: "1"},
        LogCategoryDesc {flag: LogFlags::ALL,         category: "all"},
];

/**
  | Return true if str parses as a log category
  | and set the flag
  |
  */
pub fn get_log_category(
        flag: &mut LogFlags,
        str_: &String) -> bool {
    
    if str_ == "" {
        *flag = LogFlags::ALL;
        return true;
    }

    for category_desc in LOG_CATEGORIES.iter() {
        if category_desc.category == str_ {
            *flag = category_desc.flag.clone();
            return true;
        }
    }

    false
}
