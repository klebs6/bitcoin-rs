// ---------------- [ File: bitcoinleveldb-log/src/test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/log_test.cc]

/**
  | Construct a string of the specified
  | length made out of the supplied partial
  | string.
  |
  */
fn big_string(
        partial_string: &String,
        n:              usize) -> String {
    
    todo!();
        /*
            std::string result;
      while (result.size() < n) {
        result.append(partial_string);
      }
      result.resize(n);
      return result;
        */
}

/**
  | Construct a string from a number
  |
  */
fn number_string(n: i32) -> String {
    
    todo!();
        /*
            char buf[50];
      snprintf(buf, sizeof(buf), "%d.", n);
      return std::string(buf);
        */
}

/**
  | Return a skewed potentially long string
  |
  */
fn random_skewed_string(
        i:   i32,
        rnd: *mut Random) -> String {
    
    todo!();
        /*
            return BigString(NumberString(i), rnd->Skewed(17));
        */
}

///------------------------
struct LogTest {
    dest:    log_test::StringDest,
    source:  log_test::StringSource,
    report:  log_test::ReportCollector,
    reading: bool,
    writer:  *mut LogWriter,
    reader:  *mut LogReader,
}

mod log_test {

    use super::*;

    ///------------------------
    pub struct StringDest {
        contents: String,
    }

    impl WritableFile for StringDest {

    }

    impl WritableFileClose for StringDest {
        
        fn close(&mut self) -> crate::Status {
            
            todo!();
            /*
                return Status::OK();
            */
        }
    }
        
    impl WritableFileFlush for StringDest {

        fn flush(&mut self) -> crate::Status {
            
            todo!();
            /*
                return Status::OK();
            */
        }
    }

    impl WritableFileSync for StringDest {
        
        fn sync(&mut self) -> crate::Status {
            
            todo!();
            /*
                return Status::OK();
            */
        }
    }
        
    impl WritableFileAppend for StringDest {

        fn append(&mut self, slice: &Slice) -> crate::Status {
            
            todo!();
            /*
                contents_.append(slice.data(), slice.size());
              return Status::OK();
            */
        }
    }
        
    impl GetName for StringDest {

        fn get_name(&self) -> &'static str {
            
            todo!();
            /*
                return "";
            */
        }
    }

    ///-----------------
    pub struct StringSource {
        contents:         Slice,
        force_error:      bool,
        returned_partial: bool,
    }

    impl SequentialFile for StringSource { }

    impl Default for StringSource {
        
        fn default() -> Self {
            todo!();
            /*
            : force_error(false),
            : returned_partial(false),

            
            */
        }
    }

    impl SequentialFileRead for StringSource {

        fn read(&mut self, 
            n:       usize,
            result:  *mut Slice,
            scratch: *mut u8) -> crate::Status {
            
            todo!();
            /*
                ASSERT_TRUE(!returned_partial_) << "must not Read() after eof/error";

              if (force_error_) {
                force_error_ = false;
                returned_partial_ = true;
                return Status::Corruption("read error");
              }

              if (contents_.size() < n) {
                n = contents_.size();
                returned_partial_ = true;
              }
              *result = Slice(contents_.data(), n);
              contents_.remove_prefix(n);
              return Status::OK();
            */
        }
    }

    impl SequentialFileSkip for StringSource {

        fn skip(&mut self, n: u64) -> crate::Status {
            
            todo!();
            /*
                if (n > contents_.size()) {
                contents_.clear();
                return Status::NotFound("in-memory file skipped past end");
              }

              contents_.remove_prefix(n);

              return Status::OK();
            */
        }
    }

    impl GetName for StringSource {

        fn get_name(&self) -> &'static str {
            
            todo!();
            /*
                return "";
            */
        }
    }

    ///-------------
    pub struct ReportCollector {
        dropped_bytes: usize,
        message:       String,
    }

    impl LogReaderReporter for ReportCollector {

        fn corruption(&mut self, 
            bytes:  usize,
            status: &Status)  {
            
            todo!();
            /*
                dropped_bytes_ += bytes;
              message_.append(status.ToString());
            */
        }
    }

    impl Default for ReportCollector {
        
        fn default() -> Self {
            todo!();
            /*
            : dropped_bytes(0),

            
            */
        }
    }

    lazy_static!{
        /*
        // Record metadata for testing initial offset functionality
                static size_t initial_offset_record_sizes_[];
                static uint64_t initial_offset_last_record_offsets_[];
                static int num_initial_offset_records_;

                size_t LogTest::initial_offset_record_sizes_[] = {
                    10000,  // Two sizable records in first block
                    10000,
                    2 * log::kBlockSize - 1000,  // Span three blocks
                    1,
                    13716,                          // Consume all but two bytes of block 3.
                    log::kBlockSize - kHeaderSize,  // Consume the entirety of block 4.
                };

                uint64_t LogTest::initial_offset_last_record_offsets_[] = {
                    0,
                    kHeaderSize + 10000,
                    2 * (kHeaderSize + 10000),
                    2 * (kHeaderSize + 10000) + (2 * log::kBlockSize - 1000) + 3 * kHeaderSize,
                    2 * (kHeaderSize + 10000) + (2 * log::kBlockSize - 1000) + 3 * kHeaderSize +
                        kHeaderSize + 1,
                    3 * log::kBlockSize,
                };

                // LogTest::initial_offset_last_record_offsets_ must be defined before this.
                int LogTest::num_initial_offset_records_ =
                    sizeof(LogTest::initial_offset_last_record_offsets_) / sizeof(uint64_t);
        */
    }
}

