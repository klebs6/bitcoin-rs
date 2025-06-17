// ---------------- [ File: bitcoin-support/src/getuniquepath.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/getuniquepath.h]

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Join `base` with an 8‑hex‑char random component that is *very* likely unique.
pub fn get_unique_path(base: &Path) -> PathBuf {
    // Use high‑resolution timestamp mixed with address entropy for uniqueness
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards")
        .as_nanos();

    // XOR with the function address to add a little extra variability
    let mix = nanos ^ get_unique_path as usize as u128;
    let name = format!("{:08x}", mix & 0xffff_ffff); // 8 hex chars

    base.join(name)
}

#[cfg(test)]
mod get_unique_path_tests {
    use super::*;

    #[traced_test]
    fn test_unique_path_is_different() {
        let base = Path::new("/tmp");
        let p1 = get_unique_path(base);
        let p2 = get_unique_path(base);
        assert_ne!(p1, p2);
        assert!(p1.starts_with(base));
        assert_eq!(p1.file_name().unwrap().to_str().unwrap().len(), 8);
    }
}
