crate::ix!();

impl FileState {
    
    pub fn truncate(&mut self) {
        use std::alloc::{dealloc, Layout};

        trace!("FileState::truncate: truncating in‑memory file and freeing all blocks");

        let mut blocks_ref = self.blocks_mutex.borrow_mut();
        let mut guard = match blocks_ref.lock() {
            Ok(guard) => guard,
            Err(poisoned) => {
                warn!("FileState::truncate: blocks_mutex poisoned; recovering");
                poisoned.into_inner()
            }
        };

        if guard.blocks.is_empty() {
            debug!("FileState::truncate: no blocks to free");
            guard.size = 0;
            return;
        }

        let layout = match Layout::array::<u8>(FILE_STATE_BLOCK_SIZE) {
            Ok(layout) => layout,
            Err(err) => {
                error!(
                    "FileState::truncate: failed to construct allocation layout: {:?}",
                    err
                );
                guard.blocks.clear();
                guard.size = 0;
                return;
            }
        };

        for (idx, block_ptr) in guard.blocks.iter().copied().enumerate() {
            if block_ptr.is_null() {
                warn!(
                    "FileState::truncate: encountered null block pointer at index {}",
                    idx
                );
                continue;
            }
            unsafe {
                dealloc(block_ptr, layout);
            }
        }

        guard.blocks.clear();
        guard.size = 0;
        debug!("FileState::truncate: completed; file size reset to 0");
    }
}
