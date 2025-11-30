// ---------------- [ File: bitcoinleveldb-env/src/set_current_file.rs ]
crate::ix!();

/**
  | Make the CURRENT file point to the descriptor
  | file with the specified number.
  |
  */
pub fn set_current_file(
    env:                Rc<RefCell<dyn Env>>,
    dbname:             &String,
    descriptor_number:  u64,
) -> Status {

    trace!(
        dbname = %dbname,
        descriptor_number,
        "set_current_file: start"
    );

    // C++: std::string manifest = DescriptorFileName(dbname, descriptor_number);
    let manifest = descriptor_file_name(dbname, descriptor_number);

    // C++: Slice contents = manifest;
    let mut contents = Slice::from(&manifest);

    // C++: assert(contents.starts_with(dbname + "/"));
    let prefix_string = {
        let mut s = dbname.clone();
        s.push('/');
        s
    };
    let prefix_slice = Slice::from(&prefix_string);
    assert!(
        contents.starts_with(&prefix_slice),
        "set_current_file: manifest '{}' does not start with '{}'",
        manifest,
        prefix_string,
    );

    // C++: contents.remove_prefix(dbname.size() + 1);
    contents.remove_prefix(dbname.len() + 1);

    // C++: std::string tmp = TempFileName(dbname, descriptor_number);
    let tmp = temp_file_name(dbname, descriptor_number);

    // C++: Status s = WriteStringToFileSync(env, contents.ToString() + "\n", tmp);
    let content_string = {
        let mut s = contents.to_string();
        s.push('\n');
        s
    };
    let content_slice = Slice::from(&content_string);

    trace!(
        dbname = %dbname,
        tmp = %tmp,
        "set_current_file: writing CURRENT contents to temporary file"
    );

    let mut status = write_string_to_file_sync(env.clone(), &content_slice, &tmp);

    if status.is_ok() {
        let current_name = current_file_name(dbname);
        trace!(
            tmp = %tmp,
            current = %current_name,
            "set_current_file: renaming temporary file to CURRENT"
        );
        status = {
            let mut env_ref = env.borrow_mut();
            env_ref.rename_file(&tmp, &current_name)
        };
    }

    if !status.is_ok() {
        trace!(
            tmp = %tmp,
            status_str = %status.to_string(),
            "set_current_file: operation failed, deleting temporary file"
        );
        let mut env_ref = env.borrow_mut();
        let _ = env_ref.delete_file(&tmp);
    }

    debug!(
        dbname = %dbname,
        descriptor_number,
        ok = status.is_ok(),
        "set_current_file: completed"
    );
    status
}
