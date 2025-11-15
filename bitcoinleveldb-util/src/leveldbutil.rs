// ---------------- [ File: bitcoinleveldb-util/src/leveldbutil.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/leveldbutil.cc]

/// Print usage instructions for `leveldbutil`.
pub fn usage() {
    info!("Printing usage message for leveldbutil");
    eprintln!(
        "Usage: leveldbutil command...\n   dump files...         -- dump contents of specified files"
    );
}
