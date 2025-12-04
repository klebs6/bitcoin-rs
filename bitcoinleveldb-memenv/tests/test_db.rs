// ---------------- [ File: bitcoinleveldb-memenv/tests/test_db.rs ]
use bitcoinleveldb_memenv::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_slice::*;
use bitcoin_imports::*;

#[traced_test]
fn mem_env_test_db() {

    let mut env = make_mem_env();

    let dir = "/dir-db".to_string();
    let status = env.create_dir(&dir);
    assert!(status.is_ok());

    let fname = "/dir-db/db-file".to_string();

    // Create a writable file in the in-memory environment.
    let mut wf_ptr: *mut Box<dyn WritableFile> = core::ptr::null_mut();
    let status = env.new_writable_file(
        &fname,
        &mut wf_ptr as *mut *mut Box<dyn WritableFile>,
    );
    assert!(status.is_ok());

    unsafe {
        if !wf_ptr.is_null() {
            let writable: &mut Box<dyn WritableFile> = &mut *wf_ptr;

            // Write a few key/value-like records to exercise the env + file API.
            let records = [
                Slice::from("aaa=foo".as_bytes()),
                Slice::from("bbb=bar".as_bytes()),
                Slice::from("ccc=baz".as_bytes()),
            ];

            for rec in records.iter() {
                let st = WritableFileAppend::append(writable.as_mut(), rec);
                assert!(st.is_ok());
            }

            let flush_status = WritableFileFlush::flush(writable.as_mut());
            assert!(flush_status.is_ok());

            let sync_status = WritableFileSync::sync(writable.as_mut());
            assert!(sync_status.is_ok());

            let close_status = WritableFileClose::close(writable.as_mut());
            assert!(close_status.is_ok());

            let _outer: Box<Box<dyn WritableFile>> = Box::from_raw(wf_ptr);
        }
    }

    // Verify that data written through the WritableFile can be read back
    // through a RandomAccessFile constructed by the same in-memory env.
    let mut rand_ptr: *mut Box<dyn RandomAccessFile> = core::ptr::null_mut();
    let status = env.new_random_access_file(
        &fname,
        &mut rand_ptr as *mut *mut Box<dyn RandomAccessFile>,
    );
    assert!(status.is_ok());

    unsafe {
        if !rand_ptr.is_null() {
            let rand: &Box<dyn RandomAccessFile> = &*rand_ptr;

            // Read back some prefix of the file and ensure it is non-empty
            // and consistent with what we wrote.
            let mut scratch = vec![0_u8; 64];
            let mut result = Slice::default();

            let st = RandomAccessFileRead::read(
                rand.as_ref(),
                0,
                scratch.len(),
                &mut result as *mut Slice,
                scratch.as_mut_ptr(),
            );
            assert!(st.is_ok());

            let read_len = *result.size();
            assert!(read_len > 0);
            assert!(read_len <= scratch.len());

            // We know the file begins with "aaa=foo".
            let expected_prefix = b"aaa=foo";
            assert!(read_len >= expected_prefix.len());
            assert_eq!(&scratch[..expected_prefix.len()], expected_prefix);

            let _outer: Box<Box<dyn RandomAccessFile>> = Box::from_raw(rand_ptr);
        }
    }

    // Finally, ensure the file can be deleted cleanly from the in-memory env.
    let delete_status = env.delete_file(&fname);
    assert!(delete_status.is_ok());
    assert!(!env.file_exists(&fname));
}
