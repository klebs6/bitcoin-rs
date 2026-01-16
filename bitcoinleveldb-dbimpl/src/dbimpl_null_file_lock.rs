// ---------------- [ File: bitcoinleveldb-dbimpl/src/dbimpl_null_file_lock.rs ]
crate::ix!();

pub struct DbImplNullFileLock;

impl FileLock for DbImplNullFileLock {}
