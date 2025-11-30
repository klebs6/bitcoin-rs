// ---------------- [ File: bitcoinleveldb-posixtools/src/posix_file_lock.rs ]
crate::ix!();

/// Instances are thread-safe because
/// they are immutable.
/// 
pub struct PosixFileLock {
    fd:       i32,
    filename: String,
}

impl FileLock for PosixFileLock {}

impl Named for PosixFileLock {
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("[posix-file-lock]".to_string())
    }
}

impl PosixFileLock {

    pub fn new(fd: i32, filename: String) -> Self {
        trace!(
            fd,
            file = %filename,
            "PosixFileLock::new: constructing new POSIX file lock"
        );
        Self { fd, filename }
    }

    pub fn fd(&self) -> i32 {
        trace!(
            fd = self.fd,
            "PosixFileLock::fd accessor"
        );
        self.fd
    }

    pub fn filename(&self) -> &String {
        trace!(
            file = %self.filename,
            "PosixFileLock::filename accessor"
        );
        &self.filename
    }
}
