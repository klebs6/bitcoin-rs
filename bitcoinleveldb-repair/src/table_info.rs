// ---------------- [ File: bitcoinleveldb-repair/src/table_info.rs ]
crate::ix!();

pub struct RepairerTableInfo {
    meta:         FileMetaData,
    max_sequence: SequenceNumber,
}
