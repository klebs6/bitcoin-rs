crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/crypto/ctaes/ctaes.h]

/// Bitsliced AES state (8 × 16‑bit slices).
#[derive(Clone, Debug, Getters, Builder)]
#[builder(pattern = "owned")]
#[getset(get = "pub")]
pub struct AESState {
    pub(crate) slice: [u16; 8],
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

impl From<[u8;32]> for AES256_ctx {
    fn from(x: [u8;32]) -> Self {
        todo!();
    }
}
