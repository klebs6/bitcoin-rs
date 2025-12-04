// ---------------- [ File: bitcoinleveldb-table/src/delete_entry.rs ]
crate::ix!();

pub fn delete_entry(_key_: &Slice, value: *mut c_void) {
    unsafe {
        if value.is_null() {
            trace!("delete_entry: value is null; nothing to delete");
            return;
        }

        let tf = &mut *(value as *mut TableAndFile);
        let table_ptr = tf.table();
        let file_ptr  = tf.file();

        trace!(
            "delete_entry: deleting TableAndFile at {:?} (table={:?}, file={:?})",
            value,
            table_ptr,
            file_ptr
        );

        if !table_ptr.is_null() {
            trace!(
                "delete_entry: deleting table object at {:?}",
                table_ptr
            );
            let _table_box: Box<table::Table> =
                Box::from_raw(table_ptr);
        }

        if !file_ptr.is_null() {
            trace!(
                "delete_entry: deleting RandomAccessFile at {:?}",
                file_ptr
            );
            let _file_box: Box<dyn RandomAccessFile> =
                Box::from_raw(file_ptr);
        }

        let _tf_box: Box<TableAndFile> =
            Box::from_raw(value as *mut TableAndFile);
    }
}
