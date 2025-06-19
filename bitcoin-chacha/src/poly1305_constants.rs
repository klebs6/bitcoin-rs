crate::ix!();

pub const CHACHA20_POLY1305_AEAD_KEY_LEN: usize = 32;
pub const CHACHA20_POLY1305_AEAD_AAD_LEN: usize = 3;  // 3 bytes length
pub const CHACHA20_ROUND_OUTPUT:          usize = 64; // 64 bytes per round
pub const AAD_PACKAGES_PER_ROUND:         usize = 21; // 64 / 3 round down

pub const POLY1305_TAGLEN: usize = 16;
pub const POLY1305_KEYLEN: usize = 32;
