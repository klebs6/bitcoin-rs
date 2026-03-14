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
lazy_static! {
    pub static ref BITCOINLEVELDB_POSIX_LOCK_TABLE_GLOBAL: Mutex<PosixLockTableInner> =
        Mutex::new(PosixLockTableInner::default());
}

pub struct PosixLockTable  {
    mu_: (),
}

impl PosixLockTable {

    #[LOCKS_EXCLUDED(mu_)]
    pub fn insert(&mut self, fname: &str) -> bool {
        trace!(
            file = fname,
            "PosixLockTable::insert: acquiring process-global mutex"
        );

        let mut guard = BITCOINLEVELDB_POSIX_LOCK_TABLE_GLOBAL.lock();

        let inserted = guard.locked_files.insert(fname.to_string());

        debug!(
            file = fname,
            inserted,
            "PosixLockTable::insert: updated process-global locked file set"
        );

        inserted
    }

    #[LOCKS_EXCLUDED(mu_)]
    pub fn remove(&mut self, fname: &str) {
        trace!(
            file = fname,
            "PosixLockTable::remove: acquiring process-global mutex"
        );

        let mut guard = BITCOINLEVELDB_POSIX_LOCK_TABLE_GLOBAL.lock();

        let removed = guard.locked_files.remove(fname);

        debug!(
            file = fname,
            removed,
            "PosixLockTable::remove: updated process-global locked file set"
        );
    }
}

impl Default for PosixLockTable {

    fn default() -> Self {
        trace!("PosixLockTable::default: constructing process-global lock-table handle");
        Self { mu_: () }
    }
}

//---------------------------------------
pub struct PosixLockTableInner {
    locked_files: HashSet<String>,
}

impl Default for PosixLockTableInner {

    fn default() -> Self {
        trace!(
            "PosixLockTableInner::default: creating empty process-global lock set"
        );
        Self {
            locked_files: HashSet::new(),
        }
    }
}
