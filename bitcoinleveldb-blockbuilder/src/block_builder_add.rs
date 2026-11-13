// ---------------- [ File: bitcoinleveldb-blockbuilder/src/block_builder_add.rs ]
crate::ix!();

impl BlockBuilder {

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

        let _: u32 = key_size.try_into()
            .expect("BlockBuilder::add: key length does not fit into u32");
        let value_size_u32: u32 = value_size.try_into()
            .expect("BlockBuilder::add: value length does not fit into u32");

        // A caller can supply a Slice into an earlier buffer/last-key view.
        // Copy only those inputs before any append can invalidate their owner.
        let owned_key = self.input_overlaps_storage(key_)
            .then(|| key_.as_bytes().to_vec());
        let owned_value = self.input_overlaps_storage(value)
            .then(|| value.as_bytes().to_vec());
        let key_view = owned_key.as_ref().map(|bytes| Slice::from(bytes.as_slice()));
        let value_view = owned_value.as_ref().map(|bytes| Slice::from(bytes.as_slice()));
        let key_ = key_view.as_ref().unwrap_or(key_);
        let value = value_view.as_ref().unwrap_or(value);

        let opts: &Options = unsafe {
            let opts_ptr = self.options_ptr();
            assert!(
                !opts_ptr.is_null(),
                "BlockBuilder::add: options pointer is null"
            );
            &*opts_ptr
        };

        let interval: i32 = *self.block_restart_interval();

        assert!(
            *self.counter() <= interval,
            "BlockBuilder::add: counter {} > block_restart_interval {}",
            self.counter(),
            interval
        );

        let last_key_slice = if self.last_key().is_empty() {
            Slice::default()
        } else {
            Slice::from(self.last_key().as_slice())
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
            let offset: u32 = self.buffer_len().try_into()
                .expect("BlockBuilder::add: restart offset does not fit into u32");
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
            bitcoinleveldb_coding::put_varint32_bytes(buf, shared.try_into()
                .expect("BlockBuilder::add: shared length does not fit into u32"));
            bitcoinleveldb_coding::put_varint32_bytes(buf, non_shared.try_into()
                .expect("BlockBuilder::add: non-shared length does not fit into u32"));
            bitcoinleveldb_coding::put_varint32_bytes(buf, value_size_u32);
        }

        {
            let key_delta = &key_.as_bytes()[shared..];
            let value_bytes = value.as_bytes();
            let buf: &mut Vec<u8> = self.buffer_as_mut_vec();
            buf.extend_from_slice(key_delta);
            buf.extend_from_slice(value_bytes);
        }

        {
            let last_vec: &mut Vec<u8> = self.last_key_mut();
            last_vec.truncate(shared);
            last_vec.extend_from_slice(&key_.as_bytes()[shared..]);
        }

        let last_slice_check = Slice::from(self.last_key().as_slice());
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
        assert_eq!(builder.last_key().as_slice(), b"alpha");
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