impl Default for LogTest {
    
    fn default() -> Self {
        todo!();
        /*


            : reading_(false),
            writer_(new LogWriter(&dest_)),
            reader_(new Reader(&source_, &report_, true /*checksum*/,
                               0 /*initial_offset*/))
        */
    }
}

impl Drop for LogTest {
    fn drop(&mut self) {
        todo!();
        /*
            delete writer_;
        delete reader_;
        */
    }
}

impl LogTest {

    pub fn reopen_for_append(&mut self)  {
        
        todo!();
        /*
            delete writer_;
        writer_ = new LogWriter(&dest_, dest_.contents_.size());
        */
    }
    
    pub fn write(&mut self, msg: &String)  {
        
        todo!();
        /*
            ASSERT_TRUE(!reading_) << "Write() after starting to read";
        writer_->AddRecord(Slice(msg));
        */
    }
    
    pub fn written_bytes(&self) -> usize {
        
        todo!();
        /*
            return dest_.contents_.size();
        */
    }
    
    pub fn read(&mut self) -> String {
        
        todo!();
        /*
            if (!reading_) {
          reading_ = true;
          source_.contents_ = Slice(dest_.contents_);
        }
        std::string scratch;
        Slice record;
        if (reader_->ReadRecord(&record, &scratch)) {
          return record.ToString();
        } else {
          return "EOF";
        }
        */
    }
    
    pub fn increment_byte(&mut self, 
        offset: i32,
        delta:  i32)  {
        
        todo!();
        /*
            dest_.contents_[offset] += delta;
        */
    }
    
    pub fn set_byte(&mut self, 
        offset:   i32,
        new_byte: u8)  {
        
        todo!();
        /*
            dest_.contents_[offset] = new_byte;
        */
    }
    
    pub fn shrink_size(&mut self, bytes: i32)  {
        
        todo!();
        /*
            dest_.contents_.resize(dest_.contents_.size() - bytes);
        */
    }
    
    pub fn fix_checksum(&mut self, 
        header_offset: i32,
        len:           i32)  {
        
        todo!();
        /*
            // Compute crc of type/len/data
        uint32_t crc = crc32c::Value(&dest_.contents_[header_offset + 6], 1 + len);
        crc = crc32c::Mask(crc);
        EncodeFixed32(&dest_.contents_[header_offset], crc);
        */
    }
    
    pub fn force_error(&mut self)  {
        
        todo!();
        /*
            source_.force_error_ = true;
        */
    }
    
    pub fn dropped_bytes(&self) -> usize {
        
        todo!();
        /*
            return report_.dropped_bytes_;
        */
    }
    
    pub fn report_message(&self) -> String {
        
        todo!();
        /*
            return report_.message_;
        */
    }

    /**
      | Returns OK iff recorded error message
      | contains "msg"
      |
      */
    pub fn match_error(&self, msg: &String) -> String {
        
        todo!();
        /*
            if (report_.message_.find(msg) == std::string::npos) {
          return report_.message_;
        } else {
          return "OK";
        }
        */
    }
    
    pub fn write_initial_offset_log(&mut self)  {
        
        todo!();
        /*
            for (int i = 0; i < num_initial_offset_records_; i++) {
          std::string record(initial_offset_record_sizes_[i],
                             static_cast<char>('a' + i));
          Write(record);
        }
        */
    }
    
