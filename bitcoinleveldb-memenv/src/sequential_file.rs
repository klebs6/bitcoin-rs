// ---------------- [ File: bitcoinleveldb-memenv/src/sequential_file.rs ]
crate::ix!();

pub struct SequentialFileImpl {
    file: *mut FileState,
    pos:  u64,
}

impl SequentialFile for SequentialFileImpl { }

impl SequentialFileRead for SequentialFileImpl {

    fn read(&mut self, 
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status {
        
        todo!();
        /*
            Status s = file_->Read(pos_, n, result, scratch);
        if (s.ok()) {
          pos_ += result->size();
        }
        return s;
        */
    }
}

impl SequentialFileSkip for SequentialFileImpl {

    fn skip(&mut self, n: u64) -> crate::Status {
        
        todo!();
        /*
            if (pos_ > file_->Size()) {
          return Status::IOError("pos_ > file_->Size()");
        }
        const uint64_t available = file_->Size() - pos_;
        if (n > available) {
          n = available;
        }
        pos_ += n;
        return Status::OK();
        */
    }
}

impl Drop for SequentialFileImpl {
    fn drop(&mut self) {
        todo!();
        /*
            file_->Unref();
        */
    }
}

impl Named for SequentialFileImpl {

    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("[memenv]".to_string())
    }
}

impl SequentialFileImpl {

    pub fn new(file: *mut FileState) -> Self {
    
        todo!();
        /*
        : file(file),
        : pos(0),

            file_->Ref();
        */
    }
}
