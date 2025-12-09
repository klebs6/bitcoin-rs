// ---------------- [ File: bitcoinleveldb-versionsetutil/src/file_set.rs ]
crate::ix!();

pub type VersionSetBuilderFileSet = HashSet<*mut FileMetaData,BySmallestKeyComparator>;
