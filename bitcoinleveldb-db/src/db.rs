// ---------------- [ File: bitcoinleveldb-db/src/db.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/db.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/c.cc]

#[derive(Getters)]
#[getset(get="pub")]
pub struct LevelDB {
    rep: Rc<RefCell<Box<dyn DB>>>,
}

impl LevelDB {
    pub fn new(rep: Box<dyn DB>) -> Self {
        Self {
            rep: Rc::new(RefCell::new(rep)),
        }
    }
}

#[repr(C)]
#[derive(Getters)]
#[getset(get = "pub")]
pub struct LevelDBSnapshot {
    shadow: SnapshotImpl,
    db_rep: Rc<RefCell<Box<dyn DB>>>,
    snap:   Option<Box<dyn Snapshot>>,
}

impl LevelDBSnapshot {
    fn snapshot_shadow_from_option(
        snap: Option<&Box<dyn Snapshot>>
    ) -> SnapshotImpl {
        match snap {
            Some(s) => unsafe {
                let p: *const dyn Snapshot = (&**s) as *const dyn Snapshot;
                let impl_p: *const SnapshotImpl = p as *const SnapshotImpl;
                core::ptr::read(impl_p)
            },
            None => unsafe {
                core::mem::MaybeUninit::<SnapshotImpl>::zeroed().assume_init()
            },
        }
    }

    pub fn new(
        db_rep: Rc<RefCell<Box<dyn DB>>>,
        snap:   Option<Box<dyn Snapshot>>
    ) -> Self {
        let shadow: SnapshotImpl =
            Self::snapshot_shadow_from_option(snap.as_ref());

        Self {
            shadow,
            db_rep,
            snap,
        }
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
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__db_rs__testdb")
    }

    #[traced_test]
    fn bitcoinleveldb_db__db_rs__leveldb_new_preserves_rep_pointer_identity() {
        let dbname_bytes: Vec<u8> = bitcoinleveldb_db__db_rs__make_unique_dbname_bytes();
        let dbname_cstr: *const u8 = dbname_bytes.as_ptr();

        let env = PosixEnv::shared();
        let opts: Options = Options::with_env(env);
        let cstr = unsafe { std::ffi::CStr::from_ptr(dbname_cstr as *const core::ffi::c_char) };
        let dbname: String = cstr.to_string_lossy().into_owned();

        let rep: Box<dyn DB> = Box::new(DBImpl::new(&opts, &dbname));
        let before: *const () = ((&*rep) as &dyn DB) as *const dyn DB as *const ();

        let wrapper: LevelDB = LevelDB::new(rep);

        let after: *const () = {
            let borrowed = wrapper.rep().borrow();
            ((&**borrowed) as &dyn DB) as *const dyn DB as *const ()
        };

        assert_eq!(before, after);
    }

    #[traced_test]
    fn bitcoinleveldb_db__db_rs__snapshot_drop_is_noop_when_snap_is_none() {
        let dbname_bytes: Vec<u8> = bitcoinleveldb_db__db_rs__make_unique_dbname_bytes();
        let dbname_cstr: *const u8 = dbname_bytes.as_ptr();

        let env = PosixEnv::shared();
        let opts: Options = Options::with_env(env);
        let cstr = unsafe { std::ffi::CStr::from_ptr(dbname_cstr as *const core::ffi::c_char) };
        let dbname: String = cstr.to_string_lossy().into_owned();

        let rep: Rc<RefCell<Box<dyn DB>>> =
            Rc::new(RefCell::new(Box::new(DBImpl::new(&opts, &dbname)) as Box<dyn DB>));

        let snap: LevelDBSnapshot = LevelDBSnapshot::new(rep, None);

        drop(snap);

        assert!(true);
    }
}
