// ---------------- [ File: bitcoinleveldb-db/src/db.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/db.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/c.cc]

#[derive(Getters)]
#[getset(get="pub")]
pub struct LevelDB {
    rep: Rc<RefCell<DBImpl>>,
}

#[derive(Getters)]
#[getset(get = "pub")]
pub struct LevelDBSnapshot {
    db_rep: Rc<RefCell<DBImpl>>,
    snap:   Option<Box<dyn Snapshot>>,
}

impl Snapshot for LevelDBSnapshot {}

impl Drop for LevelDBSnapshot {
    fn drop(&mut self) {
        if let Some(s) = self.snap.take() {
            trace!(target: "bitcoinleveldb_db::c_api", "LevelDBSnapshot::drop releasing snapshot");
            self.db_rep.borrow_mut().release_snapshot(s);
        }
    }
}

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct LevelDBReadOptions {
    rep: ReadOptions,
}

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct LevelDBWriteOptions {
    rep: WriteOptions,
}

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct LevelDBOptions {
    rep: Options,
}

#[derive(Getters)]
#[getset(get="pub")]
pub struct LevelDBCache {
    rep: Rc<RefCell<crate::Cache>>,
}

#[derive(Getters)]
#[getset(get="pub")]
pub struct LevelDBSeqFile {
    rep: Rc<RefCell<dyn SequentialFile>>,
}

#[derive(Getters)]
#[getset(get="pub")]
pub struct LevelDBRandomFile {
    rep: Rc<RefCell<dyn RandomAccessFile>>,
}

#[derive(Getters)]
#[getset(get="pub")]
pub struct LevelDBWritableFile {
    rep: Rc<RefCell<dyn WritableFile>>,
}

#[derive(Getters)]
#[getset(get="pub")]
pub struct LevelDBLogger {
    rep: Rc<RefCell<dyn Logger>>,
}

#[derive(Getters)]
#[getset(get="pub")]
pub struct LevelDBFileLock {
    rep: Rc<RefCell<Box<dyn FileLock>>>,
}

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct LevelDBWriteBatch {
    rep: WriteBatch,
}

///-----------------
#[derive(Getters)]
#[getset(get="pub")]
pub struct LevelDBEnv {
    rep:        Rc<RefCell<dyn Env>>,
    is_default: bool,
}

