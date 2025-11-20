// ---------------- [ File: bitcoinleveldb-env/src/write_string_to_file.rs ]
crate::ix!();

pub fn do_write_string_to_file(
        env:         Rc<RefCell<dyn Env>>,
        data:        &Slice,
        fname:       &String,
        should_sync: bool) -> crate::Status {
    
    todo!();
        /*
            WritableFile* file;
      crate::Status s = env->NewWritableFile(fname, &file);
      if (!s.ok()) {
        return s;
      }
      s = file->Append(data);
      if (s.ok() && should_sync) {
        s = file->Sync();
      }
      if (s.ok()) {
        s = file->Close();
      }
      delete file;  // Will auto-close if we did not close above
      if (!s.ok()) {
        env->DeleteFile(fname);
      }
      return s;
        */
}

/**
  | A utility routine: write "data" to the
  | named file.
  |
  */
pub fn write_string_to_file(
        env:   Rc<RefCell<dyn Env>>,
        data:  &Slice,
        fname: &String) -> crate::Status {
    
    todo!();
        /*
            return DoWriteStringToFile(env, data, fname, false);
        */
}

/**
  | A utility routine: write "data" to the
  | named file and Sync() it.
  |
  */
pub fn write_string_to_file_sync(
        env:   Rc<RefCell<dyn Env>>,
        data:  &Slice,
        fname: &String) -> crate::Status {
    
    todo!();
        /*
            return DoWriteStringToFile(env, data, fname, true);
        */
}
