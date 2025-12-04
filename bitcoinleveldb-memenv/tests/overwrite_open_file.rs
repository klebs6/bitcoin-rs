// ---------------- [ File: bitcoinleveldb-memenv/tests/overwrite_open_file.rs ]
use bitcoinleveldb_memenv::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_file::*;
use bitcoin_imports::*;

#[traced_test]
fn mem_env_test_overwrite_open_file() {
    let mut env = make_mem_env();

    let write1 = b"Write #1 data";
    let write2 = b"Write #2 data";
    let file_len = write1.len();
    assert_eq!(file_len, write2.len());

    let fname = "/memenv/overwrite-open-file.dat".to_string();

    // First write: "Write #1 data"
    let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
    let status = env.new_writable_file(
        &fname,
        &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
    );
    assert!(status.is_ok());

    unsafe {
        assert!(!wf_ptr.is_null());
        let writable: &mut Box<dyn WritableFile> = &mut *wf_ptr;

        let slice1 = Slice::from(write1.as_ref());
        let st1 = WritableFileAppend::append(writable.as_mut(), &slice1);
        assert!(st1.is_ok());

        let close_status = WritableFileClose::close(writable.as_mut());
        assert!(close_status.is_ok());

        let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
    }

    // Open RandomAccessFile before overwriting.
    let mut rand_ptr: *mut Box<dyn RandomAccessFile> = core::ptr::null_mut();
    let status = env.new_random_access_file(
        &fname,
        &mut rand_ptr as *mut *mut Box<dyn RandomAccessFile>,
    );
    assert!(status.is_ok());

    // Overwrite the file with "Write #2 data".
    let mut wf2_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
    let status = env.new_writable_file(
        &fname,
        &mut wf2_ptr as *mut *mut Box<dyn WritableFile>,
    );
    assert!(status.is_ok());

    unsafe {
        assert!(!wf2_ptr.is_null());
        let writable: &mut Box<dyn WritableFile> = &mut *wf2_ptr;

        let slice2 = Slice::from(write2.as_ref());
        let st2 = WritableFileAppend::append(writable.as_mut(), &slice2);
        assert!(st2.is_ok());

        let close_status = WritableFileClose::close(writable.as_mut());
        assert!(close_status.is_ok());

        let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf2_ptr);
    }

    // Verify that the *old* RandomAccessFile sees the new data.
    unsafe {
        assert!(!rand_ptr.is_null());
        let rand: &Box<dyn RandomAccessFile> = &*rand_ptr;

        let mut scratch = vec![0_u8; file_len];
        let mut result = Slice::default();

        let status = RandomAccessFileRead::read(
            rand.as_ref(),
            0,
            file_len,
            &mut result as *mut Slice,
            scratch.as_mut_ptr(),
        );
        assert!(status.is_ok());
        assert_eq!(*result.size(), file_len);
        assert_eq!(&scratch[..file_len], write2);

        let _outer: Box<Box<dyn RandomAccessFile>> = Box::from_raw(rand_ptr);
    }
}
