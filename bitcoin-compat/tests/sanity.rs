use bitcoin_compat::*;
use bitcoin_imports::*;

/// All internal probes must pass on every supported
/// platform and toolchain.
#[traced_test]
fn sanity_checks_pass() {
    assert!(
        glibcxx_sanity_test(),
        "glibcxx_sanity_test reported failure"
    );
}
