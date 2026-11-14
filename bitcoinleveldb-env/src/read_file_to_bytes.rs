// ---------------- [ File: bitcoinleveldb-env/src/read_file_to_bytes.rs ]
crate::ix!();

/// Read exact file bytes into a cleared caller-owned vector.
///
/// On a read error, retain bytes from earlier successful reads and return that
/// error. A fragment accompanying a failed read is not appended. The destination
/// must be non-null, aligned, writable, and uniquely borrowed for this call.
pub fn read_file_to_bytes(
    env:   Rc<RefCell<dyn Env>>,
    fname: &String,
    data:  *mut Vec<u8>,
) -> Status {
    use std::ptr;

    trace!(file = %fname, "read_file_to_bytes: start");

    // C++: data->clear();
    assert!(!data.is_null(), "read_file_to_bytes: output is null");
    let data_ref: &mut Vec<u8> = unsafe { &mut *data };
    data_ref.clear();

    // C++: SequentialFile* file; Status s = env->NewSequentialFile(fname, &file);
    let mut file_ptr: *mut Box<dyn SequentialFile> = ptr::null_mut();
    let mut status = {
        let mut env_ref = env.borrow_mut();
        env_ref.new_sequential_file(fname, &mut file_ptr)
    };

    if !status.is_ok() {
        debug!(
            file = %fname,
            status_str = %status.to_string(),
            "read_file_to_bytes: NewSequentialFile failed"
        );
        return status;
    }

    // Take ownership of the allocated file object (mirrors C++ delete file).
    // We were given a `*mut Box<dyn SequentialFile>` so we first reconstruct
    // that outer Box, then borrow the inner `Box<dyn SequentialFile>` mutably.
    let mut file_holder: Box<Box<dyn SequentialFile>> = unsafe {
        assert!(
            !file_ptr.is_null(),
            "Env::NewSequentialFile returned a null file pointer"
        );
        Box::from_raw(file_ptr)
    };

    let file: &mut Box<dyn SequentialFile> = file_holder.as_mut();

    const K_BUFFER_SIZE: usize = 8192;
    let mut scratch  = vec![0u8; K_BUFFER_SIZE];
    let mut fragment = Slice::default();

    loop {
        // C++: s = file->Read(kBufferSize, &fragment, space);
        status = file.read(K_BUFFER_SIZE, &mut fragment, scratch.as_mut_ptr());
        if !status.is_ok() {
            debug!(
                file = %fname,
                status_str = %status.to_string(),
                "read_file_to_bytes: read failed"
            );
            break;
        }

        // C++: data->append(fragment.data(), fragment.size());
        if !fragment.empty() {
            data_ref.extend_from_slice(fragment.as_bytes());
        }

        // C++: if (fragment.empty()) break;
        if fragment.empty() {
            debug!(
                file = %fname,
                "read_file_to_bytes: reached EOF (empty fragment)"
            );
            break;
        }
    }

    // `file_holder` drops here, mirroring `delete file` in C++.
    debug!(
        file = %fname,
        ok = status.is_ok(),
        "read_file_to_bytes: completed"
    );
    status
}
