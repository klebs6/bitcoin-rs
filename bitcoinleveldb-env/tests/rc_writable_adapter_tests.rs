// ---------------- [ File: bitcoinleveldb-env/tests/rc_writable_adapter_tests.rs ]
use bitcoinleveldb_env::*;
use bitcoinleveldb_file::*;
use bitcoin_imports::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_log::*;
use bitcoin_imports::*;
use bitcoin_support::*;

#[derive(Default)]
struct MockInnerWritableFile {
    appended_chunks: Vec<String>,
    flush_count:     usize,
    close_count:     usize,
    sync_count:      usize,
    fail_append:     bool,
    fail_flush:      bool,
    fail_close:      bool,
    fail_sync:       bool,
}

impl MockInnerWritableFile {
    fn new() -> Self {
        trace!("MockInnerWritableFile::new");
        Self::default()
    }
}

impl WritableFile for MockInnerWritableFile {}

impl Named for MockInnerWritableFile {
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("[mock-inner-writable-file]".to_string())
    }
}

impl WritableFileAppend for MockInnerWritableFile {
    fn append(&mut self, data: &Slice) -> Status {
        trace!(
            len = *data.size(),
            fail_append = self.fail_append,
            "MockInnerWritableFile::append"
        );

        if self.fail_append {
            let msg = "append failure".to_string();
            let msg_slice = Slice::from(&msg);
            return Status::io_error(&msg_slice, None);
        }

        unsafe {
            let len = *data.size();
            if len > 0 {
                let ptr = *data.data() as *const u8;
                let bytes = std::slice::from_raw_parts(ptr, len);
                let chunk = String::from_utf8_lossy(bytes).into_owned();
                self.appended_chunks.push(chunk);
            } else {
                self.appended_chunks.push(String::new());
            }
        }

        Status::ok()
    }
}

impl WritableFileFlush for MockInnerWritableFile {
    fn flush(&mut self) -> Status {
        trace!(
            fail_flush = self.fail_flush,
            "MockInnerWritableFile::flush"
        );

        self.flush_count += 1;
        if self.fail_flush {
            let msg = "flush failure".to_string();
            let msg_slice = Slice::from(&msg);
            return Status::io_error(&msg_slice, None);
        }
        Status::ok()
    }
}

impl WritableFileClose for MockInnerWritableFile {
    fn close(&mut self) -> Status {
        trace!(
            fail_close = self.fail_close,
            "MockInnerWritableFile::close"
        );

        self.close_count += 1;
        if self.fail_close {
            let msg = "close failure".to_string();
            let msg_slice = Slice::from(&msg);
            return Status::io_error(&msg_slice, None);
        }
        Status::ok()
    }
}

impl WritableFileSync for MockInnerWritableFile {
    fn sync(&mut self) -> Status {
        trace!(
            fail_sync = self.fail_sync,
            "MockInnerWritableFile::sync"
        );

        self.sync_count += 1;
        if self.fail_sync {
            let msg = "sync failure".to_string();
            let msg_slice = Slice::from(&msg);
            return Status::io_error(&msg_slice, None);
        }
        Status::ok()
    }
}

#[traced_test]
fn rc_writable_file_adapter_forwards_successful_operations() {
    trace!("rc_writable_file_adapter_forwards_successful_operations: start");

    let inner = Rc::new(RefCell::new(MockInnerWritableFile::new()));
    let mut adapter = RcWritableFileAdapter { inner: inner.clone() };

    // Verify GetName is stable.
    assert_eq!(
        adapter.name(),
        "[rc-writable-file-adapter]",
        "RcWritableFileAdapter::get_name should return its adapter tag"
    );

    // Prepare a simple payload.
    let payload = "hello-adapter".to_string();
    let slice = Slice::from(&payload);

    // Append.
    let status = adapter.append(&slice);
    assert!(
        status.is_ok(),
        "append should succeed: {}",
        status.to_string()
    );

    // Flush.
    let status = adapter.flush();
    assert!(
        status.is_ok(),
        "flush should succeed: {}",
        status.to_string()
    );

    // Sync.
    let status = adapter.sync();
    assert!(
        status.is_ok(),
        "sync should succeed: {}",
        status.to_string()
    );

    // Close.
    let status = adapter.close();
    assert!(
        status.is_ok(),
        "close should succeed: {}",
        status.to_string()
    );

    // Inspect the inner file state.
    let inner_ref = inner.borrow();
    assert_eq!(
        inner_ref.appended_chunks.len(),
        1,
        "exactly one chunk should have been appended"
    );
    assert_eq!(
        inner_ref.appended_chunks[0],
        payload,
        "payload contents should be forwarded unchanged"
    );
    assert_eq!(inner_ref.flush_count, 1, "one flush call expected");
    assert_eq!(inner_ref.sync_count, 1, "one sync call expected");
    assert_eq!(inner_ref.close_count, 1, "one close call expected");

    info!("rc_writable_file_adapter_forwards_successful_operations: completed");
}

#[traced_test]
fn rc_writable_file_adapter_propagates_inner_errors() {
    trace!("rc_writable_file_adapter_propagates_inner_errors: start");

    let inner = Rc::new(RefCell::new(MockInnerWritableFile {
        fail_append: true,
        fail_flush:  true,
        fail_close:  true,
        fail_sync:   true,
        ..MockInnerWritableFile::default()
    }));

    let mut adapter = RcWritableFileAdapter { inner: inner.clone() };

    let payload = "fail-me".to_string();
    let slice = Slice::from(&payload);

    // append should fail
    let status = adapter.append(&slice);
    assert!(
        status.is_io_error(),
        "append failure should propagate as IO error: {}",
        status.to_string()
    );

    // flush should fail
    let status = adapter.flush();
    assert!(
        status.is_io_error(),
        "flush failure should propagate as IO error: {}",
        status.to_string()
    );

    // sync should fail
    let status = adapter.sync();
    assert!(
        status.is_io_error(),
        "sync failure should propagate as IO error: {}",
        status.to_string()
    );

    // close should fail
    let status = adapter.close();
    assert!(
        status.is_io_error(),
        "close failure should propagate as IO error: {}",
        status.to_string()
    );

    let inner_ref = inner.borrow();
    assert!(
        inner_ref.appended_chunks.is_empty(),
        "on append failure, no data should be recorded"
    );

    info!("rc_writable_file_adapter_propagates_inner_errors: completed");
}
