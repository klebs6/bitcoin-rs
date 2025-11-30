// ---------------- [ File: bitcoinleveldb-env/tests/env_windows.rs ]
use bitcoinleveldb_env::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/env_windows_test.cc]

const MMAP_LIMIT: i32 = 4;

#[cfg(windows)]
struct EnvWindowsTest {
    env: Rc<RefCell<dyn Env>>,
}

#[cfg(windows)]
impl Default for EnvWindowsTest {
    fn default() -> Self {
        trace!("EnvWindowsTest::default: acquiring default Env");
        let env = posix_default_env();
        Self { env }
    }
}

#[cfg(windows)]
impl EnvWindowsTest {
    pub fn set_file_limits(mmap_limit: i32) {
        trace!(
            mmap_limit,
            "EnvWindowsTest::set_file_limits -> EnvWindowsTestHelper::set_read_only_mmap_limit"
        );
        EnvWindowsTestHelper::set_read_only_mmap_limit(mmap_limit);
    }
}

#[cfg(windows)]
#[traced_test]
fn env_windows_test_open_on_read() {
    use std::io::Write;

    trace!("env_windows_test_open_on_read: start");

    let test_env = EnvWindowsTest::default();
    let env_rc = test_env.env.clone();

    // Write test data to a single file that will be opened |n| times.
    let mut test_dir = String::new();
    {
        let mut env = env_rc.borrow_mut();
        let status = env.get_test_directory(&mut test_dir);
        assert!(
            status.is_ok(),
            "GetTestDirectory failed: {}",
            status.to_string()
        );
    }

    let test_file = format!("{}/open_on_read.txt", test_dir);
    debug!(file = %test_file, "env_windows_test_open_on_read: using file");

    const K_FILE_DATA: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

    {
        let mut f = std::fs::File::create(&test_file)
            .expect("env_windows_test_open_on_read: failed to create test file");
        f.write_all(K_FILE_DATA)
            .expect("env_windows_test_open_on_read: failed writing test data");
    }

    // Open test file some number above the mmap limit.
    let num_files = MMAP_LIMIT + 5;
    let mut files: Vec<*mut Box<dyn RandomAccessFile>> = vec![std::ptr::null_mut(); num_files as usize];

    for i in 0..num_files {
        let mut file_ptr: *mut Box<dyn RandomAccessFile> = std::ptr::null_mut();
        let status = {
            let mut env = env_rc.borrow_mut();
            env.new_random_access_file(&test_file, &mut file_ptr)
        };
        assert!(
            status.is_ok(),
            "NewRandomAccessFile failed (i={}): {}",
            i,
            status.to_string()
        );
        files[i as usize] = file_ptr;
    }

    // Verify each handle can read its corresponding byte.
    for i in 0..num_files {
        let file_ptr = files[i as usize];
        let mut holder: Box<Box<dyn RandomAccessFile>> = unsafe {
            assert!(!file_ptr.is_null(), "NewRandomAccessFile returned null");
            Box::from_raw(file_ptr)
        };
        let file: &Box<dyn RandomAccessFile> = holder.as_ref();

        let mut read_result = Slice::default();
        let mut scratch: u8 = 0;

        let status = file.read(
            i as u64,
            1,
            &mut read_result,
            &mut scratch as *mut u8,
        );
        assert!(
            status.is_ok(),
            "RandomAccessFile::Read failed (i={}): {}",
            i,
            status.to_string()
        );

        assert_eq!(
            *read_result.size(),
            1,
            "Expected single-byte read (i={})",
            i
        );
        let ch = read_result[0] as char;
        let expected = K_FILE_DATA[i as usize] as char;
        assert_eq!(
            ch, expected,
            "Byte mismatch at position {} (expected {:?}, got {:?})",
            i, expected, ch
        );
        // holder drops here (delete file).
    }

    {
        let mut env = env_rc.borrow_mut();
        let status = env.delete_file(&test_file);
        assert!(
            status.is_ok(),
            "DeleteFile failed: {}",
            status.to_string()
        );
    }

    info!("env_windows_test_open_on_read: completed");
}
