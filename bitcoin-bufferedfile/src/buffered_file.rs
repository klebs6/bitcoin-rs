// ---------------- [ File: bitcoin-bufferedfile/src/buffered_file.rs ]
crate::ix!();

/**
  | Non-refcounted RAII wrapper around
  | a FILE* that implements a ring buffer
  | to deserialize from. It guarantees
  | the ability to rewind a given number
  | of bytes.
  | 
  | Will automatically close the file when
  | it goes out of scope if not null.
  | 
  | If you need to close the file early, use
  | file.fclose() instead of fclose(file).
  |
  */
#[no_copy]
pub struct BufferedFile {
    n_type:       i32,
    n_version:    i32,

    /**
      | source file
      |
      */
    src:          *mut libc::FILE,

    /**
      | how many bytes have been read from source
      |
      */
    n_src_pos:    u64,

    /**
      | how many bytes have been read from this
      |
      */
    n_read_pos:   u64,

    /**
      | up to which position we're allowed to
      | read
      |
      */
    n_read_limit: u64,

    /**
      | how many bytes we guarantee to rewind
      |
      */
    n_rewind:     u64,

    /**
      | the buffer
      |
      */
    vch_buf:      Vec<u8>,
}

impl Drop for BufferedFile {
    fn drop(&mut self) {
        // SAFETY: `fclose` is only called when `src` is non‑null; afterwards we
        // null‑out the pointer so a double close can never occur.
        self.fclose();
    }
}

impl BufferedFile {
    
    /// Construct a new `BufferedFile`.
    ///
    /// # Panics
    ///
    /// * If `n_rewind_in >= n_buf_size` (rewind area must be **strictly**
    ///   smaller than the buffer).
    pub fn new(
        file_in:      *mut libc::FILE,
        n_buf_size:   u64,
        n_rewind_in:  u64,
        n_type_in:    i32,
        n_version_in: i32,
    ) -> Self {
        if n_rewind_in >= n_buf_size {
            tracing::error!(
                rewind     = n_rewind_in,
                buf_size   = n_buf_size,
                "Rewind limit must be less than buffer size"
            );
            panic!("Rewind limit must be less than buffer size");
        }

        Self {
            n_type:       n_type_in,
            n_version:    n_version_in,
            src:          file_in,
            n_src_pos:    0,
            n_read_pos:   0,
            n_read_limit: u64::MAX,
            n_rewind:     n_rewind_in,
            vch_buf:      vec![0_u8; n_buf_size as usize],
        }
    }

    /// Fill the internal circular buffer from the underlying file.
    ///
    /// Returns `true` iff **any** bytes were read.
    ///
    /// # Panics
    ///
    /// * On I/O failure _or_ end‑of‑file with no data.
    pub fn fill(&mut self) -> bool {
        let buf_len = self.vch_buf.len();
        debug_assert!(buf_len > 0, "buffer length is never zero");

        let pos       = (self.n_src_pos % buf_len as u64) as usize;
        let mut read  = buf_len - pos;

        let n_avail_u64 = buf_len as u64
            - (self.n_src_pos - self.n_read_pos)
            - self.n_rewind;
        let n_avail     = n_avail_u64 as usize;

        if n_avail < read {
            read = n_avail;
        }
        if read == 0 {
            tracing::trace!(
                pos,
                n_avail,
                "No space available to read more data into buffer"
            );
            return false;
        }

        let dest = unsafe { self.vch_buf.as_mut_ptr().add(pos) } as *mut libc::c_void;
        let n_bytes = unsafe { libc::fread(dest, 1, read, self.src) };

        if n_bytes == 0 {
            let eof = unsafe { libc::feof(self.src) } != 0;
            let msg = if eof {
                "BufferedFile::fill: end of file"
            } else {
                "BufferedFile::fill: fread failed"
            };
            tracing::error!(?eof, "{}", msg);
            panic!("{msg}");
        }

        self.n_src_pos += n_bytes as u64;
        tracing::trace!(read = n_bytes, n_src_pos = self.n_src_pos, "Buffer filled");
        true
    }
    
    /// Explicitly close the wrapped `FILE*`.
    pub fn fclose(&mut self) {
        if !self.src.is_null() {
            unsafe { libc::fclose(self.src) };
            self.src = std::ptr::null_mut();
            tracing::debug!("Underlying FILE* closed");
        }
    }

