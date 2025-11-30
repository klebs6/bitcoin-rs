// ---------------- [ File: bitcoinleveldb-log/tests/basic.rs ]
use bitcoinleveldb_log::*;
use bitcoinleveldb_rand::*;
use bitcoinleveldb_slice::*;
use bitcoinleveldb_status::*;
use bitcoinleveldb_file::*;
use bitcoinleveldb_crc32::*;
use bitcoinleveldb_coding::*;
use bitcoin_support::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_test.cc]

const K_BLOCK_SIZE: i32 = LOG_BLOCK_SIZE;
const K_HEADER_SIZE: i32 = LOG_HEADER_SIZE;

const K_MIDDLE_TYPE: u8 = LogRecordType::Middle as u8;
const K_LAST_TYPE:   u8 = LogRecordType::Last as u8;
const K_FIRST_TYPE:  u8 = LogRecordType::First as u8;
const K_FULL_TYPE:   u8 = LogRecordType::Full as u8;

fn big_string(partial_string: &String, n: usize) -> String {
    trace!(
        "big_string: partial_len={} target_len={}",
        partial_string.len(),
        n
    );

    let mut result = String::new();
    while result.len() < n {
        result.push_str(partial_string);
    }
    result.truncate(n);
    result
}

fn number_string(n: i32) -> String {
    trace!("number_string: n={}", n);
    format!("{}.", n)
}

fn random_skewed_string(i: i32, rnd: *mut Random) -> String {
    trace!("random_skewed_string: i={} rnd_ptr={:?}", i, rnd);

    if rnd.is_null() {
        error!("random_skewed_string: null Random pointer");
        return number_string(i);
    }

    unsafe {
        let skew = (*rnd).skewed(17) as usize;
        trace!(
            "random_skewed_string: i={} skewed_len={} (max_log=17)",
            i,
            skew
        );
        big_string(&number_string(i), skew)
    }
}

