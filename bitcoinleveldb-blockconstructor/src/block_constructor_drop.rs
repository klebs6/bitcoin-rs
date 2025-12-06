// ---------------- [ File: bitcoinleveldb-blockconstructor/src/block_constructor_drop.rs ]
crate::ix!();

impl Drop for BlockConstructor {
    fn drop(&mut self) {
        let block_ptr = self.block_ptr();
        let data_len  = self.data_string().len();

        trace!(
            "Dropping BlockConstructor {{ block: {:?}, data_len: {} }}",
            block_ptr,
            data_len
        );

        if !block_ptr.is_null() {
            unsafe {
                debug!(
                    "BlockConstructor::drop: destroying Block at {:?}",
                    block_ptr
                );
                let _owned: Box<Block> = Box::from_raw(block_ptr);
                // `_owned` is dropped here; Block::drop will run and handle its own state.
            }
            *self.block_ptr_mut() = core::ptr::null_mut();
        } else {
            trace!("BlockConstructor::drop: no block to destroy (block pointer is null)");
        }
    }
}

#[cfg(test)]
mod block_constructor_drop_lifetime_and_cleanup_tests {
    use super::*;

    fn new_block_constructor_with_single_entry_for_drop_tests() -> BlockConstructor {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());
        let mut ctor = BlockConstructor::new(cmp_box);

        let mut options = Options::default();
        let mut kv      = KVMap::default();
        kv.insert("k".to_string(), "v".to_string());

        trace!(
            "new_block_constructor_with_single_entry_for_drop_tests: priming BlockConstructor with finish_impl"
        );
        let status = ctor.finish_impl(&options, &kv);
        assert!(
            status.is_ok(),
            "finish_impl in helper must succeed so Drop sees a live Block"
        );

        ctor
    }

    #[traced_test]
    fn drop_on_constructor_without_block_is_noop() {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());
        let ctor = BlockConstructor::new(cmp_box);

        trace!(
            "drop_on_constructor_without_block_is_noop: dropping constructor with block_ptr={:?}, data_len={}",
            ctor.block_ptr(),
            ctor.data_string().len()
        );

        // Explicitly drop to exercise Drop::drop code path; absence of panic is success.
        core::mem::drop(ctor);
    }

    #[traced_test]
    fn drop_on_constructor_with_live_block_cleans_up_without_panic() {
        let ctor = new_block_constructor_with_single_entry_for_drop_tests();

        trace!(
            "drop_on_constructor_with_live_block_cleans_up_without_panic: dropping constructor with block_ptr={:?}, data_len={}",
            ctor.block_ptr(),
            ctor.data_string().len()
        );

        core::mem::drop(ctor);
    }

    #[traced_test]
    fn drop_after_multiple_finish_impl_calls_does_not_double_free() {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(BytewiseComparatorImpl::default());
        let mut ctor = BlockConstructor::new(cmp_box);

        let mut options = Options::default();

        let mut kv1 = KVMap::default();
        kv1.insert("k1".to_string(), "v1".to_string());

        trace!(
            "drop_after_multiple_finish_impl_calls_does_not_double_free: first finish_impl"
        );
        let status1 = ctor.finish_impl(&options, &kv1);
        assert!(status1.is_ok());

        let mut kv2 = KVMap::default();
        kv2.insert("k2".to_string(), "v2".to_string());
        kv2.insert("k3".to_string(), "v3".to_string());

        trace!(
            "drop_after_multiple_finish_impl_calls_does_not_double_free: second finish_impl"
        );
        let status2 = ctor.finish_impl(&options, &kv2);
        assert!(status2.is_ok());

        trace!(
            "drop_after_multiple_finish_impl_calls_does_not_double_free: dropping constructor with final block_ptr={:?}, data_len={}",
            ctor.block_ptr(),
            ctor.data_string().len()
        );

        core::mem::drop(ctor);
    }
}
