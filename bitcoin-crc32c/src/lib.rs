// ---------------- [ File: bitcoin-crc32c/src/lib.rs ]
#![feature(test)]

#[macro_use] mod imports; use imports::*;

#[cfg(target_arch = "aarch64")]
x!{arm64}

#[cfg(target_arch = "x86_64")]
x!{sse42}

x!{interface}
x!{portable}
x!{prefetch}
x!{read_le}
x!{round_up}
x!{platform}
