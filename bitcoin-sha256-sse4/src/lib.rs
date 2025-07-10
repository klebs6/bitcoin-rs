#![cfg(any(__x86_64__,__amd64__))]

#[macro_use] mod imports; use imports::*;

x!{bitcoin_sha256_sse4}
