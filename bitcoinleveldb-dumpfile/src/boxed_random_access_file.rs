// ---------------- [ File: bitcoinleveldb-dumpfile/src/boxed_random_access_file.rs ]
crate::ix!();

pub struct BoxedRandomAccessFile {
    inner: Box<dyn RandomAccessFile>,
}

impl From<Box<dyn RandomAccessFile>> for BoxedRandomAccessFile {

    fn from(x: Box<dyn RandomAccessFile>) -> Self {
        Self { inner: x }
    }
}

impl RandomAccessFile for BoxedRandomAccessFile {}

impl RandomAccessFileRead for BoxedRandomAccessFile {
    fn read(&self, offset: u64, n: usize, result: *mut Slice, scratch: *mut u8) -> Status {
        RandomAccessFileRead::read(&*self.inner, offset, n, result, scratch)
    }
}

impl Named for BoxedRandomAccessFile {
    fn name(&self) -> Cow<'_,str> {
        Cow::Owned("boxed-rafile".to_string())
    }
}
