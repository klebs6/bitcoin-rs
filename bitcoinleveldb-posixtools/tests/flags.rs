// ---------------- [ File: bitcoinleveldb-posixtools/tests/flags.rs ]
use bitcoinleveldb_posixtools::*;
use bitcoin_imports::*;

#[traced_test]
fn flags_default_mmap_limit_respects_pointer_size_and_buffer_size() {
    trace!(
        "flags_default_mmap_limit_respects_pointer_size_and_buffer_size: start"
    );

    let pointer_size = std::mem::size_of::<*const ()>();

    if pointer_size >= 8 {
        assert_eq!(
            DEFAULT_MMAP_LIMIT,
            4096,
            "DEFAULT_MMAP_LIMIT should be 4096 for 64-bit targets"
        );
    } else {
        assert_eq!(
            DEFAULT_MMAP_LIMIT,
            0,
            "DEFAULT_MMAP_LIMIT should be 0 for 32-bit targets"
        );
    }

    info!(
        pointer_size,
        default_mmap_limit = DEFAULT_MMAP_LIMIT,
        "flags_default_mmap_limit_respects_pointer_size_and_buffer_size: completed"
    );
}
