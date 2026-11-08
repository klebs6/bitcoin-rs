//! Bounded, in-memory contracts for the public shared WritableFile conversion.
//! These tests perform no filesystem operations, use no threads, and construct
//! every Slice from live Rust byte storage.

use bitcoin_imports::Named;
use bitcoinleveldb_file::{
    WritableFile, WritableFileAppend, WritableFileClose, WritableFileFlush,
    WritableFileSync,
};
use bitcoinleveldb_slice::Slice;
use bitcoinleveldb_status::{Status, StatusCode};
use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
enum Event {
    Append(Vec<u8>),
    Close,
    Flush,
    Sync,
    Drop,
}

struct RecordingFile {
    name: String,
    bytes: Vec<u8>,
    events: Rc<RefCell<Vec<Event>>>,
    drops: Rc<Cell<usize>>,
    statuses: [Status; 4],
    panic_next_append: bool,
}

impl WritableFile for RecordingFile {}

impl Named for RecordingFile {
    fn name(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}

impl WritableFileAppend for RecordingFile {
    fn append(&mut self, data: &Slice) -> Status {
        if self.panic_next_append {
            self.panic_next_append = false;
            panic!("ordinary test-double append panic");
        }
        let bytes = data.as_bytes();
        self.events.borrow_mut().push(Event::Append(bytes.to_vec()));
        self.bytes.extend_from_slice(bytes);
        self.statuses[0].clone()
    }
}

impl WritableFileClose for RecordingFile {
    fn close(&mut self) -> Status {
        self.events.borrow_mut().push(Event::Close);
        self.statuses[1].clone()
    }
}

impl WritableFileFlush for RecordingFile {
    fn flush(&mut self) -> Status {
        self.events.borrow_mut().push(Event::Flush);
        self.statuses[2].clone()
    }
}

impl WritableFileSync for RecordingFile {
    fn sync(&mut self) -> Status {
        self.events.borrow_mut().push(Event::Sync);
        self.statuses[3].clone()
    }
}

impl Drop for RecordingFile {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
        self.events.borrow_mut().push(Event::Drop);
    }
}

type Fixture = (
    Rc<RefCell<RecordingFile>>,
    Rc<RefCell<Vec<Event>>>,
    Rc<Cell<usize>>,
);

fn fixture() -> Fixture {
    let events = Rc::new(RefCell::new(Vec::new()));
    let drops = Rc::new(Cell::new(0));
    let file = Rc::new(RefCell::new(RecordingFile {
        name: "recording-file".to_owned(),
        bytes: Vec::new(),
        events: events.clone(),
        drops: drops.clone(),
        statuses: std::array::from_fn(|_| Status::ok()),
        panic_next_append: false,
    }));
    (file, events, drops)
}

fn boxed_alias(file: &Rc<RefCell<RecordingFile>>) -> Box<dyn WritableFile> {
    let shared: Rc<RefCell<dyn WritableFile>> = file.clone();
    Box::<dyn WritableFile>::from(shared)
}

fn passed(name: &str) {
    eprintln!("SHARED_WRITABLE_FILE_CONTRACT_OK case={name}");
}

#[test]
fn shared_writable_file_forwards_calls_bytes_and_success_statuses() {
    let (file, events, _drops) = fixture();
    let mut boxed = boxed_alias(&file);
    let bytes = [0u8, 0xff, b'a', 0, b'z'];

    assert!(boxed.append(&Slice::from(bytes.as_slice())).is_ok());
    assert!(boxed.flush().is_ok());
    assert!(boxed.append(&Slice::from(&b""[..])).is_ok());
    assert!(boxed.sync().is_ok());
    assert!(boxed.close().is_ok());

    assert_eq!(file.borrow().bytes, bytes);
    assert_eq!(
        *events.borrow(),
        vec![
            Event::Append(bytes.to_vec()),
            Event::Flush,
            Event::Append(Vec::new()),
            Event::Sync,
            Event::Close,
        ]
    );
    passed("shared_writable_file_forwards_calls_bytes_and_success_statuses");
}

#[test]
fn shared_writable_file_preserves_each_error_code_and_message() {
    let (file, events, _drops) = fixture();
    let primary = Slice::from(&b"primary message"[..]);
    let detail = Slice::from(&b"secondary detail"[..]);
    let expected = [
        Status::io_error(&primary, Some(&detail)),
        Status::invalid_argument(&primary, Some(&detail)),
        Status::not_supported(&primary, Some(&detail)),
        Status::corruption(&primary, Some(&detail)),
    ];
    file.borrow_mut().statuses = expected.clone();
    let mut boxed = boxed_alias(&file);
    let actual = [
        boxed.append(&Slice::from(&b"payload"[..])),
        boxed.close(),
        boxed.flush(),
        boxed.sync(),
    ];

    for (observed, wanted) in actual.iter().zip(expected.iter()) {
        assert!(!observed.is_ok());
        assert_eq!(observed.code(), wanted.code());
        assert_eq!(observed.to_string(), wanted.to_string());
        assert!(observed.to_string().contains("primary message"));
        assert!(observed.to_string().contains("secondary detail"));
    }
    assert_eq!(actual[0].code(), StatusCode::IOError);
    assert_eq!(actual[1].code(), StatusCode::InvalidArgument);
    assert_eq!(actual[2].code(), StatusCode::NotSupported);
    assert_eq!(actual[3].code(), StatusCode::Corruption);
    assert_eq!(
        *events.borrow(),
        vec![Event::Append(b"payload".to_vec()), Event::Close, Event::Flush, Event::Sync]
    );
    passed("shared_writable_file_preserves_each_error_code_and_message");
}

