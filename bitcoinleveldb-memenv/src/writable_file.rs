// ---------------- [ File: bitcoinleveldb-memenv/src/writable_file.rs ]
crate::ix!();

pub struct WritableFileImpl {
    file: *mut FileState,
}

impl WritableFile for WritableFileImpl {}

impl Drop for WritableFileImpl {
    fn drop(&mut self) {
        todo!();
        /*
            file_->Unref();
        */
    }
}

impl Named for WritableFileImpl {

    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("[memenv]".to_string())
    }
}

impl WritableFileImpl {

    pub fn new(file: *mut FileState) -> Self {
    
        todo!();
        /*
        : file(file),

            file_->Ref();
        */
    }
}

impl WritableFileAppend for WritableFileImpl {

    fn append(&mut self, data: &Slice) -> crate::Status {
        
        todo!();
        /*
            return file_->Append(data);
        */
    }
}
    
impl WritableFileClose for WritableFileImpl {
    fn close(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}
    
impl WritableFileFlush for WritableFileImpl {
    fn flush(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}
    
impl WritableFileSync for WritableFileImpl {
    fn sync(&mut self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}
