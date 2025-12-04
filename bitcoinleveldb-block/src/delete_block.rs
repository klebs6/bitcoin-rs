// ---------------- [ File: bitcoinleveldb-block/src/delete_block.rs ]
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
