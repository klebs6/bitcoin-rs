// ---------------- [ File: bitcoinleveldb-memenv/src/random_access_file.rs ]
crate::ix!();

pub struct RandomAccessFileImpl {
    file: *mut FileState,
}

impl RandomAccessFile for RandomAccessFileImpl { }

impl RandomAccessFileRead for RandomAccessFileImpl { 

    fn read(&self, 
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8) -> crate::Status {
        
        todo!();
        /*
            return file_->Read(offset, n, result, scratch);
        */
    }
}

impl Drop for RandomAccessFileImpl {
    fn drop(&mut self) {
        todo!();
        /*
            file_->Unref();
        */
    }
}

impl Named for RandomAccessFileImpl {

    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("[memenv]".to_string())
    }
}
 
impl RandomAccessFileImpl {

    pub fn new(file: *mut FileState) -> Self {
    
        todo!();
        /*
        : file(file),

            file_->Ref();
        */
    }
}