mod log_test {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Default, Getters, MutGetters)]
    #[getset(get = "pub", get_mut = "pub")]
    pub struct StringDest {
        contents: Vec<u8>,
    }

    impl StringDest {
        pub fn recorded_bytes(&self) -> &[u8] {
            self.contents()
        }
    }

    impl WritableFile for StringDest {}

    impl WritableFileClose for StringDest {
        fn close(&mut self) -> Status {
            trace!("StringDest::close");
            Status::ok()
        }
    }

    impl WritableFileFlush for StringDest {
        fn flush(&mut self) -> Status {
            trace!("StringDest::flush");
            Status::ok()
        }
    }

    impl WritableFileSync for StringDest {
        fn sync(&mut self) -> Status {
            trace!("StringDest::sync");
            Status::ok()
        }
    }

    impl WritableFileAppend for StringDest {
        fn append(&mut self, slice: &Slice) -> Status {
            unsafe {
                let data_ptr_ptr = slice.data();
                if data_ptr_ptr.is_null() {
                    debug!("StringDest::append: null outer data ptr");
                    return Status::ok();
                }
                let data_ptr = *data_ptr_ptr;
                if data_ptr.is_null() {
                    debug!("StringDest::append: null inner data ptr");
                    return Status::ok();
                }
                let len = *slice.size();
                let src = core::slice::from_raw_parts(data_ptr, len);
                trace!(
                    "StringDest::append: appending {} bytes (prev_len={})",
                    len,
                    self.contents.len()
                );
                self.contents.extend_from_slice(src);
            }
            Status::ok()
        }
    }

    impl Named for StringDest {
        fn name(&self) -> Cow<'_,str> {
            Cow::Owned("log_test_string_dest".to_string())
        }
    }

    #[derive(Default)]
    pub struct StringSource {
        contents:         Vec<u8>,
        read_offset:      usize,
        pub force_error:  bool,
        pub returned_partial: bool,
    }

    impl StringSource {
        pub fn new(contents: Vec<u8>, force_error: bool) -> Self {
            trace!(
                "StringSource::new: contents_len={} force_error={}",
                contents.len(),
                force_error
            );
            Self {
                contents,
                read_offset: 0,
                force_error,
                returned_partial: false,
            }
        }
    }

    impl SequentialFile for StringSource {}

    impl SequentialFileRead for StringSource {
        fn read(
            &mut self,
            n: usize,
            result: *mut Slice,
            _scratch: *mut u8,
        ) -> Status {
            trace!(
                "StringSource::read: requested_n={} read_offset={} contents_len={} force_error={} returned_partial={}",
                n,
                self.read_offset,
                self.contents.len(),
                self.force_error,
                self.returned_partial
            );

            assert!(
                !self.returned_partial,
                "StringSource::read: must not Read() after eof/error"
            );

            if self.force_error {
                self.force_error = false;
                self.returned_partial = true;

                let msg = "read error".to_owned();
                let msg_slice = Slice::from(&msg);

                warn!("StringSource::read: injecting corruption 'read error'");
                return Status::corruption(&msg_slice, None);
            }

            let available = self.contents.len().saturating_sub(self.read_offset);
            let mut actual_n = n;

            if available < n {
                actual_n = available;
                self.returned_partial = true;
                debug!(
                    "StringSource::read: partial read {} (< {}), marking returned_partial",
                    actual_n,
                    n
                );
            }

            let mut out_slice = Slice::default();

            if actual_n > 0 {
                unsafe {
                    let ptr = self.contents.as_ptr().add(self.read_offset);
                    out_slice = Slice::from_ptr_len(ptr, actual_n);
                }
            }

            if !result.is_null() {
                unsafe {
                    *result = out_slice;
                }
            }

            self.read_offset = self.read_offset.saturating_add(actual_n);

            trace!(
                "StringSource::read: returning {} bytes, new_offset={}",
                actual_n,
                self.read_offset
            );

            Status::ok()
        }
    }

    impl SequentialFileSkip for StringSource {
        fn skip(&mut self, n: u64) -> Status {
            trace!(
                "StringSource::skip: n={} read_offset={} contents_len={}",
                n,
                self.read_offset,
                self.contents.len()
            );

            let n_usize = n as usize;
            let available = self.contents.len().saturating_sub(self.read_offset);

            if n_usize > available {
                self.read_offset = self.contents.len();

                let msg = "in-memory file skipped past end".to_owned();
                let msg_slice = Slice::from(&msg);

                warn!(
                    "StringSource::skip: skipped past end (n={} available={}), returning NotFound",
                    n_usize,
                    available
                );
                return Status::not_found(&msg_slice, None);
            }

            self.read_offset = self.read_offset.saturating_add(n_usize);

            trace!(
                "StringSource::skip: new read_offset={}",
                self.read_offset
            );

            Status::ok()
        }
    }

    impl Named for StringSource {
        fn name(&self) -> Cow<'_,str> {
            Cow::Owned("log_test_string_source".to_string())
        }
    }

    #[derive(Default, Getters, MutGetters)]
    #[getset(get = "pub", get_mut = "pub")]
    pub struct ReportCollector {
        dropped_bytes: usize,
        message:       String,
    }

    impl ReportCollector {
        pub fn corruption(&mut self, bytes: usize, status: &Status) {
            self.dropped_bytes = self.dropped_bytes.saturating_add(bytes);
            let msg = status.to_string();
            trace!(
                "ReportCollector::corruption: bytes={} status={}",
                bytes,
                msg
            );
            self.message.push_str(&msg);
        }
    }

    pub struct SharedReport {
        shared: Rc<RefCell<ReportCollector>>,
    }

    impl SharedReport {
        pub fn new(shared: Rc<RefCell<ReportCollector>>) -> Self {
            Self { shared }
        }
    }

    impl LogReaderReporter for SharedReport {
        fn corruption(&mut self, bytes: usize, status: &Status) {
            trace!(
                "SharedReport::corruption: forwarding {} bytes",
                bytes
            );
            let mut rc = self.shared.borrow_mut();
            rc.corruption(bytes, status);
        }
    }

    pub const INITIAL_OFFSET_RECORD_SIZES: [usize; 6] = {
        let bs = K_BLOCK_SIZE as usize;
        let hs = K_HEADER_SIZE as usize;
        [
            10000,
            10000,
            2 * bs - 1000,
            1,
            13716,
            bs - hs,
        ]
    };

    pub const INITIAL_OFFSET_LAST_RECORD_OFFSETS: [u64; 6] = {
        let bs = K_BLOCK_SIZE as u64;
        let hs = K_HEADER_SIZE as u64;

        [
            0,
            hs + 10000,
            2 * (hs + 10000),
            2 * (hs + 10000) + (2 * bs - 1000) + 3 * hs,
            2 * (hs + 10000) + (2 * bs - 1000) + 3 * hs + hs + 1,
            3 * bs,
        ]
    };

    pub const NUM_INITIAL_OFFSET_RECORDS: usize = INITIAL_OFFSET_LAST_RECORD_OFFSETS.len();
}

