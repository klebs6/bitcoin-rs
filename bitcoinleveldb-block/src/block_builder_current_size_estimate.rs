// ---------------- [ File: bitcoinleveldb-block/src/block_builder_current_size_estimate.rs ]
crate::ix!();

impl BlockBuilder {

    /**
      | Returns an estimate of the current (uncompressed)
      | size of the block we are building.
      |
      */
    pub fn current_size_estimate(&self) -> usize {
        let estimate = self.buffer_len()                        // raw data buffer
            + self.restarts_len() * core::mem::size_of::<u32>() // restart array
            + core::mem::size_of::<u32>();                      // restart array length

        trace!(
            "BlockBuilder::current_size_estimate: buffer_len={}, restarts_len={}, estimate={}",
            self.buffer_len(),
            self.restarts_len(),
            estimate
        );

        estimate
    }
}

#[cfg(test)]
mod block_builder_size_estimate_tests {
    use super::*;
    use tracing::{debug, trace};

    fn new_boxed_options_and_builder() -> (Box<Options>, BlockBuilder) {
        let opts = Box::new(Options::default());
        let ptr: *const Options = &*opts;
        let builder = BlockBuilder::new(ptr);
        (opts, builder)
    }

    #[traced_test]
    fn current_size_estimate_matches_manual_calculation() {
        let (_opts, mut builder) = new_boxed_options_and_builder();

        let key1   = Slice::from("a".as_bytes());
        let value1 = Slice::from("v1".as_bytes());
        let key2   = Slice::from("b".as_bytes());
        let value2 = Slice::from("v2".as_bytes());

        builder.add(&key1, &value1);
        builder.add(&key2, &value2);

        let estimate = builder.current_size_estimate();
        let manual   = builder.buffer_len()
            + builder.restarts_len() * core::mem::size_of::<u32>()
            + core::mem::size_of::<u32>();

        trace!("current_size_estimate={}, manual={}", estimate, manual);

        debug!(
            "buffer_len={}, restarts_len={}",
            builder.buffer_len(),
            builder.restarts_len()
        );

        assert_eq!(estimate, manual);
    }
}
