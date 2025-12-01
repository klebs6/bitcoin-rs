// ---------------- [ File: bitcoinleveldb-posix/src/get_argv0.rs ]
crate::ix!();

/**
  | Global set by main() and read in
  | TestCloseOnExec.
  |
  | The argv[0] value is stored in a std::vector
  | instead of a std::string because std::string
  | does not return a mutable pointer to its buffer
  | until C++17.
  |
  | The vector stores the string pointed to by
  | argv[0], plus the trailing null.
  */
#[cfg(HAVE_O_CLOEXEC)]
pub fn get_argv_zero() -> *mut Vec<u8> {
    use std::sync::OnceLock;

    trace!("get_argv_zero: initializing cached argv[0] buffer");

    static PROGRAM_NAME: OnceLock<Vec<u8>> = OnceLock::new();

    let vec_ref: &Vec<u8> = PROGRAM_NAME.get_or_init(|| {
        let raw_bytes: Vec<u8> = match std::env::current_exe() {
            Ok(path) => {
                let as_string = path.to_string_lossy().into_owned();
                as_string.into_bytes()
            }
            Err(err) => {
                warn!(
                    "get_argv_zero: failed to determine current_exe: {:?}, \
                     falling back to default program name",
                    err
                );
                b"env_posix_tests".to_vec()
            }
        };

        let mut buffer = raw_bytes;
        if !buffer.ends_with(&[0]) {
            buffer.push(0);
        }

        debug!(
            "get_argv_zero: cached program name {:?}",
            String::from_utf8_lossy(&buffer)
        );

        buffer
    });

    vec_ref as *const Vec<u8> as *mut Vec<u8>
}
