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

    /// Transfer this shared handle into a boxed forwarding adapter.
    ///
    /// The adapter keeps the existing file alive without cloning the Rc or
    /// changing its contents. Operations use the same RefCell borrow rules as
    /// direct access to the shared file. Dropping the adapter releases its Rc;
    /// closing or syncing the file remains an explicit caller operation.
    fn from(dest: Rc<RefCell<dyn WritableFile>>) -> Self {
        Box::new(SharedWritableFileAdapter { inner: dest })
    }
}

struct SharedWritableFileAdapter {
    inner: Rc<RefCell<dyn WritableFile>>,
}

impl WritableFile for SharedWritableFileAdapter {}

impl WritableFileAppend for SharedWritableFileAdapter {
    fn append(&mut self, data: &Slice) -> crate::Status {
        self.inner.borrow_mut().append(data)
    }
}

impl WritableFileClose for SharedWritableFileAdapter {
    fn close(&mut self) -> crate::Status {
        self.inner.borrow_mut().close()
    }
}

impl WritableFileFlush for SharedWritableFileAdapter {
    fn flush(&mut self) -> crate::Status {
        self.inner.borrow_mut().flush()
    }
}

impl WritableFileSync for SharedWritableFileAdapter {
    fn sync(&mut self) -> crate::Status {
        self.inner.borrow_mut().sync()
    }
}

impl Named for SharedWritableFileAdapter {
    fn name(&self) -> Cow<'_,str> {
        // An inner borrowed name cannot outlive the temporary RefCell guard.
        Cow::Owned(self.inner.borrow().name().into_owned())
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
