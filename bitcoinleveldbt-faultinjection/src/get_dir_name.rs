// ---------------- [ File: bitcoinleveldbt-faultinjection/src/get_dir_name.rs ]
crate::ix!();

/**
  | Assume a filename, and not a directory
  | name like "/foo/bar/"
  |
  */
pub fn get_dir_name(filename: &String) -> String {
    trace!(
        target: "bitcoinleveldbt_faultinjection::fault_injection_test",
        event = "get_dir_name_entry",
        filename_len = filename.len()
    );

    let out = match filename.rfind(|c| c == '/' || c == '\\') {
        Some(found) => filename[..found].to_string(),
        None => String::new(),
    };

    trace!(
        target: "bitcoinleveldbt_faultinjection::fault_injection_test",
        event = "get_dir_name_exit",
        dir_len = out.len()
    );

    out
}
