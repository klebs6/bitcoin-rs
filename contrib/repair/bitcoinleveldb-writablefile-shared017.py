#!/usr/bin/env python3
"""Continue the reviewed repair017 controller stop with corrected diagnostic classification. Rust payloads and master are preserved."""
import argparse
import datetime as dt
import fcntl
import hashlib
import json
import os
from pathlib import Path
import resource
import shlex
import signal
import stat
import sys
import types
import uuid
MIB017=1024*1024
CONTROLLER_REVISION=2
R1_RUN='20261108T044236Z-6afe5698'
R1_CONTROLLER_SHA='d06c6b1a70e268b3e800f0ad4bbe893908b0df73ece1534dc8746c827ddcac68'
R1_REPORT_SHA='ad8bbf2d71897f81fc6c1abc22b744a6e7db3c008de2bbdd3c98edd521394351'
R1_REPORT_BYTES=151342
R1_STATE_SHA='0b39e595734ee1ead8a7625382f37c5201b8630c2192b73512a17ea3e4f807e9'
R1_STATE_BYTES=1149
WORKFLOW='leveldb-writablefile-shared017'
BRANCH='repair/leveldb-writablefile-shared017'
SUFFIX='-leveldb-writablefile-shared017'
BASE='a788779b925d8813f8601787f432837726ba97d1'
PRIOR015_RUN='20261108T035429Z-804585ba'
PRIOR015_CONTROLLER_SHA='d2ab9c4583284d91eec10db22914e5779e0400b1ba0324df95088a9f59bbfe1a'
PRIOR016_RUN='20261108T041654Z-097ad300'
PRIOR016_CONTROLLER_SHA='f65690030c2773da0ca3d77e244cfce844790f0c031a46aba64d463289627873'
PRIOR016_REPORT_SHA='328fd36ce5f835fe62d63b7b38b0a8aef0ceee1f0493898ecbf68a17a7808c05'
PRIOR016_REPORT_BYTES=119510
EXPECTED_TESTS=19
SOURCE_PATH='bitcoinleveldb-file/src/writable_file.rs'
SHARED_TEST_PATH='bitcoinleveldb-file/tests/writable_file_shared.rs'
FIND_PATH='bitcoinleveldb-file/src/find_file.rs'
OVERLAP_TEST_PATH='bitcoinleveldb-versionsetutil/tests/find_file_and_overlap.rs'
REPLAY_PATH='contrib/repair/bitcoinleveldb-writablefile-shared017.py'
DOCUMENT_PATH='doc/testing/leveldb-writablefile-shared017.md'
RECEIPT_PATH='doc/repair/bitcoinleveldb-writablefile-shared017.json'
INITIAL_PATHS=['bitcoinleveldb-file/src/writable_file.rs', 'bitcoinleveldb-file/tests/writable_file_shared.rs', 'bitcoinleveldb-file/src/find_file.rs', 'bitcoinleveldb-versionsetutil/tests/find_file_and_overlap.rs', 'contrib/repair/bitcoinleveldb-writablefile-shared017.py', 'doc/testing/leveldb-writablefile-shared017.md']
RUST_PATHS=['bitcoinleveldb-file/src/writable_file.rs', 'bitcoinleveldb-file/tests/writable_file_shared.rs', 'bitcoinleveldb-file/src/find_file.rs', 'bitcoinleveldb-versionsetutil/tests/find_file_and_overlap.rs']
BEFORE_BINDINGS={'bitcoinleveldb-file/src/writable_file.rs': '7ef8df19c58a6b04e12f70cb36747b9dcce66d3642997af6d1744b7398c83410', 'bitcoinleveldb-file/src/find_file.rs': '42db6099ab6cecdeb90dee3ff949814d7158d4a96d5d5da3c5b88199fa42423f', 'bitcoinleveldb-versionsetutil/src/lib.rs': 'be8eba310fcd85ddef4a13f19e29b8e52b13df850530b75a9fb4f5ec578236b7', 'bitcoinleveldb-versionsetutil/src/imports.rs': '9ec4b73a59a7450c8134098851d49a7dc16c46045d5e6175c988b336ca4cba62', 'bitcoinleveldb-versionsetutil/src/some_file_overlaps_range.rs': '827cf175997c9ff2060c764e7a7317bd9a35f7e5feaacf2dbf22872a313af5b9', 'bitcoinleveldb-versionsetutil/Cargo.toml': '2db7a6b85ac5b4f22df3c9f994bca64834a527b28597a04fb902ab76d844d4eb', 'shell.nix': '8f316c4f007426302e26962e13def8c7d5bc0efa85d5e1549c55e85ba0d7c651'}
SOURCE_BYTES=b"// ---------------- [ File: bitcoinleveldb-file/src/writable_file.rs ]\ncrate::ix!();\n\n/**\n  | A file abstraction for sequential writing.  The\n  | implementation must provide buffering since\n  | callers may append small fragments at a time to\n  | the file.\n  */\npub trait WritableFile: \nWritableFileAppend \n+ WritableFileClose \n+ WritableFileFlush \n+ WritableFileSync \n\n/*\n   | Get a name for the file, only for error\n   | reporting\n   |\n   */\n+ Named {}\n\npub trait WritableFileAppend {\n    fn append(&mut self, data: &Slice) -> crate::Status;\n}\n\npub trait WritableFileClose {\n    fn close(&mut self) -> crate::Status;\n}\n\npub trait WritableFileFlush {\n    fn flush(&mut self) -> crate::Status;\n}\n\npub trait WritableFileSync {\n    fn sync(&mut self) -> crate::Status;\n}\n\nimpl From<Rc<RefCell<dyn WritableFile>>> for Box<dyn WritableFile> {\n\n    /// Transfer this shared handle into a boxed forwarding adapter.\n    ///\n    /// The adapter keeps the existing file alive without cloning the Rc or\n    /// changing its contents. Operations use the same RefCell borrow rules as\n    /// direct access to the shared file. Dropping the adapter releases its Rc;\n    /// closing or syncing the file remains an explicit caller operation.\n    fn from(dest: Rc<RefCell<dyn WritableFile>>) -> Self {\n        Box::new(SharedWritableFileAdapter { inner: dest })\n    }\n}\n\nstruct SharedWritableFileAdapter {\n    inner: Rc<RefCell<dyn WritableFile>>,\n}\n\nimpl WritableFile for SharedWritableFileAdapter {}\n\nimpl WritableFileAppend for SharedWritableFileAdapter {\n    fn append(&mut self, data: &Slice) -> crate::Status {\n        self.inner.borrow_mut().append(data)\n    }\n}\n\nimpl WritableFileClose for SharedWritableFileAdapter {\n    fn close(&mut self) -> crate::Status {\n        self.inner.borrow_mut().close()\n    }\n}\n\nimpl WritableFileFlush for SharedWritableFileAdapter {\n    fn flush(&mut self) -> crate::Status {\n        self.inner.borrow_mut().flush()\n    }\n}\n\nimpl WritableFileSync for SharedWritableFileAdapter {\n    fn sync(&mut self) -> crate::Status {\n        self.inner.borrow_mut().sync()\n    }\n}\n\nimpl Named for SharedWritableFileAdapter {\n    fn name(&self) -> Cow<'_,str> {\n        // An inner borrowed name cannot outlive the temporary RefCell guard.\n        Cow::Owned(self.inner.borrow().name().into_owned())\n    }\n}\n\n//-----------------------------------[impl-for-box]\nimpl WritableFile for Box<dyn WritableFile> {}\n\nimpl WritableFileAppend for Box<dyn WritableFile> {\n\n    fn append(&mut self, data: &Slice) -> crate::Status {\n        (**self).append(data)\n    }\n}\n\nimpl WritableFileClose for Box<dyn WritableFile> {\n\n    fn close(&mut self) -> crate::Status {\n        (**self).close()\n    }\n}\n\nimpl WritableFileFlush for Box<dyn WritableFile> {\n\n    fn flush(&mut self) -> crate::Status {\n        (**self).flush()\n    }\n}\n\nimpl WritableFileSync for Box<dyn WritableFile> {\n\n    fn sync(&mut self) -> crate::Status {\n        (**self).sync()\n    }\n}\n\nimpl Named for Box<dyn WritableFile> {\n    fn name(&self) -> Cow<'_,str> {\n        (**self).name()\n    }\n}\n"
SHARED_TEST_BYTES=b'//! Bounded, in-memory contracts for the public shared WritableFile conversion.\n//! These tests perform no filesystem operations, use no threads, and construct\n//! every Slice from live Rust byte storage.\n\nuse bitcoin_imports::Named;\nuse bitcoinleveldb_file::{\n    WritableFile, WritableFileAppend, WritableFileClose, WritableFileFlush,\n    WritableFileSync,\n};\nuse bitcoinleveldb_slice::Slice;\nuse bitcoinleveldb_status::{Status, StatusCode};\nuse std::borrow::Cow;\nuse std::cell::{Cell, RefCell};\nuse std::panic::{catch_unwind, AssertUnwindSafe};\nuse std::rc::Rc;\n\n#[derive(Debug, PartialEq, Eq)]\nenum Event {\n    Append(Vec<u8>),\n    Close,\n    Flush,\n    Sync,\n    Drop,\n}\n\nstruct RecordingFile {\n    name: String,\n    bytes: Vec<u8>,\n    events: Rc<RefCell<Vec<Event>>>,\n    drops: Rc<Cell<usize>>,\n    statuses: [Status; 4],\n    panic_next_append: bool,\n}\n\nimpl WritableFile for RecordingFile {}\n\nimpl Named for RecordingFile {\n    fn name(&self) -> Cow<\'_, str> {\n        Cow::Borrowed(&self.name)\n    }\n}\n\nimpl WritableFileAppend for RecordingFile {\n    fn append(&mut self, data: &Slice) -> Status {\n        if self.panic_next_append {\n            self.panic_next_append = false;\n            panic!("ordinary test-double append panic");\n        }\n        let bytes = data.as_bytes();\n        self.events.borrow_mut().push(Event::Append(bytes.to_vec()));\n        self.bytes.extend_from_slice(bytes);\n        self.statuses[0].clone()\n    }\n}\n\nimpl WritableFileClose for RecordingFile {\n    fn close(&mut self) -> Status {\n        self.events.borrow_mut().push(Event::Close);\n        self.statuses[1].clone()\n    }\n}\n\nimpl WritableFileFlush for RecordingFile {\n    fn flush(&mut self) -> Status {\n        self.events.borrow_mut().push(Event::Flush);\n        self.statuses[2].clone()\n    }\n}\n\nimpl WritableFileSync for RecordingFile {\n    fn sync(&mut self) -> Status {\n        self.events.borrow_mut().push(Event::Sync);\n        self.statuses[3].clone()\n    }\n}\n\nimpl Drop for RecordingFile {\n    fn drop(&mut self) {\n        self.drops.set(self.drops.get() + 1);\n        self.events.borrow_mut().push(Event::Drop);\n    }\n}\n\ntype Fixture = (\n    Rc<RefCell<RecordingFile>>,\n    Rc<RefCell<Vec<Event>>>,\n    Rc<Cell<usize>>,\n);\n\nfn fixture() -> Fixture {\n    let events = Rc::new(RefCell::new(Vec::new()));\n    let drops = Rc::new(Cell::new(0));\n    let file = Rc::new(RefCell::new(RecordingFile {\n        name: "recording-file".to_owned(),\n        bytes: Vec::new(),\n        events: events.clone(),\n        drops: drops.clone(),\n        statuses: std::array::from_fn(|_| Status::ok()),\n        panic_next_append: false,\n    }));\n    (file, events, drops)\n}\n\nfn boxed_alias(file: &Rc<RefCell<RecordingFile>>) -> Box<dyn WritableFile> {\n    let shared: Rc<RefCell<dyn WritableFile>> = file.clone();\n    Box::<dyn WritableFile>::from(shared)\n}\n\nfn passed(name: &str) {\n    eprintln!("SHARED_WRITABLE_FILE_CONTRACT_OK case={name}");\n}\n\n#[test]\nfn shared_writable_file_forwards_calls_bytes_and_success_statuses() {\n    let (file, events, _drops) = fixture();\n    let mut boxed = boxed_alias(&file);\n    let bytes = [0u8, 0xff, b\'a\', 0, b\'z\'];\n\n    assert!(boxed.append(&Slice::from(bytes.as_slice())).is_ok());\n    assert!(boxed.flush().is_ok());\n    assert!(boxed.append(&Slice::from(&b""[..])).is_ok());\n    assert!(boxed.sync().is_ok());\n    assert!(boxed.close().is_ok());\n\n    assert_eq!(file.borrow().bytes, bytes);\n    assert_eq!(\n        *events.borrow(),\n        vec![\n            Event::Append(bytes.to_vec()),\n            Event::Flush,\n            Event::Append(Vec::new()),\n            Event::Sync,\n            Event::Close,\n        ]\n    );\n    passed("shared_writable_file_forwards_calls_bytes_and_success_statuses");\n}\n\n#[test]\nfn shared_writable_file_preserves_each_error_code_and_message() {\n    let (file, events, _drops) = fixture();\n    let primary = Slice::from(&b"primary message"[..]);\n    let detail = Slice::from(&b"secondary detail"[..]);\n    let expected = [\n        Status::io_error(&primary, Some(&detail)),\n        Status::invalid_argument(&primary, Some(&detail)),\n        Status::not_supported(&primary, Some(&detail)),\n        Status::corruption(&primary, Some(&detail)),\n    ];\n    file.borrow_mut().statuses = expected.clone();\n    let mut boxed = boxed_alias(&file);\n    let actual = [\n        boxed.append(&Slice::from(&b"payload"[..])),\n        boxed.close(),\n        boxed.flush(),\n        boxed.sync(),\n    ];\n\n    for (observed, wanted) in actual.iter().zip(expected.iter()) {\n        assert!(!observed.is_ok());\n        assert_eq!(observed.code(), wanted.code());\n        assert_eq!(observed.to_string(), wanted.to_string());\n        assert!(observed.to_string().contains("primary message"));\n        assert!(observed.to_string().contains("secondary detail"));\n    }\n    assert_eq!(actual[0].code(), StatusCode::IOError);\n    assert_eq!(actual[1].code(), StatusCode::InvalidArgument);\n    assert_eq!(actual[2].code(), StatusCode::NotSupported);\n    assert_eq!(actual[3].code(), StatusCode::Corruption);\n    assert_eq!(\n        *events.borrow(),\n        vec![Event::Append(b"payload".to_vec()), Event::Close, Event::Flush, Event::Sync]\n    );\n    passed("shared_writable_file_preserves_each_error_code_and_message");\n}\n\n#[test]\nfn shared_writable_file_observes_external_changes_and_shared_identity() {\n    let (file, events, _drops) = fixture();\n    let mut first = boxed_alias(&file);\n    let mut second = boxed_alias(&file);\n\n    assert!(first.append(&Slice::from(&b"first"[..])).is_ok());\n    {\n        let mut original = file.borrow_mut();\n        original.bytes.extend_from_slice(b"/external/");\n        original.name = "renamed-through-original".to_owned();\n    }\n    assert_eq!(first.name().as_ref(), "renamed-through-original");\n    assert_eq!(second.name().as_ref(), "renamed-through-original");\n    assert!(second.append(&Slice::from(&b"second"[..])).is_ok());\n\n    assert_eq!(file.borrow().bytes, b"first/external/second");\n    assert_eq!(\n        *events.borrow(),\n        vec![Event::Append(b"first".to_vec()), Event::Append(b"second".to_vec())]\n    );\n    assert_eq!(Rc::strong_count(&file), 3);\n    passed("shared_writable_file_observes_external_changes_and_shared_identity");\n}\n\n#[test]\nfn shared_writable_file_name_is_owned_snapshot_without_retained_borrow() {\n    let (file, _events, _drops) = fixture();\n    let boxed = boxed_alias(&file);\n    let name = boxed.name();\n\n    assert!(matches!(&name, Cow::Owned(_)));\n    assert_eq!(name.as_ref(), "recording-file");\n    {\n        let mut original = file.try_borrow_mut().expect("name() retained its RefCell guard");\n        original.name = "new-name".to_owned();\n    }\n    assert_eq!(name.as_ref(), "recording-file");\n    assert_eq!(boxed.name().as_ref(), "new-name");\n    passed("shared_writable_file_name_is_owned_snapshot_without_retained_borrow");\n}\n\n#[test]\nfn shared_writable_file_conversion_and_drop_do_not_perform_io() {\n    let (file, events, drops) = fixture();\n    file.borrow_mut().bytes = b"existing bytes".to_vec();\n    let shared: Rc<RefCell<dyn WritableFile>> = file.clone();\n    assert_eq!(Rc::strong_count(&file), 2);\n\n    let boxed = Box::<dyn WritableFile>::from(shared);\n\n    assert_eq!(Rc::strong_count(&file), 2, "conversion should consume its one Rc");\n    assert_eq!(file.borrow().bytes, b"existing bytes");\n    assert!(events.borrow().is_empty());\n    drop(boxed);\n    assert_eq!(Rc::strong_count(&file), 1);\n    assert_eq!(drops.get(), 0);\n    assert!(events.borrow().is_empty(), "dropping an alias should not close or sync");\n    drop(file);\n    assert_eq!(drops.get(), 1);\n    assert_eq!(*events.borrow(), vec![Event::Drop]);\n    passed("shared_writable_file_conversion_and_drop_do_not_perform_io");\n}\n\n#[test]\nfn shared_writable_file_keeps_last_owner_alive_until_box_drop() {\n    let (file, events, drops) = fixture();\n    let weak = Rc::downgrade(&file);\n    let mut boxed = boxed_alias(&file);\n    drop(file);\n\n    assert_eq!(weak.strong_count(), 1);\n    assert_eq!(drops.get(), 0);\n    assert_eq!(boxed.name().as_ref(), "recording-file");\n    assert!(boxed.flush().is_ok());\n    drop(boxed);\n\n    assert!(weak.upgrade().is_none());\n    assert_eq!(drops.get(), 1);\n    assert_eq!(*events.borrow(), vec![Event::Flush, Event::Drop]);\n    passed("shared_writable_file_keeps_last_owner_alive_until_box_drop");\n}\n\n#[test]\nfn shared_writable_file_preserves_refcell_borrow_conflicts() {\n    let (file, events, _drops) = fixture();\n    let mut boxed = boxed_alias(&file);\n    let held_read = file.borrow();\n\n    let conflict = catch_unwind(AssertUnwindSafe(|| {\n        boxed.append(&Slice::from(&b"blocked"[..]));\n    }));\n    assert!(conflict.is_err());\n    assert!(events.borrow().is_empty());\n    drop(held_read);\n\n    assert!(boxed.append(&Slice::from(&b"accepted"[..])).is_ok());\n    assert_eq!(file.borrow().bytes, b"accepted");\n    assert_eq!(*events.borrow(), vec![Event::Append(b"accepted".to_vec())]);\n    passed("shared_writable_file_preserves_refcell_borrow_conflicts");\n}\n\n#[test]\nfn shared_writable_file_releases_borrow_when_inner_operation_unwinds() {\n    let (file, events, _drops) = fixture();\n    file.borrow_mut().panic_next_append = true;\n    let mut boxed = boxed_alias(&file);\n\n    let operation = catch_unwind(AssertUnwindSafe(|| {\n        boxed.append(&Slice::from(&b"not-appended"[..]));\n    }));\n    assert!(operation.is_err());\n    assert!(events.borrow().is_empty());\n    {\n        let original = file.try_borrow_mut().expect("unwinding retained an adapter borrow");\n        assert!(!original.panic_next_append);\n        assert!(original.bytes.is_empty());\n    }\n    assert!(boxed.append(&Slice::from(&b"after-unwind"[..])).is_ok());\n    assert_eq!(file.borrow().bytes, b"after-unwind");\n    assert_eq!(*events.borrow(), vec![Event::Append(b"after-unwind".to_vec())]);\n    passed("shared_writable_file_releases_borrow_when_inner_operation_unwinds");\n}\n'
FIND_BYTES=b'// ---------------- [ File: bitcoinleveldb-file/src/find_file.rs ]\ncrate::ix!();\n\n/**\n  | Return the smallest index i such that\n  | files[i]->largest >= key.\n  |\n  | Return files.size() if there is no such file.\n  |\n  | REQUIRES: "files" contains a sorted list of\n  | non-overlapping files.\n  */\npub fn find_file(\n    icmp:  &InternalKeyComparator,\n    files: &[*mut FileMetaData],\n    key_:  &Slice,\n) -> i32 {\n    use tracing::{trace, debug};\n\n    trace!("find_file: begin binary search over {} files", files.len());\n\n    let mut left:  u32 = 0;\n    let mut right: u32 = files.len() as u32;\n\n    while left < right {\n        let mid = (left + right) / 2;\n\n        // SAFETY:\n        // The original C++ used raw pointers. We preserve this exactly.\n        // The caller provides a Vec<*mut FileMetaData> and the semantics\n        // require dereferencing these pointers just as in C++.\n        let f_ptr = files[mid as usize];\n        let f: &FileMetaData = unsafe {\n            &*f_ptr\n        };\n\n        let largest_encoded = f.largest().encode();\n        let cmp = icmp.compare(&largest_encoded, key_);\n\n        debug!(\n            mid       = mid,\n            left      = left,\n            right     = right,\n            cmp       = cmp,\n            file_num  = f.number(),\n            file_size = f.file_size(),\n            "find_file: inspection step"\n        );\n\n        if cmp < 0 {\n            // f->largest < key  \xe2\x86\x92 all files \xe2\x89\xa4 mid are uninteresting\n            left = mid + 1;\n        } else {\n            // f->largest \xe2\x89\xa5 key \xe2\x86\x92 mid might contain answer\n            right = mid;\n        }\n    }\n\n    trace!("find_file: completed with index={}", right);\n\n    right as i32\n}\n\n\n#[cfg(test)]\nmod find_file_leveldb_semantics_tests {\n    use super::*;\n\n    /// Helper: build a valid InternalKey for a given user key and sequence number.\n    fn build_internal_key_for_tests(user_key: &str, seq: SequenceNumber) -> InternalKey {\n        trace!(\n            "build_internal_key_for_tests: user_key={}, seq={}",\n            user_key,\n            seq\n        );\n\n        let user_bytes = user_key.as_bytes();\n        let user_slice = Slice::from(user_bytes);\n\n        InternalKey::new(&user_slice, seq, ValueType::TypeValue)\n    }\n\n    /// Helper: build a FileMetaData with the given smallest/largest user-key range.\n    ///\n    /// The range is defined purely in terms of user keys; sequence and value type\n    /// are fixed for test purposes and are consistent across all files in a test.\n    fn build_file_meta_for_tests(\n        file_number: u64,\n        smallest_user_key: &str,\n        largest_user_key: &str,\n        seq: SequenceNumber,\n    ) -> *mut FileMetaData {\n        trace!(\n            "build_file_meta_for_tests: file_number={}, smallest_user_key={}, largest_user_key={}, seq={}",\n            file_number,\n            smallest_user_key,\n            largest_user_key,\n            seq\n        );\n\n        let smallest = build_internal_key_for_tests(smallest_user_key, seq);\n        let largest  = build_internal_key_for_tests(largest_user_key,  seq);\n\n        let mut meta = FileMetaData::default();\n\n        // Keep ref-counting and seek budget consistent with default semantics.\n        meta.set_refs(0);\n        meta.set_allowed_seeks(1 << 30);\n        meta.set_number(file_number);\n        meta.set_file_size(0);\n        meta.set_smallest(smallest);\n        meta.set_largest(largest);\n\n        let boxed = Box::new(meta);\n        let ptr   = Box::into_raw(boxed);\n\n        debug!(\n            file_number = file_number,\n            smallest_user_key = smallest_user_key,\n            largest_user_key  = largest_user_key,\n            ptr = ?ptr,\n            "build_file_meta_for_tests: constructed FileMetaData"\n        );\n\n        ptr\n    }\n\n    /// Helper: construct an InternalKeyComparator using the null (bytewise) comparator.\n    fn make_test_comparator() -> InternalKeyComparator {\n        trace!("make_test_comparator: constructing InternalKeyComparator with null_slice_comparator");\n        let cmp = InternalKeyComparator::new(null_slice_comparator());\n        debug!(\n            user_comparator_ptr = ?cmp.user_comparator(),\n            "make_test_comparator: constructed comparator"\n        );\n        cmp\n    }\n\n    #[traced_test]\n    fn find_file_returns_zero_for_empty_file_vector() {\n        trace!("find_file_returns_zero_for_empty_file_vector: start");\n\n        let icmp: InternalKeyComparator = make_test_comparator();\n        let files: Vec<*mut FileMetaData> = Vec::new();\n\n        let seq: SequenceNumber = 100;\n        let key_internal        = build_internal_key_for_tests("k", seq);\n        let key_slice           = key_internal.encode();\n\n        let index = find_file(&icmp, &files, &key_slice);\n\n        debug!(\n            index,\n            num_files = files.len(),\n            key_user = "k",\n            "find_file on empty file list"\n        );\n\n        assert_eq!(\n            index, 0,\n            "Expected index 0 (files.len()) when searching an empty file list"\n        );\n\n        trace!("find_file_returns_zero_for_empty_file_vector: end");\n    }\n\n    #[traced_test]\n    fn find_file_binary_search_over_three_disjoint_ranges() {\n        trace!("find_file_binary_search_over_three_disjoint_ranges: start");\n\n        let icmp: InternalKeyComparator = make_test_comparator();\n        let seq: SequenceNumber         = 50;\n\n        // Three disjoint, sorted ranges in internal-key order:\n        //   file 0: ["a", "c"]\n        //   file 1: ["f", "h"]\n        //   file 2: ["k", "n"]\n        let f0 = build_file_meta_for_tests(0, "a", "c", seq);\n        let f1 = build_file_meta_for_tests(1, "f", "h", seq);\n        let f2 = build_file_meta_for_tests(2, "k", "n", seq);\n\n        let files: Vec<*mut FileMetaData> = vec![f0, f1, f2];\n\n        // Each case is (user_key, expected_index).\n        let cases: [(&str, i32); 13] = [\n            // Before the first file\'s smallest user key\n            ("0", 0),\n            // Inside the first file\'s range\n            ("a", 0),\n            ("b", 0),\n            ("c", 0),\n            // Between file 0 and file 1 (no range contains "d"),\n            // but the first file with largest >= key is file 1.\n            ("d", 1),\n            // Inside the second file\'s range\n            ("f", 1),\n            ("g", 1),\n            ("h", 1),\n            // Between file 1 and file 2\n            ("i", 2),\n            // Inside the third file\'s range\n            ("k", 2),\n            ("m", 2),\n            ("n", 2),\n            // After all ranges: index == files.len()\n            ("o", 3),\n        ];\n\n        for (user_key, expected_index) in cases.iter() {\n            let key_internal = build_internal_key_for_tests(*user_key, seq);\n            let key_slice    = key_internal.encode();\n\n            let actual_index = find_file(&icmp, &files, &key_slice);\n\n            debug!(\n                user_key = *user_key,\n                expected_index = *expected_index,\n                actual_index,\n                "find_file case evaluation for three-range configuration"\n            );\n\n            assert_eq!(\n                actual_index,\n                *expected_index,\n                "Unexpected index for key {:?} in three-range test",\n                user_key\n            );\n        }\n\n        unsafe {\n            drop(Box::from_raw(f0));\n            drop(Box::from_raw(f1));\n            drop(Box::from_raw(f2));\n        }\n\n        trace!("find_file_binary_search_over_three_disjoint_ranges: end");\n    }\n\n    #[traced_test]\n    fn find_file_single_file_edge_cases_across_key_space() {\n        trace!("find_file_single_file_edge_cases_across_key_space: start");\n\n        let icmp: InternalKeyComparator = make_test_comparator();\n        let seq: SequenceNumber         = 75;\n\n        // Single file covering ["d", "m"] in user-key space.\n        let f = build_file_meta_for_tests(7, "d", "m", seq);\n        let files: Vec<*mut FileMetaData> = vec![f];\n\n        // Keys spanning before, inside, and after the single range.\n        let cases: [(&str, i32); 5] = [\n            // Before smallest_user_key: still index 0 because largest >= key.\n            ("b", 0),\n            // At the boundaries and inside the file range.\n            ("d", 0),\n            ("h", 0),\n            ("m", 0),\n            // After largest_user_key: index == files.len() == 1.\n            ("z", 1),\n        ];\n\n        for (user_key, expected_index) in cases.iter() {\n            let key_internal = build_internal_key_for_tests(*user_key, seq);\n            let key_slice    = key_internal.encode();\n\n            let actual_index = find_file(&icmp, &files, &key_slice);\n\n            debug!(\n                user_key = *user_key,\n                expected_index = *expected_index,\n                actual_index,\n                "find_file single-file edge-case evaluation"\n            );\n\n            assert_eq!(\n                actual_index,\n                *expected_index,\n                "Unexpected index for key {:?} in single-file test",\n                user_key\n            );\n        }\n\n        unsafe {\n            drop(Box::from_raw(f));\n        }\n\n        trace!("find_file_single_file_edge_cases_across_key_space: end");\n    }\n}\n\n'
OVERLAP_TEST_BYTES=b'//! Search and overlap specifications retained at the overlap helper owner.\n//! The original six-case module below is unchanged.\n\nuse bitcoin_imports::*;\nuse bitcoinleveldb_comparator::bytewise_comparator;\nuse bitcoinleveldb_file::{find_file, FileMetaData};\nuse bitcoinleveldb_key::{InternalKey, InternalKeyComparator, ValueType};\nuse bitcoinleveldb_slice::Slice;\nuse bitcoinleveldb_versionsetutil::some_file_overlaps_range;\nuse tracing::{debug, trace};\n\n#[cfg(test)]\nmod find_file_and_overlap_spec {\n    use super::*;\n\n    struct FindFileHarness {\n        icmp: InternalKeyComparator,\n        files_owned: Vec<Box<FileMetaData>>,\n        file_ptrs: Vec<*mut FileMetaData>,\n        disjoint_sorted_files: bool,\n    }\n\n    impl FindFileHarness {\n        fn new() -> Self {\n            FindFileHarness {\n                icmp: InternalKeyComparator::new(bytewise_comparator()),\n                files_owned: Vec::new(),\n                file_ptrs: Vec::new(),\n                disjoint_sorted_files: true,\n            }\n        }\n\n        fn add_with_seq(\n            &mut self,\n            smallest: &str,\n            largest: &str,\n            smallest_seq: u64,\n            largest_seq: u64,\n        ) {\n            let smallest_slice = Slice::from(smallest);\n            let largest_slice = Slice::from(largest);\n\n            let smallest_ik =\n                InternalKey::new(&smallest_slice, smallest_seq, ValueType::TypeValue);\n            let largest_ik =\n                InternalKey::new(&largest_slice, largest_seq, ValueType::TypeValue);\n\n            let mut f = FileMetaData::default();\n            let number = (self.file_ptrs.len() + 1) as u64;\n            f.set_number(number);\n            f.set_smallest(smallest_ik);\n            f.set_largest(largest_ik);\n\n            let mut boxed = Box::new(f);\n            let ptr: *mut FileMetaData = &mut *boxed;\n\n            trace!(\n                file_number = number,\n                smallest = smallest,\n                largest = largest,\n                "FindFileHarness::add_with_seq: created file"\n            );\n\n            self.files_owned.push(boxed);\n            self.file_ptrs.push(ptr);\n        }\n\n        fn add(&mut self, smallest: &str, largest: &str) {\n            self.add_with_seq(smallest, largest, 100, 100);\n        }\n\n        fn find(&self, key: &str) -> i32 {\n            let key_slice = Slice::from(key);\n            let internal = InternalKey::new(&key_slice, 100, ValueType::TypeValue);\n            let encoded = internal.encode();\n\n            let idx = find_file(&self.icmp, &self.file_ptrs, &encoded);\n\n            debug!(\n                key = key,\n                index = idx,\n                "FindFileHarness::find: completed search"\n            );\n\n            idx\n        }\n\n        fn overlaps(&self, smallest: Option<&str>, largest: Option<&str>) -> bool {\n            let mut s_slice = Slice::default();\n            let mut l_slice = Slice::default();\n            let mut smallest_ptr: *const Slice = core::ptr::null();\n            let mut largest_ptr: *const Slice = core::ptr::null();\n\n            if let Some(s) = smallest {\n                s_slice = Slice::from(s);\n                smallest_ptr = &s_slice as *const Slice;\n            }\n            if let Some(l) = largest {\n                l_slice = Slice::from(l);\n                largest_ptr = &l_slice as *const Slice;\n            }\n\n            let result = some_file_overlaps_range(\n                &self.icmp,\n                self.disjoint_sorted_files,\n                &self.file_ptrs,\n                smallest_ptr,\n                largest_ptr,\n            );\n\n            debug!(\n                smallest = smallest.unwrap_or("<null>"),\n                largest = largest.unwrap_or("<null>"),\n                result = result,\n                "FindFileHarness::overlaps: overlap check"\n            );\n\n            result\n        }\n    }\n\n    #[traced_test]\n    fn verify_find_file_on_empty_set() {\n        let harness = FindFileHarness::new();\n\n        assert_eq!(0, harness.find("foo"));\n        assert!(!harness.overlaps(Some("a"), Some("z")));\n        assert!(!harness.overlaps(None, Some("z")));\n        assert!(!harness.overlaps(Some("a"), None));\n        assert!(!harness.overlaps(None, None));\n    }\n\n    #[traced_test]\n    fn verify_find_file_with_single_file() {\n        let mut harness = FindFileHarness::new();\n        harness.add("p", "q");\n\n        assert_eq!(0, harness.find("a"));\n        assert_eq!(0, harness.find("p"));\n        assert_eq!(0, harness.find("p1"));\n        assert_eq!(0, harness.find("q"));\n        assert_eq!(1, harness.find("q1"));\n        assert_eq!(1, harness.find("z"));\n\n        assert!(!harness.overlaps(Some("a"), Some("b")));\n        assert!(!harness.overlaps(Some("z1"), Some("z2")));\n        assert!(harness.overlaps(Some("a"), Some("p")));\n        assert!(harness.overlaps(Some("a"), Some("q")));\n        assert!(harness.overlaps(Some("a"), Some("z")));\n        assert!(harness.overlaps(Some("p"), Some("p1")));\n        assert!(harness.overlaps(Some("p"), Some("q")));\n        assert!(harness.overlaps(Some("p"), Some("z")));\n        assert!(harness.overlaps(Some("p1"), Some("p2")));\n        assert!(harness.overlaps(Some("p1"), Some("z")));\n        assert!(harness.overlaps(Some("q"), Some("q")));\n        assert!(harness.overlaps(Some("q"), Some("q1")));\n        assert!(!harness.overlaps(None, Some("j")));\n        assert!(!harness.overlaps(Some("r"), None));\n        assert!(harness.overlaps(None, Some("p")));\n        assert!(harness.overlaps(None, Some("p1")));\n        assert!(harness.overlaps(Some("q"), None));\n        assert!(harness.overlaps(None, None));\n    }\n\n    #[traced_test]\n    fn verify_find_file_with_multiple_non_overlapping_files() {\n        let mut harness = FindFileHarness::new();\n        harness.add("150", "200");\n        harness.add("200", "250");\n        harness.add("300", "350");\n        harness.add("400", "450");\n\n        assert_eq!(0, harness.find("100"));\n        assert_eq!(0, harness.find("150"));\n        assert_eq!(0, harness.find("151"));\n        assert_eq!(0, harness.find("199"));\n        assert_eq!(0, harness.find("200"));\n        assert_eq!(1, harness.find("201"));\n        assert_eq!(1, harness.find("249"));\n        assert_eq!(1, harness.find("250"));\n        assert_eq!(2, harness.find("251"));\n        assert_eq!(2, harness.find("299"));\n        assert_eq!(2, harness.find("300"));\n        assert_eq!(2, harness.find("349"));\n        assert_eq!(2, harness.find("350"));\n        assert_eq!(3, harness.find("351"));\n        assert_eq!(3, harness.find("400"));\n        assert_eq!(3, harness.find("450"));\n        assert_eq!(4, harness.find("451"));\n\n        assert!(!harness.overlaps(Some("100"), Some("149")));\n        assert!(!harness.overlaps(Some("251"), Some("299")));\n        assert!(!harness.overlaps(Some("451"), Some("500")));\n        assert!(!harness.overlaps(Some("351"), Some("399")));\n\n        assert!(harness.overlaps(Some("100"), Some("150")));\n        assert!(harness.overlaps(Some("100"), Some("200")));\n        assert!(harness.overlaps(Some("100"), Some("300")));\n        assert!(harness.overlaps(Some("100"), Some("400")));\n        assert!(harness.overlaps(Some("100"), Some("500")));\n        assert!(harness.overlaps(Some("375"), Some("400")));\n        assert!(harness.overlaps(Some("450"), Some("450")));\n        assert!(harness.overlaps(Some("450"), Some("500")));\n    }\n\n    #[traced_test]\n    fn verify_some_file_overlaps_range_with_null_boundaries_disjoint() {\n        let mut harness = FindFileHarness::new();\n        harness.add("150", "200");\n        harness.add("200", "250");\n        harness.add("300", "350");\n        harness.add("400", "450");\n\n        assert!(!harness.overlaps(None, Some("149")));\n        assert!(!harness.overlaps(Some("451"), None));\n        assert!(harness.overlaps(None, None));\n        assert!(harness.overlaps(None, Some("150")));\n        assert!(harness.overlaps(None, Some("199")));\n        assert!(harness.overlaps(None, Some("200")));\n        assert!(harness.overlaps(None, Some("201")));\n        assert!(harness.overlaps(None, Some("400")));\n        assert!(harness.overlaps(None, Some("800")));\n        assert!(harness.overlaps(Some("100"), None));\n        assert!(harness.overlaps(Some("200"), None));\n        assert!(harness.overlaps(Some("449"), None));\n        assert!(harness.overlaps(Some("450"), None));\n    }\n\n    #[traced_test]\n    fn verify_overlap_sequence_checks_in_find_file_harness() {\n        let mut harness = FindFileHarness::new();\n        harness.add_with_seq("200", "200", 5000, 3000);\n\n        assert!(!harness.overlaps(Some("199"), Some("199")));\n        assert!(!harness.overlaps(Some("201"), Some("300")));\n        assert!(harness.overlaps(Some("200"), Some("200")));\n        assert!(harness.overlaps(Some("190"), Some("200")));\n        assert!(harness.overlaps(Some("200"), Some("210")));\n    }\n\n    #[traced_test]\n    fn verify_overlapping_files_with_non_disjoint_flag() {\n        let mut harness = FindFileHarness::new();\n        harness.add("150", "600");\n        harness.add("400", "500");\n        harness.disjoint_sorted_files = false;\n\n        assert!(!harness.overlaps(Some("100"), Some("149")));\n        assert!(!harness.overlaps(Some("601"), Some("700")));\n        assert!(harness.overlaps(Some("100"), Some("150")));\n        assert!(harness.overlaps(Some("100"), Some("200")));\n        assert!(harness.overlaps(Some("100"), Some("300")));\n        assert!(harness.overlaps(Some("100"), Some("400")));\n        assert!(harness.overlaps(Some("100"), Some("500")));\n        assert!(harness.overlaps(Some("375"), Some("400")));\n        assert!(harness.overlaps(Some("450"), Some("450")));\n        assert!(harness.overlaps(Some("450"), Some("500")));\n        assert!(harness.overlaps(Some("450"), Some("700")));\n        assert!(harness.overlaps(Some("600"), Some("700")));\n    }\n}\n'
DOCUMENT_BYTES=b"# Shared WritableFile repair017, controller revision002\n\nThe production adapter and all four Rust source/test payloads are unchanged\nfrom original017. The adapter forwards the shared WritableFile interface and\nreturns an owned name snapshot after releasing its temporary RefCell borrow.\nThe six misplaced overlap tests remain at the production helper's owning crate;\nall88assertions and the original nineteen selected cases are preserved.\n\nOriginal017 successfully compiled both exact Cargo commands and produced all\nfour expected test executables. Its observer then rejected an explanatory\nfailure-note associated with E0133 warnings in a dependency, despite Cargo0,\nbuild-finished=true and no error or ICE diagnostics. No selected native test ran.\nThis revision fixes that observer condition: explanatory notes remain recorded;\nCargo/build failure, primary error/ICE, incomplete protocol, wrong commands and\nwrong executable inventories still block success. No diagnostic is suppressed.\n\nRevision002 admits only that exact frozen original017 run, controller, report,\nreserved journal and six-file staged candidate at the reviewed base. It first\narchives the complete original journal byte for byte in a new run directory,\nrecords the reviewed transition, and replaces only the candidate replay script\nand this document. The four Rust payloads are checked byte-identical. The old\nrun's controller/report and predecessor015/016 reports, journals and worktrees\nstay retained. A different error, changed staged tree, native pending/failure\nflag, commit or partial transition blocks automatic continuation.\n\nThe existing repair/leveldb-writablefile-shared017 worktree and private Cargo\ncache are reused. No new candidate branch or cache copy is made. Both exact\noffline locked single-job no-run builds are rerun with the corrected observer:\n\n- bitcoinleveldb-file: --lib --tests, producing three test executables\n- bitcoinleveldb-versionsetutil: --test find_file_and_overlap, producing one\n\nAll nineteen selected native cases then run fresh:8 shared-file contracts,\n2 filename regressions,3 direct file-search tests and6 relocated search/overlap\nspecifications. No previous test success is inferred or reused from original017.\nEach exact command is bound to its Cargo-discovered executable and fingerprint;\nthe eight shared-file cases require their explicit completion markers. Safe\ncaught-panic contracts retain ordinary RefCell/unwind behavior.\n\nBuilds retain4GiB/600second limits; selected cases512MiB/30seconds, with one CPU,\nswap disabled, task/file/output/data limits and the existing2GiB free-space\nfloor. An interrupted or failed revision002 native case stays in the journal\nand blocks retry. An incomplete revision002 attempt requires review; it cannot\nregain admission as the original reserved state.\n\nOne seven-file candidate commit is created only after all nineteen cases,\nsource/index checks, executable checks and original-state preservation pass.\nMaster remains unchanged. A complete unchanged qualification may be reverified\nwithout building or executing again. This is selected coverage only; complete\nworkspace coverage and full randomized stress are not claimed.\n\nThe runner prints and saves WRITABLE_FILE_REPAIR_RETURN.json. After the\ncoordinator exits, the authorized return-buffer is replaced with the complete\noutput and exact return file. That late transport replacement is separate from\nthe coordinator's preservation comparison.\n"
RELOCATION={'original_find_file_sha256': '42db6099ab6cecdeb90dee3ff949814d7158d4a96d5d5da3c5b88199fa42423f', 'retained_prefix_sha256': '1e1225a726d79e37104769ab6adb2e62e7ca9c079035aba3ad26663b3898bfdd', 'moved_module_sha256': 'c38f0fb8eb5d071aa7611fb0be2fb3d78e8f151c488ea950a7ee226500784e77', 'moved_module_bytes': 9461, 'test_count': 6, 'assertion_count': 88, 'find_assertion_count': 24, 'overlap_assertion_count': 64, 'module_bytes_unchanged': True}
TARGET_CASES={'bitcoinleveldb-file::writable_file_shared': ['shared_writable_file_forwards_calls_bytes_and_success_statuses', 'shared_writable_file_preserves_each_error_code_and_message', 'shared_writable_file_observes_external_changes_and_shared_identity', 'shared_writable_file_name_is_owned_snapshot_without_retained_borrow', 'shared_writable_file_conversion_and_drop_do_not_perform_io', 'shared_writable_file_keeps_last_owner_alive_until_box_drop', 'shared_writable_file_preserves_refcell_borrow_conflicts', 'shared_writable_file_releases_borrow_when_inner_operation_unwinds'], 'bitcoinleveldb-file::filename': ['file_name_test_parse', 'file_name_test_construction'], 'bitcoinleveldb-file::bitcoinleveldb_file': ['find_file_returns_zero_for_empty_file_vector', 'find_file_binary_search_over_three_disjoint_ranges', 'find_file_single_file_edge_cases_across_key_space'], 'bitcoinleveldb-versionsetutil::find_file_and_overlap': ['verify_find_file_on_empty_set', 'verify_find_file_with_single_file', 'verify_find_file_with_multiple_non_overlapping_files', 'verify_some_file_overlaps_range_with_null_boundaries_disjoint', 'verify_overlap_sequence_checks_in_find_file_harness', 'verify_overlapping_files_with_non_disjoint_flag']}