struct LogTest {
    dest:                log_test::StringDest,
    report:              std::rc::Rc<std::cell::RefCell<log_test::ReportCollector>>,
    reading:             bool,
    reader:              Option<LogReader>,
    initial_offset:      u64,
    force_error_next_read: bool,
}

impl Default for LogTest {
    fn default() -> Self {
        trace!("LogTest::default: constructing LogTest");

        let dest = log_test::StringDest::default();
        let report =
            std::rc::Rc::new(std::cell::RefCell::new(log_test::ReportCollector::default()));

        Self {
            dest,
            report,
            reading: false,
            reader: None,
            initial_offset: 0,
            force_error_next_read: false,
        }
    }
}

impl Drop for LogTest {
    fn drop(&mut self) {
        trace!("LogTest::drop: tearing down LogTest");
    }
}

impl LogTest {
    fn append_record_bytes(&mut self, record: &[u8]) {
        let block_size = K_BLOCK_SIZE as usize;
        let header_size = K_HEADER_SIZE as usize;

        let mut block_offset = self.dest.contents().len() % block_size;
        let mut remaining = record.len();
        let mut pos = 0usize;
        let mut begin = true;

        trace!(
            "LogTest::append_record_bytes: record_len={} initial_block_offset={}",
            record.len(),
            block_offset
        );

        loop {
            if block_size - block_offset < header_size {
                let leftover = block_size - block_offset;
                if leftover > 0 {
                    debug!(
                        "LogTest::append_record_bytes: writing {} bytes of trailer padding",
                        leftover
                    );
                    self.dest
                        .contents_mut()
                        .extend(core::iter::repeat(0u8).take(leftover));
                }
                block_offset = 0;
            }

            let avail = block_size - block_offset - header_size;
            let fragment_length = if remaining < avail {
                remaining
            } else {
                avail
            };

            let end = remaining == fragment_length;

            let record_type = match (begin, end) {
                (true, true) => LogRecordType::Full,
                (true, false) => LogRecordType::First,
                (false, true) => LogRecordType::Last,
                (false, false) => LogRecordType::Middle,
            };

            let type_byte = record_type as u8;

            let frag = if fragment_length > 0 {
                &record[pos..pos + fragment_length]
            } else {
                &record[0..0]
            };

            let mut header = [0u8; LOG_HEADER_SIZE as usize];

            header[4] = (fragment_length & 0xff) as u8;
            header[5] = ((fragment_length >> 8) & 0xff) as u8;
            header[6] = type_byte;

            let crc = unsafe {
                let mut crc0 = 0u32;
                let type_crc = crc32c_value((&type_byte as *const u8), 1);
                crc0 = type_crc;
                let extended = if !frag.is_empty() {
                    crc32c_extend(crc0, frag.as_ptr(), frag.len())
                } else {
                    crc0
                };
                crc32c_mask(extended)
            };

            unsafe {
                encode_fixed32(header.as_mut_ptr(), crc);
            }

            trace!(
                "LogTest::append_record_bytes: emitting fragment_len={} type={:?} block_offset={}",
                fragment_length,
                record_type,
                block_offset
            );

            self.dest.contents_mut().extend_from_slice(&header);
            self.dest.contents_mut().extend_from_slice(frag);

            block_offset += header_size + fragment_length;
            pos += fragment_length;
            remaining -= fragment_length;
            begin = false;

            if end {
                break;
            }
        }
    }

    pub fn reopen_for_append(&mut self) {
        trace!("LogTest::reopen_for_append: no-op (writer is stateless)");
    }

    pub fn write(&mut self, msg: &String) {
        trace!("LogTest::write: len={}", msg.len());
        self.append_record_bytes(msg.as_bytes());
    }

    pub fn written_bytes(&self) -> usize {
        let len = self.dest.contents().len();
        trace!("LogTest::written_bytes: {}", len);
        len
    }

