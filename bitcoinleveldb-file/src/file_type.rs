// ---------------- [ File: bitcoinleveldb-file/src/file_type.rs ]
crate::ix!();

pub enum FileType {
    LogFile,
    DBLockFile,
    TableFile,
    DescriptorFile,
    CurrentFile,
    TempFile,
    InfoLogFile  // Either the current one, or an old one
}