def bootstrap_read(path, limit):
    path = Path(path)
    if not path.is_absolute() or '..' in path.parts:
        raise RuntimeError('Expected an absolute ordinary path.')
    fd = os.open('/', os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:-1]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
            os.close(fd); fd = child
        child = os.open(path.name, os.O_RDONLY | os.O_NOFOLLOW, dir_fd=fd)
        with os.fdopen(child,'rb') as stream:
            before = os.fstat(stream.fileno())
            if not stat.S_ISREG(before.st_mode) or before.st_size > limit:
                raise RuntimeError('File is indirect or too large.')
            raw = stream.read(limit+1)
            after = os.fstat(stream.fileno())
        fields = ('st_dev','st_ino','st_mode','st_size','st_mtime_ns','st_ctime_ns')
        current = path.lstat()
        if len(raw)>limit or any(getattr(before,k)!=getattr(after,k) or getattr(before,k)!=getattr(current,k) for k in fields):
            raise RuntimeError('File changed during read.')
        return raw
    finally:
        os.close(fd)

"""Finite Cargo observation for the two bounded WritableFile repair builds.

The parent must launch this observer under the reviewed resource guard. This
module never builds unless cargo_observer is explicitly invoked with one of its two exact
offline no-run command shapes. Cargo stderr remains attached to that guard.
"""
import json
import os
from pathlib import Path
import re
import selectors
import subprocess
import sys
import time
import uuid
from urllib.parse import unquote, urlsplit

