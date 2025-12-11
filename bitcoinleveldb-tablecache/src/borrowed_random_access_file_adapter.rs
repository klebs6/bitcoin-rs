// ---------------- [ File: bitcoinleveldb-tablecache/src/borrowed_random_access_file_adapter.rs ]
crate::ix!();

pub struct BorrowedRandomAccessFileAdapter {
    inner: *mut Box<dyn RandomAccessFile>,
    name:  String,
}

impl BorrowedRandomAccessFileAdapter {

    pub fn new(inner: *mut Box<dyn RandomAccessFile>, name: &String) -> Self {
        trace!(
            "BorrowedRandomAccessFileAdapter::new: inner={:?}, name='{}'",
            inner,
            name
        );

        BorrowedRandomAccessFileAdapter {
            inner,
            name: name.clone(),
        }
    }

    #[inline]
    pub fn inner_handle_ptr(&self) -> *mut Box<dyn RandomAccessFile> {
        self.inner
    }
}

impl RandomAccessFile for BorrowedRandomAccessFileAdapter {}

impl RandomAccessFileRead for BorrowedRandomAccessFileAdapter {
    fn read(
        &self,
        offset:  u64,
        n:       usize,
        result:  *mut Slice,
        scratch: *mut u8,
    ) -> Status {
        trace!(
            "BorrowedRandomAccessFileAdapter::read: inner={:?}, offset={}, n={}",
            self.inner,
            offset,
            n
        );

        unsafe {
            if self.inner.is_null() {
                let msg = b"BorrowedRandomAccessFileAdapter::read: inner file holder pointer is null";
                let msg_slice = Slice::from(&msg[..]);
                error!(
                    "BorrowedRandomAccessFileAdapter::read: inner pointer null; returning corruption status"
                );
                return Status::corruption(&msg_slice, None);
            }

            let inner_holder: &mut Box<dyn RandomAccessFile> = &mut *self.inner;
            let inner_file: &dyn RandomAccessFile = inner_holder.as_ref();

            RandomAccessFileRead::read(
                inner_file,
                offset,
                n,
                result,
                scratch,
            )
        }
    }
}

impl Named for BorrowedRandomAccessFileAdapter {
    fn name(&self) -> std::borrow::Cow<'_, str> {
        trace!(
            "BorrowedRandomAccessFileAdapter::name: inner={:?}, cached_name='{}'",
            self.inner,
            self.name
        );

        unsafe {
            if self.inner.is_null() {
                // Underlying file missing → return cached name
                return std::borrow::Cow::Owned(self.name.clone());
            }

            // SAFETY: inner is a *mut Box<dyn RandomAccessFile>
            let holder: &mut Box<dyn RandomAccessFile> = &mut *self.inner;

            // Underlying file returns Cow<'_, str>. We must clone it into an owned String.
            let underlying_cow = holder.name();

            let owned = match underlying_cow {
                std::borrow::Cow::Borrowed(s) => s.to_owned(),
                std::borrow::Cow::Owned(s)    => s,
            };

            trace!(
                "BorrowedRandomAccessFileAdapter::name: returning owned underlying name='{}'",
                owned
            );

            std::borrow::Cow::Owned(owned)
        }
    }
}
