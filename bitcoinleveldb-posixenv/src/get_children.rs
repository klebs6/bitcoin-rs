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
