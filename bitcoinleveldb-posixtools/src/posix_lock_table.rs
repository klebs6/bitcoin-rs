// ---------------- [ File: bitcoinleveldb-posixtools/src/posix_lock_table.rs ]
crate::ix!();

/// Tracks the files locked by PosixEnv::LockFile().
/// 
/// We maintain a separate set instead of relying on fcntl(F_SETLK) because
/// fcntl(F_SETLK) does not provide any protection against multiple uses from
/// the same process.
/// 
/// Instances are thread-safe because all member data is guarded by a mutex.
///
pub struct PosixLockTable {
    mu: Mutex<PosixLockTableInner>,
}

impl PosixLockTable {

    #[LOCKS_EXCLUDED(mu_)]
    pub fn insert(&mut self, fname: &str) -> bool {
        trace!(
            file = fname,
            "PosixLockTable::insert: acquiring mutex"
        );

        let mut guard = self.mu.lock();

        let inserted = guard.locked_files.insert(fname.to_string());

        debug!(
            file = fname,
            inserted,
            "PosixLockTable::insert: updated locked file set"
        );

        inserted
    }

    #[LOCKS_EXCLUDED(mu_)]
    pub fn remove(&mut self, fname: &str) {
        trace!(
            file = fname,
            "PosixLockTable::remove: acquiring mutex"
        );

        let mut guard = self.mu.lock();

        let removed = guard.locked_files.remove(fname);

        debug!(
            file = fname,
            removed,
            "PosixLockTable::remove: updated locked file set"
        );
    }
}

impl Default for PosixLockTable {
    fn default() -> Self {
        trace!("PosixLockTable::default: creating new lock table");
        PosixLockTable {
            mu: Mutex::new(PosixLockTableInner::default()),
        }
    }
}

//---------------------------------------
pub struct PosixLockTableInner {
    locked_files: HashSet<String>,
}

impl Default for PosixLockTableInner {
    fn default() -> Self {
        trace!(
            "PosixLockTableInner::default: creating empty lock set"
        );
        Self {
            locked_files: HashSet::new(),
        }
    }
}
