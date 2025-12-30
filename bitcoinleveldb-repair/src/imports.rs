// ---------------- [ File: bitcoinleveldb-repair/src/imports.rs ]
pub(crate) use bitcoin_derive::*;
pub(crate) use bitcoin_imports::*;
pub(crate) use bitcoinleveldb_cache::*;
pub(crate) use bitcoinleveldb_comparator::*;
pub(crate) use bitcoinleveldb_env::*;
pub(crate) use bitcoinleveldb_file::*;
pub(crate) use bitcoinleveldb_filter::*;
pub(crate) use bitcoinleveldb_key::*;
pub(crate) use bitcoinleveldb_options::*;
pub(crate) use bitcoinleveldb_status::*;
pub(crate) use bitcoinleveldb_table::*;
pub(crate) use bitcoinleveldb_versionedit::*;
pub(crate) use bitcoinleveldb_slice::*;
pub(crate) use bitcoinleveldb_memtable::*;
pub(crate) use bitcoinleveldb_iterator::*;
pub(crate) use bitcoinleveldb_tablecache::*;
pub(crate) use bitcoinleveldb_tablebuilder::*;
pub(crate) use bitcoinleveldb_log::*;
pub(crate) use bitcoinleveldb_logreader::*;
pub(crate) use bitcoinleveldb_writebatch::*;
pub(crate) use bitcoinleveldb_iteratorinner::*;
pub(crate) use bitcoinleveldb_batch::*;

#[cfg(test)]
mod imports_smoke_suite {
    use super::*;
    use tracing::{debug, info, trace, warn};

    #[traced_test]
    fn imports_module_reexports_core_leveldb_types_for_tests() {
        // This test is intentionally "smoke" oriented: it validates that commonly used
        // types and functions remain available from the prelude and compile together.
        let mut options = Options::default();

        let a = Slice::from(&b"a"[..]);
        let b = Slice::from(&b"b"[..]);

        let ok = crate::Status::ok();
        info!(ok = ok.is_ok(), status = %ok.to_string(), "constructed Status::ok");

        let dbname = "imports-smoke-db";
        let log = log_file_name(&dbname.to_string(), 1);
        trace!(log = %log, "computed log_file_name");

        let mut number: u64 = 0;
        let mut ty: FileType = FileType::LogFile;
        let parsed = parse_file_name("000001.log", &mut number as *mut u64, &mut ty as *mut FileType);
        debug!(parsed, number, "parse_file_name on canonical log file");
        assert!(parsed);
        let _ = (&a, &b, &mut options);
    }
}