    /// `true` when **both** the buffer is exhausted _and_ the underlying file
    /// reports `EOF`.
    pub fn eof(&self) -> bool {
        self.n_read_pos == self.n_src_pos && unsafe { libc::feof(self.src) } != 0
    }

    /// Read exactly `n_size` bytes into `pch` (caller‑supplied buffer).
    ///
    /// # Panics
    ///
    /// * If the read would exceed `n_read_limit`.
    /// * If the underlying file ends before `n_size` bytes are obtained.
    pub fn read(&mut self, mut pch: *mut u8, mut n_size: usize) {
        if self.n_read_pos
            .checked_add(n_size as u64)
            .map_or(true, |n| n > self.n_read_limit)
        {
            tracing::error!(
                attempted = n_size,
                read_pos  = self.n_read_pos,
                limit     = self.n_read_limit,
                "Read attempted past buffer limit"
            );
            panic!("Read attempted past buffer limit");
        }

        let buf_len = self.vch_buf.len();

        while n_size > 0 {
            if self.n_read_pos == self.n_src_pos && !self.fill() {
                tracing::error!("Unexpected EOF while reading");
                panic!("BufferedFile::read: unexpected EOF");
            }

            let pos   = (self.n_read_pos % buf_len as u64) as usize;
            let mut n = n_size;

            if n + pos > buf_len {
                n = buf_len - pos;
            }
            if (self.n_read_pos + n as u64) > self.n_src_pos {
                n = (self.n_src_pos - self.n_read_pos) as usize;
            }

            unsafe {
                std::ptr::copy_nonoverlapping(
                    self.vch_buf.as_ptr().add(pos),
                    pch,
                    n,
                );
                pch = pch.add(n);
            }

            self.n_read_pos += n as u64;
            n_size          -= n;
        }
        tracing::trace!(bytes = n_size, "Read complete");
    }

    /**
      | return the current reading position
      |
      */
    #[inline]
    pub fn get_pos(&self) -> u64 {
        self.n_read_pos
    }

    /// Attempt to set the read position; returns `true` on success.
    pub fn set_pos(&mut self, n_pos: u64) -> bool {
        let bufsize = self.vch_buf.len() as u64;

        let success = if n_pos + bufsize < self.n_src_pos {
            // Too far back, clamp to earliest possible
            self.n_read_pos = self.n_src_pos - bufsize;
            false
        } else if n_pos > self.n_src_pos {
            // Too far forward, clamp to latest possible
            self.n_read_pos = self.n_src_pos;
            false
        } else {
            self.n_read_pos = n_pos;
            true
        };

        tracing::debug!(
            requested = n_pos,
            new_pos   = self.n_read_pos,
            ?success,
            "set_pos completed"
        );
        success
    }

    /// Restrict reads to at most `n_pos` (inclusive).  
    /// Pass `None` to remove the limit.
    pub fn set_limit(&mut self, n_pos: Option<u64>) -> bool {
        let n_pos = n_pos.unwrap_or(u64::MAX);

        if n_pos < self.n_read_pos {
            tracing::warn!(
                current_pos = self.n_read_pos,
                new_limit   = n_pos,
                "Cannot set limit behind current read position"
            );
            false
        } else {
            self.n_read_limit = n_pos;
            tracing::trace!(limit = n_pos, "Read limit updated");
            true
        }
    }

    /// Search for a given byte in the stream, and remain positioned on it
    ///
    /// Advance until `ch` is found; the read pointer ends **on** that byte.
    ///
    /// # Panics
    ///
    /// * If the byte cannot be found before EOF.
    pub fn find_byte(&mut self, ch: u8) {
        let buf_len = self.vch_buf.len();

        loop {
            if self.n_read_pos == self.n_src_pos && !self.fill() {
                tracing::error!(target = ?ch, "Byte not found before EOF");
                panic!("BufferedFile::find_byte: reached EOF");
            }

            let idx = (self.n_read_pos % buf_len as u64) as usize;
            if self.vch_buf[idx] == ch {
                tracing::trace!(byte = ch, pos = self.n_read_pos, "Target byte located");
                break;
            }
            self.n_read_pos += 1;
        }
    }
}