MAX_MACHINE_BYTES = 64 * 1024 * 1024
MAX_RECORD_BYTES = 1024 * 1024
MAX_RECORDS = 50000
MAX_EVIDENCE_BYTES = 512 * 1024
MAX_ERROR_RECORD_BYTES = 16 * 1024
MAX_ERROR_RECORDS = 32
MAX_ERROR_BYTES = 256 * 1024
MAX_WARNING_RENDERED_BYTES = 8 * 1024
MAX_ERROR_SPANS = 8
OBSERVER_SECONDS = 580
ALLOWED_PACKAGES = ("bitcoinleveldb-file", "bitcoinleveldb-versionsetutil")
FILE_COMMAND = ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", "bitcoinleveldb-file", "--lib", "--tests", "--no-run", "--message-format=json"]
OVERLAP_COMMAND = ["cargo", "test", "--offline", "--locked", "--jobs", "1", "-p", "bitcoinleveldb-versionsetutil", "--test", "find_file_and_overlap", "--no-run", "--message-format=json"]
_FILE_COMMAND_SHAPE = tuple(FILE_COMMAND)
_OVERLAP_COMMAND_SHAPE = tuple(OVERLAP_COMMAND)
FILE_TARGETS = {
    "bitcoinleveldb_file": ("lib", "src/lib.rs"),
    "filename": ("test", "tests/filename.rs"),
    "writable_file_shared": ("test", "tests/writable_file_shared.rs"),
}
OVERLAP_TARGETS = {"find_file_and_overlap": ("test", "tests/find_file_and_overlap.rs")}