    pub fn read(&mut self) -> String {
        trace!(
            "LogTest::read: enter reading={} initial_offset={}",
            self.reading,
            self.initial_offset
        );

        if !self.reading {
            self.reading = true;

            let contents    = self.dest.contents().clone();
            let force_error = self.force_error_next_read;
            self.force_error_next_read = false;

            let file     = log_test::StringSource::new(contents, force_error);
            let reporter = log_test::SharedReport::new(self.report.clone());

            let reader = LogReader::new(
                Box::new(file),
                Box::new(reporter),
                true,
                self.initial_offset,
            );

            self.reader = Some(reader);
            trace!("LogTest::read: reader created");
        }

        let reader = match self.reader.as_mut() {
            Some(r) => r,
            None => {
                error!("LogTest::read: reader missing, returning EOF");
                return "EOF".to_owned();
            }
        };

        let mut scratch: Vec<u8> = Vec::new();
        let mut record           = Slice::default();

        let ok = reader.read_record(
            &mut record,
            &mut scratch,
        );

        if !ok {
            debug!("LogTest::read: reader returned false -> EOF");
            return "EOF".to_owned();
        }

        unsafe {
            let data_ptr_ptr = record.data();
            if data_ptr_ptr.is_null() {
                debug!("LogTest::read: record has null outer pointer");
                return String::new();
            }
            let data_ptr = *data_ptr_ptr;
            if data_ptr.is_null() {
                debug!("LogTest::read: record has null inner pointer");
                return String::new();
            }
            let len   = *record.size();
            let bytes = core::slice::from_raw_parts(data_ptr, len);
            let s     = String::from_utf8_lossy(bytes).to_string();
            trace!("LogTest::read: returning record len={}", s.len());
            s
        }
    }

    pub fn increment_byte(&mut self, offset: i32, delta: i32) {
        let idx = offset as usize;
        if idx >= self.dest.contents().len() {
            warn!(
                "LogTest::increment_byte: offset {} out of range (len={})",
                idx,
                self.dest.contents().len()
            );
            return;
        }

        let v = self.dest.contents()[idx];
        let new_v = v.wrapping_add(delta as u8);
        trace!(
            "LogTest::increment_byte: offset={} old={} new={}",
            idx,
            v,
            new_v
        );
        self.dest.contents_mut()[idx] = new_v;
    }

    pub fn set_byte(&mut self, offset: i32, new_byte: u8) {
        let idx = offset as usize;
        if idx >= self.dest.contents().len() {
            warn!(
                "LogTest::set_byte: offset {} out of range (len={})",
                idx,
                self.dest.contents().len()
            );
            return;
        }

        trace!(
            "LogTest::set_byte: offset={} old={} new={}",
            idx,
            self.dest.contents()[idx],
            new_byte
        );
        self.dest.contents_mut()[idx] = new_byte;
    }

    pub fn shrink_size(&mut self, bytes: i32) {
        let bytes_usize = bytes as usize;
        let len = self.dest.contents().len();

        assert!(
            bytes_usize <= len,
            "LogTest::shrink_size: cannot shrink {} bytes from len {}",
            bytes_usize,
            len
        );

        let new_len = len - bytes_usize;
        trace!(
            "LogTest::shrink_size: shrinking from {} to {}",
            len,
            new_len
        );
        self.dest.contents_mut().truncate(new_len);
    }

    pub fn fix_checksum(&mut self, header_offset: i32, len: i32) {
        let header_off = header_offset as usize;
        let data_len = len as usize;

        let needed = header_off
            .saturating_add(6)
            .saturating_add(1 + data_len);
        assert!(
            needed <= self.dest.contents().len(),
            "LogTest::fix_checksum: header/data out of range"
        );

        let crc = unsafe {
            let ptr = self
                .dest
                .contents()
                .as_ptr()
                .add(header_off + 6);
            crc32c_value(ptr, 1 + data_len)
        };

        let masked = unsafe { crc32c_mask(crc) };

        trace!(
            "LogTest::fix_checksum: header_offset={} len={} crc={:#010x} masked={:#010x}",
            header_offset,
            len,
            crc,
            masked
        );

        unsafe {
            let header_ptr = self
                .dest
                .contents_mut()
                .as_mut_ptr()
                .add(header_off);
            encode_fixed32(header_ptr, masked);
        }
    }

    pub fn force_error(&mut self) {
        trace!("LogTest::force_error: next read will see injected error");
        self.force_error_next_read = true;
    }

    pub fn dropped_bytes(&self) -> usize {
        let report = self.report.borrow();
        let dropped = report.dropped_bytes();
        trace!("LogTest::dropped_bytes: {}", dropped);
        *dropped
    }

    pub fn report_message(&self) -> String {
        let msg = self.report.borrow().message().clone();
        trace!("LogTest::report_message: {}", msg);
        msg
    }

    pub fn match_error(&self, msg: &String) -> String {
        let full = self.report_message();
        if full.contains(msg) {
            "OK".to_owned()
        } else {
            full
        }
    }