    pub fn start_reading_at(&mut self, initial_offset: u64)  {
        
        todo!();
        /*
            delete reader_;
        reader_ = new Reader(&source_, &report_, true /*checksum*/, initial_offset);
        */
    }
    
    pub fn check_offset_past_end_returns_no_records(&mut self, offset_past_end: u64)  {
        
        todo!();
        /*
            WriteInitialOffsetLog();
        reading_ = true;
        source_.contents_ = Slice(dest_.contents_);
        Reader* offset_reader = new Reader(&source_, &report_, true /*checksum*/,
                                           WrittenBytes() + offset_past_end);
        Slice record;
        std::string scratch;
        ASSERT_TRUE(!offset_reader->ReadRecord(&record, &scratch));
        delete offset_reader;
        */
    }
    
    pub fn check_initial_offset_record(&mut self, 
        initial_offset:         u64,
        expected_record_offset: i32)  {
        
        todo!();
        /*
            WriteInitialOffsetLog();
        reading_ = true;
        source_.contents_ = Slice(dest_.contents_);
        Reader* offset_reader =
            new Reader(&source_, &report_, true /*checksum*/, initial_offset);

        // Read all records from expected_record_offset through the last one.
        ASSERT_LT(expected_record_offset, num_initial_offset_records_);
        for (; expected_record_offset < num_initial_offset_records_;
             ++expected_record_offset) {
          Slice record;
          std::string scratch;
          ASSERT_TRUE(offset_reader->ReadRecord(&record, &scratch));
          ASSERT_EQ(initial_offset_record_sizes_[expected_record_offset],
                    record.size());
          ASSERT_EQ(initial_offset_last_record_offsets_[expected_record_offset],
                    offset_reader->LastRecordOffset());
          ASSERT_EQ((char)('a' + expected_record_offset), record.data()[0]);
        }
        delete offset_reader;
        */
    }
}

#[test] fn log_test_empty() {
    todo!();
    /*
         ASSERT_EQ("EOF", Read()); 
    */
}

#[test] fn log_test_read_write() {
    todo!();
    /*
    
      Write("foo");
      Write("bar");
      Write("");
      Write("xxxx");
      ASSERT_EQ("foo", Read());
      ASSERT_EQ("bar", Read());
      ASSERT_EQ("", Read());
      ASSERT_EQ("xxxx", Read());
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ("EOF", Read());  // Make sure reads at eof work

    */
}

#[test] fn log_test_many_blocks() {
    todo!();
    /*
    
      for (int i = 0; i < 100000; i++) {
        Write(NumberString(i));
      }
      for (int i = 0; i < 100000; i++) {
        ASSERT_EQ(NumberString(i), Read());
      }
      ASSERT_EQ("EOF", Read());

    */
}

#[test] fn log_test_fragmentation() {
    todo!();
    /*
    
      Write("small");
      Write(BigString("medium", 50000));
      Write(BigString("large", 100000));
      ASSERT_EQ("small", Read());
      ASSERT_EQ(BigString("medium", 50000), Read());
      ASSERT_EQ(BigString("large", 100000), Read());
      ASSERT_EQ("EOF", Read());

    */
}

#[test] fn log_test_marginal_trailer() {
    todo!();
    /*
    
      // Make a trailer that is exactly the same length as an empty record.
      const int n = kBlockSize - 2 * kHeaderSize;
      Write(BigString("foo", n));
      ASSERT_EQ(kBlockSize - kHeaderSize, WrittenBytes());
      Write("");
      Write("bar");
      ASSERT_EQ(BigString("foo", n), Read());
      ASSERT_EQ("", Read());
      ASSERT_EQ("bar", Read());
      ASSERT_EQ("EOF", Read());

    */
}

#[test] fn log_test_marginal_trailer2() {
    todo!();
    /*
    
      // Make a trailer that is exactly the same length as an empty record.
      const int n = kBlockSize - 2 * kHeaderSize;
      Write(BigString("foo", n));
      ASSERT_EQ(kBlockSize - kHeaderSize, WrittenBytes());
      Write("bar");
      ASSERT_EQ(BigString("foo", n), Read());
      ASSERT_EQ("bar", Read());
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ(0, DroppedBytes());
      ASSERT_EQ("", ReportMessage());

    */
}

#[test] fn log_test_short_trailer() {
    todo!();
    /*
    
      const int n = kBlockSize - 2 * kHeaderSize + 4;
      Write(BigString("foo", n));
      ASSERT_EQ(kBlockSize - kHeaderSize + 4, WrittenBytes());
      Write("");
      Write("bar");
      ASSERT_EQ(BigString("foo", n), Read());
      ASSERT_EQ("", Read());
      ASSERT_EQ("bar", Read());
      ASSERT_EQ("EOF", Read());

    */
}