MAX_EXECUTABLES = 40


def _observer_spec(spec):
    if not isinstance(spec, dict):
        raise RuntimeError("Cargo observer specification is not an object.")
    command = spec.get("command")
    if not isinstance(command, list):
        raise RuntimeError("Cargo observer command must be the exact reviewed argument list.")
    if tuple(command) == _FILE_COMMAND_SHAPE:
        expected = ["bitcoinleveldb-file"]
        targets = FILE_TARGETS
    elif tuple(command) == _OVERLAP_COMMAND_SHAPE:
        expected = ["bitcoinleveldb-versionsetutil"]
        targets = OVERLAP_TARGETS
    else:
        raise RuntimeError("Cargo observer requires one exact reviewed file/overlap command; no union or alternate target is admitted.")
    supplied = spec.get("expected_packages", expected)
    if not isinstance(supplied, list) or supplied != expected:
        raise RuntimeError("Cargo expected package must equal the singleton derived from the exact command.")
    paths = {}
    for key in ("cwd", "target_directory", "evidence_path"):
        value = spec.get(key)
        if not isinstance(value, str) or not value or "\x00" in value:
            raise RuntimeError("Cargo observer path is absent or malformed: " + key)
        path = Path(value)
        if not path.is_absolute() or ".." in path.parts:
            raise RuntimeError("Cargo observer requires an absolute path: " + key)
        paths[key] = path
    if not paths["cwd"].is_dir() or not paths["evidence_path"].parent.is_dir():
        raise RuntimeError("Cargo observer working or evidence directory is absent.")
    if os.path.lexists(paths["evidence_path"]):
        raise RuntimeError("Cargo observer refuses to replace existing evidence.")
    paths["expected_packages"] = tuple(expected)
    paths["expected_artifacts"] = {
        expected[0] + "::" + name: {"kind": kind, "source": paths["cwd"] / expected[0] / source}
        for name, (kind, source) in targets.items()
    }
    return list(command), expected, paths



def _atomic_evidence(path, record):
    raw = json.dumps(record, sort_keys=True, ensure_ascii=False, separators=(",", ":"), allow_nan=False).encode("utf-8") + b"\n"
    if len(raw) > MAX_EVIDENCE_BYTES:
        raise RuntimeError("Cargo evidence exceeds its retained-byte allowance.")
    path = Path(path)
    temporary = path.with_name(path.name + ".tmp-" + uuid.uuid4().hex)
    fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
        # Linking installs the completed file atomically and cannot overwrite a
        # concurrently created receipt. Both files are in the same directory.
        os.link(temporary, path, follow_symlinks=False)
    finally:
        try:
            temporary.unlink()
        except FileNotFoundError:
            pass


def _stop_cargo(child):
    if child.poll() is None:
        child.terminate()
        try:
            child.wait(timeout=2)
        except subprocess.TimeoutExpired:
            child.kill()
            child.wait(timeout=2)
    else:
        child.wait(timeout=2)


def _artifact(record, expected, paths, evidence):
    target = record.get("target")
    profile = record.get("profile")
    if not isinstance(target, dict) or not isinstance(profile, dict):
        raise RuntimeError("Cargo compiler artifact lacks target/profile data.")
    if target.get("kind") == ["custom-build"]:
        evidence["build_script_artifacts_discarded"] += 1
        return
    executable = record.get("executable")
    if executable is None:
        evidence["dependency_artifacts_discarded"] += 1
        return
    kind = target.get("kind")
    if profile.get("test") is not True or kind not in (["lib"], ["test"]):
        raise RuntimeError("Cargo emitted an executable outside the expected lib/test profile.")
    name = target.get("name")
    if not isinstance(name, str) or not re.fullmatch(r"[A-Za-z0-9_][A-Za-z0-9_-]{0,127}", name):
        raise RuntimeError("Cargo test target name is malformed.")
    source = target.get("src_path")
    if not isinstance(source, str) or not source or len(source) > 4096 or "\x00" in source:
        raise RuntimeError("Cargo test source path is absent or malformed.")
    source_path = Path(source)
    if not source_path.is_absolute() or ".." in source_path.parts:
        raise RuntimeError("Cargo test source path is not an absolute package-local path.")
    try:
        relative = source_path.relative_to(paths["cwd"])
    except ValueError as error:
        raise RuntimeError("Cargo test source is outside the private WritableFile worktree.") from error
    if len(relative.parts) < 2 or relative.parts[0] not in expected:
        raise RuntimeError("Cargo test source does not belong to a reviewed package.")
    package_name = relative.parts[0]
    package_id = record.get("package_id")
    if not isinstance(package_id, str) or not package_id or len(package_id) > 4096:
        raise RuntimeError("Cargo package identity is absent or too long.")
    # Modern Cargo path IDs may end in either #version or #name@version.
    # Source membership, rather than a guessed fragment format, selects the
    # package. A supplied path identity must independently name the same root.
    if package_id.startswith("path+"):
        identity = urlsplit(package_id[5:])
        identity_path = Path(unquote(identity.path))
        if (identity.scheme != "file" or identity.netloc not in ("", "localhost")
                or not identity_path.is_absolute() or ".." in identity_path.parts
                or identity_path != paths["cwd"] / package_name):
            raise RuntimeError("Cargo path package identity differs from its source package.")
    elif "://" in package_id:
        raise RuntimeError("Cargo expected local source has an external package identity.")
    key = package_name + "::" + name
    binding = paths["expected_artifacts"].get(key)
    if binding is None or kind != [binding["kind"]] or source_path != binding["source"]:
        raise RuntimeError("Cargo executable target kind/name/source differs from the exact selected command.")
    if key in evidence["artifacts"]:
        raise RuntimeError("Cargo emitted a duplicate package/target test executable.")
    if len(evidence["artifacts"]) >= MAX_EXECUTABLES:
        raise RuntimeError("Cargo test executable count exceeds its allowance.")
    if not isinstance(executable, str) or not executable or len(executable) > 4096 or "\x00" in executable:
        raise RuntimeError("Cargo executable path is malformed or too long.")
    executable_path = Path(executable)
    prefixes = {name, name.replace("-", "_")}
    filename_ok = any(re.fullmatch(re.escape(prefix) + r"-[0-9a-f]+", executable_path.name) for prefix in prefixes)
    if (not executable_path.is_absolute() or ".." in executable_path.parts
            or executable_path.parent != paths["target_directory"] / "debug" / "deps"
            or not filename_ok):
        raise RuntimeError("Cargo executable does not belong to the isolated debug test directory.")
    if any(item["executable"] == executable for item in evidence["artifacts"].values()):
        raise RuntimeError("Cargo reused an executable path for distinct package/target identities.")
    evidence["artifacts"][key] = {
        "package": package_name, "target_name": name, "kind": list(kind),
        "src_path": source, "executable": executable,
        "package_id": package_id, "fresh": record.get("fresh") is True,
    }


def _diagnostic_json(value):
    return json.dumps(value, sort_keys=True, ensure_ascii=False,
                      separators=(",", ":"), allow_nan=False).encode("utf-8")


def _diagnostic_text(value, allowance):
    if not isinstance(value, str):
        raise RuntimeError("Cargo diagnostic text has an unexpected type.")
    raw = value.encode("utf-8")
    retained = raw[:max(0, allowance)].decode("utf-8", errors="ignore")
    return retained, len(raw) - len(retained.encode("utf-8"))


def _diagnostic_path(value, paths):
    """Classify a lexical path without opening any compiler-provided filename."""
    if not isinstance(value, str) or "\x00" in value:
        raise RuntimeError("Cargo diagnostic path is absent or malformed.")
    if len(value.encode("utf-8")) > 4096:
        text, omitted = _diagnostic_text(value, 512)
        return {"file_name": text, "qualified_path": None,
                "path_scope": "overlong_path_omitted", "omitted_path_bytes": omitted}
    if value.startswith("<") and value.endswith(">"):
        return {"file_name": value, "qualified_path": None, "path_scope": "synthetic"}
    path = Path(value)
    if not value or ".." in path.parts:
        return {"file_name": value, "qualified_path": None,
                "path_scope": "unqualified_empty_or_parent_components"}
    absolute = path if path.is_absolute() else paths["cwd"] / path
    if len(str(absolute).encode("utf-8")) > 4096:
        return {"file_name": value, "qualified_path": None,
                "path_scope": "qualified_path_exceeds_allowance"}
    scope = "external"
    try:
        relative = absolute.relative_to(paths["target_directory"])
        scope = "target_output"
    except ValueError:
        try:
            relative = absolute.relative_to(paths["cwd"])
            scope = ("expected_package_source" if relative.parts
                     and relative.parts[0] in paths.get("expected_packages", ALLOWED_PACKAGES) else "other_worktree_source")
        except ValueError:
            pass
    return {"file_name": value, "qualified_path": str(absolute), "path_scope": scope,
            "qualification": "lexical_absolute" if path.is_absolute() else "lexical_observer_cwd",
            "file_bytes_verified": False}


def _diagnostic_target(record, paths):
    target = record.get("target")
    if not isinstance(target, dict):
        raise RuntimeError("Cargo compiler diagnostic lacks a target object.")
    name, omitted_name = _diagnostic_text(target.get("name", ""), 256)
    kinds = target.get("kind", [])
    if (not isinstance(kinds, list) or any(not isinstance(kind, str) for kind in kinds)):
        raise RuntimeError("Cargo compiler diagnostic target kind is malformed.")
    retained_kinds = [_diagnostic_text(kind, 64)[0] for kind in kinds[:8]]
    package, omitted_package = _diagnostic_text(record.get("package_id", ""), 2048)
    return {"name": name, "omitted_name_bytes": omitted_name,
            "kind": retained_kinds, "omitted_kinds": max(0, len(kinds) - 8),
            "src_path": _diagnostic_path(target.get("src_path", ""), paths),
            "package_id": package, "omitted_package_id_bytes": omitted_package}


def _priority_diagnostic(record, message, paths):
    level = message["level"]
    code = message.get("code")
    if code is not None:
        if not isinstance(code, dict) or not isinstance(code.get("code"), str):
            raise RuntimeError("Cargo compiler error code is malformed.")
        code, omitted_code = _diagnostic_text(code["code"], 128)
    else:
        omitted_code = 0
    text, omitted_message = _diagnostic_text(message.get("message", ""), 4096)
    spans = message.get("spans", [])
    if not isinstance(spans, list) or any(not isinstance(span, dict) for span in spans):
        raise RuntimeError("Cargo compiler diagnostic spans are malformed.")
    children = message.get("children", [])
    if not isinstance(children, list):
        raise RuntimeError("Cargo compiler diagnostic children are malformed.")
    result = {"level": level, "code": code, "message": text,
              "target": _diagnostic_target(record, paths), "spans": [], "rendered": "",
              "omitted_code_bytes": omitted_code, "omitted_message_bytes": omitted_message,
              "spans_observed": len(spans), "spans_retained": 0, "spans_omitted": len(spans),
              "children_observed": len(children), "children_retained_separately": False,
              "rendered_bytes_observed": 0, "rendered_bytes_retained": 0,
              "rendered_bytes_omitted": 0}
    # Retain primary locations before secondary locations, preserving order
    # within either group. No span opens or executes any named source.
    spans = sorted(spans, key=lambda span: span.get("is_primary") is not True)
    for span in spans:
        if len(result["spans"]) >= MAX_ERROR_SPANS:
            break
        location = _diagnostic_path(span.get("file_name", ""), paths)
        location["is_primary"] = span.get("is_primary") is True
        for field in ("byte_start", "byte_end", "line_start", "line_end", "column_start", "column_end"):
            value = span.get(field)
            if value is not None and (type(value) is not int or not 0 <= value <= 2 ** 63 - 1):
                raise RuntimeError("Cargo diagnostic span coordinate is malformed.")
            location[field] = value
        label = span.get("label")
        if label is not None:
            location["label"], location["omitted_label_bytes"] = _diagnostic_text(label, 512)
        candidate = dict(result, spans=result["spans"] + [location],
                         spans_retained=len(result["spans"]) + 1,
                         spans_omitted=len(spans) - len(result["spans"]) - 1)
        # Reserve a few bytes for subsequent decimal omission-count growth.
        if len(_diagnostic_json(candidate)) <= MAX_ERROR_RECORD_BYTES - 64:
            result = candidate
    rendered = message.get("rendered") or ""
    raw_rendered = rendered.encode("utf-8")
    result["rendered_bytes_observed"] = len(raw_rendered)
    # The exact encoded record, including JSON escaping and omission fields,
    # determines admission. Warning output never consumes this allowance.
    allowance = min(len(raw_rendered), MAX_ERROR_RECORD_BYTES)
    while True:
        snippet, omitted = _diagnostic_text(rendered, allowance)
        result.update(rendered=snippet, rendered_bytes_retained=len(snippet.encode("utf-8")),
                      rendered_bytes_omitted=omitted)
        excess = len(_diagnostic_json(result)) - MAX_ERROR_RECORD_BYTES
        if excess <= 0:
            return result
        if allowance == 0:
            return None
        allowance = max(0, allowance - max(excess, 64))


