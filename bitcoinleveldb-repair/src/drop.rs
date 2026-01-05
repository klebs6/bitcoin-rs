// ---------------- [ File: bitcoinleveldb-repair/src/drop.rs ]
crate::ix!();

impl Drop for Repairer {

    fn drop(&mut self) {
        trace!("Repairer::drop: start");

        unsafe {
            let table_cache_ptr: *mut TableCache = *self.table_cache();
            if !table_cache_ptr.is_null() {
                debug!(
                    table_cache = ?table_cache_ptr,
                    "Repairer::drop: deleting TableCache"
                );
                drop(Box::from_raw(table_cache_ptr));
                *self.table_cache_mut() = core::ptr::null_mut();
            }
        }

        if *self.owns_info_log() {
            let info_log_ptr_opt: Option<*mut dyn Logger> = *self.options().info_log();
            if let Some(ptr) = info_log_ptr_opt {
                unsafe {
                    debug!(
                        info_log = ?ptr,
                        "Repairer::drop: deleting owned info_log"
                    );
                    let boxed: Box<dyn Logger> = Box::from_raw(ptr);
                    drop(boxed);
                }
                self.options_mut().set_info_log(None);
            }
        }

        if *self.owns_cache() {
            let cache_ptr = *self.options().block_cache();
            if !cache_ptr.is_null() {
                unsafe {
                    debug!(
                        block_cache = ?cache_ptr,
                        "Repairer::drop: deleting owned block_cache"
                    );
                    drop(Box::from_raw(cache_ptr));
                }
                self.options_mut().set_block_cache(core::ptr::null_mut());
            }
        }

        trace!("Repairer::drop: done");
    }
}

#[cfg(test)]
mod repairer_drop_safety_suite {
    use super::*;
    use crate::repairer_test_harness::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn dropping_repairer_after_run_does_not_panic() {
        let db = EphemeralDbDir::new("drop-after-run");
        let dbname: String = db.path_string();

        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let options = Options::default();

        trace!(dbname = %dbname, "constructing Repairer");
        {
            let mut repairer = Repairer::new(&dbname, &options);

            trace!(dbname = %dbname, "running Repairer::run");
            let st = repairer.run();

            info!(
                dbname = %dbname,
                ok = st.is_ok(),
                status = %st.to_string(),
                "Repairer::run completed"
            );

            // Regardless of outcome, dropping should be safe.
            let _ = st;
        }

        // If drop had a double-free or similar issue, the test process would likely abort.
        debug!(dbname = %dbname, "repairer dropped cleanly");
    }

    #[traced_test]
    fn dropping_repairer_without_running_does_not_panic() {
        let db = EphemeralDbDir::new("drop-without-run");
        let dbname: String = db.path_string();

        let sentinel = format!("{}/SENTINEL", dbname);
        touch_file(&sentinel);

        let options = Options::default();
        trace!(dbname = %dbname, "constructing Repairer only");
        let _repairer = Repairer::new(&dbname, &options);
        debug!(dbname = %dbname, "Repairer constructed; will be dropped at scope end");
    }
}
