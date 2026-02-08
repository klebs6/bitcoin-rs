// ---------------- [ File: bitcoinleveldb-db/src/leveldb_env.rs ]
crate::ix!();

pub fn leveldb_create_default_env() -> *mut LevelDBEnv {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_create_default_env entry");

    let default_opts = Options::default();

    let env_rc = match default_opts.env().as_ref() {
        Some(e) => e.clone(),
        None => {
            error!(target: "bitcoinleveldb_db::c_api", "Options::default() did not provide a default env");
            return core::ptr::null_mut();
        }
    };

    let result = Box::new(LevelDBEnv {
        rep: env_rc,
        is_default: true,
    });

    let p = Box::into_raw(result);

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_create_default_env exit"; "ptr" => (p as usize));
    p

    /*
        leveldb_env_t* result = new leveldb_env_t;
      result->rep = Env::Default();
      result->is_default = true;
      return result;
    */
}

pub fn leveldb_env_destroy(env: *mut LevelDBEnv) {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_env_destroy entry"; "env_is_null" => env.is_null());

    unsafe {
        if env.is_null() {
            warn!(target: "bitcoinleveldb_db::c_api", "leveldb_env_destroy called with null env");
            return;
        }

        // Preserve default env semantics: the default env is treated as process-lifetime.
        // We do this by leaking one clone when destroying a default handle.
        let boxed = Box::from_raw(env);
        if boxed.is_default {
            let leaked = boxed.rep.clone();
            core::mem::forget(leaked);
            trace!(target: "bitcoinleveldb_db::c_api", "leveldb_env_destroy leaked one Rc clone to keep default env alive");
        }

        drop(boxed);
    }

    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_env_destroy exit");

    /*
        if (!env->is_default) delete env->rep;
      delete env;
    */
}

pub fn leveldb_env_get_test_directory(env: *mut LevelDBEnv) -> *mut u8 {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_env_get_test_directory entry"; "env_is_null" => env.is_null());

    unsafe {
        if env.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "leveldb_env_get_test_directory received null env");
            return core::ptr::null_mut();
        }

        let mut result: String = String::new();
        let status = (*env)
            .rep
            .borrow_mut()
            .get_test_directory((&mut result) as *mut String);

        if !status.is_ok() {
            warn!(target: "bitcoinleveldb_db::c_api", "GetTestDirectory failed"; "status" => %status.to_string());
            return core::ptr::null_mut();
        }

        let bytes = result.as_bytes();
        let len = bytes.len();

        let buffer = libc::malloc(len + 1) as *mut u8;
        if buffer.is_null() {
            error!(target: "bitcoinleveldb_db::c_api", "malloc failed for test directory buffer"; "len" => len + 1);
            return core::ptr::null_mut();
        }

        if len > 0 {
            core::ptr::copy_nonoverlapping(bytes.as_ptr(), buffer, len);
        }
        *buffer.add(len) = 0;

        trace!(target: "bitcoinleveldb_db::c_api", "leveldb_env_get_test_directory exit"; "len" => len, "ptr" => (buffer as usize));

        buffer
    }

    /*
        std::string result;
      if (!env->rep->GetTestDirectory(&result).ok()) {
        return nullptr;
      }

      char* buffer = static_cast<char*>(malloc(result.size() + 1));
      memcpy(buffer, result.data(), result.size());
      buffer[result.size()] = '\0';
      return buffer;
    */
}
