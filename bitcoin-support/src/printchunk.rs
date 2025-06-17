// ---------------- [ File: bitcoin-support/src/lockedpool.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/support/lockedpool.cpp]

// -----------------------------------------------------------------------------
// [bitcoin-support/src/lockedpool.rs] – printchunk (only when ARENA_DEBUG)
// -----------------------------------------------------------------------------
#[cfg(ARENA_DEBUG)]
pub fn printchunk(base: *mut c_void, sz: usize, used: bool) {
    use std::fmt::Write;

    let mut line = String::new();
    write!(
        &mut line,
        "0x{:016X} 0x{:016X} 0x{}",
        base as usize, sz, used as u8
    )
    .unwrap();
    println!("{}", line);
}
