// ---------------- [ File: bitcoinleveldb-block/src/block_handle.rs ]
crate::ix!();

/**
  | 1-byte type + 32-bit crc
  |
  */
pub const BLOCK_TRAILER_SIZE: usize = 5;

/**
  | BlockHandle is a pointer to the extent
  | of a file that stores a data block or a
  | meta block.
  |
  */
#[derive(Default, Clone, Copy)]
pub struct BlockHandle {
    offset: u64,
    size:   u64,
}

/**
  | Maximum encoding length of a BlockHandle
  |
  */
pub const BLOCK_HANDLE_MAX_ENCODED_LENGTH: usize = 10 + 10;

impl BlockHandle {
 
    /**
      | Implementation details follow. Clients
      | should ignore.
      |
      */
    pub fn new() -> Self {
        let sentinel = !0u64;
        trace!(
            "BlockHandle::new: initializing with sentinel values ({})",
            sentinel
        );
        BlockHandle {
            offset: sentinel,
            size:   sentinel,
        }
    }

    /// The offset of the block in the file.
    /// 
    pub fn offset(&self) -> u64 {
        trace!("BlockHandle::offset called => {}", self.offset);
        self.offset
    }

    pub fn set_offset(&mut self, offset: u64) {
        trace!(
            "BlockHandle::set_offset: {} -> {}",
            self.offset,
            offset
        );
        self.offset = offset;
    }

    /**
       The size of the stored block
      */
    pub fn size(&self) -> u64 {
        trace!("BlockHandle::size called => {}", self.size);
        self.size
    }

    pub fn set_size(&mut self, size: u64) {
        trace!(
            "BlockHandle::set_size: {} -> {}",
            self.size,
            size
        );
        self.size = size;
    }
}

#[cfg(test)]
mod block_handle_basic_behavior_tests {
    use super::*;

    #[traced_test]
    fn block_handle_new_uses_sentinel_values() {
        let handle = BlockHandle::new();
        let sentinel = !0u64;

        trace!(
            "BlockHandle::new produced offset={}, size={}",
            handle.offset(),
            handle.size()
        );

        assert_eq!(handle.offset(), sentinel);
        assert_eq!(handle.size(), sentinel);
    }
}
