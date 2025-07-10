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
