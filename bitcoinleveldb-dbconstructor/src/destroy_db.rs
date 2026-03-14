// ---------------- [ File: bitcoinleveldb-dbconstructor/src/destroy_db.rs ]
crate::ix!();

/// Destroy the contents of the specified database.
///
/// Be very careful using this method.
/// 
/// Note: For backwards compatibility, if DestroyDB is unable to list the
/// database files, Status::OK() will still be returned masking this failure.
///
pub fn destroydb(dbname: &String, options: &Options) -> crate::Status {
    tracing::trace!(
        dbname = dbname.as_str(),
        "DestroyDB implementation entry"
    );

    let env_rc: std::rc::Rc<std::cell::RefCell<dyn Env>> = match options.env().as_ref() {
        Some(env) => std::rc::Rc::clone(env),
        None => bitcoinleveldb_posixenv::PosixEnv::shared(),
    };

    let mut env = env_rc.borrow_mut();

    let mut filenames: Vec<String> = Vec::new();
    let mut result = env.get_children(dbname, &mut filenames as *mut Vec<String>);

    if !result.is_ok() {
        if env.file_exists(dbname) {
            return result;
        } else {
            return crate::Status::ok();
        }
    }

    let lockname = lock_file_name(dbname);
    let mut lock: *mut Box<dyn FileLock> = std::ptr::null_mut();

    result = env.lock_file(&lockname, &mut lock as *mut *mut Box<dyn FileLock>);

    if result.is_ok() {
        let mut number: u64 = 0;
        let mut file_type = FileType::LogFile;

        for filename in filenames.iter() {
            if parse_file_name(filename, &mut number, &mut file_type)
                && file_type != FileType::DBLockFile
            {
                let mut full = dbname.clone();
                full.push('/');
                full.push_str(filename);

                let del = env.delete_file(&full);
                if result.is_ok() && !del.is_ok() {
                    result = del;
                }
            }
        }

        if !lock.is_null() {
            let _ = env.unlock_file(lock);
        }

        let _ = env.delete_file(&lockname);
        let _ = env.delete_dir(dbname);
    }

    tracing::trace!(
        dbname = dbname.as_str(),
        ok = result.is_ok(),
        "DestroyDB implementation exit"
    );

    result
}
