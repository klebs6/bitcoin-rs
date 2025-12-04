// ---------------- [ File: bitcoinleveldb-memenv/src/file_state_truncate.rs ]
crate::ix!();

impl FileState {
    
    pub fn truncate(&mut self) {
        use std::alloc::{dealloc, Layout};

        trace!("FileState::truncate: truncating in‑memory file and freeing all blocks");

        let mut blocks_ref = self.blocks_mutex().borrow_mut();
        let mut guard = blocks_ref.lock();

        if guard.blocks().is_empty() {
            debug!("FileState::truncate: no blocks to free");
            *guard.size_mut() = 0;
            return;
        }

        let layout = match Layout::array::<u8>(FILE_STATE_BLOCK_SIZE) {
            Ok(layout) => layout,
            Err(err) => {
                error!(
                    "FileState::truncate: failed to construct allocation layout: {:?}",
                    err
                );
                guard.blocks_mut().clear();
                *guard.size_mut() = 0;
                return;
            }
        };

        for (idx, block_ptr) in guard.blocks().iter().copied().enumerate() {
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

        guard.blocks_mut().clear();
        *guard.size_mut() = 0;
        debug!("FileState::truncate: completed; file size reset to 0");
    }
}

#[cfg(test)]
mod file_state_truncate_tests {
    use super::*;

    #[traced_test]
    fn truncate_on_empty_file_is_noop() {
        crate::ix!();

        let mut file = FileState::default();
        assert_eq!(file.size(), 0);

        file.truncate();
        assert_eq!(file.size(), 0);

        let blocks_ref = file.blocks_mutex().borrow();
        let guard = blocks_ref.lock();
        assert!(guard.blocks().is_empty());
        assert_eq!(*guard.size(), 0);
    }

    #[traced_test]
    fn truncate_frees_all_data_and_resets_size() {
        crate::ix!();

        let mut file = FileState::default();
        let payload = vec![1_u8; FILE_STATE_BLOCK_SIZE * 2 + 50];
        let slice = Slice::from(payload.as_slice());

        let status = file.append(&slice);
        assert!(status.is_ok());
        assert_eq!(file.size(), payload.len() as u64);

        file.truncate();
        assert_eq!(file.size(), 0);

        let blocks_ref = file.blocks_mutex().borrow();
        let guard = blocks_ref.lock();
        assert!(guard.blocks().is_empty());
        assert_eq!(*guard.size(), 0);
    }
}
