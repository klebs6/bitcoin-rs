// ---------------- [ File: bitcoinleveldb-db/src/leveldb_env.rs ]
crate::ix!();

pub fn leveldb_create_default_env() -> *mut LevelDBEnv {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_create_default_env entry");

    let default_opts = Options::default();

    let env_rc = match default_opts.env().as_ref() {
        Some(e) => e.clone(),
        None => {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "Options::default() did not provide a default env"
            );
            return core::ptr::null_mut();
        }
    };

    let result = Box::new(LevelDBEnvBuilder::default()
        .rep(env_rc)
        .is_default(true)
        .build()
        .unwrap()
    );

    let p = Box::into_raw(result);

    trace!(
        target: "bitcoinleveldb_db::c_api",
        ptr = (p as usize),
        "leveldb_create_default_env exit"
    );
    p
}

pub fn leveldb_env_destroy(env: *mut LevelDBEnv) {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        env_is_null = env.is_null(),
        "leveldb_env_destroy entry"
    );

    unsafe {
        if env.is_null() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_env_destroy called with null env"
            );
            return;
        }

        let boxed = Box::from_raw(env);
        if boxed.is_default().to_owned() {
            let leaked = boxed.rep().clone();
            core::mem::forget(leaked);
            trace!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_env_destroy leaked one Rc clone to keep default env alive"
            );
        }

        drop(boxed);
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_env_destroy exit");

}

pub fn leveldb_env_get_test_directory(env: *mut LevelDBEnv) -> *mut u8 {
    trace!(
        target: "bitcoinleveldb_db::c_api",
        env_is_null = env.is_null(),
        "leveldb_env_get_test_directory entry"
    );

    unsafe {
        if env.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                "leveldb_env_get_test_directory received null env"
            );
            return core::ptr::null_mut();
        }

        let mut result: String = String::new();
        let status = (*env)
            .rep()
            .borrow_mut()
            .get_test_directory((&mut result) as *mut String);

        if !status.is_ok() {
            warn!(
                target: "bitcoinleveldb_db::c_api",
                status = %status.to_string(),
                "GetTestDirectory failed"
            );
            return core::ptr::null_mut();
        }

        let bytes = result.as_bytes();
        let len = bytes.len();

        let buffer = libc::malloc(len + 1) as *mut u8;
        if buffer.is_null() {
            error!(
                target: "bitcoinleveldb_db::c_api",
                len = (len + 1),
                "malloc failed for test directory buffer"
            );
            return core::ptr::null_mut();
        }

        if len > 0 {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer, len);
        }
        *buffer.add(len) = 0;

        trace!(
            target: "bitcoinleveldb_db::c_api",
            len = len,
            ptr = (buffer as usize),
            "leveldb_env_get_test_directory exit"
        );

        buffer
    }

}
