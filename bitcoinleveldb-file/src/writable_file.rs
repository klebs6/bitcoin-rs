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
+ Named {}

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

//-----------------------------------[impl-for-box]
impl WritableFile for Box<dyn WritableFile> {}

impl WritableFileAppend for Box<dyn WritableFile> {

    fn append(&mut self, data: &Slice) -> crate::Status {
        (**self).append(data)
    }
}

impl WritableFileClose for Box<dyn WritableFile> {

    fn close(&mut self) -> crate::Status {
        (**self).close()
    }
}

impl WritableFileFlush for Box<dyn WritableFile> {

    fn flush(&mut self) -> crate::Status {
        (**self).flush()
    }
}

impl WritableFileSync for Box<dyn WritableFile> {

    fn sync(&mut self) -> crate::Status {
        (**self).sync()
    }
}

impl Named for Box<dyn WritableFile> {
    fn name(&self) -> Cow<'_,str> {
        (**self).name()
    }
}