    pub fn write_initial_offset_log(&mut self) {
        trace!("LogTest::write_initial_offset_log: enter");
        for (i, sz) in log_test::INITIAL_OFFSET_RECORD_SIZES
            .iter()
            .enumerate()
        {
            let ch = (b'a' + (i as u8)) as char;
            let record: String = core::iter::repeat(ch).take(*sz).collect();
            trace!(
                "LogTest::write_initial_offset_log: i={} size={}",
                i,
                sz
            );
            self.write(&record);
        }
    }

    pub fn start_reading_at(&mut self, initial_offset: u64) {
        trace!(
            "LogTest::start_reading_at: from {} to {}",
            self.initial_offset,
            initial_offset
        );
        self.initial_offset = initial_offset;
        self.reading = false;
        self.reader = None;
    }

    pub fn check_offset_past_end_returns_no_records(
        &mut self,
        offset_past_end: u64
    ) {
        trace!(
            "LogTest::check_offset_past_end_returns_no_records: offset_past_end={}",
            offset_past_end
        );

        self.write_initial_offset_log();

        let contents = self.dest.contents().clone();
        let file     = log_test::StringSource::new(contents, false);
        let reporter = log_test::SharedReport::new(self.report.clone());

        let mut reader = LogReader::new(
            Box::new(file),
            Box::new(reporter),
            true,
            self.written_bytes() as u64 + offset_past_end,
        );

        let mut scratch: Vec<u8> = Vec::new();
        let mut record           = Slice::default();

        let ok = reader.read_record(
            &mut record,
            &mut scratch,
        );

        assert!(
            !ok,
            "expected no records when starting at offset past end"
        );
    }

    pub fn check_initial_offset_record(
        &mut self,
        initial_offset:         u64,
        mut expected_record_offset: i32,
    ) {
        trace!(
            "LogTest::check_initial_offset_record: initial_offset={} expected_record_offset={}",
            initial_offset,
            expected_record_offset
        );

        self.write_initial_offset_log();

        let contents = self.dest.contents().clone();
        let file     = log_test::StringSource::new(contents, false);
        let reporter = log_test::SharedReport::new(self.report.clone());

        let mut reader = LogReader::new(
            Box::new(file),
            Box::new(reporter),
            true,
            initial_offset,
        );

        assert!(
            expected_record_offset >= 0
                && (expected_record_offset as usize)
                    < log_test::NUM_INITIAL_OFFSET_RECORDS,
            "expected_record_offset out of range"
        );

        while (expected_record_offset as usize) < log_test::NUM_INITIAL_OFFSET_RECORDS {
            let idx = expected_record_offset as usize;

            let expected_size        = log_test::INITIAL_OFFSET_RECORD_SIZES[idx];
            let expected_last_offset =
                log_test::INITIAL_OFFSET_LAST_RECORD_OFFSETS[idx];

            let mut scratch: Vec<u8> = Vec::new();
            let mut record           = Slice::default();

            let ok = reader.read_record(
                &mut record,
                &mut scratch,
            );
            assert!(ok, "expected record at index {}", idx);

            unsafe {
                let size = *record.size();
                assert_eq!(
                    expected_size, size,
                    "record size mismatch at index {}",
                    idx
                );

                let last_offset = *reader.last_record_offset();
                assert_eq!(
                    expected_last_offset, last_offset,
                    "last_record_offset mismatch at index {}",
                    idx
                );

                let data_ptr_ptr = record.data();
                assert!(
                    !data_ptr_ptr.is_null(),
                    "record data outer pointer null at index {}",
                    idx
                );
                let data_ptr = *data_ptr_ptr;
                assert!(
                    !data_ptr.is_null(),
                    "record data inner pointer null at index {}",
                    idx
                );
                let bytes     = core::slice::from_raw_parts(data_ptr, size);
                let first_ch  = bytes[0] as char;
                let expected_ch = (b'a' + (idx as u8)) as char;
                assert_eq!(
                    expected_ch, first_ch,
                    "first char mismatch at index {}",
                    idx
                );
            }

            expected_record_offset += 1;
        }
    }
}

#[traced_test]
fn log_test_empty() {
    let mut t = LogTest::default();
    assert_eq!("EOF", t.read());
}

#[traced_test]
fn log_test_read_write() {
    let mut t = LogTest::default();

    t.write(&"foo".to_owned());
    t.write(&"bar".to_owned());
    t.write(&"".to_owned());
    t.write(&"xxxx".to_owned());

    assert_eq!("foo", t.read());
    assert_eq!("bar", t.read());
    assert_eq!("", t.read());
    assert_eq!("xxxx", t.read());
    assert_eq!("EOF", t.read());
    assert_eq!("EOF", t.read());
}

