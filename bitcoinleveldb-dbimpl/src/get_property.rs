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

            if !ok || level as i32 >= NUM_LEVELS {
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

            for level in 0..NUM_LEVELS {
                let files: i32 = unsafe { (*self.versions).num_level_files(level) };
                if self.stats[level as usize].micros > 0 || files > 0 {
                    let line = format!(
                        "{:3} {:8} {:8.0} {:9.0} {:8.0} {:9.0}\n",
                        level,
                        files,
                        unsafe { (*self.versions).num_level_bytes(level) } as f64 / 1048576.0,
                        self.stats[level as usize].micros as f64 / 1e6,
                        self.stats[level as usize].bytes_read as f64 / 1048576.0,
                        self.stats[level as usize].bytes_written as f64 / 1048576.0
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
            let mut total_usage: usize = unsafe { (*self.options.block_cache()).total_charge() };

            if !self.mem.is_null() {
                total_usage += unsafe { (*self.mem).approximate_memory_usage() };
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
