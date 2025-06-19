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
    T: BtcSerialize<AutoFile>,
{
    type Output = AutoFile;

    fn shl(mut self, rhs: &'a T) -> Self::Output {
        if self.file.is_null() {
            error!("AutoFile::operator<< – file handle is nullptr");
            panic!("AutoFile::operator<< – file handle is nullptr");
        }

        // Delegate to the project’s custom serialiser (bit‑exact with C++).
        BtcSerialize::<AutoFile>::serialize(rhs, &mut self);

        self
    }
}

impl<'a, T> std::ops::Shr<&'a mut T> for AutoFile
where
    T: BtcUnserialize<AutoFile>,
{
    type Output = AutoFile;

    fn shr(mut self, rhs: &'a mut T) -> Self::Output {
        if self.file.is_null() {
            error!("AutoFile::operator>> – file handle is nullptr");
            panic!("AutoFile::operator>> – file handle is nullptr");
        }

        // Delegate to the project’s custom deserialiser (bit‑exact with C++).
        BtcUnserialize::<AutoFile>::unserialize(rhs, &mut self);

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

#[cfg(test)]
mod autofile_validation {
    use super::*;

    /* -------- initialise tracing once for all tests -------- */

    /* ------------------------------------------------------------------
       helper: create a fresh AutoFile backed by libc::tmpfile()
    ------------------------------------------------------------------ */
    unsafe fn fresh_autofile() -> AutoFile {
        let file = libc::tmpfile();
        assert!(
            !file.is_null(),
            "libc::tmpfile() returned a nullptr, cannot run tests"
        );
        AutoFile::new(file, 0, 0)
    }

    /* ---------------------------------------------------
       sanity: creation yields a non‑null handle
    --------------------------------------------------- */
    #[traced_test]
    fn creation_is_not_null() {
        unsafe {
            let af = fresh_autofile();
            assert!(!af.is_null(), "new AutoFile incorrectly reports null");
        }
    }

    /* ---------------------------------------------------
       ¬ raw pointer I/O helpers round‑trip correctly
    --------------------------------------------------- */
    #[traced_test]
    fn raw_write_and_read_roundtrip() {
        unsafe {
            let mut af = fresh_autofile();

            const PAYLOAD: &[u8] = b"bitcoin-autofile";
            af.write_ptr(PAYLOAD.as_ptr(), PAYLOAD.len());

            libc::rewind(af.get());

            let mut buf = [0u8; PAYLOAD.len()];
            af.read_ptr(buf.as_mut_ptr(), buf.len());

            assert_eq!(
                &buf[..],
                PAYLOAD,
                "bytes read differ from bytes written (raw_ptr helpers)"
            );
        }
    }

    /* ---------------------------------------------------
       ¬ operator << / >> round‑trips u32 exactly
    --------------------------------------------------- */
    #[traced_test]
    fn operator_shl_shr_roundtrip_u32() {
        unsafe {
            let mut af = fresh_autofile();

            let original: u32 = 0xDEADBEEF;
            af = af << &original;       /* write */

            libc::rewind(af.get());

            let mut decoded: u32 = 0;
            af = af >> &mut decoded;    /* read */

            assert_eq!(original, decoded, "operator << / >> round‑trip mismatch");
        }
    }

    /* ---------------------------------------------------
       ¬ ignore() correctly skips bytes
    --------------------------------------------------- */
    #[traced_test]
    fn ignore_skips_expected_bytes() {
        unsafe {
            let mut af = fresh_autofile();

            const DATA: &[u8] = b"abcdef";
            af.write_ptr(DATA.as_ptr(), DATA.len());

            libc::rewind(af.get());

            af.ignore(2); /* skip “ab” */
            let mut remaining = [0u8; 4];
            af.read_ptr(remaining.as_mut_ptr(), remaining.len());

            assert_eq!(
                &remaining[..],
                &DATA[2..],
                "ignore() did not skip the requested number of bytes"
            );
        }
    }

    /* ---------------------------------------------------
       ¬ fclose() is idempotent and nullifies the handle
    --------------------------------------------------- */
    #[traced_test]
    fn fclose_is_idempotent() {
        unsafe {
            let mut af = fresh_autofile();

            af.fclose();
            assert!(af.is_null(), "fclose() did not nullify internal pointer");

            /* a second call must be a no‑op, not a crash */
            af.fclose();
            assert!(af.is_null(), "pointer resurrected after second fclose()");
        }
    }

    /* ---------------------------------------------------
       ¬ release() transfers ownership and inertifies AutoFile
    --------------------------------------------------- */
    #[traced_test]
    fn release_transfers_and_inertifies() {
        unsafe {
            let mut af = fresh_autofile();

            let raw = af.release();
            assert!(af.is_null(), "release() left AutoFile with live pointer");

            /* clean‑up manual ownership – avoid FD leak */
            assert_eq!(
                0,
                libc::fclose(raw),
                "fclose() on released pointer reported error"
            );
        }
    }

    /* ---------------------------------------------------
       ¬ std::io::Write / Read trait impls behave correctly
    --------------------------------------------------- */
    #[traced_test]
    fn stdio_write_read_traits_roundtrip() {
        unsafe {
            let mut af = fresh_autofile();

            const MSG: &[u8] = b"trait-based-I/O";
            let n = af.write(MSG).expect("Write trait failed");
            assert_eq!(n, MSG.len(), "partial write via std::io::Write");

            af.flush().expect("flush failed");

            libc::rewind(af.get());

            let mut buf = vec![0u8; MSG.len()];
            let m = af.read(&mut buf).expect("Read trait failed");
            assert_eq!(m, MSG.len(), "partial read via std::io::Read");
            assert_eq!(&buf[..], MSG, "trait I/O round‑trip mismatch");
        }
    }
}
