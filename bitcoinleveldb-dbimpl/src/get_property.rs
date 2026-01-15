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

        if !input.starts_with(&prefix) {
            unsafe { self.mutex.unlock() };
            return false;
        }

        let prefix_len: usize = *prefix.size();
        input.remove_prefix(prefix_len);

        let num_files_prefix: Slice = Slice::from_str("num-files-at-level");
        let stats_key:        Slice = Slice::from_str("stats");
        let sstables_key:     Slice = Slice::from_str("sstables");
        let approx_mem_key:   Slice = Slice::from_str("approximate-memory-usage");

        if input.starts_with(&num_files_prefix) {
            let nfp_len: usize = *num_files_prefix.size();
            input.remove_prefix(nfp_len);

            let mut level: u64 = 0;
            let ok: bool = consume_decimal_number(&mut input, &mut level) && input.empty();

            if !ok || level >= (NUM_LEVELS as u64) {
                unsafe { self.mutex.unlock() };
                return false;
            }

            let files: i32 = unsafe { (*self.versions).num_level_files(level as i32) };
            unsafe {
                (*value) = files.to_string();
            }

            unsafe { self.mutex.unlock() };
            return true;
        } else if input.compare(&stats_key) == 0 {
            unsafe {
                (*value).push_str(
                    "                               Compactions\n\
                     Level  Files Size(MB) Time(sec) Read(MB) Write(MB)\n\
                     --------------------------------------------------\n",
                );
            }

            for level in 0..NUM_LEVELS {
                let level_i32: i32 = level as i32;
                let files: i32 = unsafe { (*self.versions).num_level_files(level_i32) };

                if *self.stats[level].micros() > 0 || files > 0 {
                    let line = format!(
                        "{:3} {:8} {:8.0} {:9.0} {:8.0} {:9.0}\n",
                        level,
                        files,
                        unsafe { (*self.versions).num_level_bytes(level_i32) } as f64 / 1048576.0,
                        *self.stats[level].micros() as f64 / 1e6,
                        *self.stats[level].bytes_read() as f64 / 1048576.0,
                        *self.stats[level].bytes_written() as f64 / 1048576.0
                    );
                    unsafe {
                        (*value).push_str(&line);
                    }
                }
            }

            unsafe { self.mutex.unlock() };
            return true;
        } else if input.compare(&sstables_key) == 0 {
            let dbg = unsafe { (*(*self.versions).current()).debug_string() };
            unsafe {
                (*value) = dbg;
            }
            unsafe { self.mutex.unlock() };
            return true;
        } else if input.compare(&approx_mem_key) == 0 {
            let mut total_usage: usize = (*self.options.block_cache()).total_charge();

            if !self.mem.is_null() {
                total_usage += unsafe { (*self.mem).approximate_memory_usage() };
            }

            if !self.imm.is_null() {
                total_usage += unsafe { (*self.imm).approximate_memory_usage() };
            }

            unsafe {
                (*value).push_str(&format!("{}", total_usage as u64));
            }

            unsafe { self.mutex.unlock() };
            return true;
        }

        unsafe { self.mutex.unlock() };
        false
    }
}

#[cfg(test)]
mod db_get_property_contract_suite {
    use super::*;
    use bitcoinleveldb_dbinterface::DBGetProperty;

    fn assert_dbimpl_implements_db_get_property() {
        fn _assert<T: DBGetProperty>() {}
        _assert::<DBImpl>();
    }

    fn compile_only_accepts_db_get_property_trait_object(_db: &mut dyn DBGetProperty) {}

    fn compile_only_get_property_call_via_trait_object(
        db: &mut dyn DBGetProperty,
        property: &str,
        out: &mut String,
    ) -> bool {
        // Intentionally takes `&mut String` so it can coerce to a raw out-pointer
        // if the underlying interface uses `*mut String`.
        db.get_property(property, out)
    }

    #[traced_test]
    fn db_get_property_contract_dbimpl_implements_trait() {
        tracing::trace!("begin DBGetProperty contract: DBImpl implements DBGetProperty");
        assert_dbimpl_implements_db_get_property();
        tracing::info!("DBGetProperty contract satisfied: DBImpl implements DBGetProperty");
    }

    #[traced_test]
    fn db_get_property_contract_trait_is_object_safe_and_callable() {
        tracing::trace!("begin DBGetProperty contract: trait object safety + callable shape");
        let _accept = compile_only_accepts_db_get_property_trait_object as fn(&mut dyn DBGetProperty);
        let _call = compile_only_get_property_call_via_trait_object
            as fn(&mut dyn DBGetProperty, &str, &mut String) -> bool;
        tracing::info!("DBGetProperty contract satisfied: usable as a trait object and callable via dyn dispatch");
    }

    #[traced_test]
    fn db_get_property_contract_method_item_is_addressable() {
        tracing::trace!("begin DBGetProperty contract: method item addressability");
        let _method_item = <DBImpl as DBGetProperty>::get_property;
        let _ = _method_item;
        tracing::info!("DBGetProperty contract satisfied: <DBImpl as DBGetProperty>::get_property method item can be referenced");
    }
}
