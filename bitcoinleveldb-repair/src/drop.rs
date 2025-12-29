// ---------------- [ File: bitcoinleveldb-repair/src/drop.rs ]
crate::ix!();

impl Drop for Repairer {
    fn drop(&mut self) {
        trace!("Repairer::drop: start");

        unsafe {
            if !self.table_cache.is_null() {
                debug!(
                    table_cache = ?self.table_cache,
                    "Repairer::drop: deleting TableCache"
                );
                drop(Box::from_raw(self.table_cache));
                self.table_cache = core::ptr::null_mut();
            }
        }

        if self.owns_info_log {
            if let Some(ptr) = *self.options.info_log() {
                unsafe {
                    debug!(
                        info_log = ?ptr,
                        "Repairer::drop: deleting owned info_log"
                    );
                    let boxed: Box<dyn Logger> = Box::from_raw(ptr);
                    drop(boxed);
                }
                self.options.set_info_log(None);
            }
        }

        if self.owns_cache {
            let cache_ptr = *self.options.block_cache();
            if !cache_ptr.is_null() {
                unsafe {
                    debug!(
                        block_cache = ?cache_ptr,
                        "Repairer::drop: deleting owned block_cache"
                    );
                    drop(Box::from_raw(cache_ptr));
                }
                self.options.set_block_cache(core::ptr::null_mut());
            }
        }

        trace!("Repairer::drop: done");
    }
}
