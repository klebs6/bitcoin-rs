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
            target: "bitcoinleveldb_db::destroy_db",
            dbname = %dbname,
        "DestroyDB implementation entry"
    );

    // Match the C++ control flow exactly.
    let env_rc = match options.env().as_ref() {
        Some(e) => e.clone(),
        None => {
            tracing::error!(
                target: "bitcoinleveldb_db::destroy_db",
                dbname = %dbname,
                "DestroyDB: options.env is None"
            );

            let msg_string = String::from("DestroyDB: env is null");
            let msg_slice = Slice::from(&msg_string);
            return crate::Status::invalid_argument(&msg_slice, None);
        }
    };

    let mut filenames: Vec<String> = Vec::new();

    let mut env = env_rc.borrow_mut();

    let mut result = env.get_children(dbname, (&mut filenames) as *mut Vec<String>);
    if !result.is_ok() {
        // Ignore error in case directory does not exist
        tracing::debug!(
            target: "bitcoinleveldb_db::destroy_db",
            dbname = %dbname,
            status = %result.to_string(),
            "DestroyDB: GetChildren failed; returning OK per compatibility"
        );
        return crate::Status::ok();
    }

    let lockname: String = lock_file_name(dbname);
    let mut lock: *mut Box<dyn FileLock> = core::ptr::null_mut();

    result = env.lock_file(&lockname, (&mut lock) as *mut *mut Box<dyn FileLock>);

    if result.is_ok() {
        for i in 0..filenames.len() {
            let mut number: u64 = 0;
            let mut ty: FileType = FileType::LogFile;

            if parse_file_name(
                &filenames[i],
                (&mut number) as *mut u64,
                (&mut ty) as *mut FileType,
            ) && ty != FileType::DBLockFile
            {
                let fname = format!("{}/{}", dbname, filenames[i]);
                let del = env.delete_file(&fname);

                if result.is_ok() && !del.is_ok() {
                    tracing::warn!(
                        target: "bitcoinleveldb_db::destroy_db",
                        file = %fname,
                        status = %del.to_string(),
                        "DestroyDB: DeleteFile failed"
                    );
                    result = del;
                }
            }
        }

        // Ignore errors since state is already gone.
        let unlock_status = env.unlock_file(lock);
        if !unlock_status.is_ok() {
            tracing::debug!(
                target: "bitcoinleveldb_db::destroy_db",
                lockname = %lockname,
                status = %unlock_status.to_string(),
                "DestroyDB: UnlockFile failed (ignored)"
            );
        }

        let del_lock_status = env.delete_file(&lockname);
        if !del_lock_status.is_ok() {
            tracing::debug!(
                target: "bitcoinleveldb_db::destroy_db",
                lockname = %lockname,
                status = %del_lock_status.to_string(),
                "DestroyDB: DeleteFile(lock) failed (ignored)"
            );
        }

        let del_dir_status = env.delete_dir(dbname);
        if !del_dir_status.is_ok() {
            tracing::debug!(
                target: "bitcoinleveldb_db::destroy_db",
                dbname = %dbname,
                status = %del_dir_status.to_string(),
                "DestroyDB: DeleteDir failed (ignored)"
            );
        }
    } else {
        tracing::debug!(
            target: "bitcoinleveldb_db::destroy_db",
            lockname = %lockname,
            status = %result.to_string(),
            "DestroyDB: LockFile failed; returning status"
        );
    }

    if result.is_ok() {
        tracing::info!(
            target: "bitcoinleveldb_db::destroy_db",
            dbname = %dbname,
            "DestroyDB ok"
        );
    } else {
        tracing::warn!(
            target: "bitcoinleveldb_db::destroy_db",
            dbname = %dbname,
            status = %result.to_string(),
            "DestroyDB exit with error"
        );
    }

    result

    /*
        Env* env = options.env;
      std::vector<std::string> filenames;
      Status result = env->GetChildren(dbname, &filenames);
      if (!result.ok()) {
        // Ignore error in case directory does not exist
        return Status::OK();
      }

      FileLock* lock;
      const std::string lockname = LockFileName(dbname);
      result = env->LockFile(lockname, &lock);
      if (result.ok()) {
        uint64_t number;
        FileType type;
        for (size_t i = 0; i < filenames.size(); i++) {
          if (ParseFileName(filenames[i], &number, &type) &&
              type != kDBLockFile) {  // Lock file will be deleted at end
            Status del = env->DeleteFile(dbname + "/" + filenames[i]);
            if (result.ok() && !del.ok()) {
              result = del;
            }
          }
        }
        env->UnlockFile(lock);  // Ignore error since state is already gone
        env->DeleteFile(lockname);
        env->DeleteDir(dbname);  // Ignore error in case dir contains other files
      }
      return result;
    */
}
