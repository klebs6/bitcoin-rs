// ---------------- [ File: bitcoinleveldb-dbinterface/src/interface.rs ]
crate::ix!();

/**
  | A DB is a persistent ordered map from keys to
  | values.
  |
  | A DB is safe for concurrent access from
  | multiple threads without any external
  | synchronization.
  */
pub trait DB:
    Put
    + DBOpen
    + Delete
    + Write
    + Get
    + NewIterator
    + GetSnapshot
    + ReleaseSnapshot
    + GetProperty
    + GetApproximateSizes
    + CompactRange 
{ }
