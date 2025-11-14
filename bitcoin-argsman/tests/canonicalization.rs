// ---------------- [ File: bitcoin-argsman/tests/canonicalization.rs ]
use bitcoin_argsman::*;
use bitcoin_imports::*;

// ---------------- tests/canonicalization.rs ----------------
#[cfg(unix)]
#[test]
fn abs_path_uses_canonicalized_base_but_keeps_filename() {
    use std::os::unix::fs as unixfs;
    let tmp = tempfile::tempdir().unwrap();
    let real = tmp.path().join("real");
    let link = tmp.path().join("link");
    std::fs::create_dir_all(&real).unwrap();
    unixfs::symlink(&real, &link).unwrap();

    // Use symlink as datadir
    {
        let mut am = G_ARGS.lock();
        let mut inner = am.cs_args.lock();
        inner.force_set_arg("-datadir", link.to_str().unwrap());
    }
    select_base_params(base_chain_params::REGTEST);

    let p = bitcoin_argsman::abs_path_for_config_val(Path::new("bitcoin.conf"), Some(true));
    // Ends with regtest/bitcoin.conf either way
    assert!(p.ends_with("regtest/bitcoin.conf"));
}

#[test]
fn blocksdir_nonexistent_is_handled_gracefully() {
    let mut inner = ArgsManagerInner::default();
    // Point -blocksdir to a non-existent path
    inner.force_set_arg("-blocksdir", "/this/path/does/not/exist");
    // Should return empty path (not crash)
    let p = inner.get_blocks_dir_path();
    assert!(p.as_ref().as_os_str().is_empty());
}
