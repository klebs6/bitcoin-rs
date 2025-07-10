// ---------------- [ File: bitcoin-sha256/src/lib.rs ]
#[macro_use] mod imports; use imports::*;

pub use bitcoin_sha256_sse4::*;
pub use bitcoin_sha256_sse41::*;
pub use bitcoin_sha256_shani::*;
pub use bitcoin_sha256_avx2::*;
pub use bitcoin_sha256_hkdf::*;

x!{beio_helpers}
x!{majority_and_choice}
x!{self_test}
x!{sha256}
x!{sha256_auto_detect}
x!{sha256_finalize}
x!{sha256_initialize}
x!{sha256_initialize_tagged}
x!{sha256_reset}
x!{sha256_round}
x!{sha256_transform}
x!{sha256_transform_block}
x!{sha256_transform_one_block}
x!{sha256_write}
x!{sha256_write_ffi}
x!{sigma}
x!{transforms}
x!{transform_d64_scalar}
x!{compute}
