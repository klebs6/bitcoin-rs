use bitcoin_compat::*;
use bitcoin_imports::*;

/// Verify that `strnlen` matches the semantics of the
/// POSIX routine for representative inputs.
#[traced_test]
fn validate_strnlen_correctness() {
    // --------------- normal NUL‑terminated buffer -------------------------
    let buf = b"hello\0world";
    unsafe {
        assert_eq!(strnlen(buf.as_ptr() as *const i8, 11), 5, "early NUL ignored");
    }

    // --------------- buffer shorter than NUL position ---------------------
    unsafe {
        assert_eq!(strnlen(buf.as_ptr() as *const i8, 3), 3, "max_len cap violated");
    }

    // --------------- no NUL within max_len --------------------------------
    let buf2 = b"abcdef";
    unsafe {
        assert_eq!(
            strnlen(buf2.as_ptr() as *const i8, buf2.len()),
            buf2.len(),
            "unterminated buffer length mismatch"
        );
    }
}
