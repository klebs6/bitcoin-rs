// ---------------- [ File: bitcoinleveldb-posixenv/src/get_children.rs ]
crate::ix!();

impl GetChildren for PosixEnv {

    fn get_children(
        &mut self,
        directory_path: &String,
        result:         *mut Vec<String>,
    ) -> crate::Status {
        trace!(
            dir = %directory_path,
            "PosixEnv::get_children: listing directory entries"
        );

        assert!(
            !result.is_null(),
            "PosixEnv::get_children: result pointer must not be null"
        );

        let out: &mut Vec<String> = unsafe { &mut *result };
        out.clear();

        let entries = match std::fs::read_dir(directory_path) {
            Ok(e) => e,
            Err(err) => {
                let errno = err.raw_os_error().unwrap_or(0);
                warn!(
                    dir   = %directory_path,
                    errno,
                    "PosixEnv::get_children: read_dir failed"
                );
                return posix_error(directory_path, errno);
            }
        };

        for entry_res in entries {
            match entry_res {
                Ok(entry) => {
                    let name_os = entry.file_name();
                    match name_os.into_string() {
                        Ok(name) => out.push(name),
                        Err(os) => {
                            warn!(
                                name = ?os,
                                "PosixEnv::get_children: non-UTF8 filename encountered; skipping entry"
                            );
                        }
                    }
                }
                Err(err) => {
                    let errno = err.raw_os_error().unwrap_or(0);
                    warn!(
                        dir   = %directory_path,
                        errno,
                        "PosixEnv::get_children: failed to iterate directory entry"
                    );
                    return posix_error(directory_path, errno);
                }
            }
        }

        debug!(
            dir   = %directory_path,
            count = out.len(),
            "PosixEnv::get_children: completed directory listing"
        );

        crate::Status::ok()
    }
}

#[cfg(test)]
mod posix_env_get_children_tests {
    use super::*;

    fn unique_directory_for_children() -> String {
        let base = std::env::temp_dir();
        let name = format!(
            "bitcoinleveldb-posixenv-get-children-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
        );
        base.join(name).to_string_lossy().to_string()
    }

    #[traced_test]
    fn get_children_lists_directory_entries() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let dirname = unique_directory_for_children();

        std::fs::create_dir(&dirname)
            .expect("precondition: create_dir should succeed");

        let file_a = format!("{}/child_a.txt", dirname);
        let file_b = format!("{}/child_b.txt", dirname);

        std::fs::write(&file_a, b"a").expect("precondition: write child_a");
        std::fs::write(&file_b, b"b").expect("precondition: write child_b");

        let mut entries = Vec::<String>::new();

        let status = env.get_children(&dirname, &mut entries as *mut Vec<String>);

        assert!(
            status.is_ok(),
            "get_children should succeed for a valid directory: {}",
            status.to_string()
        );

        entries.sort();

        assert!(
            entries.contains(&"child_a.txt".to_owned()),
            "get_children results should include child_a.txt; got: {:?}",
            entries
        );
        assert!(
            entries.contains(&"child_b.txt".to_owned()),
            "get_children results should include child_b.txt; got: {:?}",
            entries
        );

        let _ = std::fs::remove_file(&file_a);
        let _ = std::fs::remove_file(&file_b);
        let _ = std::fs::remove_dir(&dirname);
    }

    #[traced_test]
    fn get_children_returns_error_for_missing_directory() {
        let env: &'static mut PosixEnv = Box::leak(Box::new(PosixEnv::default()));
        let dirname = unique_directory_for_children();
        let mut entries = Vec::<String>::new();

        let status = env.get_children(&dirname, &mut entries as *mut Vec<String>);

        assert!(
            !status.is_ok(),
            "get_children should fail for non-existent directory"
        );
    }
}
