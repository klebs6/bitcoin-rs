// ---------------- [ File: bitcoinleveldb-table/src/db_iter.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_iter.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_iter.cc]

pub struct LevelDBIterator {
    rep: Rc<RefCell<LevelDBIteratorInner>>,
}
