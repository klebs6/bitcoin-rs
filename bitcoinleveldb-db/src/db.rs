// ---------------- [ File: bitcoinleveldb-db/src/db.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/db.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/c.cc]

#[derive(Getters)]
#[getset(get="pub")]
pub struct LevelDB {
    rep: Rc<RefCell<DBImpl>>,
}

impl LevelDB {

    pub fn new(rep: Rc<RefCell<DBImpl>>) -> Self {
        Self { rep }
    }
}

#[derive(Getters)]
#[getset(get = "pub")]
pub struct LevelDBSnapshot {
    db_rep: Rc<RefCell<DBImpl>>,
    snap:   Option<Box<dyn Snapshot>>,
}

impl LevelDBSnapshot {
    pub fn new(
        db_rep: Rc<RefCell<DBImpl>>,
        snap:   Option<Box<dyn Snapshot>>
    ) -> Self {
        Self { db_rep, snap }
    }
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

#[derive(Default,Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct LevelDBReadOptions {
    rep: ReadOptions,
}

#[derive(Default,Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct LevelDBWriteOptions {
    rep: WriteOptions,
}

#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct LevelDBOptions {
    rep: Options,
}

impl Default for LevelDBOptions {

    fn default() -> Self {
        let env = PosixEnv::shared();
        Self { rep: Options::with_env(env) }
    }
}

#[derive(Default,Getters)]
#[getset(get="pub")]
pub struct LevelDBCache {
    rep: Rc<RefCell<crate::Cache>>,
}

impl LevelDBCache {
    pub fn new(cache: crate::Cache) -> Self {
        Self { rep: Rc::new(RefCell::new(cache)) }
    }
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

#[derive(Default,Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct LevelDBWriteBatch {
    rep: WriteBatch,
}

///-----------------
#[derive(Builder,Getters)]
#[getset(get="pub")]
#[builder(setter(into))]
pub struct LevelDBEnv {
    rep:        Rc<RefCell<dyn Env>>,
    is_default: bool,
}

#[cfg(test)]
mod bitcoinleveldb_db__db_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__db_rs__make_unique_dbname_bytes() -> Vec<u8> {
        let unique_box: Box<u8> = Box::new(0u8);
        let unique_ptr: *mut u8 = Box::into_raw(unique_box);
        let unique_tag: usize = unique_ptr as usize;
        unsafe {
            drop(Box::from_raw(unique_ptr));
        }

        let name: String = format!("bitcoinleveldb_db__db_rs__testdb_{}", unique_tag);
        let mut bytes: Vec<u8> = name.into_bytes();
        bytes.push(0u8);
        bytes
    }

    #[traced_test]
    fn bitcoinleveldb_db__db_rs__leveldb_new_preserves_rep_pointer_identity() {
        let dbname_bytes: Vec<u8> = bitcoinleveldb_db__db_rs__make_unique_dbname_bytes();
        let dbname_cstr: *const u8 = dbname_bytes.as_ptr();

        let env = PosixEnv::shared();
        let opts: Options = Options::with_env(env);
        let cstr = unsafe { std::ffi::CStr::from_ptr(dbname_cstr as *const core::ffi::c_char) };
        let dbname: String = cstr.to_string_lossy().into_owned();

        let rep: Rc<RefCell<DBImpl>> = Rc::new(RefCell::new(DBImpl::new(&opts, &dbname)));
        let wrapper: LevelDB = LevelDB::new(rep.clone());

        let ok: bool = Rc::ptr_eq(wrapper.rep(), &rep);
        assert!(ok);
    }

    #[traced_test]
    fn bitcoinleveldb_db__db_rs__snapshot_drop_is_noop_when_snap_is_none() {
        let dbname_bytes: Vec<u8> = bitcoinleveldb_db__db_rs__make_unique_dbname_bytes();
        let dbname_cstr: *const u8 = dbname_bytes.as_ptr();

        let env = PosixEnv::shared();
        let opts: Options = Options::with_env(env);
        let cstr = unsafe { std::ffi::CStr::from_ptr(dbname_cstr as *const core::ffi::c_char) };
        let dbname: String = cstr.to_string_lossy().into_owned();

        let rep: Rc<RefCell<DBImpl>> = Rc::new(RefCell::new(DBImpl::new(&opts, &dbname)));
        let snap: LevelDBSnapshot = LevelDBSnapshot::new(rep, None);

        drop(snap);

        assert!(true);
    }
}
