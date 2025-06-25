//! Interface‑level tests for `is_selectable_socket`.

use bitcoin_compat::*;
use bitcoin_imports::*;

#[cfg(not(target_os = "windows"))]
#[traced_test]
fn verify_selectable_socket_unix() {
    // A low descriptor should be selectable.
    let low: CSocket = 0;
    assert!(is_selectable_socket(&low));

    // A descriptor equal to FD_SETSIZE is *not*
    // selectable under classic `select(2)`.
    let high: CSocket = libc::FD_SETSIZE as CSocket;
    assert!(!is_selectable_socket(&high));
}

#[cfg(target_os = "windows")]
#[traced_test]
fn verify_selectable_socket_windows() {
    // Windows implementation always returns `true`.
    let any: CSocket = 123;
    assert!(is_selectable_socket(&any));
}
