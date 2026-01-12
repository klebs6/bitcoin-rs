// ---------------- [ File: bitcoinleveldb-dbimpl/src/get_property.rs ]
crate::ix!();

impl DBGetProperty for DBImpl {

    fn get_property(&mut self, property: &str, value: *mut String) -> bool {
        unsafe {
            (*value).clear();
        }

        self.mutex.lock();

        let mut input: Slice = Slice::from_str(property);
        let prefix: Slice = Slice::from_str("leveldb.");

        if !input.starts_with(prefix) {
            self.mutex.unlock();
            return false;
        }

        input.remove_prefix(prefix.size());

        if input.starts_with_str("num-files-at-level") {
            input.remove_prefix_str("num-files-at-level");

            let mut level: u64 = 0;
            let ok: bool = consume_decimal_number(&mut input, &mut level) && input.empty();

            if !ok || level as i32 >= config::kNumLevels {
                self.mutex.unlock();
                return false;
            }

            let files: i32 = unsafe { (*self.versions).num_level_files(level as i32) };
            unsafe {
                (*value) = files.to_string();
            }

            self.mutex.unlock();
            return true;
        } else if input.eq_str("stats") {
            unsafe {
                (*value).push_str(
                    "                               Compactions\n\
                     Level  Files Size(MB) Time(sec) Read(MB) Write(MB)\n\
                     --------------------------------------------------\n",
                );
            }

            for level in 0..config::kNumLevels {
                let files: i32 = unsafe { (*self.versions).num_level_files(level) };
                if self.stats_[level as usize].micros > 0 || files > 0 {
                    let line = format!(
                        "{:3} {:8} {:8.0} {:9.0} {:8.0} {:9.0}\n",
                        level,
                        files,
                        unsafe { (*self.versions).num_level_bytes(level) } as f64 / 1048576.0,
                        self.stats_[level as usize].micros as f64 / 1e6,
                        self.stats_[level as usize].bytes_read as f64 / 1048576.0,
                        self.stats_[level as usize].bytes_written as f64 / 1048576.0
                    );
                    unsafe {
                        (*value).push_str(&line);
                    }
                }
            }

            self.mutex.unlock();
            return true;
        } else if input.eq_str("sstables") {
            let dbg = unsafe { (*(*self.versions).current()).debug_string() };
            unsafe {
                (*value) = dbg;
            }
            self.mutex.unlock();
            return true;
        } else if input.eq_str("approximate-memory-usage") {
            let mut total_usage: usize = unsafe { (*self.options_.block_cache).total_charge() };

            if !self.mem_.is_null() {
                total_usage += unsafe { (*self.mem_).approximate_memory_usage() };
            }

            if !self.imm.is_null() {
                total_usage += unsafe { (*self.imm).approximate_memory_usage() };
            }

            unsafe {
                (*value).push_str(&format!("{}", total_usage as u64));
            }

            self.mutex.unlock();
            return true;
        }

        self.mutex.unlock();
        false
    }
}

#[cfg(test)]
#[disable]
mod get_property_exhaustive_suite {
    use super::*;

    #[traced_test]
    fn get_property_handles_known_and_unknown_properties_and_formats_values() {
        let (dbname, mut db) = open_dbimpl_for_test("get_property_handles_known_and_unknown_properties_and_formats_values");

        // Unknown property.
        let mut out: String = String::new();
        let ok = <DBImpl as DBGetProperty>::get_property(&mut *db, "leveldb.unknown-property", (&mut out) as *mut String);
        tracing::info!(ok, out = %out, "unknown property");
        assert!(!ok, "unknown property must return false");
        assert!(out.is_empty(), "value must be cleared for unknown property");

        // stats
        let mut stats: String = String::new();
        let ok_stats = <DBImpl as DBGetProperty>::get_property(&mut *db, "leveldb.stats", (&mut stats) as *mut String);
        tracing::info!(ok_stats, len = stats.len(), "stats property");
        assert!(ok_stats, "stats property should be supported");
        assert!(stats.contains("Compactions"), "stats output should contain header");

        // sstables
        let mut sst: String = String::new();
        let ok_sst = <DBImpl as DBGetProperty>::get_property(&mut *db, "leveldb.sstables", (&mut sst) as *mut String);
        tracing::info!(ok_sst, len = sst.len(), "sstables property");
        assert!(ok_sst, "sstables property should be supported");

        // approximate memory usage
        let mut mem: String = String::new();
        let ok_mem = <DBImpl as DBGetProperty>::get_property(
            &mut *db,
            "leveldb.approximate-memory-usage",
            (&mut mem) as *mut String,
        );
        tracing::info!(ok_mem, mem = %mem, "approximate-memory-usage");
        assert!(ok_mem, "approximate-memory-usage should be supported");
        assert!(!mem.is_empty(), "approximate-memory-usage should be non-empty");

        drop(db);
        remove_db_dir_best_effort(&dbname);
    }
}
