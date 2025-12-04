// ---------------- [ File: bitcoinleveldb-table/src/delete_block.rs ]
crate::ix!();

pub fn delete_block(arg: *mut c_void, _ignored: *mut c_void) {
    unsafe {
        if arg.is_null() {
            trace!("delete_block: arg is null; nothing to delete");
            return;
        }

        trace!("delete_block: deleting Block at {:?}", arg);
        let _block: Box<Block> = Box::from_raw(arg as *mut Block);
        // Drop happens when _block goes out of scope.
    }
}

pub fn delete_cached_block(_key_: &Slice, value: *mut c_void) {
    unsafe {
        if value.is_null() {
            trace!("delete_cached_block: value is null; nothing to delete");
            return;
        }

        trace!(
            "delete_cached_block: deleting cached Block at {:?}",
            value
        );
        let _block: Box<Block> = Box::from_raw(value as *mut Block);
    }
}

pub fn release_block(arg: *mut c_void, h: *mut c_void) {
    unsafe {
        if arg.is_null() || h.is_null() {
            debug!(
                "release_block: arg or handle is null (arg={:?}, h={:?}); nothing to release",
                arg, h
            );
            return;
        }

        let cache = &mut *(arg as *mut Cache);
        let handle = h as *mut CacheHandle;

        trace!(
            "release_block: releasing cache handle {:?} via Cache@{:?}",
            handle,
            cache as *mut Cache
        );

        cache.release(handle);
    }
}

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

pub fn unref_entry(arg1: *mut c_void, arg2: *mut c_void) {
    unsafe {
        if arg1.is_null() || arg2.is_null() {
            debug!(
                "unref_entry: arg1 or arg2 is null (arg1={:?}, arg2={:?}); nothing to release",
                arg1, arg2
            );
            return;
        }

        let cache = &mut *(arg1 as *mut Cache);
        let handle = arg2 as *mut CacheHandle;

        trace!(
            "unref_entry: releasing cache handle {:?} via Cache@{:?}",
            handle,
            cache as *mut Cache
        );

        cache.release(handle);
    }
}
