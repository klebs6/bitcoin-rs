// ---------------- [ File: bitcoinleveldb-block/src/block_builder_finish.rs ]
crate::ix!();

impl BlockBuilder {

    /**
      | Finish building the block and return a slice
      | that refers to the block contents.  The
      | returned slice will remain valid for the
      | lifetime of this builder or until Reset() is
      | called.
      */
    pub fn finish(&mut self) -> Slice {
        trace!(
            "BlockBuilder::finish: buffer_len_before={}, restarts_len={}",
            self.buffer_len(),
            self.restarts_len()
        );

        {
            // Take a snapshot of the restart array so we do not
            // alias &mut self and &self at the same time.
            let restarts: Vec<u32> = self.restarts_slice().to_vec();

            for restart in &restarts {
                bitcoinleveldb_coding::put_fixed32(
                    self.buffer_mut(),
                    *restart,
                );
            }

            bitcoinleveldb_coding::put_fixed32(
                self.buffer_mut(),
                restarts.len() as u32,
            );
        }

        self.set_finished(true);

        let result = Slice::from(self.buffer_string());

        trace!(
            "BlockBuilder::finish: buffer_len_after={}, result_size={}",
            self.buffer_len(),
            *result.size()
        );

        result
    }
}

#[cfg(test)]
mod block_builder_finish_and_trailer_tests {
    use super::*;

    fn new_boxed_options_and_builder() -> (Box<Options>, BlockBuilder) {
        let opts = Box::new(Options::default());
        let ptr: *const Options = &*opts;
        let builder = BlockBuilder::new(ptr);
        (opts, builder)
    }

    #[traced_test]
    fn finish_appends_restart_array_and_marks_finished() {
        let (_opts, mut builder) = new_boxed_options_and_builder();

        let key1   = Slice::from("a".as_bytes());
        let value1 = Slice::from("one".as_bytes());
        let key2   = Slice::from("b".as_bytes());
        let value2 = Slice::from("two".as_bytes());

        builder.add(&key1, &value1);
        builder.add(&key2, &value2);

        let restarts_before = builder.restarts_slice().to_vec();
        trace!(
            "restarts before finish: len={}, values={:?}",
            restarts_before.len(),
            restarts_before
        );

        let slice = builder.finish();

        assert!(builder.is_finished());
        let total_len = builder.buffer_len();
        assert_eq!(total_len, *slice.size());

        let buf_bytes = builder.buffer_string().as_bytes();
        assert!(
            buf_bytes.len() >= 4,
            "buffer must contain at least num_restarts trailer"
        );

        let num_restarts_trailer = u32::from_le_bytes(
            buf_bytes[buf_bytes.len() - 4..].try_into().unwrap(),
        );

        debug!(
            "num_restarts_trailer={}, restarts_len={}",
            num_restarts_trailer,
            restarts_before.len()
        );

        assert_eq!(num_restarts_trailer as usize, restarts_before.len());
    }
}
