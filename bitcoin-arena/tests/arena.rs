use bitcoin_arena::*;
use bitcoin_imports::*;

// ---------------- [ File: bitcoin-support/tests/arena.rs ]

// Helper to apply a closure and return the original value.
trait Tap: Sized {
    fn tap<F: FnOnce(&mut Self)>(mut self, f: F) -> Self {
        f(&mut self);
        self
    }
}

impl<T> Tap for *mut T {}

/// Comprehensive stress‑test ported from the original C++ suite.
#[traced_test]
fn arena_full_coverage() {
    // ------------------------------------------------------------
    //  Basic arena
    // ------------------------------------------------------------
    let synth_base: *mut c_void = 0x0800_0000usize as *mut c_void;
    let synth_size: usize       = 1_024 * 1_024; // 1 MiB
    let mut arena              = unsafe { Arena::new(synth_base, synth_size, 16) };

    // Single alloc / free
    info!("single allocation");
    let chunk = arena.alloc(1_000);
    assert!(!chunk.is_null());
    arena.free(chunk);
    assert_eq!(arena.stats().used(), 0);
    assert_eq!(arena.stats().free(), synth_size);

    // Double‑free panic
    assert!(std::panic::catch_unwind(|| arena.free(chunk)).is_err());

    // Three allocations, phased frees
    let a0 = arena.alloc(128);
    let a1 = arena.alloc(256);
    let a2 = arena.alloc(512);
    assert_eq!(arena.stats().used(), 896);
    arena.free(a0);
    arena.free(a1);
    let a3 = arena.alloc(128);
    arena.free(a2);
    arena.free(a3);
    assert_eq!(arena.stats().used(), 0);

    // Sweep allocations (1 KiB blocks)
    let mut addr: Vec<*mut c_void> = (0..1_024).map(|_| arena.alloc(1_024)).collect();
    assert_eq!(arena.stats().free(), 0);
    assert!(arena.alloc(1_024).is_null());
    for p in &addr { arena.free(*p); }
    addr.clear();
    assert_eq!(arena.stats().free(), synth_size);

    // Reverse‑order frees
    addr = (0..1_024).map(|_| arena.alloc(1_024)).collect();
    for p in addr.iter().rev() { arena.free(*p); }
    addr.clear();

    // Unequal‑sized allocations
    addr = (0..2_048).map(|x| arena.alloc(x + 1)).collect();
    for x in 0..2_048 {
        let idx = ((x * 23) % 2_048) ^ 242;
        arena.free(addr[idx]);
    }
    addr.clear();

    // Pseudo‑random stress (LFSR)
    addr.resize(2_048, null_mut());
    let mut s: u32 = 0x1234_5678;
    for _ in 0..5_000 {
        let idx = (s & ((addr.len() - 1) as u32)) as usize;
        if s & 0x8000_0000 != 0 {
            arena.free(addr[idx]);
            addr[idx] = null_mut();
        } else if addr[idx].is_null() {
            arena.alloc(((s >> 16) & 2_047) as usize)
                 .tap(|p| addr[idx] = *p);
        }
        let lsb = s & 1;
        s >>= 1;
        if lsb != 0 { s ^= 0xf00f_00f0; }
    }
    for p in &addr { arena.free(*p); }

    debug!("final stats: {:?}", arena.stats());
    assert_eq!(arena.stats().used(), 0);
    assert_eq!(arena.stats().free(), synth_size);
}