#[traced_test]
fn log_test_many_blocks() {
    let mut t = LogTest::default();

    for i in 0..100_000 {
        t.write(&number_string(i));
    }

    for i in 0..100_000 {
        assert_eq!(number_string(i), t.read());
    }

    assert_eq!("EOF", t.read());
}

#[traced_test]
fn log_test_fragmentation() {
    let mut t = LogTest::default();

    t.write(&"small".to_owned());
    t.write(&big_string(&"medium".to_owned(), 50_000));
    t.write(&big_string(&"large".to_owned(), 100_000));

    assert_eq!("small", t.read());
    assert_eq!(big_string(&"medium".to_owned(), 50_000), t.read());
    assert_eq!(big_string(&"large".to_owned(), 100_000), t.read());
    assert_eq!("EOF", t.read());
}

#[traced_test]
fn log_test_marginal_trailer() {
    let mut t = LogTest::default();

    let n = K_BLOCK_SIZE as usize - 2 * K_HEADER_SIZE as usize;
    t.write(&big_string(&"foo".to_owned(), n));
    assert_eq!(
        (K_BLOCK_SIZE - K_HEADER_SIZE) as usize,
        t.written_bytes()
    );
    t.write(&"".to_owned());
    t.write(&"bar".to_owned());

    assert_eq!(big_string(&"foo".to_owned(), n), t.read());
    assert_eq!("", t.read());
    assert_eq!("bar", t.read());
    assert_eq!("EOF", t.read());
}

#[traced_test]
fn log_test_marginal_trailer2() {
    let mut t = LogTest::default();

    let n = K_BLOCK_SIZE as usize - 2 * K_HEADER_SIZE as usize;
    t.write(&big_string(&"foo".to_owned(), n));
    assert_eq!(
        (K_BLOCK_SIZE - K_HEADER_SIZE) as usize,
        t.written_bytes()
    );
    t.write(&"bar".to_owned());

    assert_eq!(big_string(&"foo".to_owned(), n), t.read());
    assert_eq!("bar", t.read());
    assert_eq!("EOF", t.read());
    assert_eq!(0, t.dropped_bytes());
    assert_eq!("", t.report_message());
}

#[traced_test]
fn log_test_short_trailer() {
    let mut t = LogTest::default();

    let n = K_BLOCK_SIZE as usize - 2 * K_HEADER_SIZE as usize + 4;
    t.write(&big_string(&"foo".to_owned(), n));
    assert_eq!(
        (K_BLOCK_SIZE - K_HEADER_SIZE + 4) as usize,
        t.written_bytes()
    );
    t.write(&"".to_owned());
    t.write(&"bar".to_owned());

    assert_eq!(big_string(&"foo".to_owned(), n), t.read());
    assert_eq!("", t.read());
    assert_eq!("bar", t.read());
    assert_eq!("EOF", t.read());
}

#[traced_test]
fn log_test_aligned_eof() {
    let mut t = LogTest::default();

    let n = K_BLOCK_SIZE as usize - 2 * K_HEADER_SIZE as usize + 4;
    t.write(&big_string(&"foo".to_owned(), n));
    assert_eq!(
        (K_BLOCK_SIZE - K_HEADER_SIZE + 4) as usize,
        t.written_bytes()
    );
    assert_eq!(big_string(&"foo".to_owned(), n), t.read());
    assert_eq!("EOF", t.read());
}

#[traced_test]
fn log_test_open_for_append() {
    let mut t = LogTest::default();

    t.write(&"hello".to_owned());
    t.reopen_for_append();
    t.write(&"world".to_owned());

    assert_eq!("hello", t.read());
    assert_eq!("world", t.read());
    assert_eq!("EOF", t.read());
}

#[traced_test]
fn log_test_random_read() {
    let mut t = LogTest::default();

    let n = 500;
    let mut write_rnd = Random::new(301);
    for i in 0..n {
        let s = random_skewed_string(i, &mut write_rnd as *mut Random);
        t.write(&s);
    }

    let mut read_rnd = Random::new(301);
    for i in 0..n {
        let expected = random_skewed_string(i, &mut read_rnd as *mut Random);
        assert_eq!(expected, t.read());
    }

    assert_eq!("EOF", t.read());
}

