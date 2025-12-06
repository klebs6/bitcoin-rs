// ---------------- [ File: bitcoinleveldb-table/src/table_and_file.rs ]
crate::ix!();

#[derive(Getters)]
#[getset(get_copy = "pub")]
pub struct TableAndFile {
    file:  *mut dyn RandomAccessFile,
    table: *mut table::Table,
}

impl TableAndFile {

    #[inline]
    pub fn file_ptr(&self) -> *mut dyn RandomAccessFile {
        let ptr = self.file;
        trace!(
            "TableAndFile::file_ptr: returning file pointer {:?}",
            ptr
        );
        ptr
    }

    #[inline]
    pub fn table_ptr(&self) -> *mut table::Table {
        let ptr = self.table;
        trace!(
            "TableAndFile::table_ptr: returning table pointer {:?}",
            ptr
        );
        ptr
    }
}

