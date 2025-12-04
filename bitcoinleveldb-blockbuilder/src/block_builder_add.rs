// ---------------- [ File: bitcoinleveldb-blockbuilder/src/block_builder_add.rs ]
crate::ix!();

impl BlockBuilder {

    /**
      | REQUIRES: Finish() has not been called since
      | the last call to Reset().
      |
      | REQUIRES: key is larger than any previously
      | added key
      */
    pub fn add(&mut self, key_: &Slice, value: &Slice) {
        let key_size:   usize = *key_.size();
        let value_size: usize = *value.size();

        trace!(
            "BlockBuilder::add: key_len={}, value_len={}, buffer_len_before={}, counter={}, finished={}",
            key_size,
            value_size,
            self.buffer_len(),
            self.counter(),
            self.is_finished()
        );

        assert!(
            !self.is_finished(),
            "BlockBuilder::add: called after finish()"
        );

        let opts: &Options = unsafe {
            let opts_ptr = self.options_ptr();
            assert!(
                !opts_ptr.is_null(),
                "BlockBuilder::add: options pointer is null"
            );
            &*opts_ptr
        };

        let interval: i32 = *opts.block_restart_interval();

        assert!(
            *self.counter() <= interval,
            "BlockBuilder::add: counter {} > block_restart_interval {}",
            self.counter(),
            interval
        );

        let last_key_slice = if self.last_key().is_empty() {
            Slice::default()
        } else {
            Slice::from(self.last_key())
        };

        if !self.buffer_is_empty() {
            let cmp = opts.comparator().compare(key_, &last_key_slice);
            assert!(
                cmp > 0,
                "BlockBuilder::add: keys must be added in strictly increasing order; cmp={}",
                cmp
            );
        }

        let mut shared: usize = 0;

        if *self.counter() < interval {
            let last_len = *last_key_slice.size();
            let key_len  = key_size;
            let min_len  = core::cmp::min(last_len, key_len);

            while shared < min_len && last_key_slice[shared] == (*key_)[shared] {
                shared += 1;
            }
        } else {
            let offset = self.buffer_len() as u32;
            trace!(
                "BlockBuilder::add: starting new restart point at offset {}",
                offset
            );
            self.restarts_push(offset);
            self.set_counter(0);
        }

        let non_shared = key_size.saturating_sub(shared);

        trace!(
            "BlockBuilder::add: shared_prefix={}, non_shared={}, restart_index={}, counter={}",
            shared,
            non_shared,
            self.restarts_len().saturating_sub(1),
            self.counter()
        );

        {
            let buf: &mut Vec<u8> = self.buffer_as_mut_vec();
            put_varint32_vec(buf, shared as u32);
            put_varint32_vec(buf, non_shared as u32);
            put_varint32_vec(buf, value_size as u32);
        }

        unsafe {
            let key_ptr   = (*key_.data()).add(shared);
            let key_delta = core::slice::from_raw_parts(key_ptr, non_shared);

            let value_ptr: *const u8 = *value.data();
            let value_bytes          = core::slice::from_raw_parts(value_ptr, value_size);

            let buf: &mut Vec<u8> = self.buffer_as_mut_vec();
            buf.extend_from_slice(key_delta);
            buf.extend_from_slice(value_bytes);
        }

        {
            let last_vec: &mut Vec<u8> = unsafe { self.last_key_mut().as_mut_vec() };
            last_vec.truncate(shared);
            unsafe {
                let key_ptr   = (*key_.data()).add(shared);
                let key_delta = core::slice::from_raw_parts(key_ptr, non_shared);
                last_vec.extend_from_slice(key_delta);
            }
        }

        let last_slice_check = Slice::from(self.last_key());
        debug_assert!(
            last_slice_check == *key_,
            "BlockBuilder::add: last_key_ does not match added key"
        );

        self.inc_counter();

        trace!(
            "BlockBuilder::add: buffer_len_after={}, counter={}",
            self.buffer_len(),
            self.counter()
        );
    }
}

#[cfg(test)]
mod block_builder_addition_and_prefix_compression_tests {
    use super::*;
    use std::panic;

    fn new_boxed_options_and_builder() -> (Box<Options>, BlockBuilder) {
        let opts = Box::new(Options::default());
        let ptr: *const Options = &*opts;
        trace!(
            "creating BlockBuilder for add tests; block_restart_interval={}",
            *opts.block_restart_interval()
        );
        let builder = BlockBuilder::new(ptr);
        (opts, builder)
    }

    #[traced_test]
    fn block_builder_add_single_entry_sets_last_key_and_increments_counter() {
        let (_opts, mut builder) = new_boxed_options_and_builder();

        let key   = Slice::from("alpha".as_bytes());
        let value = Slice::from("v1".as_bytes());

        assert!(builder.empty());
        builder.add(&key, &value);

        debug!(
            "after single add: buffer_len={}, counter={}",
            builder.buffer_len(),
            *builder.counter()
        );

        assert_eq!(*builder.counter(), 1);
        assert_eq!(builder.last_key(), "alpha");
        assert!(!builder.empty());
    }

    #[traced_test]
    fn block_builder_add_enforces_strictly_increasing_keys() {
        let (_opts, mut builder) = new_boxed_options_and_builder();

        let key1   = Slice::from("a".as_bytes());
        let value1 = Slice::from("v1".as_bytes());
        let key2   = Slice::from("a".as_bytes());
        let value2 = Slice::from("v2".as_bytes());

        builder.add(&key1, &value1);

        trace!(
            "attempting to add duplicate key; expecting panic due to ordering assertion"
        );
        let result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            builder.add(&key2, &value2);
        }));

        assert!(
            result.is_err(),
            "BlockBuilder::add should panic on non-increasing keys"
        );
    }

    #[traced_test]
    fn block_builder_add_triggers_restart_after_interval() {
        let (opts, mut builder) = new_boxed_options_and_builder();

        let interval: i32 = *opts.block_restart_interval();
        trace!(
            "using restart interval {} for restart behavior test",
            interval
        );

        builder.set_counter(interval);

        let key   = Slice::from("zzz".as_bytes());
        let value = Slice::from("value".as_bytes());

        let prev_restart_len = builder.restarts_len();
        builder.add(&key, &value);

        debug!(
            "after forced-restart add: restarts_len={} (was {}), counter={}",
            builder.restarts_len(),
            prev_restart_len,
            *builder.counter()
        );

        assert!(builder.restarts_len() >= prev_restart_len + 1);
        assert_eq!(*builder.counter(), 1);
    }
}
