// ---------------- [ File: bitcoinleveldb-db/src/leveldb_free.rs ]
crate::ix!();

pub fn leveldb_free(ptr: *mut core::ffi::c_void) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_free entry"; "ptr_is_null" => ptr.is_null());

    unsafe {
        libc::free(ptr);
    }
}
