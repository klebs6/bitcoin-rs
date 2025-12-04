// ---------------- [ File: bitcoinleveldb-memenv/tests/large_write.rs ]
use bitcoinleveldb_memenv::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_file::*;
use bitcoin_imports::*;

#[traced_test]
fn mem_env_test_large_write() {
    let mut env = make_mem_env();

    const K_WRITE_SIZE: usize = 300 * 1024;

    let mut scratch = vec![0_u8; K_WRITE_SIZE * 2];

    // Build write_data pattern: bytes 0..K_WRITE_SIZE (mod 256).
    let mut write_data = Vec::with_capacity(K_WRITE_SIZE);
    for i in 0..K_WRITE_SIZE {
        write_data.push((i % 256) as u8);
    }

    // Write "foo" + write_data.
    let fname = "/dir/f".to_string();
    let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();

    let status = env.new_writable_file(
        &fname,
        &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
    );
    assert!(status.is_ok());

    unsafe {
        assert!(!wf_ptr.is_null());
        let writable: &mut Box<dyn WritableFile> = &mut *wf_ptr;

        let slice_foo = Slice::from("foo".as_bytes());
        let st1 = WritableFileAppend::append(writable.as_mut(), &slice_foo);
        assert!(st1.is_ok());

        let slice_data = Slice::from(write_data.as_slice());
        let st2 = WritableFileAppend::append(writable.as_mut(), &slice_data);
        assert!(st2.is_ok());

        let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
    }

    // Sequentially read back.
    let mut seq_ptr: *mut Box<dyn SequentialFile> = core::ptr::null_mut();
    let status = env.new_sequential_file(
        &fname,
        &mut seq_ptr as *mut *mut Box<dyn SequentialFile>,
    );
    assert!(status.is_ok());

    unsafe {
        assert!(!seq_ptr.is_null());
        let seq: &mut Box<dyn SequentialFile> = &mut *seq_ptr;

        let mut result = Slice::default();

        // Read first 3 bytes -> "foo".
        let st = SequentialFileRead::read(
            seq.as_mut(),
            3,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(st.is_ok());
        assert_eq!(*result.size(), 3);
        assert_eq!(&scratch[..3], b"foo");

        // Now read the remaining K_WRITE_SIZE bytes into read_data.
        let mut read: usize = 0;
        let mut read_data = Vec::with_capacity(K_WRITE_SIZE);

        while read < K_WRITE_SIZE {
            let mut chunk = Slice::default();
            let to_read = K_WRITE_SIZE - read;

            let st = SequentialFileRead::read(
                seq.as_mut(),
                to_read,
                &mut chunk as *mut Slice,
                scratch.as_mut_ptr(),
            );
            assert!(st.is_ok());

            let chunk_size = *chunk.size();
            if chunk_size == 0 {
                break;
            }

            read_data.extend_from_slice(&scratch[..chunk_size]);
            read += chunk_size;
        }

        assert_eq!(read, K_WRITE_SIZE);
        assert_eq!(read_data, write_data);

        let _outer: Box<Box<dyn SequentialFile>> = Box::from_raw(seq_ptr);
    }
}
