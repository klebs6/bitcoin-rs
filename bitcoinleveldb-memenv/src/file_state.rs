// ---------------- [ File: bitcoinleveldb-memenv/src/file_state.rs ]
crate::ix!();

pub const FILE_STATE_BLOCK_SIZE: usize = 8 * 1024;

#[no_copy]
#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct FileStateRefs {
    refs: i32,
}

#[no_copy]
#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct FileStateBlocks {
    blocks: Vec<*mut u8>,
    size:   u64,
}

//----------------------------------------
#[no_copy]
#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct FileState {
    refs_mutex:   Mutex<FileStateRefs>,
    blocks_mutex: RefCell<Mutex<FileStateBlocks>>,
}

impl Default for FileState {

    /// FileStates are reference counted.
    /// The initial reference count is zero
    /// and the caller must call Ref() at least
    /// once.
    fn default() -> Self {
        trace!(
            "FileState::default: creating new FileState with zero refs and empty blocks"
        );
        FileState {
            refs_mutex: Mutex::new(FileStateRefs { refs: 0 }),
            blocks_mutex: RefCell::new(Mutex::new(FileStateBlocks {
                blocks: Vec::new(),
                size:   0,
            })),
        }
    }
}

impl Drop for FileState {

    /// Private since only Unref() should be
    /// used to delete it (mirrors C++ comment).
    fn drop(&mut self) {
        trace!("FileState::drop: invoked; truncating all in‑memory blocks");
        self.truncate();
    }
}

#[cfg(test)]
mod file_state_core_tests {
    use super::*;

    #[traced_test]
    fn file_state_default_initializes_empty() {
        crate::ix!();

        let file = FileState::default();

        // Size must start at zero.
        assert_eq!(file.size(), 0);

        // Reference count starts at zero.
        {
            let guard = file.refs_mutex().lock();
            assert_eq!(*guard.refs(), 0);
        }

        // Blocks collection must be empty and size must be zero.
        {
            let blocks_ref = file.blocks_mutex().borrow();
            let guard = blocks_ref.lock();
            assert!(guard.blocks().is_empty());
            assert_eq!(*guard.size(), 0);
        }
    }

    #[traced_test]
    fn file_state_drop_after_append_is_safe() {
        crate::ix!();

        {
            let mut file = FileState::default();
            let payload = vec![7_u8; FILE_STATE_BLOCK_SIZE * 2 + 17];
            let slice = Slice::from(payload.as_slice());

            let status = file.append(&slice);
            assert!(status.is_ok());
            assert_eq!(file.size(), payload.len() as u64);
        } // drop should not panic and must free all blocks via truncate()
    }
}