#[traced_test]
fn log_test_read_error() {
    let mut t = LogTest::default();

    t.write(&"foo".to_owned());
    t.force_error();
    assert_eq!("EOF", t.read());
    assert_eq!(K_BLOCK_SIZE as usize, t.dropped_bytes());
    assert_eq!(
        "OK",
        t.match_error(&"read error".to_owned())
    );
}

#[traced_test]
fn log_test_bad_record_type() {
    let mut t = LogTest::default();

    t.write(&"foo".to_owned());
    t.increment_byte(6, 100);
    t.fix_checksum(0, 3);

    assert_eq!("EOF", t.read());
    assert_eq!(3, t.dropped_bytes());
    assert_eq!(
        "OK",
        t.match_error(&"unknown record type".to_owned())
    );
}

#[traced_test]
fn log_test_truncated_trailing_record_is_ignored() {
    let mut t = LogTest::default();

    t.write(&"foo".to_owned());
    t.shrink_size(4);
    assert_eq!("EOF", t.read());
    assert_eq!(0, t.dropped_bytes());
    assert_eq!("", t.report_message());
}

#[traced_test]
fn log_test_bad_length() {
    let mut t = LogTest::default();

    let payload_size = K_BLOCK_SIZE as usize - K_HEADER_SIZE as usize;
    t.write(&big_string(&"bar".to_owned(), payload_size));
    t.write(&"foo".to_owned());

    t.increment_byte(4, 1);
    assert_eq!("foo", t.read());
    assert_eq!(K_BLOCK_SIZE as usize, t.dropped_bytes());
    assert_eq!(
        "OK",
        t.match_error(&"bad record length".to_owned())
    );
}

#[traced_test]
fn log_test_bad_length_at_end_is_ignored() {
    let mut t = LogTest::default();

    t.write(&"foo".to_owned());
    t.shrink_size(1);

    assert_eq!("EOF", t.read());
    assert_eq!(0, t.dropped_bytes());
    assert_eq!("", t.report_message());
}

#[traced_test]
fn log_test_checksum_mismatch() {
    let mut t = LogTest::default();

    t.write(&"foo".to_owned());
    t.increment_byte(0, 10);

    assert_eq!("EOF", t.read());
    assert_eq!(10, t.dropped_bytes());
    assert_eq!(
        "OK",
        t.match_error(&"checksum mismatch".to_owned())
    );
}

#[traced_test]
fn log_test_unexpected_middle_type() {
    let mut t = LogTest::default();

    t.write(&"foo".to_owned());
    t.set_byte(6, K_MIDDLE_TYPE);
    t.fix_checksum(0, 3);

    assert_eq!("EOF", t.read());
    assert_eq!(3, t.dropped_bytes());
    assert_eq!(
        "OK",
        t.match_error(&"missing start".to_owned())
    );
}

#[traced_test]
fn log_test_unexpected_last_type() {
    let mut t = LogTest::default();

    t.write(&"foo".to_owned());
    t.set_byte(6, K_LAST_TYPE);
    t.fix_checksum(0, 3);

    assert_eq!("EOF", t.read());
    assert_eq!(3, t.dropped_bytes());
    assert_eq!(
        "OK",
        t.match_error(&"missing start".to_owned())
    );
}

#[traced_test]
fn log_test_unexpected_full_type() {
    let mut t = LogTest::default();

    t.write(&"foo".to_owned());
    t.write(&"bar".to_owned());
    t.set_byte(6, K_FIRST_TYPE);
    t.fix_checksum(0, 3);

    assert_eq!("bar", t.read());
    assert_eq!("EOF", t.read());
    assert_eq!(3, t.dropped_bytes());
    assert_eq!(
        "OK",
        t.match_error(&"partial record without end".to_owned())
    );
}

#[traced_test]
fn log_test_unexpected_first_type() {
    let mut t = LogTest::default();

    t.write(&"foo".to_owned());
    t.write(&big_string(&"bar".to_owned(), 100_000));
    t.set_byte(6, K_FIRST_TYPE);
    t.fix_checksum(0, 3);

    assert_eq!(
        big_string(&"bar".to_owned(), 100_000),
        t.read()
    );
    assert_eq!("EOF", t.read());
    assert_eq!(3, t.dropped_bytes());
    assert_eq!(
        "OK",
        t.match_error(&"partial record without end".to_owned())
    );
}

