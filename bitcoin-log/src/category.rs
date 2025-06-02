// ---------------- [ File: bitcoin-log/src/category.rs ]
crate::ix!();

#[derive(Builder,Getters)]
#[builder(setter(into))]
#[getset(get="pub")]
pub struct LogCategory {
    category: &'static str,
    active:   bool,
}

#[derive(Clone, Copy, Getters)]
#[getset(get = "pub")]
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

pub fn get_log_category(
    flag: &mut LogFlags,
    str_: &String
) -> bool {
    if str_.is_empty() {
        *flag = LogFlags::ALL;
        return true;
    }

    for category_desc in LOG_CATEGORIES.iter() {
        if category_desc.category() == str_ {
            *flag = *category_desc.flag();
            return true;
        }
    }

    false
}
