// ---------------- [ File: bitcoinleveldb-table/src/table_and_file.rs ]
crate::ix!();

#[derive(Getters, MutGetters)]
#[getset(get_copy = "pub", get_mut = "pub")]
pub struct TableAndFile {
    file:  *mut dyn RandomAccessFile,
    table: *mut table::Table,
}
