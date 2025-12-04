// ---------------- [ File: bitcoinleveldb-cache/src/release_block.rs ]
crate::ix!();

pub fn release_block(arg: *mut c_void, h: *mut c_void) {
    unsafe {
        if arg.is_null() || h.is_null() {
            debug!(
                "release_block: arg or handle is null (arg={:?}, h={:?}); nothing to release",
                arg, h
            );
            return;
        }

        let cache = &mut *(arg as *mut Cache);
        let handle = h as *mut CacheHandle;

        trace!(
            "release_block: releasing cache handle {:?} via Cache@{:?}",
            handle,
            cache as *mut Cache
        );

        cache.release(handle);
    }
}
