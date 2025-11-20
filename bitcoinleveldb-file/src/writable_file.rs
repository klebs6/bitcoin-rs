// ---------------- [ File: bitcoinleveldb-file/src/writable_file.rs ]
crate::ix!();

/**
  | A file abstraction for sequential writing.  The
  | implementation must provide buffering since
  | callers may append small fragments at a time to
  | the file.
  */
pub trait WritableFile: 
WritableFileAppend 
+ WritableFileClose 
+ WritableFileFlush 
+ WritableFileSync 

/*
   | Get a name for the file, only for error
   | reporting
   |
   */
+ GetName {}

pub trait WritableFileAppend {
    fn append(&mut self, data: &Slice) -> crate::Status;
}

pub trait WritableFileClose {
    fn close(&mut self) -> crate::Status;
}

pub trait WritableFileFlush {
    fn flush(&mut self) -> crate::Status;
}

pub trait WritableFileSync {
    fn sync(&mut self) -> crate::Status;
}

impl From<Rc<RefCell<dyn WritableFile>>> for Box<dyn WritableFile> {

    /**
      | Create a writer that will append data to
      | "*dest".
      |
      | "*dest" must be initially empty.
      |
      | "*dest" must remain live while this LogWriter is
      | in use.
      */
    fn from(dest: Rc<RefCell<dyn WritableFile>>) -> Self {
    
        todo!();
        /*
           : dest(dest),
           : block_offset(0),
           InitTypeCrc(type_crc_);
        */
    }
}
