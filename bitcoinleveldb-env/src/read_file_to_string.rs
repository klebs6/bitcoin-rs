// ---------------- [ File: bitcoinleveldb-env/src/read_file_to_string.rs ]
crate::ix!();

/**
  | A utility routine: read contents of
  | named file into *data
  |
  */
pub fn read_file_to_string(
        env:   Rc<RefCell<dyn Env>>,
        fname: &String,
        data:  *mut String) -> crate::Status {
    
    todo!();
        /*
            data->clear();
      SequentialFile* file;
      crate::Status s = env->NewSequentialFile(fname, &file);
      if (!s.ok()) {
        return s;
      }
      static const int kBufferSize = 8192;
      char* space = new char[kBufferSize];
      while (true) {
        Slice fragment;
        s = file->Read(kBufferSize, &fragment, space);
        if (!s.ok()) {
          break;
        }
        data->append(fragment.data(), fragment.size());
        if (fragment.empty()) {
          break;
        }
      }
      delete[] space;
      delete file;
      return s;
        */
}
