// ---------------- [ File: bitcoinleveldb-env/src/write_string_to_file.rs ]
crate::ix!();

/**
  | A utility routine: write "data" to the
  | named file.
  |
  */
pub fn write_string_to_file(
    env:  Rc<RefCell<dyn Env>>,
    data: &Slice,
    fname:&String,
) -> Status {

    trace!(
        file = %fname,
        data_len = *data.size(),
        "write_string_to_file: delegating to do_write_string_to_file (should_sync=false)"
    );
    do_write_string_to_file(env, data, fname, false)
}

/**
  | A utility routine: write "data" to the
  | named file and Sync() it.
  |
  */
pub fn write_string_to_file_sync(
    env:  Rc<RefCell<dyn Env>>,
    data: &Slice,
    fname:&String,
) -> Status {

    trace!(
        file = %fname,
        data_len = *data.size(),
        "write_string_to_file_sync: delegating to do_write_string_to_file (should_sync=true)"
    );
    do_write_string_to_file(env, data, fname, true)
}