def _compiler_diagnostic(record, paths, evidence):
    message = record.get("message")
    if not isinstance(message, dict):
        raise RuntimeError("Cargo compiler message is not an object.")
    level = message.get("level")
    if level not in {"error", "warning", "note", "help", "failure-note", "ice"}:
        raise RuntimeError("Cargo compiler diagnostic level is absent or unknown.")
    rendered = message.get("rendered")
    if rendered is not None and not isinstance(rendered, str):
        raise RuntimeError("Cargo rendered diagnostic is not text.")
    evidence["diagnostic_records"] += 1
    evidence["diagnostic_levels"][level] = evidence["diagnostic_levels"].get(level, 0) + 1
    raw = (rendered or "").encode("utf-8")
    if level in {"error", "failure-note", "ice"}:
        evidence["priority_records_observed"] += 1
        candidate = _priority_diagnostic(record, message, paths)
        encoded = _diagnostic_json(candidate) if candidate is not None else b""
        reason = ("record_byte_allowance" if candidate is None else
                  "record_count_allowance" if evidence["priority_records_retained"] >= MAX_ERROR_RECORDS else
                  "total_byte_allowance" if evidence["priority_bytes_retained"] + len(encoded) > MAX_ERROR_BYTES else None)
        if reason is not None:
            evidence["priority_records_omitted"] += 1
            counts = evidence["priority_omission_reasons"]
            counts[reason] = counts.get(reason, 0) + 1
            snippet = b""
        else:
            evidence["priority_diagnostics"].append(candidate)
            evidence["priority_records_retained"] += 1
            evidence["priority_bytes_retained"] += len(encoded)
            snippet = candidate["rendered"].encode("utf-8")
            evidence["priority_rendered_bytes"] += len(snippet)
    else:
        allowance = max(0, MAX_WARNING_RENDERED_BYTES - evidence["warning_rendered_bytes"])
        snippet = _diagnostic_text(rendered or "", allowance)[0].encode("utf-8")
        evidence["warning_rendered_bytes"] += len(snippet)
    if snippet:
        sys.stderr.write(snippet.decode("utf-8"))
        sys.stderr.flush()
    evidence["rendered_bytes"] += len(snippet)
    evidence["rendered_bytes_omitted"] += len(raw) - len(snippet)


def _record(raw, expected, paths, evidence):
    if len(raw) > MAX_RECORD_BYTES:
        raise RuntimeError("Cargo machine record exceeds its per-record allowance.")
    if not raw.strip():
        return
    evidence["records"] += 1
    if evidence["records"] > MAX_RECORDS:
        raise RuntimeError("Cargo machine record count exceeds its allowance.")
    try:
        record = json.loads(raw.decode("utf-8"))
    except (ValueError, UnicodeError) as error:
        raise RuntimeError("Cargo stdout contains malformed machine JSON.") from error
    if not isinstance(record, dict):
        raise RuntimeError("Cargo machine record is not an object.")
    reason = record.get("reason")
    if reason == "compiler-artifact":
        _artifact(record, expected, paths, evidence)
    elif reason == "build-script-executed":
        evidence["build_script_records_discarded"] += 1
    elif reason == "compiler-message":
        _compiler_diagnostic(record, paths, evidence)
    elif reason == "build-finished":
        if evidence["build_finished"] is not None or type(record.get("success")) is not bool:
            raise RuntimeError("Cargo build-finished record is duplicate or malformed.")
        evidence["build_finished"] = record["success"]
    elif reason == "future-incompat-report":
        evidence["other_records_discarded"] += 1
    else:
        raise RuntimeError("Cargo emitted an unknown machine-record reason.")


def cargo_observer(spec):
    """Run only the fixed offline compilation command and retain small evidence.

    Return 0 only when the exact command succeeded and emitted its complete selected target inventory.
    On malformed output or an output allowance violation, stop Cargo promptly.
    """
    evidence = {
        "schema": "bitcoinrs-writable017-cargo-observer-v1", "command": None,
        "cwd": None, "target_directory": None, "expected_packages": [],
        "cargo_exit_code": None, "observer_exit_code": 2, "error": None,
        "artifacts": {}, "machine_bytes": 0, "records": 0,
        "dependency_artifacts_discarded": 0, "build_script_records_discarded": 0,
        "build_script_artifacts_discarded": 0,
        "diagnostic_records": 0, "other_records_discarded": 0,
        "rendered_bytes": 0, "rendered_bytes_omitted": 0,
        "diagnostic_levels": {}, "priority_diagnostics": [],
        "priority_records_observed": 0, "priority_records_retained": 0,
        "priority_records_omitted": 0, "priority_bytes_retained": 0,
        "priority_omission_reasons": {}, "warning_rendered_bytes": 0,
        "priority_rendered_bytes": 0,
        "build_finished": None,
    }
    child, paths = None, None
    try:
        command, expected, paths = _observer_spec(spec)
        evidence.update(command=command, cwd=str(paths["cwd"]),
                        target_directory=str(paths["target_directory"]), expected_packages=expected)
        env = dict(os.environ)
        env.update(CARGO_TARGET_DIR=str(paths["target_directory"]), CARGO_BUILD_JOBS="1", CARGO_INCREMENTAL="0")
        child = subprocess.Popen(command, cwd=paths["cwd"], env=env,
                                 stdin=subprocess.DEVNULL, stdout=subprocess.PIPE)
        os.set_blocking(child.stdout.fileno(), False)
        buffer = bytearray()
        deadline = time.monotonic() + OBSERVER_SECONDS
        with selectors.DefaultSelector() as selector:
            selector.register(child.stdout, selectors.EVENT_READ)
            while selector.get_map():
                if time.monotonic() >= deadline:
                    raise RuntimeError("Cargo observer reached its wall-clock allowance.")
                for key, _ in selector.select(0.1):
                    try:
                        chunk = os.read(key.fileobj.fileno(), 65536)
                    except BlockingIOError:
                        continue
                    if not chunk:
                        selector.unregister(key.fileobj)
                        if buffer:
                            _record(bytes(buffer), expected, paths, evidence)
                            buffer.clear()
                        continue
                    evidence["machine_bytes"] += len(chunk)
                    if evidence["machine_bytes"] > MAX_MACHINE_BYTES:
                        raise RuntimeError("Cargo machine stdout exceeds its total-byte allowance.")
                    buffer.extend(chunk)
                    while b"\n" in buffer:
                        newline = buffer.index(b"\n")
                        _record(bytes(buffer[:newline]), expected, paths, evidence)
                        del buffer[:newline + 1]
                    if len(buffer) > MAX_RECORD_BYTES:
                        raise RuntimeError("Cargo machine record exceeds its per-record allowance.")
        remaining = max(0.1, deadline - time.monotonic())
        evidence["cargo_exit_code"] = child.wait(timeout=remaining)
        if evidence["cargo_exit_code"] != 0:
            raise RuntimeError("Cargo exited unsuccessfully.")
        if evidence["build_finished"] is not True:
            raise RuntimeError("Cargo did not report a successful completed build.")
        # rustc may label explanatory output (for example --explain E0133
        # following an unsafe-op warning) as failure-note even on success.
        # Keep that output prioritized, but use error/ICE plus Cargo's actual
        # process/build-finished result to decide whether compilation passed.
        if (evidence["diagnostic_levels"].get("error", 0)
                or evidence["diagnostic_levels"].get("ice", 0)):
            raise RuntimeError("Cargo reported error diagnostics despite a successful process/build result.")
        if set(evidence["artifacts"]) != set(paths["expected_artifacts"]):
            raise RuntimeError("Cargo did not emit the exact complete executable inventory for its selected command.")
        evidence["observer_exit_code"] = 0
    except Exception as error:
        evidence["error"] = (type(error).__name__ + ": " + str(error))[:2048]
        if child is not None:
            try:
                _stop_cargo(child)
            except Exception as stop_error:
                evidence["cleanup_error"] = str(stop_error)[:512]
            evidence["cargo_exit_code"] = child.poll()
    finally:
        if child is not None and child.stdout is not None:
            child.stdout.close()
    if paths is None:
        sys.stderr.write("WRITABLE017_CARGO_OBSERVER_REJECTED " + (evidence["error"] or "invalid specification") + "\n")
        return 2
    try:
        _atomic_evidence(paths["evidence_path"], evidence)
    except Exception as error:
        sys.stderr.write("WRITABLE017_CARGO_EVIDENCE_ERROR " + str(error)[:1024] + "\n")
        return 3
    print("WRITABLE017_CARGO_OBSERVER records=" + str(evidence["records"])
          + " executables=" + str(len(evidence["artifacts"]))
          + " native_exit=" + str(evidence["cargo_exit_code"])
          + " observer_exit=" + str(evidence["observer_exit_code"]), flush=True)
    return evidence["observer_exit_code"]


"""Post-exit transport for the bounded compiler diagnostic016.

Only the separately authorized original return-buffer is replaced after the
coordinator has exited and printed/saved return JSON identities agree.
"""
import fcntl
import hashlib
import json
import os
from pathlib import Path
import signal
import stat
import subprocess
import sys
import threading
import uuid


_TRANSPORT_MAX_BYTES = 8 * 1024 * 1024


_TRANSPORT_WORKFLOW = "leveldb-writablefile-shared017"


_TRANSPORT_BEGIN = b"BEGIN_WRITABLE_FILE_REPAIR_RETURN"


_TRANSPORT_END = b"END_WRITABLE_FILE_REPAIR_RETURN"


class _TransportReplacementError(RuntimeError):
    buffer_replaced = True


def _transport_open_directory(path):
    path = Path(path)
    if not path.is_absolute() or ".." in path.parts:
        raise RuntimeError("Transport requires an absolute ordinary directory.")
    fd = os.open("/", os.O_RDONLY | os.O_DIRECTORY)
    try:
        for part in path.parts[1:]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW, dir_fd=fd)
            os.close(fd)
            fd = child
        return fd
    except BaseException:
        os.close(fd)
        raise


def _transport_stamp(info):
    return tuple(getattr(info, key) for key in
                 ("st_dev", "st_ino", "st_mode", "st_uid", "st_size", "st_mtime_ns", "st_ctime_ns"))


def _transport_read_at(directory_fd, name, limit, own=False):
    fd = os.open(name, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=directory_fd)
    try:
        before = os.fstat(fd)
        if not stat.S_ISREG(before.st_mode) or before.st_size > limit:
            raise RuntimeError("Transport input is not a bounded ordinary file: " + name)
        if own and before.st_uid != os.getuid():
            raise RuntimeError("Transport return-buffer is not owned by the current user.")
        pieces, total, digest = [], 0, hashlib.sha256()
        while True:
            chunk = os.read(fd, min(65536, limit + 1 - total))
            if not chunk:
                break
            total += len(chunk)
            if total > limit:
                raise RuntimeError("Transport input grew beyond its byte allowance.")
            pieces.append(chunk)
            digest.update(chunk)
        after = os.fstat(fd)
        current = os.stat(name, dir_fd=directory_fd, follow_symlinks=False)
        if (total != before.st_size or _transport_stamp(before) != _transport_stamp(after)
                or _transport_stamp(before) != _transport_stamp(current)):
            raise RuntimeError("Transport input changed during its ordinary-file read.")
        fp = {"kind": "file", "mode": stat.S_IMODE(before.st_mode), "size": total,
              "sha256": digest.hexdigest()}
        return b"".join(pieces), fp, _transport_stamp(before)
    finally:
        os.close(fd)


def _transport_read_file(path, limit):
    path = Path(path)
    parent = _transport_open_directory(path.parent)
    try:
        return _transport_read_at(parent, path.name, limit)
    finally:
        os.close(parent)


def _transport_emit(raw):
    view = memoryview(raw)
    while view:
        try:
            written = os.write(1, view)
        except InterruptedError:
            continue
        if written <= 0:
            raise RuntimeError("Terminal output did not accept captured bytes.")
        view = view[written:]


