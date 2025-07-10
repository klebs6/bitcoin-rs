crate::ix!();

/**
  | A hasher class for SHA-256.
  |
  | Translated with guidance from G.W.
  | Bickerstaff, 12/2021
  */
#[derive(Serialize,Deserialize)]
pub struct Sha256 {
    s:     [u32; 8],

    #[serde(with = "BigArray")]
    buf:   [u8; 64],
    bytes: u64,
}

//-------------------------------------------[.cpp/bitcoin/src/crypto/sha256.cpp]
impl Write for Sha256 {

    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.write_ptr(buf.as_ptr(), buf.len());
        Ok(buf.len())
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        /* stateless – nothing buffered that needs flushing */
        Ok(())
    }
}

impl Sha256 {

    /// Construct a fresh hasher (convenience wrapper around `Default`).
    pub fn new() -> Self { Self::default() }

    /// Initialize SHA‑256 state words to the IV defined in FIPS 180‑4.
    pub fn initialize(&mut self) {
        // Safety: caller guarantees `s` points to at least eight `u32`s.
        unsafe { sha256_initialize(self.s.as_mut_ptr()) };
    }

    /// Stream data supplied by any iterator **exactly** `len` bytes long.
    ///
    /// # Panics
    /// * If the iterator yields fewer than `len` bytes.
    pub fn write_from_iterator(
        &mut self,
        mut data: Box<dyn Iterator<Item = u8>>,
        len: usize,
    ) {
        use std::vec::Vec;
        let mut buf = Vec::<u8>::with_capacity(len);
        for _ in 0..len {
            match data.next() {
                Some(b) => buf.push(b),
                None => panic!("write_from_iterator: iterator ended before `len` bytes were read"),
            }
        }
        self.write_ptr(buf.as_ptr(), len);
    }

    /// **Low‑level** pointer‑based writer used by all higher‑level entry points.
    ///
    /// Safety invariants mirror the original C++ logic and are upheld internally.
    pub fn write_ptr(&mut self, mut data: *const u8, mut len: usize) {
        trace!(
            target: "sha256",
            len,
            bytes_before = self.bytes,
            "Sha256::write_ptr: begin"
        );

        let end = unsafe { data.add(len) };
        let mut bufsize = (self.bytes % 64) as usize;

        /* ----------------------------------------------------------
         * 1. Top‑up internal buffer to 64 B if it is already partially
         *    filled **and** we have enough incoming data to reach 64 B.
         * -------------------------------------------------------- */
        if bufsize != 0 && bufsize + len >= 64 {
            let fill = 64 - bufsize;
            unsafe {
                copy_nonoverlapping(
                    data,
                    self.buf.as_mut_ptr().add(bufsize),
                    fill,
                );
            }
            self.bytes += fill as u64;
            data = unsafe { data.add(fill) };
            len -= fill;
            bufsize = 0;

            /* process the now‑full internal buffer */
            unsafe {
                sha256_transform(self.s.as_mut_ptr(), self.buf.as_ptr(), 1);
            }
        }

        /* ----------------------------------------------------------
         * 2. While we still have full 64‑byte blocks, process them
         *    directly from the caller’s memory without staging.
         * -------------------------------------------------------- */
        if len >= 64 {
            let blocks = len / 64;
            let proc_bytes = blocks * 64;
            unsafe {
                sha256_transform(self.s.as_mut_ptr(), data, blocks);
                data = data.add(proc_bytes);
            }
            self.bytes += proc_bytes as u64;
            len -= proc_bytes;
        }

        /* ----------------------------------------------------------
         * 3. Any trailing bytes (< 64) are buffered for later.
         * -------------------------------------------------------- */
        if len > 0 {
            unsafe {
                copy_nonoverlapping(
                    data,
                    self.buf.as_mut_ptr().add(bufsize),
                    len,
                );
            }
            self.bytes += len as u64;
        }

        trace!(
            target: "sha256",
            bytes_after = self.bytes,
            "Sha256::write_ptr: end"
        );
    }


