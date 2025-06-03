// ---------------- [ File: bitcoinleveldb-util/src/leveldbutil.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/leveldbutil.cc]

/// A trait matching the C++ interface that returns a raw pointer to a name.
pub trait Name {
    fn name(&self) -> *const u8;
}

/// A trait that gets a name as a `&'static str`, purely for error reporting.
pub trait GetName {
    fn get_name(&self) -> &'static str;
}

/// Print usage instructions for `leveldbutil`.
pub fn usage() {
    info!("Printing usage message for leveldbutil");
    eprintln!(
        "Usage: leveldbutil command...\n   dump files...         -- dump contents of specified files"
    );
}
