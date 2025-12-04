// ---------------- [ File: bitcoinleveldb-block/src/block_builder_reset.rs ]
crate::ix!();
  
impl BlockBuilder {

    /**
      | Reset the contents as if the BlockBuilder
      | was just constructed.
      |
      */
    pub fn reset(&mut self) {
        trace!(
            "BlockBuilder::reset: buffer_len_before={}, restarts_len_before={}, counter_before={}, finished_before={}",
            self.buffer_len(),
            self.restarts_len(),
            self.counter(),
            self.is_finished()
        );

        self.buffer_clear();
        self.restarts_clear();
        self.restarts_push(0);
        self.set_counter(0);
        self.set_finished(false);
        self.last_key_mut().clear();

        trace!(
            "BlockBuilder::reset: buffer_len_after={}, restarts_len_after={}, counter_after={}, finished_after={}",
            self.buffer_len(),
            self.restarts_len(),
            self.counter(),
            self.is_finished()
        );
    }
}

#[cfg(test)]
mod block_builder_reset_state_tests {
    use super::*;
    use tracing::{debug, trace};

    fn new_boxed_options_and_builder() -> (Box<Options>, BlockBuilder) {
        let opts = Box::new(Options::default());
        let ptr: *const Options = &*opts;
        let builder = BlockBuilder::new(ptr);
        (opts, builder)
    }

    #[traced_test]
    fn reset_restores_builder_to_fresh_state() {
        let (_opts, mut builder) = new_boxed_options_and_builder();

        let key1   = Slice::from("k1".as_bytes());
        let value1 = Slice::from("v1".as_bytes());
        let key2   = Slice::from("k2".as_bytes());
        let value2 = Slice::from("v2".as_bytes());

        builder.add(&key1, &value1);
        builder.add(&key2, &value2);
        builder.finish();

        trace!(
            "before reset: buffer_len={}, restarts_len={}, counter={}, finished={}",
            builder.buffer_len(),
            builder.restarts_len(),
            *builder.counter(),
            builder.is_finished()
        );

        builder.reset();

        debug!(
            "after reset: buffer_len={}, restarts_len={}, counter={}, finished={}, last_key_len={}",
            builder.buffer_len(),
            builder.restarts_len(),
            *builder.counter(),
            builder.is_finished(),
            builder.last_key().len()
        );

        assert!(builder.empty());
        assert_eq!(builder.buffer_len(), 0);
        assert_eq!(builder.restarts_len(), 1);
        assert_eq!(builder.restarts_slice(), &[0u32]);
        assert_eq!(*builder.counter(), 0);
        assert!(!builder.is_finished());
        assert!(builder.last_key().is_empty());
    }
}