#[test]
fn shared_writable_file_observes_external_changes_and_shared_identity() {
    let (file, events, _drops) = fixture();
    let mut first = boxed_alias(&file);
    let mut second = boxed_alias(&file);

    assert!(first.append(&Slice::from(&b"first"[..])).is_ok());
    {
        let mut original = file.borrow_mut();
        original.bytes.extend_from_slice(b"/external/");
        original.name = "renamed-through-original".to_owned();
    }
    assert_eq!(first.name().as_ref(), "renamed-through-original");
    assert_eq!(second.name().as_ref(), "renamed-through-original");
    assert!(second.append(&Slice::from(&b"second"[..])).is_ok());

    assert_eq!(file.borrow().bytes, b"first/external/second");
    assert_eq!(
        *events.borrow(),
        vec![Event::Append(b"first".to_vec()), Event::Append(b"second".to_vec())]
    );
    assert_eq!(Rc::strong_count(&file), 3);
    passed("shared_writable_file_observes_external_changes_and_shared_identity");
}

#[test]
fn shared_writable_file_name_is_owned_snapshot_without_retained_borrow() {
    let (file, _events, _drops) = fixture();
    let boxed = boxed_alias(&file);
    let name = boxed.name();

    assert!(matches!(&name, Cow::Owned(_)));
    assert_eq!(name.as_ref(), "recording-file");
    {
        let mut original = file.try_borrow_mut().expect("name() retained its RefCell guard");
        original.name = "new-name".to_owned();
    }
    assert_eq!(name.as_ref(), "recording-file");
    assert_eq!(boxed.name().as_ref(), "new-name");
    passed("shared_writable_file_name_is_owned_snapshot_without_retained_borrow");
}

#[test]
fn shared_writable_file_conversion_and_drop_do_not_perform_io() {
    let (file, events, drops) = fixture();
    file.borrow_mut().bytes = b"existing bytes".to_vec();
    let shared: Rc<RefCell<dyn WritableFile>> = file.clone();
    assert_eq!(Rc::strong_count(&file), 2);

    let boxed = Box::<dyn WritableFile>::from(shared);

    assert_eq!(Rc::strong_count(&file), 2, "conversion should consume its one Rc");
    assert_eq!(file.borrow().bytes, b"existing bytes");
    assert!(events.borrow().is_empty());
    drop(boxed);
    assert_eq!(Rc::strong_count(&file), 1);
    assert_eq!(drops.get(), 0);
    assert!(events.borrow().is_empty(), "dropping an alias should not close or sync");
    drop(file);
    assert_eq!(drops.get(), 1);
    assert_eq!(*events.borrow(), vec![Event::Drop]);
    passed("shared_writable_file_conversion_and_drop_do_not_perform_io");
}

#[test]
fn shared_writable_file_keeps_last_owner_alive_until_box_drop() {
    let (file, events, drops) = fixture();
    let weak = Rc::downgrade(&file);
    let mut boxed = boxed_alias(&file);
    drop(file);

    assert_eq!(weak.strong_count(), 1);
    assert_eq!(drops.get(), 0);
    assert_eq!(boxed.name().as_ref(), "recording-file");
    assert!(boxed.flush().is_ok());
    drop(boxed);

    assert!(weak.upgrade().is_none());
    assert_eq!(drops.get(), 1);
    assert_eq!(*events.borrow(), vec![Event::Flush, Event::Drop]);
    passed("shared_writable_file_keeps_last_owner_alive_until_box_drop");
}

#[test]
fn shared_writable_file_preserves_refcell_borrow_conflicts() {
    let (file, events, _drops) = fixture();
    let mut boxed = boxed_alias(&file);
    let held_read = file.borrow();

    let conflict = catch_unwind(AssertUnwindSafe(|| {
        boxed.append(&Slice::from(&b"blocked"[..]));
    }));
    assert!(conflict.is_err());
    assert!(events.borrow().is_empty());
    drop(held_read);

    assert!(boxed.append(&Slice::from(&b"accepted"[..])).is_ok());
    assert_eq!(file.borrow().bytes, b"accepted");
    assert_eq!(*events.borrow(), vec![Event::Append(b"accepted".to_vec())]);
    passed("shared_writable_file_preserves_refcell_borrow_conflicts");
}

#[test]
fn shared_writable_file_releases_borrow_when_inner_operation_unwinds() {
    let (file, events, _drops) = fixture();
    file.borrow_mut().panic_next_append = true;
    let mut boxed = boxed_alias(&file);

    let operation = catch_unwind(AssertUnwindSafe(|| {
        boxed.append(&Slice::from(&b"not-appended"[..]));
    }));
    assert!(operation.is_err());
    assert!(events.borrow().is_empty());
    {
        let original = file.try_borrow_mut().expect("unwinding retained an adapter borrow");
        assert!(!original.panic_next_append);
        assert!(original.bytes.is_empty());
    }
    assert!(boxed.append(&Slice::from(&b"after-unwind"[..])).is_ok());
    assert_eq!(file.borrow().bytes, b"after-unwind");
    assert_eq!(*events.borrow(), vec![Event::Append(b"after-unwind".to_vec())]);
    passed("shared_writable_file_releases_borrow_when_inner_operation_unwinds");
}
