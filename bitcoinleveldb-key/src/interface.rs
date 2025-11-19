// ---------------- [ File: bitcoinleveldb-key/src/interface.rs ]
crate::ix!();

pub trait Key {

    /**
      | Return the key for the current entry.  The
      | underlying storage for the returned slice is
      | valid only until the next modification of the
      | iterator.
      |
      | REQUIRES: Valid()
      */
    fn key(&self) -> Slice;
}

pub trait Value {

    /**
      | Return the value for the current entry.  The
      | underlying storage for the returned slice is
      | valid only until the next modification of the
      | iterator.
      |
      | REQUIRES: Valid()
      */
    fn value(&self) -> Slice;
}
