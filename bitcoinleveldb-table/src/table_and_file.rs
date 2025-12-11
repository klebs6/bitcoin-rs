// ---------------- [ File: bitcoinleveldb-table/src/table_and_file.rs ]
crate::ix!();

#[derive(Getters)]
#[getset(get_copy = "pub")]
pub struct TableAndFile {
    file:  *mut Box<dyn RandomAccessFile>,
    table: *mut Table,
}

impl TableAndFile {

    pub fn new(file: *mut Box<dyn RandomAccessFile>, table: *mut Table) -> Self {
        Self { file, table }
    }

    #[inline]
    pub fn file_ptr(&self) -> *mut Box<dyn RandomAccessFile> {
        let ptr = self.file;
        trace!(
            "TableAndFile::file_ptr: returning file holder pointer {:?}",
            ptr
        );
        ptr
    }

    #[inline]
    pub fn table_ptr(&self) -> *mut Table {
        let ptr = self.table;
        trace!(
            "TableAndFile::table_ptr: returning table pointer {:?}",
            ptr
        );
        ptr
    }

    #[cfg(test)]
    #[inline]
    pub fn new_for_tests(
        file:  *mut Box<dyn RandomAccessFile>,
        table: *mut table::Table,
    ) -> Self {
        trace!(
            "TableAndFile::new_for_tests: creating TableAndFile with file={:?}, table={:?}",
            file,
            table
        );
        TableAndFile { file, table }
    }
}

#[cfg(test)]
mod table_and_file_pointer_accessors_behavior {
    use super::*;
    use bitcoin_imports::Named;
    use bitcoinleveldb_file::RandomAccessFileRead;
    use std::borrow::Cow;

    struct DummyRandomAccessFile;

    impl Named for DummyRandomAccessFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Owned("DummyRandomAccessFile".to_string())
        }
    }

    impl RandomAccessFileRead for DummyRandomAccessFile {
        fn read(
            &self,
            offset: u64,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> Status {
            trace!(
                "DummyRandomAccessFile::read: offset={}, n={}, scratch={:?}",
                offset,
                n,
                scratch
            );
            unsafe {
                *result = Slice::default();
            }
            Status::ok()
        }
    }

    impl RandomAccessFile for DummyRandomAccessFile {}

    #[traced_test]
    fn table_and_file_accessors_round_trip_non_null_pointers() {
        let inner_file: Box<dyn RandomAccessFile> =
            Box::new(DummyRandomAccessFile);
        let file_holder: Box<Box<dyn RandomAccessFile>> = Box::new(inner_file);
        let file_holder_ptr: *mut Box<dyn RandomAccessFile> =
            Box::into_raw(file_holder);

        let mut table = Table::new(core::ptr::null_mut());
        let table_ptr: *mut table::Table = &mut table;

        let tf = TableAndFile::new_for_tests(file_holder_ptr, table_ptr);

        trace!(
            "table_and_file_accessors_round_trip_non_null_pointers: tf.file_ptr={:?}, tf.table_ptr={:?}",
            tf.file_ptr(),
            tf.table_ptr()
        );

        assert_eq!(tf.file_ptr(), file_holder_ptr);
        assert_eq!(tf.table_ptr(), table_ptr);
    }

    #[traced_test]
    fn table_and_file_accessors_round_trip_null_pointers() {
        let null_file: *mut Box<dyn RandomAccessFile> = core::ptr::null_mut();
        let null_table: *mut table::Table = core::ptr::null_mut();

        let tf = TableAndFile::new_for_tests(null_file, null_table);

        trace!(
            "table_and_file_accessors_round_trip_null_pointers: tf.file_ptr={:?}, tf.table_ptr={:?}",
            tf.file_ptr(),
            tf.table_ptr()
        );

        assert!(tf.file_ptr().is_null());
        assert!(tf.table_ptr().is_null());
    }

    #[traced_test]
    fn table_and_file_accessors_are_side_effect_free() {
        let inner_file: Box<dyn RandomAccessFile> =
            Box::new(DummyRandomAccessFile);
        let file_holder: Box<Box<dyn RandomAccessFile>> = Box::new(inner_file);
        let file_holder_ptr: *mut Box<dyn RandomAccessFile> =
            Box::into_raw(file_holder);

        let mut table = Table::new(core::ptr::null_mut());
        let table_ptr: *mut table::Table = &mut table;

        let tf = TableAndFile::new_for_tests(file_holder_ptr, table_ptr);

        let first_file_ptr = tf.file_ptr();
        let second_file_ptr = tf.file_ptr();
        let first_table_ptr = tf.table_ptr();
        let second_table_ptr = tf.table_ptr();

        trace!(
            "table_and_file_accessors_are_side_effect_free: first_file_ptr={:?}, second_file_ptr={:?}, first_table_ptr={:?}, second_table_ptr={:?}",
            first_file_ptr,
            second_file_ptr,
            first_table_ptr,
            second_table_ptr
        );

        assert_eq!(first_file_ptr, second_file_ptr);
        assert_eq!(first_table_ptr, second_table_ptr);
    }
}
