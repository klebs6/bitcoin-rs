// ---------------- [ File: bitcoinleveldb-env/src/read_file_to_string.rs ]
crate::ix!();

/// Read a UTF-8 text file into a cleared caller-owned string.
///
/// Validate the complete buffer after reading, so a code point may span read
/// fragments. Successful reads with invalid UTF-8 return Corruption and leave
/// the string empty. An I/O/open error takes precedence over a text error; its
/// successful-read prefix is retained only when the whole prefix is valid UTF-8.
/// For arbitrary file contents use `read_file_to_bytes` instead.
///
/// The destination must be non-null, aligned, writable, and uniquely borrowed
/// for the duration of this call.
pub fn read_file_to_string(
    env:   Rc<RefCell<dyn Env>>,
    fname: &String,
    data:  *mut String,
) -> Status {
    assert!(!data.is_null(), "read_file_to_string: output is null");
    let output = unsafe { &mut *data };
    output.clear();

    let mut bytes = Vec::<u8>::new();
    let status = read_file_to_bytes(env, fname, &mut bytes);
    match String::from_utf8(bytes) {
        Ok(text) => {
            *output = text;
            status
        }
        Err(_) if !status.is_ok() => status,
        Err(_) => Status::corruption(
            &Slice::from("file contents are not valid UTF-8"),
            Some(&Slice::from(fname)),
        ),
    }
}