#[traced_test]
fn log_test_missing_last_is_ignored() {
    let mut t = LogTest::default();

    t.write(&big_string(&"bar".to_owned(), K_BLOCK_SIZE as usize));
    t.shrink_size(14);

    assert_eq!("EOF", t.read());
    assert_eq!("", t.report_message());
    assert_eq!(0, t.dropped_bytes());
}

#[traced_test]
fn log_test_partial_last_is_ignored() {
    let mut t = LogTest::default();

    t.write(&big_string(&"bar".to_owned(), K_BLOCK_SIZE as usize));
    t.shrink_size(1);

    assert_eq!("EOF", t.read());
    assert_eq!("", t.report_message());
    assert_eq!(0, t.dropped_bytes());
}

#[traced_test]
fn log_test_skip_into_multi_record() {
    let mut t = LogTest::default();

    t.write(&big_string(&"foo".to_owned(), 3 * K_BLOCK_SIZE as usize));
    t.write(&"correct".to_owned());
    t.start_reading_at(K_BLOCK_SIZE as u64);

    assert_eq!("correct", t.read());
    assert_eq!("", t.report_message());
    assert_eq!(0, t.dropped_bytes());
    assert_eq!("EOF", t.read());
}

#[traced_test]
fn log_test_error_joins_records() {
    let mut t = LogTest::default();

    t.write(&big_string(&"foo".to_owned(), K_BLOCK_SIZE as usize));
    t.write(&big_string(&"bar".to_owned(), K_BLOCK_SIZE as usize));
    t.write(&"correct".to_owned());

    for offset in K_BLOCK_SIZE..(2 * K_BLOCK_SIZE) {
        t.set_byte(offset, b'x');
    }

    assert_eq!("correct", t.read());
    assert_eq!("EOF", t.read());

    let dropped = t.dropped_bytes();
    assert!(
        dropped >= 2 * K_BLOCK_SIZE as usize
            && dropped <= 2 * K_BLOCK_SIZE as usize + 100,
        "dropped bytes {} not in expected range",
        dropped
    );
}

#[traced_test]
fn log_test_read_start() {
    let mut t = LogTest::default();
    t.check_initial_offset_record(0, 0);
}

#[traced_test]
fn log_test_read_second_one_off() {
    let mut t = LogTest::default();
    t.check_initial_offset_record(1, 1);
}

#[traced_test]
fn log_test_read_second_ten_thousand() {
    let mut t = LogTest::default();
    t.check_initial_offset_record(10_000, 1);
}

#[traced_test]
fn log_test_read_second_start() {
    let mut t = LogTest::default();
    t.check_initial_offset_record(10_007, 1);
}

#[traced_test]
fn log_test_read_third_one_off() {
    let mut t = LogTest::default();
    t.check_initial_offset_record(10_008, 2);
}

#[traced_test]
fn log_test_read_third_start() {
    let mut t = LogTest::default();
    t.check_initial_offset_record(20_014, 2);
}

#[traced_test]
fn log_test_read_fourth_one_off() {
    let mut t = LogTest::default();
    t.check_initial_offset_record(20_015, 3);
}

#[traced_test]
fn log_test_read_fourth_first_block_trailer() {
    let mut t = LogTest::default();
    t.check_initial_offset_record(
        (K_BLOCK_SIZE - 4) as u64,
        3,
    );
}

#[traced_test]
fn log_test_read_fourth_middle_block() {
    let mut t = LogTest::default();
    t.check_initial_offset_record(
        (K_BLOCK_SIZE + 1) as u64,
        3,
    );
}

#[traced_test]
fn log_test_read_fourth_last_block() {
    let mut t = LogTest::default();
    t.check_initial_offset_record(
        (2 * K_BLOCK_SIZE + 1) as u64,
        3,
    );
}

#[traced_test]
fn log_test_read_fourth_start() {
    let mut t = LogTest::default();
    let offset = 2 * (K_HEADER_SIZE + 1000)
        + (2 * K_BLOCK_SIZE - 1000)
        + 3 * K_HEADER_SIZE;
    t.check_initial_offset_record(offset as u64, 3);
}

#[traced_test]
fn log_test_read_initial_offset_into_block_padding() {
    let mut t = LogTest::default();
    t.check_initial_offset_record(
        (3 * K_BLOCK_SIZE - 3) as u64,
        5,
    );
}

#[traced_test]
fn log_test_read_end() {
    let mut t = LogTest::default();
    t.check_offset_past_end_returns_no_records(0);
}

#[traced_test]
fn log_test_read_past_end() {
    let mut t = LogTest::default();
    t.check_offset_past_end_returns_no_records(5);
}
