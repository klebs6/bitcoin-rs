// ---------------- [ File: bitcoin-block/src/directory_commit.rs ]
crate::ix!();

/**
  | Sync directory contents. This is required
  | on some environments to ensure that
  | newly created files are committed to
  | disk.
  |
  */
#[cfg(not(WIN32))]
pub fn directory_commit(dirname: &Path)  {
    
    unsafe {

        let file: *mut libc::FILE = {

            let dirname = dirname.as_os_str().to_str().unwrap().as_ptr() as *const i8;

            libc::fopen(
                dirname,
                "r".as_ptr() as *const i8
            )
        };

        if file != null_mut() {
            libc::fsync(libc::fileno(file));
            libc::fclose(file);
        }
    }
}
