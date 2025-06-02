crate::ix!();

impl Logger {

    /// Return the current bitmask of log categories
    pub fn get_category_mask(&self) -> u32 {
        self.categories().load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Returns a string with the log categories in alphabetical order
    pub fn log_categories_string(&self) -> String {
        let list = self.log_categories_list();
        let mut names: Vec<String> = list
            .into_iter()
            .map(|c| c.category().to_string())
            .collect();
        names.sort_unstable();
        names.join(", ")
    }

    pub fn enable_category_with_flags(&mut self, flag: LogFlags) {
        let oldval = self.categories().load(std::sync::atomic::Ordering::Relaxed);
        let newval = oldval | (flag as u32);
        self.categories().store(newval, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn enable_category(&mut self, str_: &String) -> bool {
        let mut f = LogFlags::NONE;
        if !get_log_category(&mut f, str_) {
            return false;
        }
        self.enable_category_with_flags(f);
        true
    }

    pub fn disable_category_with_flags(&mut self, flag: LogFlags) {
        let oldval = self.categories().load(std::sync::atomic::Ordering::Relaxed);
        let newval = oldval & !(flag as u32);
        self.categories().store(newval, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn disable_category(&mut self, str_: &String) -> bool {
        let mut f = LogFlags::NONE;
        if !get_log_category(&mut f, str_) {
            return false;
        }
        self.disable_category_with_flags(f);
        true
    }

    pub fn will_log_category(&self, category: LogFlags) -> bool {
        let mask = self.categories().load(std::sync::atomic::Ordering::Relaxed);
        (mask & (category as u32)) != 0
    }

    pub fn log_categories_list(&self) -> Vec<LogCategory> {
        let mut descs: Vec<LogCategoryDesc> = LOG_CATEGORIES.to_vec();
        descs.sort_by(|a, b| a.category().cmp(b.category()));

        let mut ret = Vec::new();
        for d in descs {
            if *d.flag() == LogFlags::NONE || *d.flag() == LogFlags::ALL {
                continue;
            }
            let active = self.will_log_category(*d.flag());
            ret.push(
                LogCategoryBuilder::default()
                    .category(*d.category())
                    .active(active)
                    .build()
                    .unwrap()
            );
        }
        ret
    }
}

#[cfg(test)]
mod logger_categories_methods_tests {
    use super::*;

    /// Tests that `get_category_mask` accurately reflects changes from enable/disable.
    #[traced_test]
    #[serial]
    fn test_get_category_mask() {
        info!("Testing get_category_mask changes after enable/disable_category_with_flags.");

        let mut logger = Logger::default();
        logger.categories().store(0, std::sync::atomic::Ordering::Relaxed);

        // Initially => 0
        assert_eq!(
            logger.get_category_mask(),
            0,
            "Category mask must start at 0"
        );

        // Enable NET => expect bitmask changed
        logger.enable_category_with_flags(LogFlags::NET);
        let mask_net = logger.get_category_mask();
        assert_ne!(mask_net, 0, "Enabling NET must change mask from 0");
        assert_eq!(
            mask_net & (LogFlags::NET as u32),
            LogFlags::NET as u32,
            "NET bit must be set in mask"
        );

        // Disable NET => back to 0
        logger.disable_category_with_flags(LogFlags::NET);
        assert_eq!(
            logger.get_category_mask(),
            0,
            "Disabling NET must bring mask back to 0"
        );

        trace!("test_get_category_mask passed.");
    }

    /// Ensures `log_categories_string` is alphabetically sorted and reflects current categories.
    #[traced_test]
    #[serial]
    fn test_log_categories_string() {
        info!("Testing log_categories_string for sorted, active categories.");

        let mut logger = Logger::default();
        logger.categories().store(0, std::sync::atomic::Ordering::Relaxed);
        logger.enable_category_with_flags(LogFlags::RPC);
        logger.enable_category_with_flags(LogFlags::ZMQ);
        logger.enable_category_with_flags(LogFlags::UTIL);

        let cat_str = logger.log_categories_string();
        debug!("log_categories_string => {}", cat_str);

        // Must contain 'rpc', 'util', 'zmq' in alphabetical order => "rpc, util, zmq"
        let expected = vec!["rpc", "util", "zmq"].join(", ");
        assert_eq!(
            cat_str, expected,
            "log_categories_string must be alphabetical and match the active categories"
        );

        trace!("test_log_categories_string passed.");
    }

    /// Tests `enable_category` with known/unknown strings, ensuring correct return values and changes.
    #[traced_test]
    #[serial]
    fn test_enable_category() {
        info!("Testing enable_category with known and unknown strings.");

        let mut logger = Logger::default();
        logger.categories().store(0, std::sync::atomic::Ordering::Relaxed);

        // 1) Known => 'mempool'
        let ok = logger.enable_category(&"mempool".to_string());
        assert!(ok, "enable_category('mempool') must succeed");
        assert!(
            logger.will_log_category(LogFlags::MEMPOOL),
            "mempool bit must be set after enabling"
        );

        // 2) Unknown => 'somejunk'
        let bad = logger.enable_category(&"somejunk".to_string());
        assert!(!bad, "enable_category('somejunk') must fail");
        assert!(
            !logger.will_log_category(LogFlags::NONE),
            "Should not set anything for unknown categories"
        );

        trace!("test_enable_category passed.");
    }

    /// Tests `disable_category` with known/unknown strings, ensuring correct return values.
    #[traced_test]
    #[serial]
    fn test_disable_category() {
        info!("Testing disable_category with known and unknown strings.");

        let mut logger = Logger::default();
        logger.categories().store(0, std::sync::atomic::Ordering::Relaxed);
        // Enable REINDEX
        logger.enable_category_with_flags(LogFlags::REINDEX);
        assert!(
            logger.will_log_category(LogFlags::REINDEX),
            "Should have REINDEX set after enabling it"
        );

        // 1) Known => 'reindex'
        let ok = logger.disable_category(&"reindex".to_string());
        assert!(ok, "disable_category('reindex') must succeed");
        assert!(
            !logger.will_log_category(LogFlags::REINDEX),
            "REINDEX must be cleared after disabling"
        );

        // 2) Unknown => 'somejunk'
        let bad = logger.disable_category(&"somejunk".to_string());
        assert!(!bad, "disable_category('somejunk') must fail");

        trace!("test_disable_category passed.");
    }

    /// Tests `will_log_category` in detail by toggling a single category.
    #[traced_test]
    #[serial]
    fn test_will_log_category() {
        info!("Testing will_log_category by toggling a category bit.");

        let mut logger = Logger::default();
        logger.categories().store(0, std::sync::atomic::Ordering::Relaxed);

        assert!(
            !logger.will_log_category(LogFlags::HTTP),
            "HTTP must be false initially"
        );

        logger.enable_category_with_flags(LogFlags::HTTP);
        assert!(
            logger.will_log_category(LogFlags::HTTP),
            "HTTP must be true after enabling"
        );

        logger.disable_category_with_flags(LogFlags::HTTP);
        assert!(
            !logger.will_log_category(LogFlags::HTTP),
            "HTTP must be false again after disabling"
        );

        trace!("test_will_log_category passed.");
    }

    /// Tests that `log_categories_list()` returns all categories (except NONE or ALL) sorted,
    /// and each is marked active/inactive correctly based on the logger’s bitmask.
    #[traced_test]
    #[serial]
    fn test_log_categories_list() {
        info!("Testing log_categories_list returns sorted and correctly active entries.");

        let mut logger = Logger::default();
        logger.categories().store(0, std::sync::atomic::Ordering::Relaxed);

        // Enable a few categories: NET, RPC, LOCK
        logger.enable_category_with_flags(LogFlags::NET);
        logger.enable_category_with_flags(LogFlags::RPC);
        logger.enable_category_with_flags(LogFlags::LOCK);

        let list = logger.log_categories_list();
        let mut active_names = Vec::new();
        let mut inactive_names = Vec::new();

        for cat in &list {
            if *cat.active() {
                active_names.push(*cat.category());
            } else {
                inactive_names.push(*cat.category());
            }
        }

        // Must be sorted in ascending order
        let mut all_names = list.iter().map(|c| c.category()).collect::<Vec<_>>();
        let sorted_copy = {
            let mut tmp = all_names.clone();
            tmp.sort_unstable();
            tmp
        };
        assert_eq!(all_names, sorted_copy, "Categories must be sorted in ascending order.");

        // Check that the ones we enabled are in active
        assert!(active_names.contains(&"net"), "NET must appear active.");
        assert!(active_names.contains(&"rpc"), "RPC must appear active.");
        assert!(active_names.contains(&"lock"), "LOCK must appear active.");

        // Sample check that 'mempool' is inactive
        assert!(inactive_names.contains(&"mempool"), "mempool must appear inactive since not enabled");

        trace!("test_log_categories_list passed.");
    }
}
