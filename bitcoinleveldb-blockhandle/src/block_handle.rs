// ---------------- [ File: bitcoinleveldb-blockhandle/src/block_handle.rs ]
crate::ix!();

/// 1-byte type + 32-bit crc
/// 
pub const BLOCK_TRAILER_SIZE: usize = 5;

/// BlockHandle is a pointer to the extent
/// of a file that stores a data block or a
/// meta block.
/// 
#[derive(Default, Clone, Copy)]
pub struct BlockHandle {
    offset: u64,
    size:   u64,
}

/// Maximum encoding length of a BlockHandle
/// 
pub const BLOCK_HANDLE_MAX_ENCODED_LENGTH: usize = 10 + 10;

impl BlockHandle {
 
    /// Implementation details follow. Clients
    /// should ignore.
    /// 
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

    /// The size of the stored block
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

    #[traced_test]
    fn block_handle_default_initializes_to_zero() {
        let handle = BlockHandle::default();

        trace!(
            "BlockHandle::default produced offset={}, size={}",
            handle.offset(),
            handle.size()
        );

        assert_eq!(handle.offset(), 0);
        assert_eq!(handle.size(), 0);
    }

    #[traced_test]
    fn block_handle_setters_and_getters_round_trip_values() {
        let mut handle = BlockHandle::new();

        handle.set_offset(123);
        handle.set_size(456);

        trace!(
            "BlockHandle after initial mutation offset={}, size={}",
            handle.offset(),
            handle.size()
        );

        assert_eq!(handle.offset(), 123);
        assert_eq!(handle.size(), 456);

        handle.set_offset(9_999);
        trace!(
            "BlockHandle after offset update offset={}, size={}",
            handle.offset(),
            handle.size()
        );
        assert_eq!(handle.offset(), 9_999);
        assert_eq!(handle.size(), 456);

        handle.set_size(1_111);
        trace!(
            "BlockHandle after size update offset={}, size={}",
            handle.offset(),
            handle.size()
        );
        assert_eq!(handle.offset(), 9_999);
        assert_eq!(handle.size(), 1_111);
    }

    #[traced_test]
    fn block_handle_supports_large_u64_values() {
        let mut handle = BlockHandle::new();
        let large_offset = u64::MAX - 1;
        let large_size   = u64::MAX / 2;

        handle.set_offset(large_offset);
        handle.set_size(large_size);

        trace!(
            "BlockHandle with large values offset={}, size={}",
            handle.offset(),
            handle.size()
        );

        assert_eq!(handle.offset(), large_offset);
        assert_eq!(handle.size(), large_size);
    }
}
