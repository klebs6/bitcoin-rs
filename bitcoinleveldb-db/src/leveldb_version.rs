// ---------------- [ File: bitcoinleveldb-db/src/leveldb_version.rs ]
crate::ix!();

// Update CMakeLists.txt if you change these
pub const MAJOR_VERSION: i32 = 1;
pub const MINOR_VERSION: i32 = 22;

pub fn leveldb_major_version() -> i32 {
    
    todo!();
        /*
            return kMajorVersion;
        */
}

pub fn leveldb_minor_version() -> i32 {
    
    todo!();
        /*
            return kMinorVersion;
        */
}

pub fn leveldb_major_version() -> i32 {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_major_version");
    MAJOR_VERSION

    /*
        return kMajorVersion;
    */
}

pub fn leveldb_minor_version() -> i32 {
    trace!(target: "bitcoinleveldb_db::c_api", "leveldb_minor_version");
    MINOR_VERSION

    /*
        return kMinorVersion;
    */
}
