crate::ix!();

impl Sha256 {

    /// Reset the context to its IV, zeroing buffer & counters.
    #[inline]
    pub fn reset(&mut self) -> &mut Self {
        *self.bytes_mut() = 0;
        self.buf_mut().fill(0);
        unsafe { sha256_initialize(self.s_mut().as_mut_ptr()) };
        trace!(target: "sha256", "Sha256::reset: context re‑initialised");
        self
    }
}
