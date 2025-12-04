// ---------------- [ File: bitcoinleveldb-memenv/tests/read_write.rs ]
use bitcoinleveldb_memenv::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_file::*;
use bitcoin_imports::*;

#[traced_test]
fn mem_env_test_read_write() {
    let mut env = make_mem_env();

    let dir = "/dir".to_string();
    let status = env.create_dir(&dir);
    assert!(status.is_ok());

    let fname = "/dir/f".to_string();

    // Write "hello " + "world" into /dir/f.
    let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
    let status = env.new_writable_file(
        &fname,
        &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
    );
    assert!(status.is_ok());

    unsafe {
        assert!(!wf_ptr.is_null());
        let writable: &mut Box<dyn WritableFile> = &mut *wf_ptr;

        let slice1 = Slice::from("hello ".as_bytes());
        let st1 = WritableFileAppend::append(writable.as_mut(), &slice1);
        assert!(st1.is_ok());

        let slice2 = Slice::from("world".as_bytes());
        let st2 = WritableFileAppend::append(writable.as_mut(), &slice2);
        assert!(st2.is_ok());

        let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
    }

    // Read sequentially.
    let mut seq_ptr: *mut Box<dyn SequentialFile> = core::ptr::null_mut();
    let status = env.new_sequential_file(
        &fname,
        &mut seq_ptr as *mut *mut Box<dyn SequentialFile>,
    );
    assert!(status.is_ok());

    let mut scratch = vec![0_u8; 100];

    unsafe {
        assert!(!seq_ptr.is_null());
        let seq: &mut Box<dyn SequentialFile> = &mut *seq_ptr;

        let mut result = Slice::default();

        // Read first 5 bytes: "hello".
        let st = SequentialFileRead::read(
            seq.as_mut(),
            5,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(st.is_ok());
        assert_eq!(*result.size(), 5);
        assert_eq!(&scratch[..5], b"hello");

        // Skip one byte (space).
        let st_skip = SequentialFileSkip::skip(seq.as_mut(), 1);
        assert!(st_skip.is_ok());

        // Read "world".
        let st = SequentialFileRead::read(
            seq.as_mut(),
            1000,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(st.is_ok());
        assert_eq!(*result.size(), 5);
        assert_eq!(&scratch[..5], b"world");

        // Try reading past EOF -> size 0.
        let st = SequentialFileRead::read(
            seq.as_mut(),
            1000,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(st.is_ok());
        assert_eq!(*result.size(), 0);

        // Skip past end of file -> OK.
        let st_skip2 = SequentialFileSkip::skip(seq.as_mut(), 100);
        assert!(st_skip2.is_ok());

        // Further read should still yield empty result.
        let st = SequentialFileRead::read(
            seq.as_mut(),
            1000,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(st.is_ok());
        assert_eq!(*result.size(), 0);

        let _outer_seq: Box<Box<dyn SequentialFile>> = Box::from_raw(seq_ptr);
    }

    // Random reads.
    let mut rand_ptr: *mut Box<dyn RandomAccessFile> = core::ptr::null_mut();
    let status = env.new_random_access_file(
        &fname,
        &mut rand_ptr as *mut *mut Box<dyn RandomAccessFile>,
    );
    assert!(status.is_ok());

    unsafe {
        assert!(!rand_ptr.is_null());
        let rand: &Box<dyn RandomAccessFile> = &*rand_ptr;

        let mut result = Slice::default();

        // Read "world" from offset 6.
        let st = RandomAccessFileRead::read(
            rand.as_ref(),
            6,
            5,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(st.is_ok());
        assert_eq!(*result.size(), 5);
        assert_eq!(&scratch[..5], b"world");

        // Read "hello" from offset 0.
        let st = RandomAccessFileRead::read(
            rand.as_ref(),
            0,
            5,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(st.is_ok());
        assert_eq!(*result.size(), 5);
        assert_eq!(&scratch[..5], b"hello");

        // Read "d" from offset 10.
        let st = RandomAccessFileRead::read(
            rand.as_ref(),
            10,
            100,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(st.is_ok());
        assert_eq!(*result.size(), 1);
        assert_eq!(scratch[0], b'd');

        // Too high offset should fail.
        let st = RandomAccessFileRead::read(
            rand.as_ref(),
            1000,
            5,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(!st.is_ok());

        let _outer_rand: Box<Box<dyn RandomAccessFile>> = Box::from_raw(rand_ptr);
    }
}
