// ---------------- [ File: bitcoinleveldb-dbimpl/src/imports.rs ]
// If we belong to a prefix group, we'd do `pub(crate) use prefix_3p::*;`
// For now, placeholder comment.
pub(crate) use bitcoin_imports::*;
pub(crate) use bitcoin_derive::*;
pub(crate) use bitcoinleveldb_status::*;
pub(crate) use bitcoinleveldb_slice::*;
pub(crate) use bitcoinleveldb_slice::Range;
pub(crate) use bitcoinleveldb_options::*;
pub(crate) use bitcoinleveldb_compaction::*;
pub(crate) use bitcoinleveldb_iterator::*;
pub(crate) use bitcoinleveldb_key::*;
pub(crate) use bitcoinleveldb_snapshot::*;
pub(crate) use bitcoinleveldb_cache::*;
pub(crate) use bitcoinleveldb_file::*;
pub(crate) use bitcoinleveldb_compactionstats::*;
pub(crate) use bitcoinleveldb_writebatch::*;
pub(crate) use bitcoinleveldb_dbimplwriter::*;
pub(crate) use bitcoinleveldb_env::*;
pub(crate) use bitcoinleveldb_tablecache::*;
pub(crate) use bitcoinleveldb_memtable::*;
pub(crate) use bitcoinleveldb_logwriter::*;
pub(crate) use bitcoinleveldb_dbinterface::*;
pub(crate) use bitcoinleveldb_version::*;
pub(crate) use bitcoinleveldb_versionedit::*;
pub(crate) use bitcoinleveldb_batch::*;
pub(crate) use bitcoinleveldb_comparator::*;
pub(crate) use bitcoinleveldb_versionset::*;
pub(crate) use bitcoinleveldb_log::*;
pub(crate) use bitcoinleveldb_logreader::*;
pub(crate) use bitcoinleveldb_iteratorinner::*;
pub(crate) use bitcoinleveldb_versionsetinterface::*;
pub(crate) use bitcoinleveldb_cfg::*;
pub(crate) use bitcoinleveldb_merger::*;
pub(crate) use bitcoinleveldb_tablebuilder::*;
pub(crate) use bitcoinleveldb_dbiterstate::*;

//----------------------------------------------[extras-compat-adapt]
pub(crate) trait OptionIsNullExt {
    fn is_null(&self) -> bool;
}

impl<T> OptionIsNullExt for Option<T> {
    #[inline]
    fn is_null(&self) -> bool {
        self.is_none()
    }
}

pub(crate) trait RawMutexAssertHeldExt {
    fn assert_held(&self);
}

impl RawMutexAssertHeldExt for RawMutex {
    #[inline]
    fn assert_held(&self) {
        // LevelDB-style debug-only contract check; intentionally a no-op here.
        #[cfg(debug_assertions)]
        {
            let _ = self;
        }
    }
}

pub(crate) trait CondvarSignalExt {
    fn signal(&self);
    fn signal_all(&self);
}

impl CondvarSignalExt for Condvar {
    #[inline]
    fn signal(&self) {
        self.notify_one();
    }

    #[inline]
    fn signal_all(&self) {
        self.notify_all();
    }
}

pub(crate) trait InternalKeyConstPtrDebugExt {
    fn debug_string(self) -> String;
}

impl InternalKeyConstPtrDebugExt for *const InternalKey {
    #[inline]
    fn debug_string(self) -> String {
        unsafe {
            self.as_ref()
                .map(|k| k.debug_string())
                .unwrap_or_else(|| "<null InternalKey>".to_string())
        }
    }
}

pub(crate) trait SnapshotImplMutPtrSequenceExt {
    fn sequence_number(self) -> SequenceNumber;
}

impl SnapshotImplMutPtrSequenceExt for *mut SnapshotImpl {
    #[inline]
    fn sequence_number(self) -> SequenceNumber {
        unsafe { self.as_ref().map(|s| *s.sequence_number()).unwrap_or(0) }
    }
}

pub(crate) trait CacheMutPtrTotalChargeExt {
    fn total_charge(self) -> usize;
}

impl CacheMutPtrTotalChargeExt for *mut Cache {
#[inline]
    fn total_charge(self) -> usize {
        unsafe { self.as_mut().map(|c| c.total_charge()).unwrap_or(0) }
    }
}


pub(crate) use bitcoinleveldb_filter::*;
