// ---------------- [ File: bitcoinleveldb-cache/src/unref_entry.rs ]
crate::ix!();

pub fn unref_entry(arg1: *mut c_void, arg2: *mut c_void) {
    unsafe {
        if arg1.is_null() || arg2.is_null() {
            debug!(
                "unref_entry: arg1 or arg2 is null (arg1={:?}, arg2={:?}); nothing to release",
                arg1, arg2
            );
            return;
        }

        let cache = &mut *(arg1 as *mut Cache);
        let handle = arg2 as *mut CacheHandle;

        trace!(
            "unref_entry: releasing cache handle {:?} via Cache@{:?}",
            handle,
            cache as *mut Cache
        );

        cache.release(handle);
    }
}
