// ---------------- [ File: bitcoin-aes/src/aes_state.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/ctaes/ctaes.h]

/// Bitsliced AES state (8 × 16‑bit slices).
#[derive(Copy,Default,Clone, Debug, Getters, Builder)]
#[builder(pattern = "owned")]
#[getset(get = "pub")]
pub struct AESState {
    pub(crate) slice: [u16; 8],
}

impl AESState {
    /// Build directly from an 8‑word slice.
    #[inline(always)]
    pub fn from_slice(slice: [u16; 8]) -> Self {
        Self { slice }
    }

    /// Generate a random state (test‑only helper).
    #[cfg(test)]
    pub fn random<R: rand::Rng + ?Sized>(rng: &mut R) -> Self {
        let mut slice = [0u16; 8];
        for word in &mut slice {
            *word = rng.gen();
        }
        Self { slice }
    }
}

#[derive(Default)]
pub struct AES128_ctx {
    pub(crate) rk: [AESState; 11],
}

#[derive(Default)]
pub struct AES192_ctx {
    pub(crate) rk: [AESState; 13],
}

#[derive(Default)]
pub struct AES256_ctx {
    pub(crate) rk: [AESState; 15],
}

impl From<[u8; 32]> for AES256_ctx {
    /// Construct an `AES256_ctx` with its round‑key schedule fully
    /// expanded from the provided 256‑bit key.
    fn from(key: [u8; 32]) -> Self {
        tracing::info!(target: "aes", "AES256_ctx::from – key‑schedule setup");

        let mut ctx = Self::default();
        unsafe { aes256_init(&mut ctx as *mut _, key.as_ptr()) };

        tracing::debug!(target: "aes", "AES256_ctx::from – completed");
        ctx
    }
}

