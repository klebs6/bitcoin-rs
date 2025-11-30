// ---------------- [ File: bitcoinleveldb-posixrafile/src/posix_random_access_file.rs ]
crate::ix!();

/**
  | Implements random read access in a file using
  | pread().
  |
  | Instances of this class are thread-safe, as
  | required by the RandomAccessFile API. Instances
  | are immutable and Read() only calls thread-safe
  | library functions.
  */
#[derive(Getters, Builder, Debug)]
#[getset(get = "pub")]
pub struct PosixRandomAccessFile {

    /**
      | If false, the file is opened on every
      | read.
      |
      */
    has_permanent_fd: bool,

    /**
      | -1 if has_permanent_fd_ is false.
      |
      */
    fd: i32,

    fd_limiter: *const Limiter,

    filename: String,
}

impl RandomAccessFile for PosixRandomAccessFile { }

impl Named for PosixRandomAccessFile {

    fn name(&self) -> Cow<'_,str> {
        Cow::Borrowed(&self.filename)
    }
}

impl PosixRandomAccessFile {

    /**
      | The new instance takes ownership of `fd`.
      | `fd_limiter` must outlive this instance.
      |
      | C++ logic:
      |   has_permanent_fd_ = fd_limiter->Acquire();
      |   fd_ = has_permanent_fd_ ? fd : -1;
      |   if (!has_permanent_fd_) ::close(fd);
      */
    pub fn new(
        filename:   String,
        fd:         i32,
        fd_limiter: *mut Limiter,
    ) -> Self {

        trace!(
            file = %filename,
            fd,
            "PosixRandomAccessFile::new: constructing"
        );

        let has_permanent_fd = unsafe {
            if fd_limiter.is_null() {
                false
            } else {
                (*fd_limiter).acquire()
            }
        };

        if !has_permanent_fd && fd >= 0 {
            trace!(
                fd,
                "PosixRandomAccessFile::new: no permanent fd; closing incoming fd"
            );
            unsafe {
                libc::close(fd);
            }
        }

        let fd_stored = if has_permanent_fd { fd } else { -1 };

        let instance = Self {
            has_permanent_fd,
            fd: fd_stored,
            fd_limiter: fd_limiter as *const Limiter,
            filename,
        };

        debug!(
            has_permanent_fd = instance.has_permanent_fd,
            fd = instance.fd,
            "PosixRandomAccessFile::new: constructed"
        );

        instance
    }
}
