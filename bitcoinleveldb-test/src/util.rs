crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/testutil.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/testutil.cc]

/**
  | A wrapper that allows injection of errors.
  |
  */
pub struct ErrorEnv {
    base:                     EnvWrapper,
    writable_file_error:      bool,
    num_writable_file_errors: i32,
}

impl Default for ErrorEnv {
    
    fn default() -> Self {
        todo!();
        /*


            : EnvWrapper(NewMemEnv(Env::Default())),
            writable_file_error_(false),
            num_writable_file_errors_(0)
        */
    }
}

impl Drop for ErrorEnv {
    fn drop(&mut self) {
        todo!();
        /*
            delete target();
        */
    }
}

impl ErrorEnv {

    pub fn new_writable_file(&mut self, 
        fname:  &String,
        result: *mut *mut dyn WritableFile) -> crate::Status {
        
        todo!();
        /*
            if (writable_file_error_) {
          ++num_writable_file_errors_;
          *result = nullptr;
          return crate::Status::IOError(fname, "fake error");
        }
        return target()->NewWritableFile(fname, result);
        */
    }
    
    pub fn new_appendable_file(&mut self, 
        fname:  &String,
        result: *mut *mut dyn WritableFile) -> crate::Status {
        
        todo!();
        /*
            if (writable_file_error_) {
          ++num_writable_file_errors_;
          *result = nullptr;
          return crate::Status::IOError(fname, "fake error");
        }
        return target()->NewAppendableFile(fname, result);
        */
    }
}

/**
  | Store in *dst a random string of length
  | "len" and return a Slice that references
  | the generated data.
  |
  */
pub fn random_string(
        rnd: *mut Random,
        len: i32,
        dst: *mut String) -> Slice {
    
    todo!();
        /*
            dst->resize(len);
      for (int i = 0; i < len; i++) {
        (*dst)[i] = static_cast<char>(' ' + rnd->Uniform(95));  // ' ' .. '~'
      }
      return Slice(*dst);
        */
}

/**
  | Return a random key with the specified
  | length that may contain interesting
  | characters (e.g. \x00, \xff, etc.).
  |
  */
pub fn random_key(
        rnd: *mut Random,
        len: i32) -> String {
    
    todo!();
        /*
            // Make sure to generate a wide variety of characters so we
      // test the boundary conditions for short-key optimizations.
      static const char kTestChars[] = {'\0', '\1', 'a',    'b',    'c',
                                        'd',  'e',  '\xfd', '\xfe', '\xff'};
      std::string result;
      for (int i = 0; i < len; i++) {
        result += kTestChars[rnd->Uniform(sizeof(kTestChars))];
      }
      return result;
        */
}

/**
  | Store in *dst a string of length "len" that
  | will compress to "N*compressed_fraction" bytes
  | and return a Slice that references the
  | generated data.
  */
pub fn compressible_string(
        rnd:                 *mut Random,
        compressed_fraction: f64,
        len:                 usize,
        dst:                 *mut String) -> Slice {
    
    todo!();
        /*
            int raw = static_cast<int>(len * compressed_fraction);
      if (raw < 1) raw = 1;
      std::string raw_data;
      RandomString(rnd, raw, &raw_data);

      // Duplicate the random data until we have filled "len" bytes
      dst->clear();
      while (dst->size() < len) {
        dst->append(raw_data);
      }
      dst->resize(len);
      return Slice(*dst);
        */
}
