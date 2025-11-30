// ---------------- [ File: bitcoinleveldb-posixenv/src/file_exists.rs ]
crate::ix!();

impl FileExists for PosixEnv {

    fn file_exists(&mut self, filename: &String) -> bool {
        let exists = std::fs::metadata(filename).is_ok();
        trace!(
            file   = %filename,
            exists,
            "PosixEnv::file_exists: checked file existence"
        );
        exists
    }
}
