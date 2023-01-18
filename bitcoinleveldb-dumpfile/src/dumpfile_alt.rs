crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/dumpfile.cc]

pub fn guess_type(
        fname: &String,
        ty:    *mut FileType) -> bool {
    
    todo!();
        /*
            size_t pos = fname.rfind('/');
      std::string basename;
      if (pos == std::string::npos) {
        basename = fname;
      } else {
        basename = std::string(fname.data() + pos + 1, fname.size() - pos - 1);
      }
      uint64_t ignored;
      return ParseFileName(basename, &ignored, type);
        */
}

/**
  | Notified when log reader encounters
  | corruption.
  |
  */
pub struct CorruptionReporter {
    dst:  *mut dyn WritableFile,
}

impl LogReaderReporter for CorruptionReporter {

    fn corruption(&mut self, 
        bytes:  usize,
        status: &Status)  {
        
        todo!();
        /*
            std::string r = "corruption: ";
        AppendNumberTo(&r, bytes);
        r += " bytes; ";
        r += status.ToString();
        r.push_back('\n');
        dst_->Append(r);
        */
    }
}

/**
  | Print contents of a log file. (*func)()
  | is called on every record.
  |
  */
pub fn print_log_contents(
        env:   Rc<RefCell<dyn crate::Env>>,
        fname: &String,
        func:  fn(
                _0: u64,
                _1: Slice,
                _2: *mut dyn WritableFile
        ) -> c_void,
        dst:   *mut dyn WritableFile) -> crate::Status {
    
    todo!();
        /*
            SequentialFile* file;
      Status s = env->NewSequentialFile(fname, &file);
      if (!s.ok()) {
        return s;
      }
      CorruptionReporter reporter;
      reporter.dst_ = dst;
      LogReader reader(file, &reporter, true, 0);
      Slice record;
      std::string scratch;
      while (reader.ReadRecord(&record, &scratch)) {
        (*func)(reader.LastRecordOffset(), record, dst);
      }
      delete file;
      return Status::OK();
        */
}

/**
  | Called on every item found in a WriteBatch.
  |
  */
pub struct WriteBatchItemPrinter {
    dst:  *mut dyn WritableFile,
}

impl WriteBatchHandler for WriteBatchItemPrinter {

}

impl WriteBatchPut for WriteBatchItemPrinter {

    fn put(&mut self, 
        k:   &Slice,
        value: &Slice)  {
        
        todo!();
        /*
            std::string r = "  put '";
        AppendEscapedStringTo(&r, k);
        r += "' '";
        AppendEscapedStringTo(&r, value);
        r += "'\n";
        dst_->Append(r);
        */
    }
}

impl WriteBatchDelete for WriteBatchItemPrinter {

    fn delete(&mut self, k: &Slice)  {
        
        todo!();
        /*
            std::string r = "  del '";
        AppendEscapedStringTo(&r, k);
        r += "'\n";
        dst_->Append(r);
        */
    }
}

/**
  | Called on every log record (each one
  | of which is a WriteBatch) found in a kLogFile.
  |
  */
pub fn write_batch_printer(
        pos:    u64,
        record: Slice,
        dst:    Rc<RefCell<dyn WritableFile>>)  {
    
    todo!();
        /*
            std::string r = "--- offset ";
      AppendNumberTo(&r, pos);
      r += "; ";
      if (record.size() < 12) {
        r += "log record length ";
        AppendNumberTo(&r, record.size());
        r += " is too small\n";
        dst->Append(r);
        return;
      }
      WriteBatch batch;
      WriteBatchInternal::SetContents(&batch, record);
      r += "sequence ";
      AppendNumberTo(&r, WriteBatchInternal::Sequence(&batch));
      r.push_back('\n');
      dst->Append(r);
      WriteBatchItemPrinter batch_item_printer;
      batch_item_printer.dst_ = dst;
      Status s = batch.Iterate(&batch_item_printer);
      if (!s.ok()) {
        dst->Append("  error: " + s.ToString() + "\n");
      }
        */
}

