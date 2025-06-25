// ---------------- [ File: bitcoin-compat/src/glibcxx_sanity.rs ]
//! Rough Rust equivalents of the classic
//! *glibcxx‑sanity* checks used by Bitcoin Core.
//!
//! The original C++ code probed three independent
//! library behaviours.  We recreate the same checks
//! with portable Rust alternatives and add *robust*
//! `tracing` so that any anomaly is surfaced in
//! production logs.
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/compat/glibcxx_sanity.cpp]

/// Trigger the widening → narrowing conversion path
/// (`char`⇆`wchar_t` in C++).  
/// In Rust we round‑trip via `char` and ensure the
/// original byte is recovered.
#[inline]
pub fn sanity_test_widen(testchar: u8) -> bool {
    trace!(
        target: "compat::glibcxx_sanity",
        func = "sanity_test_widen",
        testchar = testchar
    );

    let wide = char::from(testchar); // widen
    let narrow = wide as u32 as u8;  // narrow

    let ok = narrow == testchar;
    debug!(
        target: "compat::glibcxx_sanity",
        func = "sanity_test_widen",
        result = ok
    );
    ok
}

/// Exercise the container splice/hook machinery
/// by pushing then popping a sequence in a linked
/// list while verifying FIFO/LIFO invariants.
#[inline]
pub fn sanity_test_list(size: u32) -> bool {
    trace!(
        target: "compat::glibcxx_sanity",
        func = "sanity_test_list",
        size = size
    );

    let mut lst: LinkedList<u32> = LinkedList::new();
    for i in 0..size {
        lst.push_back(i + 1);
    }

    if lst.len() as u32 != size {
        return false;
    }

    while let Some(back) = lst.back().copied() {
        if back != lst.len() as u32 {
            return false;
        }
        lst.pop_back();
    }

    true
}

/// Force an out‑of‑bounds panic on an empty string
/// and verify it is correctly caught.
#[inline]
pub fn sanity_test_range_fmt() -> bool {
    trace!(
        target: "compat::glibcxx_sanity",
        func = "sanity_test_range_fmt"
    );

    let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let s = String::new();
        // This will panic due to out‑of‑range access,
        // mirroring `std::string::at(1)` in C++.
        let _ = s.chars().nth(1).unwrap();
    }))
    .is_err();

    debug!(
        target: "compat::glibcxx_sanity",
        func = "sanity_test_range_fmt",
        result = ok
    );

    ok
}

/// Run all sanity checks and return **true** iff all
/// individual tests succeed.
#[inline]
pub fn glibcxx_sanity_test() -> bool {
    sanity_test_widen(b'a')
        && sanity_test_list(100)
        && sanity_test_range_fmt()
}
