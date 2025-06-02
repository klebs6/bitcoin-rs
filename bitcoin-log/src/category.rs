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

#[cfg(test)]
mod log_category_tests {
    use super::*;

    /// Test constructing a `LogCategory` with `LogCategoryBuilder` and ensure fields are correct.
    #[traced_test]
    #[serial]
    fn test_log_category_builder() {
        info!("Testing LogCategoryBuilder and derived getters.");

        let cat = LogCategoryBuilder::default()
            .category("mempool")
            .active(true)
            .build()
            .expect("Building LogCategory should succeed.");

        assert_eq!(*cat.category(), "mempool", "Category field must match builder input.");
        assert!(cat.active(), "Active field must match builder input.");

        trace!("test_log_category_builder passed.");
    }

    /// Test that each `LogCategoryDesc` in `LOG_CATEGORIES` has a coherent `flag` and `category`.
    #[traced_test]
    #[serial]
    fn test_log_category_desc_invariants() {
        info!("Testing invariants of all LogCategoryDesc items in LOG_CATEGORIES.");

        for desc in LOG_CATEGORIES.iter() {
            debug!("Checking desc: flag={:?}, category={}", desc.flag(), desc.category());

            // Basic check: category must never be empty
            assert!(!desc.category().is_empty(), "Category must be non‐empty string.");
            
            // Additional checks depending on your rules...
            // e.g. if `flag == LogFlags::NONE`, we expect "0" or "none".
            if *desc.flag() == LogFlags::NONE {
                let cat = desc.category();
                assert!(
                    *cat == "0" || *cat == "none",
                    "LogFlags::NONE must match category '0' or 'none', got='{}'",
                    cat
                );
            }
        }

        trace!("test_log_category_desc_invariants passed.");
    }

    /// Test `get_log_category()` with empty string => should produce LogFlags::ALL.
    #[traced_test]
    #[serial]
    fn test_get_log_category_empty_string() {
        info!("Testing get_log_category(empty) => LogFlags::ALL.");

        let mut f = LogFlags::NONE;
        let ok = get_log_category(&mut f, &"".to_string());
        assert!(ok, "Must succeed for empty string => ALL category.");
        assert_eq!(f, LogFlags::ALL, "Empty string should yield ALL.");

        trace!("test_get_log_category_empty_string passed.");
    }

    /// Test `get_log_category()` with known categories => must return correct flags.
    #[traced_test]
    #[serial]
    fn test_get_log_category_known() {
        info!("Testing get_log_category with known category strings.");

        let mut f = LogFlags::NONE;

        let ok_net = get_log_category(&mut f, &"net".to_string());
        assert!(ok_net, "Should succeed with 'net'.");
        assert_eq!(f, LogFlags::NET, "Parsing 'net' => LogFlags::NET.");

        let ok_all = get_log_category(&mut f, &"all".to_string());
        assert!(ok_all, "Should succeed with 'all'.");
        assert_eq!(f, LogFlags::ALL, "'all' => LogFlags::ALL.");

        trace!("test_get_log_category_known passed.");
    }

    /// Test `get_log_category()` with unknown categories => must fail and leave `flag` unchanged.
    #[traced_test]
    #[serial]
    fn test_get_log_category_unknown() {
        info!("Testing get_log_category with unknown strings => must fail.");

        let mut f = LogFlags::NONE;
        let bad = get_log_category(&mut f, &"unknownstuff".to_string());
        assert!(!bad, "Unknown category must return false.");
        assert_eq!(f, LogFlags::NONE, "Flag must remain NONE on failure.");

        trace!("test_get_log_category_unknown passed.");
    }

    /// Test `log_accept_category()` in isolation. We toggle the global logger’s categories and verify
    /// acceptance or rejection.
    #[traced_test]
    #[serial]
    fn test_log_accept_category_toggling() {
        info!("Testing log_accept_category with the global logger instance.");

        let global_logger = log_instance();
        // Clear all categories first
        global_logger.categories().store(0, std::sync::atomic::Ordering::Relaxed);

        // By default, 'NET' is disabled
        assert!(!log_accept_category(LogFlags::NET), "NET must be disabled initially.");

        // Enable 'NET'
        global_logger.enable_category_with_flags(LogFlags::NET);
        assert!(log_accept_category(LogFlags::NET), "Enabling NET => log_accept_category(NET)==true.");

        // Disable 'NET'
        global_logger.disable_category_with_flags(LogFlags::NET);
        assert!(!log_accept_category(LogFlags::NET), "Disabling NET => false.");

        trace!("test_log_accept_category_toggling passed.");
    }
}
