// ---------------- [ File: bitcoinleveldb-table/src/delete_entry.rs ]
crate::ix!();

pub fn delete_entry(_key_: &Slice, value: *mut c_void) {
    unsafe {
        if value.is_null() {
            trace!("delete_entry: value is null; nothing to delete");
            return;
        }

        let tf = &mut *(value as *mut TableAndFile);
        let table_ptr = tf.table_ptr();
        let file_holder_ptr = tf.file_ptr();

        trace!(
            "delete_entry: TableAndFile @ {:?}, table_ptr={:?}, file_holder_ptr={:?}",
            value,
            table_ptr,
            file_holder_ptr
        );

        if !table_ptr.is_null() {
            trace!(
                "delete_entry: deleting table object at {:?}",
                table_ptr
            );
            let _table_box: Box<table::Table> =
                Box::from_raw(table_ptr);
        }

        if !file_holder_ptr.is_null() {
            trace!(
                "delete_entry: deleting RandomAccessFile holder at {:?}",
                file_holder_ptr
            );
            let _file_holder_box: Box<Box<dyn RandomAccessFile>> =
                Box::from_raw(file_holder_ptr);
        }

        let _tf_box: Box<TableAndFile> =
            Box::from_raw(value as *mut TableAndFile);
    }
}

#[cfg(test)]
mod delete_entry_resource_release_behavior {
    use super::*;
    use bitcoin_imports::Named;
    use bitcoinleveldb_file::RandomAccessFileRead;
    use std::borrow::Cow;
    use std::cell::Cell;
    use std::rc::Rc;

    #[derive(Clone)]
    struct DropCountingRandomAccessFile {
        drops: Rc<Cell<usize>>,
    }

    impl DropCountingRandomAccessFile {
        fn new(drops: Rc<Cell<usize>>) -> Self {
            DropCountingRandomAccessFile { drops }
        }
    }

    impl Drop for DropCountingRandomAccessFile {
        fn drop(&mut self) {
            let before = self.drops.get();
            self.drops.set(before + 1);
            trace!(
                "DropCountingRandomAccessFile::drop: drops={} -> {}",
                before,
                self.drops.get()
            );
        }
    }

    impl Named for DropCountingRandomAccessFile {
        fn name(&self) -> Cow<'_, str> {
            Cow::Owned("DropCountingRandomAccessFile".to_string())
        }
    }

    impl RandomAccessFileRead for DropCountingRandomAccessFile {
        fn read(
            &self,
            offset: u64,
            n: usize,
            result: *mut Slice,
            scratch: *mut u8,
        ) -> Status {
            trace!(
                "DropCountingRandomAccessFile::read: offset={}, n={}, scratch={:?}",
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

    impl RandomAccessFile for DropCountingRandomAccessFile {}

    #[traced_test]
    fn delete_entry_releases_table_and_file() {
        let drop_counter = Rc::new(Cell::new(0));

        let inner_file: Box<dyn RandomAccessFile> =
            Box::new(DropCountingRandomAccessFile::new(drop_counter.clone()));
        let file_holder: Box<Box<dyn RandomAccessFile>> = Box::new(inner_file);
        let file_holder_ptr: *mut Box<dyn RandomAccessFile> =
            Box::into_raw(file_holder);

        let table_box: Box<table::Table> =
            Box::new(Table::new(core::ptr::null_mut()));
        let table_ptr: *mut table::Table = Box::into_raw(table_box);

        let tf = TableAndFile::new_for_tests(file_holder_ptr, table_ptr);
        let tf_box = Box::new(tf);
        let value_ptr: *mut TableAndFile = Box::into_raw(tf_box);

        trace!(
            "delete_entry_releases_table_and_file: value_ptr={:?}, table_ptr={:?}, file_holder_ptr={:?}",
            value_ptr,
            table_ptr,
            file_holder_ptr
        );

        delete_entry(&Slice::default(), value_ptr as *mut c_void);

        assert_eq!(
            drop_counter.get(),
            1,
            "file must be dropped exactly once when delete_entry is given a non-null TableAndFile"
        );
    }

    #[traced_test]
    fn delete_entry_ignores_null_value() {
        trace!("delete_entry_ignores_null_value: calling delete_entry with value=null");
        delete_entry(&Slice::default(), core::ptr::null_mut());
    }

    #[traced_test]
    fn delete_entry_with_null_table_pointer_releases_file_only() {
        let drop_counter = Rc::new(Cell::new(0));

        let inner_file: Box<dyn RandomAccessFile> =
            Box::new(DropCountingRandomAccessFile::new(drop_counter.clone()));
        let file_holder: Box<Box<dyn RandomAccessFile>> = Box::new(inner_file);
        let file_holder_ptr: *mut Box<dyn RandomAccessFile> =
            Box::into_raw(file_holder);

        let tf = TableAndFile::new_for_tests(file_holder_ptr, core::ptr::null_mut());
        let tf_box = Box::new(tf);
        let value_ptr: *mut TableAndFile = Box::into_raw(tf_box);

        trace!(
            "delete_entry_with_null_table_pointer_releases_file_only: value_ptr={:?}, file_holder_ptr={:?}",
            value_ptr,
            file_holder_ptr
        );

        delete_entry(&Slice::default(), value_ptr as *mut c_void);

        assert_eq!(
            drop_counter.get(),
            1,
            "file must be dropped exactly once even when table pointer is null"
        );
    }

    #[traced_test]
    fn delete_entry_with_null_file_pointer_does_not_drop_external_file() {
        let drop_counter = Rc::new(Cell::new(0));

        let file_box: Box<dyn RandomAccessFile> =
            Box::new(DropCountingRandomAccessFile::new(drop_counter.clone()));

        let table_box: Box<table::Table> =
            Box::new(Table::new(core::ptr::null_mut()));
        let table_ptr: *mut table::Table = Box::into_raw(table_box);

        let null_file_holder_ptr: *mut Box<dyn RandomAccessFile> =
            core::ptr::null_mut();

        let tf = TableAndFile::new_for_tests(null_file_holder_ptr, table_ptr);
        let tf_box = Box::new(tf);
        let value_ptr: *mut TableAndFile = Box::into_raw(tf_box);

        trace!(
            "delete_entry_with_null_file_pointer_does_not_drop_external_file: value_ptr={:?}, table_ptr={:?}",
            value_ptr,
            table_ptr
        );

        delete_entry(&Slice::default(), value_ptr as *mut c_void);

        assert_eq!(
            drop_counter.get(),
            0,
            "file drop counter must remain zero when the TableAndFile carries a null file pointer"
        );

        drop(file_box);

        assert_eq!(
            drop_counter.get(),
            1,
            "external Box drop must still drop the file exactly once"
        );
    }
}
