// ---------------- [ File: bitcoin-sha256/src/sha256.rs ]
crate::ix!();

pub const SHA256_OUTPUT_SIZE: usize = 32;

/**
  | A hasher class for SHA-256.
  |
  | Translated with guidance from G.W.
  | Bickerstaff, 12/2021
  */
#[derive(MutGetters,Getters,Serialize,Deserialize)]
#[getset(get="pub",get_mut="pub")]
pub struct Sha256 {
    s:     [u32; 8],

    #[serde(with = "BigArray")]
    buf:   [u8; 64],
    bytes: u64,
}

impl Sha256 {

    /// Construct a fresh hasher (convenience wrapper around `Default`).
    pub fn new() -> Self { Self::default() }

    /// Initialize SHA‑256 state words to the IV defined in FIPS 180‑4.
    pub fn initialize(&mut self) {
        // Safety: caller guarantees `s` points to at least eight `u32`s.
        unsafe { sha256_initialize(self.s_mut().as_mut_ptr()) };
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
        unsafe { sha256_initialize(ctx.s_mut().as_mut_ptr()) };

        trace!(
            target: "sha256",
            state = ?ctx.s(),
            bytes = ctx.bytes(),
            "Sha256::default: new context created"
        );

        ctx
    }
}

// -----------------------------------------------------------------------------
// Tests: constructor & initialise helpers
// -----------------------------------------------------------------------------
#[cfg(test)]
mod sha256_constructor_behavior_validation {
    use super::*;
    use std::io::Write;

    /// `Sha256::new()` must be a thin wrapper over `Default`.
    #[traced_test]
    fn new_and_default_construct_equivalent_contexts() {
        let a = Sha256::new();
        let b = Sha256::default();

        assert_eq!(a.s(),     b.s(),     "state words differ");
        assert_eq!(a.buf(),   b.buf(),   "internal buffer differs");
        assert_eq!(*a.bytes(), *b.bytes(), "byte counters differ");
    }

    /// Re‑calling `initialize()` on an existing context must overwrite the
    /// state words with the canonical IV **without** touching the buffer
    /// or byte counter.
    #[traced_test]
    fn initialize_overwrites_state_only() {
        const FILL: &[u8] = b"buffer-fill-data-for-test";

        let mut ctx = Sha256::new();
        ctx.write_all(FILL).unwrap();
        let buf_before = ctx.buf().clone();
        let bytes_before = *ctx.bytes();

        // ── SUT ─────────────────────────────────────────────────────────────
        ctx.initialize();

        // Reference IV
        const IV: [u32; 8] = [
            0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
            0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
        ];

        assert_eq!(*ctx.s(), IV, "state words not reset to IV");
        assert_eq!(ctx.buf(), &buf_before, "buffer was unexpectedly modified");
        assert_eq!(
            *ctx.bytes(), bytes_before,
            "byte counter was changed by initialize()"
        );
    }
}