def _transport_unique_pairs(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise RuntimeError("Repair transport JSON contains duplicate fields.")
        result[key] = value
    return result


def _transport_report(capture, repo, child_status):
    lines = capture.splitlines()
    begins = [index for index, line in enumerate(lines) if line == _TRANSPORT_BEGIN]
    ends = [index for index, line in enumerate(lines) if line == _TRANSPORT_END]
    if len(begins) != 1 or len(ends) != 1 or begins[0] >= ends[0]:
        raise RuntimeError("Completed output lacks one unambiguous compiler diagnostic return JSON block.")
    report = json.loads(b"\n".join(lines[begins[0] + 1:ends[0]]).decode("utf-8"), object_pairs_hook=_transport_unique_pairs)
    if (not isinstance(report, dict) or report.get("workflow") != _TRANSPORT_WORKFLOW
            or report.get("repository") != str(repo) or not isinstance(report.get("outcome"), str)):
        raise RuntimeError("Printed compiler diagnostic return does not identify this coordinator/repository.")
    value = report.get("return_file")
    if not isinstance(value, str) or not value or "\x00" in value:
        raise RuntimeError("Compiler diagnostic return-file path is absent or malformed.")
    path = Path(value)
    root = repo / ".git/codex-repairs" / _TRANSPORT_WORKFLOW / "runs"
    if not path.is_absolute() or ".." in path.parts:
        raise RuntimeError("Compiler diagnostic return-file path is not ordinary and absolute.")
    try:
        relative = path.relative_to(root)
    except ValueError as error:
        raise RuntimeError("Compiler diagnostic return file is outside the owned workflow.") from error
    if len(relative.parts) != 2 or relative.name != "WRITABLE_FILE_REPAIR_RETURN.json":
        raise RuntimeError("Compiler diagnostic return-file layout differs from the owned run layout.")
    if "run_directory" in report and report["run_directory"] != str(path.parent):
        raise RuntimeError("Repair run directory differs from its return file.")
    raw, fp, _ = _transport_read_file(path, _TRANSPORT_MAX_BYTES)
    retained = json.loads(raw.decode("utf-8"), object_pairs_hook=_transport_unique_pairs)
    if retained != report:
        raise RuntimeError("Printed compiler diagnostic return differs from its retained JSON file.")
    successful = report["outcome"] in {"qualified_candidate", "already_qualified"}
    if (child_status == 0) != successful:
        raise RuntimeError("Coordinator exit status disagrees with its recorded outcome.")
    return report, path, raw, fp


def _transport_existing_state(repo_fd):
    try:
        raw, fp, stamp = _transport_read_at(repo_fd, "return-buffer", _TRANSPORT_MAX_BYTES, own=True)
        return fp, stamp
    except FileNotFoundError:
        return {"kind": "absent"}, None


def _transport_replace_buffer(repo, payload):
    if len(payload) > _TRANSPORT_MAX_BYTES:
        raise RuntimeError("Complete output plus return JSON exceeds the 8 MiB return-buffer allowance.")
    repo_fd = _transport_open_directory(repo)
    temporary = ".return-buffer-transport-" + uuid.uuid4().hex + ".tmp"
    made = False
    replacement_done = False
    try:
        before, stamp = _transport_existing_state(repo_fd)
        mode = before["mode"] if before["kind"] == "file" else 0o600
        fd = os.open(temporary, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=repo_fd)
        made = True
        with os.fdopen(fd, "wb") as stream:
            os.fchmod(stream.fileno(), mode)
            stream.write(payload)
            stream.flush()
            os.fsync(stream.fileno())
        try:
            current = os.stat("return-buffer", dir_fd=repo_fd, follow_symlinks=False)
        except FileNotFoundError:
            current = None
        if (stamp is None and current is not None) or (stamp is not None and (current is None or _transport_stamp(current) != stamp)):
            raise RuntimeError("Return-buffer changed before its authorized atomic replacement.")
        # Replacing the directory entry never follows a target symlink and
        # never writes through a source hard link to another pathname.
        os.replace(temporary, "return-buffer", src_dir_fd=repo_fd, dst_dir_fd=repo_fd)
        replacement_done = True
        made = False
        os.fsync(repo_fd)
        written, after, _ = _transport_read_at(repo_fd, "return-buffer", _TRANSPORT_MAX_BYTES, own=True)
        if written != payload or after["mode"] != mode:
            raise RuntimeError("Published return-buffer does not match the complete authorized payload.")
        return before, after
    except BaseException as error:
        if replacement_done:
            raise _TransportReplacementError("Return-buffer was replaced, but final durability/readback verification failed: " + str(error)) from error
        raise
    finally:
        if made:
            try:
                os.unlink(temporary, dir_fd=repo_fd)
            except FileNotFoundError:
                pass
        os.close(repo_fd)


def _transport_write_receipt(directory, value):
    raw = (json.dumps(value, sort_keys=True, indent=2) + "\n").encode()
    if len(raw) > 65536:
        raise RuntimeError("Transport receipt exceeds its finite allowance.")
    parent = _transport_open_directory(directory)
    try:
        fd = os.open("TRANSPORT_RETURN.json", os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=parent)
        with os.fdopen(fd, "wb") as stream:
            stream.write(raw)
            stream.flush()
            os.fsync(stream.fileno())
        os.fsync(parent)
    finally:
        os.close(parent)


def _transport_lock(repo):
    workflow = repo / ".git/codex-repairs" / _TRANSPORT_WORKFLOW
    parent = _transport_open_directory(workflow)
    try:
        fd = os.open("run.lock", os.O_RDWR | os.O_NOFOLLOW | os.O_NONBLOCK, dir_fd=parent)
    finally:
        os.close(parent)
    try:
        info = os.fstat(fd)
        if not stat.S_ISREG(info.st_mode) or info.st_uid != os.getuid():
            raise RuntimeError("Existing repair transport lock is nonordinary or foreign-owned.")
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        return fd
    except BaseException:
        os.close(fd)
        raise


def run_with_transport(script_path, repo, apply):
    """Run one coordinator child, mirror every byte, then replace one buffer."""
    child = None
    child_status = None
    capture = bytearray()
    overflow = False
    mirror_error = None
    report = return_path = return_fp = None
    buffer_written = False
    buffer_replaced = False
    lock_fd = None
    previous_handlers = {}
    total_output = 0
    try:
        if type(apply) is not bool:
            raise RuntimeError("Transport apply/preview selection is malformed.")
        repo = Path(os.path.abspath(Path(repo).expanduser()))
        script_path = Path(os.path.abspath(Path(script_path).expanduser()))
        if any(char in str(repo) for char in ("\x00", "\n", "\r")):
            raise RuntimeError("Transport repository path framing differs.")
        fd = _transport_open_directory(repo)
        os.close(fd)
        _transport_read_file(script_path, 2 * 1024 * 1024)
        command = [sys.executable, "-I", "-S", str(script_path), "--coordinator", str(repo), "apply" if apply else "preview"]
        child = subprocess.Popen(command, cwd=repo, stdin=subprocess.DEVNULL,
                                 stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                 bufsize=0, start_new_session=True)
        if threading.current_thread() is threading.main_thread():
            def forward(signum, frame):
                if child.poll() is None:
                    try:
                        child.send_signal(signum)
                    except ProcessLookupError:
                        pass
            for signum in (signal.SIGINT, signal.SIGTERM):
                previous_handlers[signum] = signal.signal(signum, forward)
        while True:
            try:
                chunk = os.read(child.stdout.fileno(), 65536)
            except InterruptedError:
                continue
            if not chunk:
                break
            total_output += len(chunk)
            if mirror_error is None:
                try:
                    _transport_emit(chunk)
                except OSError as error:
                    mirror_error = str(error)
            room = _TRANSPORT_MAX_BYTES - len(capture)
            if len(chunk) > room:
                overflow = True
            if room > 0:
                capture.extend(chunk[:room])
        child_status = child.wait()
        if overflow:
            raise RuntimeError("Complete coordinator output exceeds the 8 MiB capture allowance; prior return-buffer was retained.")
        report, return_path, return_raw, return_fp = _transport_report(bytes(capture), repo, child_status)
        if mirror_error is not None:
            raise RuntimeError("Terminal mirroring failed: " + mirror_error)
        confirmation = ("RETURN_BUFFER_UPDATED_AFTER_COORDINATOR_EXIT " + str(repo / "return-buffer") + "\n").encode()
        payload = (bytes(capture) + b"\nBEGIN_WRITABLE_FILE_REPAIR_RETURN_FILE_JSON\n" + return_raw
                   + (b"" if return_raw.endswith(b"\n") else b"\n")
                   + b"END_WRITABLE_FILE_REPAIR_RETURN_FILE_JSON\n" + confirmation)
        if len(payload) > _TRANSPORT_MAX_BYTES:
            raise RuntimeError("Complete coordinator output plus exact return file exceeds the 8 MiB payload allowance; prior return-buffer was retained.")
        # No lock is held during the coordinator. Reacquire its existing own
        # workflow lock afterward to avoid overlapping a new snapshot/run.
        lock_fd = _transport_lock(repo)
        before, after = _transport_replace_buffer(repo, payload)
        buffer_written = True
        buffer_replaced = True
        transport = {"schema": "bitcoinrs-writable017-buffer-transport-v1", "outcome": "written",
                     "coordinator_exit_code": child_status, "coordinator_outcome": report["outcome"],
                     "coordinator_return_file": str(return_path), "coordinator_return_fingerprint": return_fp,
                     "target": str(repo / "return-buffer"), "captured_output_bytes": total_output,
                     "payload_bytes": len(payload), "payload_sha256": hashlib.sha256(payload).hexdigest(),
                     "full_output_captured": True, "authorized_overwrite_after_coordinator": True,
                     "coordinator_preservation_verified": (report.get("original_preserved") is True),
                     "preservation_scope": "Buffer publication occurs after coordinator exit and return-file validation; preservation results belong to the coordinator report.",
                     "buffer_before": before, "buffer_after": after}
        _transport_write_receipt(return_path.parent, transport)
        _transport_emit(confirmation)
        return 0 if child_status == 0 else 1
    except BaseException as error:
        buffer_replaced = buffer_replaced or getattr(error, "buffer_replaced", False)
        if child is not None and child_status is None:
            # Transport failures must not terminate/restart an in-flight
            # diagnostic. Finish draining its bounded coordinator lifecycle.
            try:
                while True:
                    chunk = os.read(child.stdout.fileno(), 65536)
                    if not chunk:
                        break
                    try:
                        _transport_emit(chunk)
                    except OSError:
                        pass
                child_status = child.wait()
            except BaseException as cleanup_error:
                sys.stderr.write("Transport could not finish observing the coordinator: " + str(cleanup_error)[:512] + "\n")
        actual = "coordinator_exit=" + str(child_status)
        if report is not None:
            actual += " coordinator_outcome=" + report["outcome"]
        suffix = ("; return-buffer was already written and verified" if buffer_written
                  else "; return-buffer was already replaced; final verification is incomplete" if buffer_replaced
                  else "; prior return-buffer was retained")
        sys.stderr.write("Return-buffer transport failed: " + type(error).__name__ + ": " + str(error)[:2048] + "; " + actual + suffix + ". No coordinator retry was started.\n")
        sys.stderr.flush()
        return 1
    finally:
        for signum, handler in previous_handlers.items():
            signal.signal(signum, handler)
        if child is not None and child.stdout is not None:
            child.stdout.close()
        if lock_fd is not None:
            os.close(lock_fd)


def classify_r1_controller_stop017(report):
    if (report.get('workflow') != WORKFLOW or report.get('outcome') != 'failed'
            or report.get('phase') != 'compile-overlap-tests' or report.get('repair_qualified') is not False
            or report.get('native_tests_started') is not False or report.get('test_count') != 0
            or report.get('cases') != [] or report.get('master_updated') is not False
            or report.get('other_refs_updated') is not False or report.get('original_preserved') is not True
            or report.get('lineage_before') != report.get('lineage_after')):
        raise RuntimeError('The prior017 run is not the exact reviewed pre-native controller stop.')
    labels = {'preflight', 'read-configured-identity', 'git-worktree', 'install-source', 'git-add', 'seed-private-cache', 'compile-file-tests', 'compile-overlap-tests'}
    if set(report.get('readouts', {})) != labels:
        raise RuntimeError('Prior017 phase inventory differs.')
    for label, row in report['readouts'].items():
        expected = 2 if label == 'compile-overlap-tests' else 0
        if row.get('verified') is not True or row.get('outcome') != 'exited' or row.get('exit_code') != expected:
            raise RuntimeError('Prior017 includes an unreviewed control/process failure.')
    for label, command, keys in [('file', FILE_COMMAND, set(TARGET_CASES) - {'bitcoinleveldb-versionsetutil::find_file_and_overlap'}),
                                 ('overlap', OVERLAP_COMMAND, {'bitcoinleveldb-versionsetutil::find_file_and_overlap'})]:
        evidence = report.get('cargo_evidence', {}).get(label, {})
        levels = evidence.get('diagnostic_levels', {})
        expected_code = 0 if label == 'file' else 2
        if (evidence.get('command') != command or evidence.get('cargo_exit_code') != 0
                or evidence.get('observer_exit_code') != expected_code or evidence.get('build_finished') is not True
                or levels.get('error', 0) or levels.get('ice', 0)
                or set(levels) - {'warning', 'note', 'help', 'failure-note'}
                or evidence.get('priority_records_omitted') != 0
                or set(evidence.get('artifacts', {})) != keys):
            raise RuntimeError('Prior017 is not the reviewed complete successful compilation with explanatory note.')
        if label == 'file':
            if evidence.get('error') is not None or evidence.get('priority_diagnostics') != []:
                raise RuntimeError('Prior file compilation includes an unreviewed diagnostic.')
        else:
            notes = evidence.get('priority_diagnostics', [])
            if (evidence.get('error') != 'RuntimeError: Cargo reported error diagnostics despite a successful process/build result.'
                    or levels.get('failure-note') != 1 or len(notes) != 1
                    or notes[0].get('level') != 'failure-note' or notes[0].get('code') is not None
                    or notes[0].get('message') != 'For more information about this error, try `rustc --explain E0133`.'
                    or notes[0].get('spans') != [] or notes[0].get('target', {}).get('name') != 'bitcoinleveldb_logwriter'):
                raise RuntimeError('The prior observer stop is not the exact explanatory-note misclassification.')
    return {'classification': 'controller_false_positive_on_explanatory_failure_note',
            'compiler_builds_succeeded': True, 'native_tests_executed': 0,
            'test_validation_reused': False, 'error_or_ice_diagnostics': 0}


def frozen_r1_evidence017(core, repo):
    directory = repo / '.git/codex-repairs' / WORKFLOW / 'runs' / R1_RUN
    report_path, controller_path = directory / 'WRITABLE_FILE_REPAIR_RETURN.json', directory / 'writable-file-controller017.py'
    report_fp, raw = core.read_file(report_path, 2 * MIB017)
    controller_fp, source = core.read_file(controller_path, 2 * MIB017)
    if (report_fp['sha256'] != R1_REPORT_SHA or report_fp['size'] != R1_REPORT_BYTES
            or controller_fp['sha256'] != R1_CONTROLLER_SHA):
        raise RuntimeError('Frozen original017 report or controller differs.')
    report = json.loads(raw); classification = classify_r1_controller_stop017(report)
    if (report.get('run_directory') != str(directory) or report.get('repository') != str(repo)
            or report.get('worktree') != str(repo.parent / (repo.name + SUFFIX))
            or report.get('controller_sha256') != R1_CONTROLLER_SHA):
        raise RuntimeError('Original017 evidence paths or controller identity differ.')
    original = types.ModuleType('writable017_r1_frozen'); original.__file__ = str(controller_path)
    sys.modules[original.__name__] = original; exec(compile(source, str(controller_path), 'exec'), original.__dict__)
    original_files = original.files017(source)
    if (set(original_files) != set(INITIAL_PATHS) or original.TARGET_CASES != TARGET_CASES
            or report.get('source_plan') != {p: core.digest(raw) for p, raw in original_files.items()}):
        raise RuntimeError('Original017 exact source/test selection plan differs.')
    current = files017(b'controller bytes are checked separately')
    for name in RUST_PATHS:
        if original_files[name] != current[name]:
            raise RuntimeError('This controller-only retry changes a Rust source or test payload.')
    proof = dict(classification, report_path=str(report_path), report_fingerprint=report_fp,
                 controller_path=str(controller_path), controller_fingerprint=controller_fp,
                 original_source_plan=report['source_plan'], rust_paths_unchanged=list(RUST_PATHS))
    return proof, report, original_files


def expected_r1_state017(core, report):
    identity = {'workflow': WORKFLOW, 'base': BASE, 'branch': BRANCH, 'worktree': report['worktree'],
                'file_hashes': report['source_plan'], 'lineage_sha256': core.digest(core.encoded(report['lineage_before']))}
    return {'identity': identity, 'phase': 'reserved'}


def review_r1_transition017(core, helper, repo, state_path):
    proof, report, original_files = frozen_r1_evidence017(core, repo)
    fp, raw = core.read_file(state_path, 2 * MIB017)
    if (fp['sha256'] != R1_STATE_SHA or fp['size'] != R1_STATE_BYTES
            or json.loads(raw) != expected_r1_state017(core, report)):
        raise RuntimeError('Original017 journal is not the exact reserved, unexecuted state; no failure flag was cleared.')
    helper.check_candidate(core, repo, repo.parent / (repo.name + SUFFIX), original_files, BASE)
    proof['previous_state_fingerprint'] = fp
    return proof, raw


def verify_transition_archive017(core, repo, transition):
    proof, _, _ = frozen_r1_evidence017(core, repo)
    path = Path(transition.get('previous_state_path', ''))
    if (path.name != 'previous-state-before-controller-review.json'
            or path.parent.parent != repo / '.git/codex-repairs' / WORKFLOW / 'runs'):
        raise RuntimeError('Retained original journal copy is outside the owned run.')
    fp, raw = core.read_file(path, 2 * MIB017)
    if (fp['sha256'] != R1_STATE_SHA or fp['size'] != R1_STATE_BYTES
            or fp != transition.get('previous_state_fingerprint')):
        raise RuntimeError('Retained original journal copy differs.')
    expected = dict(proof, previous_state_fingerprint=fp, previous_state_path=str(path))
    if transition != expected:
        raise RuntimeError('Controller transition evidence differs from the frozen reviewed stop.')
    return True


def refresh_controller017(core, helper, repo):
    state_path = repo / '.git/codex-repairs' / WORKFLOW / 'state.json'
    state = json.loads(core.read_file(state_path, 4 * MIB017)[1])
    transition = state.get('controller_transition', {})
    verify_transition_archive017(core, repo, transition)
    if (state.get('phase') != 'controller-refresh-pending' or state.get('native_pending') is not None
            or state.get('native_failure') is not None):
        raise RuntimeError('Controller refresh is not in the exact pre-native transition state.')
    raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB017); files = files017(raw)
    identity = state.get('identity', {})
    if (identity.get('controller_revision') != CONTROLLER_REVISION or identity.get('file_hashes') != {p: core.digest(b) for p, b in files.items()}
            or identity.get('workflow') != WORKFLOW or identity.get('base') != BASE
            or identity.get('branch') != BRANCH or identity.get('worktree') != str(repo.parent / (repo.name + SUFFIX))):
        raise RuntimeError('Controller refresh identity differs from this exact revision.')
    _, _, original_files = frozen_r1_evidence017(core, repo)
    worktree = repo.parent / (repo.name + SUFFIX)
    helper.check_candidate(core, repo, worktree, original_files, BASE)
    for name, mode in ((REPLAY_PATH, 0o755), (DOCUMENT_PATH, 0o644)):
        path = worktree / name
        if core.read_file(path, 2 * MIB017)[1] != original_files[name]:
            raise RuntimeError('Controller/document preimage differs.')
        temporary = path.with_name(path.name + '.r2-' + uuid.uuid4().hex)
        helper.candidate_exclusive015(core, temporary, files[name], mode); os.replace(temporary, path)
    for name in RUST_PATHS:
        if core.read_file(worktree / name, 2 * MIB017)[1] != original_files[name]:
            raise RuntimeError('A Rust payload changed during controller-only refresh.')
    print('WRITABLE017_CONTROLLER_REVISION002_INSTALLED', flush=True)
    return 0


