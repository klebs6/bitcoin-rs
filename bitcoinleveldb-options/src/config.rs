// ---------------- [ File: bitcoinleveldb-options/src/config.rs ]
crate::ix!();

pub const NUM_NON_TABLE_CACHE_FILES: i32 = 10;

pub fn table_cache_size(sanitized_options: &Options) -> i32 {
    // Reserve ten files or so for other uses and give the rest to TableCache.
    *sanitized_options.max_open_files() - NUM_NON_TABLE_CACHE_FILES
}

#[cfg(test)]
mod config_table_cache_size_suite {
    use super::*;
    use tracing::{debug, info, trace};

    #[traced_test]
    fn table_cache_size_reserves_non_table_cache_files_from_max_open_files() {
        trace!("config_table_cache_size_suite: start");

        let mut opts = Options::default();
        opts.set_max_open_files(1000);

        let got = table_cache_size(&opts);
        info!(max_open_files = *opts.max_open_files(), got, "computed table_cache_size");

        assert_eq!(got, 1000 - NUM_NON_TABLE_CACHE_FILES);

        trace!("config_table_cache_size_suite: done");
    }

    #[traced_test]
    fn table_cache_size_can_be_zero_or_negative_if_max_open_files_is_small() {
        trace!("config_table_cache_size_suite: start");

        let mut opts = Options::default();

        opts.set_max_open_files(NUM_NON_TABLE_CACHE_FILES);
        let got0 = table_cache_size(&opts);
        debug!(max_open_files = *opts.max_open_files(), got0, "boundary at exactly reserved count");
        assert_eq!(got0, 0);

        opts.set_max_open_files(NUM_NON_TABLE_CACHE_FILES - 1);
        let got_neg = table_cache_size(&opts);
        debug!(max_open_files = *opts.max_open_files(), got_neg, "below reserved count");
        assert_eq!(got_neg, -1);

        trace!("config_table_cache_size_suite: done");
    }
}
