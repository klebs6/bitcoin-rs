// ---------------- [ File: bitcoinleveldb-env/src/write_string_to_file_impl.rs ]
crate::ix!();

pub fn do_write_string_to_file(
    env:         Rc<RefCell<dyn Env>>,
    data:        &Slice,
    fname:       &String,
    should_sync: bool,
) -> Status {
    use std::ptr;

    trace!(
        file = %fname,
        should_sync,
        data_len = *data.size(),
        "do_write_string_to_file: start"
    );

    // C++: WritableFile* file; Status s = env->NewWritableFile(fname, &file);
    let mut file_ptr: *mut Box<dyn WritableFile> = ptr::null_mut();
    let mut status = {
        let mut env_ref = env.borrow_mut();
        env_ref.new_writable_file(fname, &mut file_ptr)
    };

    if !status.is_ok() {
        debug!(
            file = %fname,
            status_str = %status.to_string(),
            "do_write_string_to_file: NewWritableFile failed"
        );
        return status;
    }

    // Take ownership of allocated file (mirrors C++ delete file).
    // We were given `*mut Box<dyn WritableFile>`, so reconstruct the outer Box
    // and then borrow the inner `Box<dyn WritableFile>` mutably.
    let mut file_holder: Box<Box<dyn WritableFile>> = unsafe {
        assert!(
            !file_ptr.is_null(),
            "Env::NewWritableFile returned a null file pointer"
        );
        Box::from_raw(file_ptr)
    };

    let file: &mut Box<dyn WritableFile> = file_holder.as_mut();

    // C++: s = file->Append(data);
    status = file.append(data);

    // C++: if (s.ok() && should_sync) { s = file->Sync(); }
    if status.is_ok() && should_sync {
        trace!(file = %fname, "do_write_string_to_file: calling Sync");
        status = file.sync();
    }

    // C++: if (s.ok()) { s = file->Close(); }
    if status.is_ok() {
        trace!(file = %fname, "do_write_string_to_file: calling Close");
        status = file.close();
    }

    // `file_holder` drops here, mirroring `delete file` in C++.

    // C++: if (!s.ok()) { env->DeleteFile(fname); }
    if !status.is_ok() {
        debug!(
            file = %fname,
            status_str = %status.to_string(),
            "do_write_string_to_file: error, deleting file"
        );
        let mut env_ref = env.borrow_mut();
        let _ = env_ref.delete_file(fname);
    }

    debug!(
        file = %fname,
        should_sync,
        ok = status.is_ok(),
        "do_write_string_to_file: completed"
    );
    status
}