def runtime017(repo):
    def load(name, path, expected):
        raw = bootstrap_read(path, 2 * MIB017)
        if hashlib.sha256(raw).hexdigest() != expected:
            raise RuntimeError('Retained reviewed controller differs: ' + path.name)
        module = types.ModuleType(name); module.__file__ = str(path); sys.modules[name] = module
        exec(compile(raw, str(path), 'exec'), module.__dict__)
        return module, raw
    prior015_path = repo / '.git/codex-repairs/leveldb-writablefile-shared015/runs' / PRIOR015_RUN / 'writable-file-controller015.py'
    old, raw = load('writable017_prior015', prior015_path, PRIOR015_CONTROLLER_SHA)
    diagnostic, _ = load('writable017_prior016', repo / '.git/codex-repairs/leveldb-writablefile-compile016/runs' / PRIOR016_RUN / 'compiler-diagnostic-controller016.py', PRIOR016_CONTROLLER_SHA)
    f, core = old.modules015(repo)
    helper, _ = load('writable017_git_helpers', prior015_path, PRIOR015_CONTROLLER_SHA)
    helper.WORKFLOW, helper.BRANCH, helper.SUFFIX = WORKFLOW, BRANCH, SUFFIX
    helper.REPLAY_PATH, helper.DOCUMENT_PATH, helper.RECEIPT_PATH = REPLAY_PATH, DOCUMENT_PATH, RECEIPT_PATH
    helper._w015_initial_paths = lambda: set(INITIAL_PATHS)
    core.WORKFLOW, core.BRANCH, core.WORKTREE_SUFFIX = WORKFLOW, BRANCH, SUFFIX
    return f, core, helper, old, diagnostic, raw


def reviewed_lineage017(core, old, diagnostic, repo, old_raw):
    prior = diagnostic.review_prior016(old, core, repo, old_raw)
    directory = repo / '.git/codex-repairs/leveldb-writablefile-compile016/runs' / PRIOR016_RUN
    path = directory / 'COMPILER_DIAGNOSTIC_RETURN.json'
    fp, raw = core.read_file(path, 2 * MIB017)
    if fp['sha256'] != PRIOR016_REPORT_SHA or fp['size'] != PRIOR016_REPORT_BYTES:
        raise RuntimeError('Compiler diagnostic report differs from the returned evidence.')
    report = json.loads(raw)
    if (report.get('outcome') != 'compiler_error_captured' or report.get('phase') != 'complete'
            or report.get('repair_qualified') is not False or report.get('native_tests_started') is not False
            or report.get('test_count') != 0 or report.get('master_updated') is not False
            or report.get('original_preserved') is not True or report.get('prior_attempt_reverified') is not True
            or report.get('controller_sha256') != PRIOR016_CONTROLLER_SHA):
        raise RuntimeError('Retained diagnostic is not the reviewed compiler-only failure.')
    evidence = report.get('cargo_evidence', {})
    errors = [item for item in evidence.get('priority_diagnostics', []) if item.get('level') == 'error']
    if (len(errors) != 1 or errors[0].get('code') != 'E0425'
            or errors[0].get('message') != 'cannot find function `some_file_overlaps_range` in this scope'
            or evidence.get('priority_records_omitted') != 0
            or evidence.get('cargo_exit_code') != 101 or evidence.get('build_finished') is not False):
        raise RuntimeError('Compiler failure has a different diagnostic or incomplete primary errors.')
    if report.get('source_capture', {}).get('source_omissions') != []:
        raise RuntimeError('Returned crate source capture is incomplete.')
    inv = core.Inventory(repo)
    for name, expected in BEFORE_BINDINGS.items():
        _, content = inv.git('cat-file', 'blob', BASE + ':' + name)
        if core.digest(content) != expected:
            raise RuntimeError('Reviewed committed source/dependency differs: ' + name)
    return {'failed_repair015': prior, 'compiler016_report_path': str(path),
            'compiler016_report_fingerprint': fp,
            'compiler016_controller_fingerprint': core.read_file(directory / 'compiler-diagnostic-controller016.py', 2 * MIB017)[0],
            'compiler_error': 'E0425: misplaced overlap tests call a higher-crate helper',
            'native_tests_started_in_predecessors': False,
            'before_bindings': BEFORE_BINDINGS}


def files017(raw):
    return {SOURCE_PATH: SOURCE_BYTES, SHARED_TEST_PATH: SHARED_TEST_BYTES,
            FIND_PATH: FIND_BYTES, OVERLAP_TEST_PATH: OVERLAP_TEST_BYTES,
            REPLAY_PATH: raw, DOCUMENT_PATH: DOCUMENT_BYTES}


def choose_cases017(lists):
    if set(lists) != set(TARGET_CASES): raise RuntimeError('Compiled target inventory differs from the four reviewed executables.')
    result = []
    for key, suffixes in TARGET_CASES.items():
        if len(lists[key]) != len(set(lists[key])): raise RuntimeError('Repeated test names in executable listing.')
        for suffix in suffixes:
            matches = [name for name in lists[key] if name == suffix or name.endswith('::' + suffix)]
            if len(matches) != 1: raise RuntimeError('Exact selected case is absent or ambiguous: ' + suffix)
            result.append({'artifact_key': key, 'name': matches[0], 'suffix': suffix,
                           'marker': 'SHARED_WRITABLE_FILE_CONTRACT_OK case=' + suffix if key.endswith('::writable_file_shared') else None})
    if len(result) != EXPECTED_TESTS: raise RuntimeError('Selected case count differs from nineteen.')
    return result


def artifact_check017(core, artifacts):
    for item in artifacts.values():
        if core.read_file(Path(item['path']), 256 * MIB017, content=False)[0] != item['fingerprint']:
            raise RuntimeError('A tested executable changed.')


def receipt017(core, report, files):
    return {'workflow': WORKFLOW, 'base': BASE, 'source_plan': {name: core.digest(raw) for name, raw in files.items()},
            'before_bindings': BEFORE_BINDINGS, 'lineage': report['lineage_before'],
            'relocation': RELOCATION, 'test_count': EXPECTED_TESTS,
            'cases': [{k: v for k, v in c.items() if k != 'log'} for c in report['cases']],
            'artifacts': report['artifacts'], 'build_commands': [FILE_COMMAND, OVERLAP_COMMAND],
            'controller_revision': CONTROLLER_REVISION, 'controller_transition': report['controller_stop_review_before'],
            'run_directory': report['run_directory'], 'compiler_failure_cause_established': True,
            'historical_sparsemerge_abort_cause_established': False,
            'full_coverage_claimed': False, 'full_randomized_stress': 'not rerun'}


def reuse017(f, core, helper, repo, state, files):
    helper.check_native_journal015(state)
    if state.get('phase') != 'complete' or not state.get('qualified_report'):
        raise RuntimeError('An incomplete repair017 attempt is retained for review; automatic rerunning is blocked.')
    path = Path(state['qualified_report'])
    if (path.name != 'WRITABLE_FILE_REPAIR_RETURN.json'
            or path.parent.parent != repo / '.git/codex-repairs' / WORKFLOW / 'runs'):
        raise RuntimeError('Qualified return is outside the owned workflow.')
    fp, raw = core.read_file(path, 8 * MIB017)
    if fp != state.get('qualified_report_fingerprint'): raise RuntimeError('Qualified return fingerprint differs.')
    old = json.loads(raw); receipt = state['receipt']
    if (old.get('outcome') != 'qualified_candidate' or old.get('repair_qualified') is not True
            or old.get('original_preserved') is not True or old.get('test_count') != EXPECTED_TESTS
            or len(old.get('cases', [])) != EXPECTED_TESTS or any(c.get('outcome') != 'passed' for c in old['cases'])
            or old.get('receipt') != receipt or old.get('artifacts') != receipt['artifacts']
            or old.get('build_commands') != receipt['build_commands']
            or [{k: v for k, v in c.items() if k != 'log'} for c in old['cases']] != receipt['cases']):
        raise RuntimeError('Retained qualification differs from its committed receipt.')
    expected = {(key, suffix) for key, suffixes in TARGET_CASES.items() for suffix in suffixes}
    observed = {(c.get('artifact_key'), c.get('label')) for c in old['cases']}
    if observed != expected or len(observed) != EXPECTED_TESTS: raise RuntimeError('Retained case inventory differs.')
    complete = dict(files); complete[RECEIPT_PATH] = core.encoded(receipt)
    helper.verify_candidate_commit(f, core, repo, state['head'], state['commit_intent'], complete)
    artifact_check017(core, old['artifacts'])
    for case in old['cases']:
        row = old['readouts'][case['label']]; executable = old['artifacts'][case['artifact_key']]['path']
        if row.get('command') != [executable, '--exact', case['expected_name'], '--nocapture', '--test-threads=1']:
            raise RuntimeError('Retained case command is not bound to its tested executable.')
        text = core.read_file(Path(row['log']), MIB017)[1].decode(errors='replace')
        if text != case['log'] or core.assess_case(row, text, case['expected_name'], case['marker'])['outcome'] != 'passed':
            raise RuntimeError('Retained exact case evidence does not verify.')
    return old


