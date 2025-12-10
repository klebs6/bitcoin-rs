// ---------------- [ File: bitcoinleveldb-version/src/before_after_file.rs ]
crate::ix!();

// Helper functions mirroring LevelDB's AfterFile/BeforeFile/SomeFileOverlapsRange
pub fn after_file(user_key: Option<&Slice>, f: &FileMetaData) -> bool {
    if let Some(uk) = user_key {
        let largest = f.largest().user_key();
        bytewise_compare(slice_as_bytes(uk), slice_as_bytes(&largest)) > 0
    } else {
        // nullptr user_key occurs before all keys and is therefore never after *f
        false
    }
}

pub fn before_file(user_key: Option<&Slice>, f: &FileMetaData) -> bool {
    if let Some(uk) = user_key {
        let smallest = f.smallest().user_key();
        bytewise_compare(slice_as_bytes(uk), slice_as_bytes(&smallest)) < 0
    } else {
        // nullptr user_key occurs after all keys and is therefore never before *f
        false
    }
}
