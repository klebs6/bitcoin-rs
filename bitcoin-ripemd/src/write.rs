// ---------------- [ File: bitcoin-ripemd/src/write.rs ]
crate::ix!();

impl Ripemd160 {
    /**
      | Low‑level ingestion of raw bytes.
      | Users normally call `update`, which forwards
      | to this pointer/length API.
      */
    #[inline]
    pub fn write(&mut self, data: *const u8, len: usize) -> &mut Self {
        use core::ptr;

        unsafe {
            let mut src = data;
            let end = src.add(len);
            let mut bufsize = (self.bytes() % 64) as usize;

            /* --- finish a partially filled buffer if needed --- */
            if bufsize != 0 && bufsize + len >= 64 {
                ptr::copy_nonoverlapping(
                    src,
                    self.buf_mut().as_mut_ptr().add(bufsize),
                    64 - bufsize,
                );
                *self.bytes_mut() += (64 - bufsize) as u64;
                src = src.add(64 - bufsize);
                ripemd160_transform(self.s_mut().as_mut_ptr(), self.buf().as_ptr());
                bufsize = 0;
            }

            /* --- main fast path: full 64‑byte chunks --- */
            while end.offset_from(src) as usize >= 64 {
                ripemd160_transform(self.s_mut().as_mut_ptr(), src);
                *self.bytes_mut() += 64;
                src = src.add(64);
            }

            /* --- copy trailing bytes into the buffer --- */
            if end > src {
                let remaining = end.offset_from(src) as usize;
                ptr::copy_nonoverlapping(
                    src,
                    self.buf_mut().as_mut_ptr().add(bufsize),
                    remaining,
                );
                *self.bytes_mut() += remaining as u64;
            }
        }

        tracing::trace!(target: "ripemd160::io", "absorbed {} bytes", len);
        self
    }
}

#[cfg(test)]
mod spec_write {
    use super::*;

    /// Feeds identical data through two radically
    /// different chunking strategies and ensures the
    /// resulting internal state matches.
    #[traced_test]
    fn write_handles_arbitrary_chunking() {
        let payload = [0x99u8; 100]; /* 100 bytes crosses two blocks */

        /* one‑shot update */
        let mut hasher_a = Ripemd160::default();
        hasher_a.update(&payload);

        /* byte‑wise update */
        let mut hasher_b = Ripemd160::default();
        for byte in &payload {
            hasher_b.update(&[*byte]);
        }

        assert_eq!(
            hasher_a.s(), hasher_b.s(),
            "state mismatch after different chunkings"
        );
        assert_eq!(
            hasher_a.bytes(), hasher_b.bytes(),
            "byte counters differ"
        );
    }
}
