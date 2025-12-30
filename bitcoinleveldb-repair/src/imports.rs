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

    #[traced_test]
    fn imports_parses_log_filename_via_parse_file_name_interface() {
        trace!("imports_smoke_suite: start");

        let mut number: u64 = 0;
        let mut ty: FileType = FileType::LogFile;

        let filename: String = "000001.log".to_string();
        let parsed = parse_file_name(&filename, &mut number as *mut u64, &mut ty as *mut FileType);

        info!(
            parsed,
            number,
            ty = ?ty,
            filename = %filename,
            "imports_smoke_suite: parse_file_name result"
        );

        assert!(parsed);
        assert_eq!(number, 1);
        assert!(matches!(ty, FileType::LogFile));

        trace!("imports_smoke_suite: done");
    }
}
