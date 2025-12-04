// ---------------- [ File: bitcoinleveldb-blockhandle/src/read_block_reset_output.rs ]
crate::ix!();

pub fn read_block_reset_output(result: &mut BlockContents) {
    trace!("read_block_reset_output: resetting BlockContents output to empty state");

    result.set_data(Slice::default());
    result.set_cachable(false);
    result.set_heap_allocated(false);
}

#[cfg(test)]
mod read_block_output_reset_unit_tests {
    use super::*;

    #[traced_test]
    fn reset_output_clears_non_empty_block_contents() {
        let bytes      = b"reset-me";
        let data_slice = Slice::from(&bytes[..]);
        let mut result = BlockContents::new(
            data_slice,
            true,
            true,
        );

        read_block_reset_output(&mut result);

        trace!(
            "reset_output_clears_non_empty_block_contents: size={}, cachable={}, heap_allocated={}",
            *result.data().size(),
            result.cachable(),
            result.heap_allocated()
        );

        assert_eq!(*result.data().size(), 0);
        assert!(!result.cachable());
        assert!(!result.heap_allocated());
    }

    #[traced_test]
    fn reset_output_is_idempotent_on_already_empty_block_contents() {
        let mut result = BlockContents::new(
            Slice::default(),
            false,
            false,
        );

        read_block_reset_output(&mut result);
        read_block_reset_output(&mut result);

        trace!(
            "reset_output_is_idempotent_on_already_empty_block_contents: size={}, cachable={}, heap_allocated={}",
            *result.data().size(),
            result.cachable(),
            result.heap_allocated()
        );

        assert_eq!(*result.data().size(), 0);
        assert!(!result.cachable());
        assert!(!result.heap_allocated());
    }
}

