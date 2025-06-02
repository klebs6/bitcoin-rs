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
