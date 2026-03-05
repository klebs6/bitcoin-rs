// ---------------- [ File: bitcoinleveldb-db/src/leveldb_version.rs ]
crate::ix!();

// Update CMakeLists.txt if you change these
pub const MAJOR_VERSION: i32 = 1;
pub const MINOR_VERSION: i32 = 22;

pub fn leveldb_major_version() -> i32 {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_major_version");
    MAJOR_VERSION
}

pub fn leveldb_minor_version() -> i32 {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_minor_version");
    MINOR_VERSION
}

#[cfg(test)]
mod bitcoinleveldb_db__leveldb_version_rs__exhaustive_test_suite {
    use super::*;

    #[traced_test]
    fn bitcoinleveldb_db__leveldb_version_rs__constants_match_accessors() {
        let major: i32 = leveldb_major_version();
        let minor: i32 = leveldb_minor_version();

        assert_eq!(major, MAJOR_VERSION);
        assert_eq!(minor, MINOR_VERSION);
        assert_eq!(major, 1);
        assert_eq!(minor, 22);
    }
}