pub fn dump_log(
        env:   Rc<RefCell<dyn crate::Env>>,
        fname: &String,
        dst:   Rc<RefCell<dyn WritableFile>>) -> crate::Status {
    
    todo!();
        /*
            return PrintLogContents(env, fname, WriteBatchPrinter, dst);
        */
}

/**
  | Called on every log record (each one
  | of which is a WriteBatch) found in a kDescriptorFile.
  |
  */
pub fn version_edit_printer(
        pos:    u64,
        record: Slice,
        dst:    *mut dyn WritableFile)  {
    
    todo!();
        /*
            std::string r = "--- offset ";
      AppendNumberTo(&r, pos);
      r += "; ";
      VersionEdit edit;
      Status s = edit.DecodeFrom(record);
      if (!s.ok()) {
        r += s.ToString();
        r.push_back('\n');
      } else {
        r += edit.DebugString();
      }
      dst->Append(r);
        */
}

pub fn dump_descriptor(
        env:   Rc<RefCell<dyn crate::Env>>,
        fname: &String,
        dst:   *mut dyn WritableFile) -> crate::Status {
    
    todo!();
        /*
            return PrintLogContents(env, fname, VersionEditPrinter, dst);
        */
}

pub fn dump_table(
        env:   Rc<RefCell<dyn crate::Env>>,
        fname: &String,
        dst:   *mut dyn WritableFile) -> crate::Status {
    
    todo!();
        /*
            uint64_t file_size;
      RandomAccessFile* file = nullptr;
      Table* table = nullptr;
      Status s = env->GetFileSize(fname, &file_size);
      if (s.ok()) {
        s = env->NewRandomAccessFile(fname, &file);
      }
      if (s.ok()) {
        // We use the default comparator, which may or may not match the
        // comparator used in this database. However this should not cause
        // problems since we only use Table operations that do not require
        // any comparisons.  In particular, we do not call Seek or Prev.
        s = Table::Open(Options(), file, file_size, &table);
      }
      if (!s.ok()) {
        delete table;
        delete file;
        return s;
      }

      ReadOptions ro;
      ro.fill_cache = false;
      Iterator* iter = table->NewIterator(ro);
      std::string r;
      for (iter->SeekToFirst(); iter->Valid(); iter->Next()) {
        r.clear();
        ParsedInternalKey key;
        if (!ParseInternalKey(iter->key(), &key)) {
          r = "badkey '";
          AppendEscapedStringTo(&r, iter->key());
          r += "' => '";
          AppendEscapedStringTo(&r, iter->value());
          r += "'\n";
          dst->Append(r);
        } else {
          r = "'";
          AppendEscapedStringTo(&r, key.user_key);
          r += "' @ ";
          AppendNumberTo(&r, key.sequence);
          r += " : ";
          if (key.type == kTypeDeletion) {
            r += "del";
          } else if (key.type == kTypeValue) {
            r += "val";
          } else {
            AppendNumberTo(&r, key.type);
          }
          r += " => '";
          AppendEscapedStringTo(&r, iter->value());
          r += "'\n";
          dst->Append(r);
        }
      }
      s = iter->status();
      if (!s.ok()) {
        dst->Append("iterator error: " + s.ToString() + "\n");
      }

      delete iter;
      delete table;
      delete file;
      return Status::OK();
        */
}

pub fn dump_file(
        env:   Rc<RefCell<dyn Env>>,
        fname: &String,
        dst:   *mut dyn WritableFile) -> crate::Status {
    
    todo!();
        /*
            FileType ftype;
      if (!GuessType(fname, &ftype)) {
        return Status::InvalidArgument(fname + ": unknown file type");
      }
      switch (ftype) {
        case kLogFile:
          return DumpLog(env, fname, dst);
        case kDescriptorFile:
          return DumpDescriptor(env, fname, dst);
        case kTableFile:
          return DumpTable(env, fname, dst);
        default:
          break;
      }
      return Status::InvalidArgument(fname + ": not a dump-able file type");
        */
}
