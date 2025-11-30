// ---------------- [ File: bitcoinleveldb-dumpfile/src/print_log_contents.rs ]
crate::ix!();

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