#[test] fn log_test_aligned_eof() {
    todo!();
    /*
    
      const int n = kBlockSize - 2 * kHeaderSize + 4;
      Write(BigString("foo", n));
      ASSERT_EQ(kBlockSize - kHeaderSize + 4, WrittenBytes());
      ASSERT_EQ(BigString("foo", n), Read());
      ASSERT_EQ("EOF", Read());

    */
}

#[test] fn log_test_open_for_append() {
    todo!();
    /*
    
      Write("hello");
      ReopenForAppend();
      Write("world");
      ASSERT_EQ("hello", Read());
      ASSERT_EQ("world", Read());
      ASSERT_EQ("EOF", Read());

    */
}

#[test] fn log_test_random_read() {
    todo!();
    /*
    
      const int N = 500;
      Random write_rnd(301);
      for (int i = 0; i < N; i++) {
        Write(RandomSkewedString(i, &write_rnd));
      }
      Random read_rnd(301);
      for (int i = 0; i < N; i++) {
        ASSERT_EQ(RandomSkewedString(i, &read_rnd), Read());
      }
      ASSERT_EQ("EOF", Read());

    */
}

/**
  | Tests of all the error paths in log_reader.cc
  | follow:
  |
  */
#[test] fn log_test_read_error() {
    todo!();
    /*
    
      Write("foo");
      ForceError();
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ(kBlockSize, DroppedBytes());
      ASSERT_EQ("OK", MatchError("read error"));

    */
}

#[test] fn log_test_bad_record_type() {
    todo!();
    /*
    
      Write("foo");
      // Type is stored in header[6]
      IncrementByte(6, 100);
      FixChecksum(0, 3);
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ(3, DroppedBytes());
      ASSERT_EQ("OK", MatchError("unknown record type"));

    */
}

#[test] fn log_test_truncated_trailing_record_is_ignored() {
    todo!();
    /*
    
      Write("foo");
      ShrinkSize(4);  // Drop all payload as well as a header byte
      ASSERT_EQ("EOF", Read());
      // Truncated last record is ignored, not treated as an error.
      ASSERT_EQ(0, DroppedBytes());
      ASSERT_EQ("", ReportMessage());

    */
}

#[test] fn log_test_bad_length() {
    todo!();
    /*
    
      const int kPayloadSize = kBlockSize - kHeaderSize;
      Write(BigString("bar", kPayloadSize));
      Write("foo");
      // Least significant size byte is stored in header[4].
      IncrementByte(4, 1);
      ASSERT_EQ("foo", Read());
      ASSERT_EQ(kBlockSize, DroppedBytes());
      ASSERT_EQ("OK", MatchError("bad record length"));

    */
}

#[test] fn log_test_bad_length_at_end_is_ignored() {
    todo!();
    /*
    
      Write("foo");
      ShrinkSize(1);
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ(0, DroppedBytes());
      ASSERT_EQ("", ReportMessage());

    */
}

#[test] fn log_test_checksum_mismatch() {
    todo!();
    /*
    
      Write("foo");
      IncrementByte(0, 10);
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ(10, DroppedBytes());
      ASSERT_EQ("OK", MatchError("checksum mismatch"));

    */
}

#[test] fn log_test_unexpected_middle_type() {
    todo!();
    /*
    
      Write("foo");
      SetByte(6, kMiddleType);
      FixChecksum(0, 3);
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ(3, DroppedBytes());
      ASSERT_EQ("OK", MatchError("missing start"));

    */
}

#[test] fn log_test_unexpected_last_type() {
    todo!();
    /*
    
      Write("foo");
      SetByte(6, kLastType);
      FixChecksum(0, 3);
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ(3, DroppedBytes());
      ASSERT_EQ("OK", MatchError("missing start"));

    */
}

#[test] fn log_test_unexpected_full_type() {
    todo!();
    /*
    
      Write("foo");
      Write("bar");
      SetByte(6, kFirstType);
      FixChecksum(0, 3);
      ASSERT_EQ("bar", Read());
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ(3, DroppedBytes());
      ASSERT_EQ("OK", MatchError("partial record without end"));

    */
}

#[test] fn log_test_unexpected_first_type() {
    todo!();
    /*
    
      Write("foo");
      Write(BigString("bar", 100000));
      SetByte(6, kFirstType);
      FixChecksum(0, 3);
      ASSERT_EQ(BigString("bar", 100000), Read());
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ(3, DroppedBytes());
      ASSERT_EQ("OK", MatchError("partial record without end"));

    */
}

