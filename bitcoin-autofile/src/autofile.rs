// ---------------- [ File: bitcoin-autofile/src/autofile.rs ]
crate::ix!();

use std::io;

/// Non‑refcounted RAII wrapper for a C `FILE*`.
///
/// * Closes the file automatically on `Drop`.
/// * Call [`release`](#method.release) if you need to take ownership.
/// * Call [`fclose`](#method.fclose) if you must close early.
#[derive(Getters, Builder)]
#[builder(pattern = "owned", setter(into, strip_option))]
pub struct AutoFile {
    /// Serialization type flags (see Bitcoin Core stream semantics).
    #[getset(get = "pub")]
    n_type: i32,

    /// Serialization version (see Bitcoin Core stream semantics).
    #[getset(get = "pub")]
    n_version: i32,

    /// Raw `FILE*`; **never** expose publicly.
    file: *mut libc::FILE,
}

/* ---------------- Stream operators (C++ << / >>) ---------------- */

impl<'a, T> std::ops::Shl<&'a T> for AutoFile
where
    T: btc_serialize::Serialize<AutoFile>,
{
    type Output = AutoFile;

    fn shl(mut self, rhs: &'a T) -> Self::Output {
        if self.file.is_null() {
            error!("AutoFile::operator<< – file handle is nullptr");
            panic!("AutoFile::operator<< – file handle is nullptr");
        }

        // Delegate to the project’s custom serialiser (bit‑exact with C++).
        btc_serialize::Serialize::<AutoFile>::serialize(rhs, &mut self);

        self
    }
}

impl<'a, T> std::ops::Shr<&'a mut T> for AutoFile
where
    T: btc_unserialize::Unserialize<AutoFile>,
{
    type Output = AutoFile;

    fn shr(mut self, rhs: &'a mut T) -> Self::Output {
        if self.file.is_null() {
            error!("AutoFile::operator>> – file handle is nullptr");
            panic!("AutoFile::operator>> – file handle is nullptr");
        }

        // Delegate to the project’s custom deserialiser (bit‑exact with C++).
        btc_unserialize::Unserialize::<AutoFile>::unserialize(rhs, &mut self);

        self
    }
}

impl AutoFile {
    
    /// Create a new [`AutoFile`] from an existing `FILE*`.
    ///
    /// # Safety
    /// The caller guarantees that `file` is a valid pointer obtained from
    /// `libc::fopen` / `std::fs::File::into_raw_fd()` and that it is *not*
    /// aliased elsewhere with write access.
    pub unsafe fn new(file: *mut libc::FILE, n_type: i32, n_version: i32) -> Self {
        trace!(target: "autofile", "Creating AutoFile {{ n_type: {n_type}, n_version: {n_version}, file: {:?} }}", file);
        Self {
            n_type,
            n_version,
            file,
        }
    }

    /// Close the file early (idempotent).
    pub fn fclose(&mut self) {
        if !self.file.is_null() {
            trace!(target: "autofile", "Manually closing file {:?}", self.file);
            unsafe { libc::fclose(self.file) };
            self.file = std::ptr::null_mut();
        }
    }

    /// Transfer ownership of the underlying `FILE*`.
    ///
    /// After calling this, the `AutoFile` becomes inert.
    ///
    /// It is the responsibility of the caller to clean up the returned FILE*.
    ///
    pub fn release(&mut self) -> *mut libc::FILE {
        let ret = self.file;
        self.file = std::ptr::null_mut();
        debug!(target: "autofile", "Released ownership of file {:?}", ret);
        ret
    }

    /// Borrow the wrapped `FILE*` **without** transferring ownership.
    ///
    /// Use this only if the `AutoFile` lives longer than the raw pointer use.
    ///
    /// Ownership of the FILE* will remain with this class. Use this only if the scope of the
    /// AutoFile outlives use of the passed pointer.
    ///
    pub fn get(&self) -> *mut libc::FILE {
        self.file
    }

    /// Return true if the wrapped FILE* is nullptr, false otherwise.
    pub fn is_null(&self) -> bool {
        self.file.is_null()
    }

    /* ----------------- Stream subset  ----------------- */
    
    /// Low‑level read_ptr exactly `n_size` bytes into `pch`.
    ///
    /// # Panics
    /// Mimics the C++ behaviour and panics on error / EOF.
    pub fn read_ptr(&mut self, pch: *mut u8, n_size: usize) {
        if self.file.is_null() {
            error!("AutoFile::read_ptr: file handle is nullptr");
            panic!("AutoFile::read_ptr: file handle is nullptr");
        }

        unsafe {
            let read = libc::fread(pch as *mut libc::c_void, 1, n_size, self.file);
            if read != n_size {
                let msg = if libc::feof(self.file) != 0 {
                    "AutoFile::read_ptr: end of file"
                } else {
                    "AutoFile::read_ptr: fread failed"
                };
                error!("{msg}");
                panic!("{msg}");
            }
        }
    }

    /// Skip `n_size` bytes.
    pub fn ignore(&mut self, mut n_size: usize) {
        if self.file.is_null() {
            error!("AutoFile::ignore: file handle is nullptr");
            panic!("AutoFile::ignore: file handle is nullptr");
        }

        let mut buf = [0u8; 4096];
        while n_size != 0 {
            let now = n_size.min(buf.len());
            unsafe {
                let got = libc::fread(buf.as_mut_ptr() as *mut libc::c_void, 1, now, self.file);
                if got != now {
                    let msg = if libc::feof(self.file) != 0 {
                        "AutoFile::ignore: end of file"
                    } else {
                        "AutoFile::ignore: fread failed"
                    };
                    error!("{msg}");
                    panic!("{msg}");
                }
            }
            n_size -= now;
        }
    }

    /// Low‑level write exactly `n_size` bytes from `pch`.
    pub fn write_ptr(&mut self, pch: *const u8, n_size: usize) {
        if self.file.is_null() {
            error!("AutoFile::write_ptr: file handle is nullptr");
            panic!("AutoFile::write_ptr: file handle is nullptr");
        }

        unsafe {
            let written = libc::fwrite(pch as *const libc::c_void, 1, n_size, self.file);
            if written != n_size {
                error!("AutoFile::write_ptr: write_ptr failed (wrote {written}, expected {n_size})");
                panic!("AutoFile::write_ptr: write_ptr failed");
            }
        }
    }
}

impl io::Write for AutoFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.file.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "file handle is nullptr"));
        }

        unsafe {
            let written =
                libc::fwrite(buf.as_ptr() as *const libc::c_void, 1, buf.len(), self.file);
            if written != buf.len() {
                return Err(io::Error::new(io::ErrorKind::Other, "write failed"));
            }
            Ok(written)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.file.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "file handle is nullptr"));
        }

        unsafe {
            if libc::fflush(self.file) != 0 {
                return Err(io::Error::last_os_error());
            }
        }
        Ok(())
    }
}

impl io::Read for AutoFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.file.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "file handle is nullptr"));
        }

        unsafe {
            let got = libc::fread(buf.as_mut_ptr() as *mut libc::c_void, 1, buf.len(), self.file);
            if got == 0 && libc::feof(self.file) != 0 {
                return Ok(0);
            }
            Ok(got)
        }
    }
}

/* ---------------- RAII clean‑up ---------------- */

impl Drop for AutoFile {
    fn drop(&mut self) {
        if !self.file.is_null() {
            trace!(target: "autofile", "Dropping AutoFile, automatically closing {:?}", self.file);
            unsafe { libc::fclose(self.file) };
            self.file = std::ptr::null_mut();
        }
    }
}