    /// Finalise the hash and copy the **32‑byte** digest into `hash`.
    ///
    /// After finalisation the context **remains valid** and can be reused
    /// after calling [`reset`].
    pub fn finalize(&mut self, hash: &mut [u8; SHA256_OUTPUT_SIZE]) {
        /* ---------- Step 1: 0x80 pad followed by zeroes ------------ */
        const PAD: [u8; 64] = [0x80; 64];

        /* length in *bits* (big‑endian) – eight‑byte descriptor */
        let mut sizedesc = [0u8; 8];
        beio::u64_into(&mut sizedesc, self.bytes << 3);

        /* write 1 + ((119 − bytes) mod 64)  bytes of padding        */
        let pad_len = 1 + ((119 - (self.bytes % 64)) % 64) as usize;
        self.write_ptr(PAD.as_ptr(), pad_len);

        /* write 8‑byte length descriptor                             */
        self.write_ptr(sizedesc.as_ptr(), 8);

        /* ---------- Step 2: Serialise internal state big‑endian ---- */
        for (chunk, &word) in hash.chunks_exact_mut(4).zip(self.s.iter()) {
            beio::u32_into(chunk, word);
        }

        trace!(target: "sha256", digest = ?hash, "Sha256::finalize completed");
    }
   
    /// Reset the context to its IV, zeroing buffer & counters.
    #[inline]
    pub fn reset(&mut self) -> &mut Self {
        self.bytes = 0;
        self.buf.fill(0);
        unsafe { sha256_initialize(self.s.as_mut_ptr()) };
        trace!(target: "sha256", "Sha256::reset: context re‑initialised");
        self
    }
}

/**
  | Write the SHA‑256 IV (FIPS 180‑4 section 5.3.3) into the caller‑supplied
  | 8‑word state array *s*.
  |
  | # Safety
  | * `s` **must** point to **at least** eight valid `u32` values.
  | * The memory region referenced by `s` must be writable for the duration of
  |   the call.
  |
  | This routine is intentionally `unsafe` because it performs raw pointer
  | arithmetic.  The caller is responsible for upholding the above contract.
  |
  | Logging is performed at `TRACE` level under the `"sha256"` target so that
  | production builds can retain the calls with minimal overhead when the
  | `max_level_trace` feature is disabled.
  */
#[inline]
pub unsafe fn sha256_initialize(s: *mut u32) {
    const IV: [u32; 8] = [
        0x6a09e667,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];

    for (i, &word) in IV.iter().enumerate() {
        // SAFETY: caller guarantees `s` has room for eight `u32`s.
        *s.add(i) = word;
    }
}

impl Default for Sha256 {
    /// Construct a freshly‑initialised `Sha256` value whose internal
    /// state words equal the FIPS 180‑4 IV and whose buffer/byte‑count
    /// are zeroed.
    ///
    /// This is the canonical entry‑point used throughout `bitcoin‑core`
    /// for starting a new hash computation.
    fn default() -> Self {
        // Start with zeroed storage so that, even on panic before the
        // call to `sha256_initialize`, no uninitialised data can leak.
        let mut ctx = Sha256 {
            s:     [0u32; 8],
            buf:   [0u8; 64],
            bytes: 0,
        };

        // SAFETY: `ctx.s` is a valid, writable eight‑word region.
        unsafe { sha256_initialize(ctx.s.as_mut_ptr()) };

        trace!(
            target: "sha256",
            state = ?ctx.s,
            bytes = ctx.bytes,
            "Sha256::default: new context created"
        );

        ctx
    }
}

#[cfg(test)]
mod sha256_initialisation_tests {
    use super::*;

    /// Expected FIPS 180‑4 IV for SHA‑256, expressed in little‑endian host order.
    const IV: [u32; 8] = [
        0x6a09e667,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];

    #[traced_test]
    fn default_constructor_sets_iv_and_zeros() {
        let ctx = Sha256::default();

        // State words must equal the IV.
        assert_eq!(ctx.s, IV, "state words do not match FIPS 180‑4 IV");

        // Buffer must start zero‑filled.
        assert!(ctx.buf.iter().all(|&b| b == 0), "buffer not zero‑initialised");

        // No bytes should have been processed yet.
        assert_eq!(ctx.bytes, 0, "byte counter not initialised to zero");
    }

    #[traced_test]
    fn pointer_initialiser_writes_correct_values() {
        let mut state = [0u32; 8];

        // SAFETY: `state.as_mut_ptr()` is valid for eight u32s.
        unsafe { sha256_initialize(state.as_mut_ptr()) };

        assert_eq!(state, IV, "sha256_initialize did not write canonical IV");
    }
}