#[test] fn log_test_missing_last_is_ignored() {
    todo!();
    /*
    
      Write(BigString("bar", kBlockSize));
      // Remove the LAST block, including header.
      ShrinkSize(14);
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ("", ReportMessage());
      ASSERT_EQ(0, DroppedBytes());

    */
}

#[test] fn log_test_partial_last_is_ignored() {
    todo!();
    /*
    
      Write(BigString("bar", kBlockSize));
      // Cause a bad record length in the LAST block.
      ShrinkSize(1);
      ASSERT_EQ("EOF", Read());
      ASSERT_EQ("", ReportMessage());
      ASSERT_EQ(0, DroppedBytes());

    */
}

#[test] fn log_test_skip_into_multi_record() {
    todo!();
    /*
    
      // Consider a fragmented record:
      //    first(R1), middle(R1), last(R1), first(R2)
      // If initial_offset points to a record after first(R1) but before first(R2)
      // incomplete fragment errors are not actual errors, and must be suppressed
      // until a new first or full record is encountered.
      Write(BigString("foo", 3 * kBlockSize));
      Write("correct");
      StartReadingAt(kBlockSize);

      ASSERT_EQ("correct", Read());
      ASSERT_EQ("", ReportMessage());
      ASSERT_EQ(0, DroppedBytes());
      ASSERT_EQ("EOF", Read());

    */
}

#[test] fn log_test_error_joins_records() {
    todo!();
    /*
    
      // Consider two fragmented records:
      //    first(R1) last(R1) first(R2) last(R2)
      // where the middle two fragments disappear.  We do not want
      // first(R1),last(R2) to get joined and returned as a valid record.

      // Write records that span two blocks
      Write(BigString("foo", kBlockSize));
      Write(BigString("bar", kBlockSize));
      Write("correct");

      // Wipe the middle block
      for (int offset = kBlockSize; offset < 2 * kBlockSize; offset++) {
        SetByte(offset, 'x');
      }

      ASSERT_EQ("correct", Read());
      ASSERT_EQ("EOF", Read());
      const size_t dropped = DroppedBytes();
      ASSERT_LE(dropped, 2 * kBlockSize + 100);
      ASSERT_GE(dropped, 2 * kBlockSize);

    */
}

#[test] fn log_test_read_start() {
    todo!();
    /*
         CheckInitialOffsetRecord(0, 0); 
    */
}

#[test] fn log_test_read_second_one_off() {
    todo!();
    /*
         CheckInitialOffsetRecord(1, 1); 
    */
}

#[test] fn log_test_read_second_ten_thousand() {
    todo!();
    /*
         CheckInitialOffsetRecord(10000, 1); 
    */
}

#[test] fn log_test_read_second_start() {
    todo!();
    /*
         CheckInitialOffsetRecord(10007, 1); 
    */
}

#[test] fn log_test_read_third_one_off() {
    todo!();
    /*
         CheckInitialOffsetRecord(10008, 2); 
    */
}

#[test] fn log_test_read_third_start() {
    todo!();
    /*
         CheckInitialOffsetRecord(20014, 2); 
    */
}

#[test] fn log_test_read_fourth_one_off() {
    todo!();
    /*
         CheckInitialOffsetRecord(20015, 3); 
    */
}

#[test] fn log_test_read_fourth_first_block_trailer() {
    todo!();
    /*
    
      CheckInitialOffsetRecord(log::kBlockSize - 4, 3);

    */
}

#[test] fn log_test_read_fourth_middle_block() {
    todo!();
    /*
    
      CheckInitialOffsetRecord(log::kBlockSize + 1, 3);

    */
}

#[test] fn log_test_read_fourth_last_block() {
    todo!();
    /*
    
      CheckInitialOffsetRecord(2 * log::kBlockSize + 1, 3);

    */
}

#[test] fn log_test_read_fourth_start() {
    todo!();
    /*
    
      CheckInitialOffsetRecord(
          2 * (kHeaderSize + 1000) + (2 * log::kBlockSize - 1000) + 3 * kHeaderSize,
          3);

    */
}

#[test] fn log_test_read_initial_offset_into_block_padding() {
    todo!();
    /*
    
      CheckInitialOffsetRecord(3 * log::kBlockSize - 3, 5);

    */
}

#[test] fn log_test_read_end() {
    todo!();
    /*
         CheckOffsetPastEndReturnsNoRecords(0); 
    */
}

#[test] fn log_test_read_past_end() {
    todo!();
    /*
         CheckOffsetPastEndReturnsNoRecords(5); 
    */
}

fn logtest_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
