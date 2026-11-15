// ---------------- [ File: bitcoinleveldb-db/src/leveldb_get.rs ]
crate::ix!();

pub fn leveldb_get(
    db: *mut LevelDB,
    options: *const LevelDBReadOptions,
    key_: *const u8,
    keylen: usize,
    vallen: *mut usize,
    errptr: *mut *mut u8,
) -> *mut u8 {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        db_is_null = db.is_null(),
        options_is_null = options.is_null(),
        vallen_is_null = vallen.is_null(),
        keylen = keylen,
        "leveldb_get entry"
    );

    unsafe {
        if db.is_null() || options.is_null() || vallen.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_get received null input pointer"
            );
            let msg = Slice::from_str("leveldb_get: null input");
            let s = crate::Status::invalid_argument(&msg, None);
            let _ = save_error(errptr, &s);
            if !vallen.is_null() {
                *vallen = 0;
            }
            return core::ptr::null_mut();
        }

        let ropt: &ReadOptions = (*options).rep();
        let k = Slice::from_ptr_len(key_, keylen);

        let mut tmp = Vec::new();
        let status = (*db)
            .rep()
            .borrow_mut()
            .get_bytes(ropt, &k, &mut tmp);

        if status.is_ok() {
            // A non-null allocation distinguishes an empty found value from
            // NotFound without changing the C pointer+length interface.
            let out = libc::malloc(tmp.len().max(1)) as *mut u8;
            if out.is_null() {
                *vallen = 0;
                let status = Status::io_error(&Slice::from("leveldb_get: allocation failed"), None);
                let _ = save_error(errptr, &status);
                return core::ptr::null_mut();
            }
            if !tmp.is_empty() {
                core::ptr::copy_nonoverlapping(tmp.as_ptr(), out, tmp.len());
            }
            *vallen = tmp.len();
            trace!(
                target: "bitcoinleveldb_db::c_api",
                vallen = *vallen,
                out_is_null = out.is_null(),
                "leveldb_get ok"
            );
            out
        } else {
            *vallen = 0;
            if !status.is_not_found() {
                let _ = save_error(errptr, &status);
                warn!(
                    target: "bitcoinleveldb_db::c_api",
                    status = %status.to_string(),
                    "leveldb_get failed (non-NotFound)"
                );
            } else {
                trace!(target: "bitcoinleveldb_db::c_api", "leveldb_get not found");
            }
            core::ptr::null_mut()
        }
    }

}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_get_rs__exhaustive_test_suite {
    use super::*;

    fn bitcoinleveldb_db__leveldb_get_rs__make_unique_dbname_bytes() -> Vec<u8> {
        crate::bitcoinleveldb_db__make_temp_dbname_bytes("bitcoinleveldb_db__get_rs__testdb")
    }

    unsafe fn bitcoinleveldb_db__leveldb_get_rs__free_err_if_non_null(err: *mut u8) {
        if !err.is_null() {
            crate::leveldb_free::leveldb_free(err as *mut core::ffi::c_void);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_get_rs__null_inputs_set_error_and_return_null() {
        unsafe {
            let mut err: *mut u8 = core::ptr::null_mut();
            let mut vallen: usize = 999usize;

            let out: *mut u8 = leveldb_get(
                core::ptr::null_mut(),
                core::ptr::null(),
                core::ptr::null(),
                0usize,
                (&mut vallen) as *mut usize,
                (&mut err) as *mut *mut u8,
            );

            assert!(out.is_null());
            assert_eq!(vallen, 0usize);
            assert!(!err.is_null());
            bitcoinleveldb_db__leveldb_get_rs__free_err_if_non_null(err);
        }
    }

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_get_rs__get_roundtrip_returns_bytes_and_len_and_errptr_stays_null() {
        unsafe {
            let options: *mut LevelDBOptions = crate::leveldb_options::leveldb_options_create();
            assert!(!options.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(options, 1u8);

            let dbname_bytes: Vec<u8> =
                bitcoinleveldb_db__leveldb_get_rs__make_unique_dbname_bytes();

            let mut oerr: *mut u8 = core::ptr::null_mut();

            let db: *mut LevelDB = crate::leveldb_open::leveldb_open(
                options,
                dbname_bytes.as_ptr(),
                (&mut oerr) as *mut *mut u8,
            );

            assert!(oerr.is_null());
            assert!(!db.is_null());

            let wopt: *mut LevelDBWriteOptions = crate::leveldb_writeoptions::leveldb_writeoptions_create();
            let ropt: *mut LevelDBReadOptions = crate::leveldb_readoptions::leveldb_readoptions_create();
            assert!(!wopt.is_null());
            assert!(!ropt.is_null());

            let key: [u8; 3] = [b'k', b'0', b'1'];
            let val: [u8; 3] = [b'v', b'0', b'1'];

            let mut perr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_put::leveldb_put(
                db,
                wopt,
                key.as_ptr(),
                key.len(),
                val.as_ptr(),
                val.len(),
                (&mut perr) as *mut *mut u8,
            );
            assert!(perr.is_null());

            let mut vallen: usize = 0usize;
            let mut gerr: *mut u8 = core::ptr::null_mut();
            let out: *mut u8 = leveldb_get(
                db,
                ropt,
                key.as_ptr(),
                key.len(),
                (&mut vallen) as *mut usize,
                (&mut gerr) as *mut *mut u8,
            );

            assert!(gerr.is_null());
            assert!(!out.is_null());
            assert_eq!(vallen, val.len());

            let out_bytes: Vec<u8> =
                unsafe { core::slice::from_raw_parts(out as *const u8, vallen).to_vec() };
            assert_eq!(out_bytes.as_slice(), val.as_slice());

            crate::leveldb_free::leveldb_free(out as *mut core::ffi::c_void);

            let missing_key: [u8; 3] = [b'k', b'9', b'9'];
            let mut miss_len: usize = 123usize;
            let mut merr: *mut u8 = core::ptr::null_mut();
            let miss_out: *mut u8 = leveldb_get(
                db,
                ropt,
                missing_key.as_ptr(),
                missing_key.len(),
                (&mut miss_len) as *mut usize,
                (&mut merr) as *mut *mut u8,
            );

            assert!(miss_out.is_null());
            assert_eq!(miss_len, 0usize);
            assert!(merr.is_null());

            crate::leveldb_readoptions::leveldb_readoptions_destroy(ropt);
            crate::leveldb_writeoptions::leveldb_writeoptions_destroy(wopt);

            crate::leveldb_close::leveldb_close(db);

            let mut derr: *mut u8 = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(
                options,
                dbname_bytes.as_ptr(),
                (&mut derr) as *mut *mut u8,
            );

            bitcoinleveldb_db__leveldb_get_rs__free_err_if_non_null(derr);
            crate::leveldb_options::leveldb_options_destroy(options);
        }
    }
}


#[cfg(test)]
mod binary_get_contracts {
    use super::*;
    struct Database {
        db: *mut LevelDB,
        options: *mut LevelDBOptions,
        read: *mut LevelDBReadOptions,
        write: *mut LevelDBWriteOptions,
        name: Vec<u8>,
        snapshots: Vec<(*const LevelDBSnapshot, *mut LevelDBReadOptions)>,
    }
    impl Database {
        fn new() -> Self {
            let mut f = Self {
                db: core::ptr::null_mut(), options: crate::leveldb_options::leveldb_options_create(),
                read: crate::leveldb_readoptions::leveldb_readoptions_create(),
                write: crate::leveldb_writeoptions::leveldb_writeoptions_create(),
                name: crate::bitcoinleveldb_db__make_temp_dbname_bytes("binary-get081"),
                snapshots: Vec::new(),
            };
            assert!(!f.options.is_null() && !f.read.is_null() && !f.write.is_null());
            crate::leveldb_options::leveldb_options_set_create_if_missing(f.options, 1);
            f.reopen(); f
        }
        fn reopen(&mut self) {
            assert!(self.snapshots.is_empty());
            if !self.db.is_null() { crate::leveldb_close::leveldb_close(self.db); self.db = core::ptr::null_mut(); }
            let mut error = core::ptr::null_mut();
            self.db = crate::leveldb_open::leveldb_open(self.options, self.name.as_ptr(), &mut error);
            assert!(error.is_null() && !self.db.is_null());
        }
        fn put(&self, key: &[u8], value: &[u8]) {
            let mut error = core::ptr::null_mut();
            crate::leveldb_put::leveldb_put(self.db, self.write, key.as_ptr(), key.len(), value.as_ptr(), value.len(), &mut error);
            assert!(error.is_null());
        }
        fn get(&self, key: &[u8], read: *mut LevelDBReadOptions) -> Option<Vec<u8>> {
            let mut len = usize::MAX; let mut error = core::ptr::null_mut();
            let out = leveldb_get(self.db, read, key.as_ptr(), key.len(), &mut len, &mut error);
            assert!(error.is_null());
            if out.is_null() { assert_eq!(len, 0); return None; }
            let bytes = unsafe { core::slice::from_raw_parts(out, len).to_vec() };
            crate::leveldb_free::leveldb_free(out as *mut core::ffi::c_void);
            Some(bytes)
        }
        fn snapshot(&mut self) -> *mut LevelDBReadOptions {
            let snapshot = crate::leveldb_create_snapshot::leveldb_create_snapshot(self.db);
            let read = crate::leveldb_readoptions::leveldb_readoptions_create();
            assert!(!snapshot.is_null() && !read.is_null());
            crate::leveldb_readoptions::leveldb_readoptions_set_snapshot(read, snapshot);
            self.snapshots.push((snapshot, read)); read
        }
        fn release_snapshots(&mut self) {
            for (snapshot, read) in self.snapshots.drain(..) {
                crate::leveldb_readoptions::leveldb_readoptions_destroy(read);
                crate::leveldb_release_snapshot::leveldb_release_snapshot(self.db, snapshot);
            }
        }
        fn compact_and_verify_sst(&self) {
            crate::leveldb_compact_range::leveldb_compact_range(self.db, core::ptr::null(), 0, core::ptr::null(), 0);
            let directory = std::str::from_utf8(&self.name[..self.name.len()-1]).unwrap();
            let has_table = std::fs::read_dir(directory).unwrap().any(|entry| {
                let path = entry.unwrap().path();
                matches!(path.extension().and_then(|s| s.to_str()), Some("ldb") | Some("sst"))
            });
            assert!(has_table, "compaction must create an SST before checking file-backed retrieval");
        }
    }
    impl Drop for Database {
        fn drop(&mut self) {
            self.release_snapshots();
            crate::leveldb_readoptions::leveldb_readoptions_destroy(self.read);
            crate::leveldb_writeoptions::leveldb_writeoptions_destroy(self.write);
            if !self.db.is_null() { crate::leveldb_close::leveldb_close(self.db); }
            let mut error = core::ptr::null_mut();
            crate::leveldb_destroy_db::leveldb_destroy_db(self.options, self.name.as_ptr(), &mut error);
            if !error.is_null() { crate::leveldb_free::leveldb_free(error as *mut core::ffi::c_void); }
            crate::leveldb_options::leveldb_options_destroy(self.options);
        }
    }

    #[test]
    fn binary_get_roundtrip_in_memory_snapshot_sst_and_reopen() {
        let mut f = Database::new();
        let key: Vec<u8> = (0u8..=255).collect();
        let old: Vec<u8> = (0u8..=255).collect();
        let new: Vec<u8> = old.iter().rev().copied().collect();
        f.put(&key, &old);
        let snapshot = f.snapshot();
        f.put(&key, &new);
        assert_eq!(f.get(&key, f.read), Some(new.clone()));
        assert_eq!(f.get(&key, snapshot), Some(old.clone()));
        // Both versions must remain readable after the mutable table is
        // flushed to a real SST while this snapshot is still held.
        f.compact_and_verify_sst();
        assert_eq!(f.get(&key, f.read), Some(new.clone()));
        assert_eq!(f.get(&key, snapshot), Some(old.clone()));
        // Exercise the checked Rust text bridge through the actual DBImpl vtable.
        let mut text = String::from("unchanged");
        let status = unsafe { (*f.db).rep().borrow_mut().get((*f.read).rep(), &Slice::from(&key[..]), &mut text) };
        assert!(status.is_corruption()); assert_eq!(text, "unchanged");
        let mut error = core::ptr::null_mut();
        crate::leveldb_delete::leveldb_delete(f.db, f.write, key.as_ptr(), key.len(), &mut error);
        assert!(error.is_null()); assert_eq!(f.get(&key, f.read), None);
        assert_eq!(f.get(&key, snapshot), Some(old));
        let mut bytes = vec![0xaa, 0xff];
        let status = unsafe { (*f.db).rep().borrow_mut().get_bytes((*f.read).rep(), &Slice::from(&key[..]), &mut bytes) };
        assert!(status.is_not_found()); assert_eq!(bytes, vec![0xaa, 0xff]);
        f.release_snapshots();
        f.put(&key, &new);
        f.put(b"empty", &[]);
        assert_eq!(f.get(b"empty", f.read), Some(Vec::new()));
        f.compact_and_verify_sst();
        f.reopen();
        assert_eq!(f.get(&key, f.read), Some(new));
        assert_eq!(f.get(b"empty", f.read), Some(Vec::new()));
        assert_eq!(f.get(b"missing", f.read), None);
    }
}
