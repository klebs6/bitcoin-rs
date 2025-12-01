// ---------------- [ File: bitcoinleveldb-memenv/src/file_state.rs ]
crate::ix!();

pub const FILE_STATE_BLOCK_SIZE: usize = 8 * 1024;

#[no_copy]
pub struct FileStateRefs {
    refs: i32,
}

#[no_copy]
pub struct FileStateBlocks {
    blocks: Vec<*mut u8>,
    size:   u64,
}

//----------------------------------------
#[no_copy]
pub struct FileState {
    refs_mutex:   Mutex<FileStateRefs>,
    blocks_mutex: RefCell<Mutex<FileStateBlocks>>,
}

impl Default for FileState {

    /// FileStates are reference counted.
    /// The initial reference count is zero
    /// and the caller must call Ref() at least
    /// once.
    /// 
    fn default() -> Self {
        trace!("FileState::default: creating new FileState with zero refs and empty blocks");
        FileState {
            refs_mutex: Mutex::new(FileStateRefs { refs: 0 }),
            blocks_mutex: RefCell::new(Mutex::new(FileStateBlocks {
                blocks: Vec::new(),
                size: 0,
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
