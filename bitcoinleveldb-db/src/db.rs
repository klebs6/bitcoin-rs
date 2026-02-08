// ---------------- [ File: bitcoinleveldb-db/src/db.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/db.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/c.cc]

pub struct LevelDB {
    rep: Rc<RefCell<dyn DB>>,
}

pub struct LevelDBSnapshot {
    rep: Rc<dyn Snapshot>,
}

pub struct LevelDBReadOptions {
    rep: ReadOptions,
}

pub struct LevelDBWriteOptions {
    rep: WriteOptions,
}

pub struct LevelDBOptions {
    rep: Options,
}

pub struct LevelDBCache {
    rep: Rc<RefCell<crate::Cache>>,
}

pub struct LevelDBSeqFile {
    rep: Rc<RefCell<dyn SequentialFile>>,
}

pub struct LevelDBRandomFile {
    rep: Rc<RefCell<dyn RandomAccessFile>>,
}

pub struct LevelDBWritableFile {
    rep: Rc<RefCell<dyn WritableFile>>,
}

pub struct LevelDBLogger {
    rep: Rc<RefCell<dyn Logger>>,
}

pub struct LevelDBFileLock {
    rep: Rc<RefCell<Box<dyn FileLock>>>,
}

///-----------------
pub struct LevelDBEnv {
    rep:        Rc<RefCell<dyn Env>>,
    is_default: bool,
}
