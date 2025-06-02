crate::ix!();

impl Logger {

    /// Return the current bitmask of log categories as a raw `u32`.
    pub fn get_category_mask(&self) -> u32 {
        trace!("Logger::get_category_mask => retrieving categories from AtomicU32");
        self.categories.load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Returns a string with the log categories in alphabetical order.
    pub fn log_categories_string(&self) -> String {
        trace!("Logger::log_categories_string => retrieving sorted category list");
        let list = self.log_categories_list();
        // We now call `.category()` to fetch the &str
        let mut names: Vec<String> = list.into_iter()
            .map(|c| c.category().to_string())
            .collect();

        names.sort_unstable();
        let result = names.join(", ");
        trace!("Logger::log_categories_string => result='{}'", result);
        result
    }

    pub fn enable_category_with_flags(&mut self, flag: LogFlags) {
        trace!("Logger::enable_category_with_flags => enabling category bits: 0x{:X}", flag as u32);
        let oldval = self.categories.load(std::sync::atomic::Ordering::Relaxed);
        let newval = oldval | (flag as u32);
        self.categories.store(newval, std::sync::atomic::Ordering::Relaxed);
        trace!("Logger::enable_category_with_flags => old=0x{:X}, new=0x{:X}", oldval, newval);
    }

    pub fn enable_category(&mut self, str_: &String) -> bool {
        trace!("Logger::enable_category => attempt parse category='{}'", str_);
        let mut f = LogFlags::NONE;
        if !get_log_category(&mut f, str_) {
            trace!("Logger::enable_category => parse failed => false");
            return false;
        }
        self.enable_category_with_flags(f);
        trace!("Logger::enable_category => parse success => enabled 0x{:X}", f as u32);
        true
    }

    pub fn disable_category_with_flags(&mut self, flag: LogFlags) {
        trace!("Logger::disable_category_with_flags => disabling category bits: 0x{:X}", flag as u32);
        let oldval = self.categories.load(std::sync::atomic::Ordering::Relaxed);
        let newval = oldval & !(flag as u32);
        self.categories.store(newval, std::sync::atomic::Ordering::Relaxed);
        trace!("Logger::disable_category_with_flags => old=0x{:X}, new=0x{:X}", oldval, newval);
    }

    pub fn disable_category(&mut self, str_: &String) -> bool {
        trace!("Logger::disable_category => attempt parse category='{}'", str_);
        let mut f = LogFlags::NONE;
        if !get_log_category(&mut f, str_) {
            trace!("Logger::disable_category => parse failed => false");
            return false;
        }
        self.disable_category_with_flags(f);
        trace!("Logger::disable_category => parse success => disabled 0x{:X}", f as u32);
        true
    }

    pub fn will_log_category(&self, category: LogFlags) -> bool {
        let mask = self.categories.load(std::sync::atomic::Ordering::Relaxed);
        let result = (mask & (category as u32)) != 0;
        trace!("Logger::will_log_category => category=0x{:X}, mask=0x{:X}, result={}", category as u32, mask, result);
        result
    }

    pub fn log_categories_list(&self) -> Vec<LogCategory> {
        trace!("Logger::log_categories_list => building sorted list of categories except NONE & ALL");

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

        trace!("Logger::log_categories_list => found {} categories", ret.len());
        ret
    }
}
