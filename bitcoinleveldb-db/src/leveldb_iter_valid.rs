// ---------------- [ File: bitcoinleveldb-db/src/leveldb_iter_valid.rs ]
crate::ix!();

pub fn leveldb_iter_valid(iter: *const LevelDBIterator) -> u8 {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        iter_is_null = iter.is_null(),
        "leveldb_iter_valid entry"
    );

    unsafe {
        if iter.is_null() {
            return 0;
        }

        let v = (*iter).valid();
        v as u8
    }
}
