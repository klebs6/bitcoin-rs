// ---------------- [ File: bitcoinleveldb-block/src/block_create.rs ]
crate::ix!();

impl Block {

    /**
      | Initialize the block with the specified
      | contents.
      |
      */
    pub fn new(contents: &BlockContents) -> Self {
        let data_slice  = contents.data();
        let data_ptr    = *data_slice.data();
        let mut size    = *data_slice.size();
        let mut restart_offset: u32 = 0;

        trace!(
            "Block::new: initializing from contents: ptr={:p}, size={}, heap_allocated={}, cachable={}",
            data_ptr,
            size,
            contents.is_heap_allocated(),
            contents.is_cachable()
        );

        let trailer_len = core::mem::size_of::<u32>();

        if size < trailer_len {
            error!(
                "Block::new: contents too small ({}) for restart array; marking block as error",
                size
            );
            size = 0;
        } else {
            let max_restarts_allowed =
                (size - trailer_len) / trailer_len;

            let tmp_block = Block {
                data:           data_ptr,
                size,
                restart_offset: 0,
                owned:          contents.is_heap_allocated(),
            };

            let num_restarts = tmp_block.num_restarts() as usize;
            if num_restarts > max_restarts_allowed {
                error!(
                    "Block::new: NumRestarts ({}) exceeds maximum allowed ({}); marking block as error",
                    num_restarts,
                    max_restarts_allowed
                );
                size = 0;
            } else {
                let off = size
                    .saturating_sub((1 + num_restarts) * trailer_len);
                restart_offset = off as u32;
                trace!(
                    "Block::new: restart_offset={} (num_restarts={})",
                    restart_offset,
                    num_restarts
                );
            }
        }

        Block {
            data:           data_ptr,
            size,
            restart_offset,
            owned: contents.is_heap_allocated(),
        }
    }
}

#[cfg(test)]
mod block_creation_and_restart_offset_tests {
    use super::*;

    fn build_block_contents_from_bytes(bytes: &[u8], heap_allocated: bool) -> BlockContents {
        let slice = Slice::from(bytes);
        BlockContents::new(slice, false, heap_allocated)
    }

    #[traced_test]
    fn block_new_sets_error_marker_for_too_small_contents() {
        let bytes = vec![0u8; core::mem::size_of::<u32>() - 1];
        let contents = build_block_contents_from_bytes(&bytes, false);

        let block = Block::new(&contents);
        trace!(
            "Block::new with undersized contents: resulting size={}",
            block.size()
        );
        assert_eq!(block.size(), 0);
    }

    #[traced_test]
    fn block_new_computes_restart_offset_for_valid_single_restart_block() {
        let mut bytes = vec![0u8; 8];
        // restarts[0] = 0
        let restart_offset_val: u32 = 0;
        bytes[0..4].copy_from_slice(&restart_offset_val.to_le_bytes());
        // num_restarts = 1 at end
        bytes[4..8].copy_from_slice(&1u32.to_le_bytes());

        let contents = build_block_contents_from_bytes(&bytes, false);
        let block    = Block::new(&contents);

        debug!(
            "Block::new with valid contents: size={}, restart_offset={}, num_restarts={}",
            block.size(),
            block.restart_offset(),
            block.num_restarts()
        );

        assert_eq!(block.size(), 8);
        assert_eq!(block.restart_offset(), 0);
        assert_eq!(block.num_restarts(), 1);
    }

    #[traced_test]
    fn block_new_marks_error_when_num_restarts_exceeds_maximum() {
        let mut bytes = vec![0u8; 8];
        // bogus num_restarts=3, but there is space only for at most 1 restart
        bytes[4..8].copy_from_slice(&3u32.to_le_bytes());

        let contents = build_block_contents_from_bytes(&bytes, false);
        let block    = Block::new(&contents);

        trace!(
            "Block::new with invalid num_restarts: resulting size={}",
            block.size()
        );
        assert_eq!(block.size(), 0);
    }
}