def run017(repo):
    report = {'workflow': WORKFLOW, 'controller_revision': CONTROLLER_REVISION, 'outcome': 'failed', 'phase': 'preflight', 'repair_qualified': False,
              'master_updated': False, 'other_refs_updated': False, 'builds_started': False,
              'native_tests_started': False, 'debuggers_started': False, 'validation_reused': False,
              'base': BASE, 'expected_test_count': EXPECTED_TESTS, 'test_count': 0, 'cases': [], 'readouts': {},
              'build_commands': [FILE_COMMAND, OVERLAP_COMMAND], 'relocation': RELOCATION,
              'compiler_failure_cause_established': True, 'historical_sparsemerge_abort_cause_established': False,
              'full_coverage_claimed': False, 'full_randomized_stress': 'not rerun',
              'postrun_transport': 'Replace the authorized return-buffer only after coordinator exit.',
              'preservation_scope': 'Original and predecessor work through coordinator exit; the later authorized buffer publication is separate.'}
    held, initial, directory, state, state_path, core, worktree, lineage, transition = [], None, None, None, None, None, None, None, None
    success = False
    try:
        repo = Path(os.path.abspath(Path(repo).expanduser())); report['repository'] = str(repo)
        f, core, helper, old, diagnostic, old_raw = runtime017(repo)
        core.checked_environment(); common = core.repository(core.Inventory(repo)); held = core.acquire_locks(common)
        own = common / 'codex-repairs' / WORKFLOW; own.mkdir(mode=0o700, exist_ok=True)
        fd = core.open_dir(own); os.close(fd)
        lock = os.open(own / 'run.lock', os.O_RDWR | os.O_CREAT | os.O_NOFOLLOW, 0o600); held.append((lock, own / 'run.lock'))
        if not stat.S_ISREG(os.fstat(lock).st_mode): raise RuntimeError('Workflow lock is not ordinary.')
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        worktree = repo.parent / (repo.name + SUFFIX)
        initial = core.normalized_original(core.protected_snapshot(core, repo), worktree)
        report['preservation_before_sha256'] = core.digest(core.encoded(initial)); report['locks_held'] = [str(p) for _, p in held]
        runs = own / 'runs'; runs.mkdir(mode=0o700, exist_ok=True); fd = core.open_dir(runs); os.close(fd)
        directory = runs / (dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%SZ') + '-' + uuid.uuid4().hex[:8]); directory.mkdir(mode=0o700)
        script = directory / 'writable-file-controller017.py'; script_raw = bootstrap_read(Path(__file__).absolute(), 2 * MIB017)
        core.exclusive(script, script_raw, 0o700); files = files017(script_raw)
        report.update(run_directory=str(directory), worktree=str(worktree), controller_sha256=core.digest(script_raw),
                      source_plan={name: core.digest(raw) for name, raw in files.items()}, guard_sha256=core.GUARD_SHA)
        lineage = reviewed_lineage017(core, old, diagnostic, repo, old_raw); report['lineage_before'] = lineage
        state_path = own / 'state.json'
        identity = {'workflow': WORKFLOW, 'base': BASE, 'branch': BRANCH, 'worktree': str(worktree),
                    'file_hashes': report['source_plan'], 'lineage_sha256': core.digest(core.encoded(lineage)), 'controller_revision': CONTROLLER_REVISION}
        if not os.path.lexists(state_path):
            raise RuntimeError('Revision002 requires the exact retained original017 attempt; no candidate was created.')
        state = json.loads(core.read_file(state_path, 8 * MIB017)[1])
        if state.get('identity') == identity:
            transition = state.get('controller_transition', {})
            verify_transition_archive017(core, repo, transition)
            report['controller_stop_review_before'] = transition
            previous = reuse017(f, core, helper, repo, state, files)
            report.update(outcome='already_qualified', phase='complete', repair_qualified=True, validation_reused=True,
                          test_count=previous['test_count'], cases=previous['cases'], readouts=previous['readouts'],
                          artifacts=previous['artifacts'], candidate_head=state['head'], candidate_tree=state['commit_intent']['tree'],
                          qualified_report=state['qualified_report'])
            success = True
        else:
            transition, previous_state_raw = review_r1_transition017(core, helper, repo, state_path)
            previous_state_path = directory / 'previous-state-before-controller-review.json'
            core.exclusive(previous_state_path, previous_state_raw)
            transition['previous_state_path'] = str(previous_state_path)
            verify_transition_archive017(core, repo, transition)
            report['controller_stop_review_before'] = transition
            state = {'identity': identity, 'phase': 'controller-refresh-pending', 'controller_transition': transition}
            core.atomic_json(state_path, state)
            inv = core.Inventory(repo)
            guard = core.load_guard().Guard(directory / 'guard', script, dict(inv.env, BITCOIN_RS_WRITABLE017_REPO=str(repo)))
            def preserve():
                if core.normalized_original(core.protected_snapshot(core, repo), worktree) != initial:
                    raise RuntimeError('Protected original/predecessor state changed.')
                if reviewed_lineage017(core, old, diagnostic, repo, old_raw) != lineage:
                    raise RuntimeError('Retained predecessor evidence changed.')
                verify_transition_archive017(core, repo, transition)
            def phase(label, command, kind='source', cwd=None, env=None):
                preserve(); report['phase'] = label; print('Protected writable-file repair017 phase: ' + label, flush=True)
                row = guard.run(label, command, cwd or repo, env or {}, kind=kind); report['readouts'][label] = row
                core.require_guard(row); return row
            def git(label, args, cwd, extra=None, tag=None):
                tag = tag or label; spec, out = directory / (tag + '.git.spec.json'), directory / (tag + '.git.json')
                value = {'label': label, 'args': args, 'cwd': str(cwd), 'output': str(out)}; value.update(extra or {})
                core.exclusive(spec, core.encoded(value)); phase('git-' + tag, [sys.executable, '-I', '-S', str(script), '--git', str(repo), str(spec)], 'build', cwd)
                evidence = json.loads(core.read_file(out, 4 * MIB017)[1]); report.setdefault('git_evidence', {})[tag] = evidence
                if evidence.get('exit_code') != 0 or evidence.get('args') != args: raise RuntimeError('Exact Git operation evidence differs.')
                return evidence['stdout'].strip()
            phase('preflight', [sys.executable, '-I', '-S', '-c', "print('GUARD_PREFLIGHT_OK')"], 'probe')
            identity_spec, identity_out = directory / 'identity-input.json', directory / 'identity.json'
            core.exclusive(identity_spec, core.encoded({key: os.environ[key] for key in f.IDENTITY_ENV if key in os.environ}))
            phase('read-configured-identity', [sys.executable, '-I', '-S', str(script), '--identity', str(repo), str(identity_spec), str(identity_out)])
            ident = json.loads(core.read_file(identity_out, MIB017)[1]); report['configured_identity'] = ident
            identity_env = {'GIT_' + role.upper() + '_' + field.upper(): ident[role][field.lower()] for role in ('author', 'committer') for field in ('NAME', 'EMAIL')}
            phase('refresh-controller', [sys.executable, '-I', '-S', str(script), '--refresh-controller', str(repo)], 'source', worktree)
            git('add', ['add', '--', *sorted(files)], worktree)
            helper.check_candidate(core, repo, worktree, files, BASE)
            state['phase'] = 'controller-updated'; core.atomic_json(state_path, state)
            target = own / 'cargo-target'; fd = core.open_dir(target); os.close(fd)
            report['target_directory'] = str(target)
            report['existing_private_cache_reused'] = True
            report['cache_copy_started'] = False
            compiled = {}
            for label, command in (('file', FILE_COMMAND), ('overlap', OVERLAP_COMMAND)):
                spec, evidence = directory / (label + '.cargo.spec.json'), directory / (label + '.cargo-artifacts.json')
                core.exclusive(spec, core.encoded({'command': command, 'cwd': str(worktree), 'target_directory': str(target), 'evidence_path': str(evidence)}))
                nix = ['nix-shell', '--option', 'max-jobs', '0', '--option', 'builders', '', '--option', 'substitute', 'false', str(worktree / 'shell.nix'), '--run', shlex.join([sys.executable, '-I', '-S', str(script), '--cargo-observer', str(spec)])]
                report['builds_started'] = True
                try: phase('compile-' + label + '-tests', nix, 'build', worktree, {'CARGO_TARGET_DIR': str(target), 'CARGO_BUILD_JOBS': '1', 'CARGO_INCREMENTAL': '0'})
                finally:
                    if evidence.exists(): report.setdefault('cargo_evidence', {})[label] = json.loads(core.read_file(evidence, MIB017)[1])
                current = report['cargo_evidence'][label]['artifacts']
                if set(compiled) & set(current): raise RuntimeError('Duplicate compiled package/target identity.')
                compiled.update(current); helper.check_candidate(core, repo, worktree, files, BASE)
            if set(compiled) != set(TARGET_CASES): raise RuntimeError('Builds did not produce exactly the four selected executables.')
            artifacts, lists = {}, {}
            for key, item in compiled.items():
                path = Path(item['executable']); fp = core.read_file(path, 256 * MIB017, content=False)[0]
                artifacts[key] = {'path': str(path), 'fingerprint': fp}
                row = phase('list-' + item['target_name'], [str(path), '--list'], 'case', worktree, {'RUST_LOG': 'off'})
                text = core.read_file(Path(row['log']), MIB017)[1].decode(errors='replace')
                lists[key] = [line[:-6] for line in text.splitlines() if line.endswith(': test')]
            report['artifacts'] = artifacts; selected = choose_cases017(lists)
            for item in selected:
                preserve(); helper.check_candidate(core, repo, worktree, files, BASE); artifact_check017(core, artifacts)
                label = item['suffix']; state['native_pending'] = {'run': str(directory), 'case': item['name'], 'artifact_key': item['artifact_key']}; core.atomic_json(state_path, state)
                report.update(phase=label, native_tests_started=True); print('Protected writable-file repair017 phase: ' + label, flush=True)
                command = [artifacts[item['artifact_key']]['path'], '--exact', item['name'], '--nocapture', '--test-threads=1']
                row = guard.run(label, command, worktree, {'RUST_LOG': 'off', 'RUST_BACKTRACE': '0'}, kind='case', tmpdir=directory / ('case-data-' + label)); report['readouts'][label] = row
                text = core.read_file(Path(row['log']), MIB017)[1].decode(errors='replace') if row.get('log') else ''
                assessed = core.assess_case(row, text, item['name'], item['marker']); report['cases'].append(dict(assessed, label=label, artifact_key=item['artifact_key'], log=text)); report['test_count'] = len(report['cases'])
                if assessed['outcome'] != 'passed':
                    state['native_failure'] = dict(state['native_pending']); core.atomic_json(state_path, state)
                    raise RuntimeError('Selected native case failed; failed evidence is retained and automatic qualification is blocked.')
                state.pop('native_pending'); core.atomic_json(state_path, state)
            helper.check_native_journal015(state)
            if report['test_count'] != EXPECTED_TESTS: raise RuntimeError('Successful selected case count differs.')
            preserve(); helper.check_candidate(core, repo, worktree, files, BASE); artifact_check017(core, artifacts)
            receipt = receipt017(core, report, files); receipt_path = worktree / RECEIPT_PATH
            receipt_path.parent.mkdir(parents=True, exist_ok=True); fd = core.open_dir(receipt_path.parent); os.close(fd)
            helper.candidate_exclusive015(core, receipt_path, core.encoded(receipt), 0o644); files[RECEIPT_PATH] = core.encoded(receipt)
            git('add', ['add', '--', *sorted(files)], worktree, tag='add-final'); helper.check_candidate(core, repo, worktree, files, BASE)
            tree = git('tree', ['write-tree'], worktree)
            if not helper._w015_oid(tree): raise RuntimeError('Candidate tree identity is absent.')
            message_bytes = b'fix: forward shared WritableFile handles and place overlap tests at their owner\n\nPreserve shared identity, statuses, name and explicit close semantics.\nMove the unchanged six-case overlap specification along the existing dependency direction.\n'
            message = directory / 'commit-message.txt'; core.exclusive(message, message_bytes)
            state.update(phase='commit-intent', receipt=receipt, commit_intent={'tree': tree, 'parents': [BASE], 'identity': ident, 'message_sha256': core.digest(message_bytes)}); core.atomic_json(state_path, state)
            head = git('commit-tree', ['commit-tree', tree, '-p', BASE, '-F', str(message)], worktree, {'tree': tree, 'identity_env': identity_env})
            if not helper._w015_oid(head): raise RuntimeError('Candidate commit identity is absent.')
            state['created_commit'] = head; core.atomic_json(state_path, state)
            git('head', ['update-ref', 'refs/heads/' + BRANCH, head, BASE], worktree, {'head': head})
            helper.verify_candidate_commit(f, core, repo, head, state['commit_intent'], files); artifact_check017(core, artifacts); preserve()
            state['head'] = head; report.update(outcome='qualified_candidate', phase='complete', repair_qualified=True, candidate_head=head, candidate_tree=tree, changed_paths=sorted(files), receipt=receipt); success = True
    except BaseException as error:
        report.update(outcome='failed', repair_qualified=False, error=type(error).__name__ + ': ' + str(error))
    finally:
        if initial is not None:
            try:
                after = core.normalized_original(core.protected_snapshot(core, repo), worktree); report['preservation_after_sha256'] = core.digest(core.encoded(after)); report['original_preserved'] = after == initial
                if after != initial: raise RuntimeError('Protected original/predecessor state changed.')
                if lineage is not None:
                    report['lineage_after'] = reviewed_lineage017(core, old, diagnostic, repo, old_raw)
                    if report['lineage_after'] != lineage: raise RuntimeError('Retained predecessor evidence changed.')
                if transition is not None:
                    report['controller_stop_review_reverified'] = verify_transition_archive017(core, repo, transition)
            except BaseException as error:
                success = False; report.update(outcome='failed', repair_qualified=False, preservation_error=str(error))
        if directory is not None:
            report['return_file'] = str(directory / 'WRITABLE_FILE_REPAIR_RETURN.json')
            try:
                core.atomic_json(Path(report['return_file']), report)
                if success and report['outcome'] == 'qualified_candidate':
                    state.update(phase='complete', qualified_report=report['return_file'], qualified_report_fingerprint=core.read_file(Path(report['return_file']), 8 * MIB017)[0]); core.atomic_json(state_path, state)
            except BaseException as error:
                report.update(outcome='failed', repair_qualified=False, publication_error=str(error)); core.atomic_json(Path(report['return_file']), report)
        for fd, _ in held: os.close(fd)
    return report


def main017():
    sys.dont_write_bytecode = True; resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    if len(sys.argv) == 3 and sys.argv[1] == '--guard-child':
        plan = json.loads(bootstrap_read(Path(sys.argv[2]), MIB017)); _, core, _, _, _, _ = runtime017(Path(plan['env']['BITCOIN_RS_WRITABLE017_REPO']))
        return core.load_guard().guard_child(Path(sys.argv[2]))
    if len(sys.argv) == 5 and sys.argv[1] == '--identity':
        f, _, _, _, _, _ = runtime017(Path(sys.argv[2])); return f.identity_worker(Path(sys.argv[2]), Path(sys.argv[3]), Path(sys.argv[4]))
    if len(sys.argv) == 4 and sys.argv[1] == '--git':
        _, core, helper, _, _, _ = runtime017(Path(sys.argv[2])); return helper.run_git_operation(core, Path(sys.argv[2]), Path(sys.argv[3]))
    if len(sys.argv) == 3 and sys.argv[1] == '--refresh-controller':
        repo = Path(sys.argv[2]); _, core, helper, _, _, _ = runtime017(repo); return refresh_controller017(core, helper, repo)
    if len(sys.argv) == 3 and sys.argv[1] == '--cargo-observer': return cargo_observer(json.loads(bootstrap_read(Path(sys.argv[2]), MIB017)))
    if len(sys.argv) == 4 and sys.argv[1] == '--coordinator' and sys.argv[3] == 'apply':
        resource.setrlimit(resource.RLIMIT_AS, (768 * MIB017,) * 2); signal.signal(signal.SIGALRM, lambda *_: (_ for _ in ()).throw(RuntimeError('Repair017 coordinator deadline reached.'))); signal.alarm(3600)
        report = run017(Path(sys.argv[2])); print('BEGIN_WRITABLE_FILE_REPAIR_RETURN', flush=True); print(json.dumps(report, indent=2), flush=True); print('END_WRITABLE_FILE_REPAIR_RETURN', flush=True)
        return 0 if report['outcome'] in {'qualified_candidate', 'already_qualified'} else 1
    parser = argparse.ArgumentParser(description=__doc__); parser.add_argument('--repo', type=Path, default=Path.home() / 'bitcoin-rs'); args = parser.parse_args()
    return run_with_transport(Path(__file__).absolute(), Path(os.path.abspath(args.repo.expanduser())), True)


if __name__ == '__main__': raise SystemExit(main017())
