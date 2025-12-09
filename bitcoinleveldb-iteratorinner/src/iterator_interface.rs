// ---------------- [ File: bitcoinleveldb-iteratorinner/src/iterator_interface.rs ]
crate::ix!();

pub trait LevelDBIteratorInterface:
    LevelDBIteratorValid
    + LevelDBIteratorSeekToFirst
    + LevelDBIteratorSeekToLast
    + LevelDBIteratorSeek
    + LevelDBIteratorNext
    + LevelDBIteratorPrev
    + LevelDBIteratorKey
    + LevelDBIteratorValue
    + LevelDBIteratorStatus 
{ }

pub trait LevelDBIteratorValid {

    /**
      | An iterator is either positioned at
      | a key/value pair, or not valid. This
      | method returns true iff the iterator
      | is valid.
      |
      */
    fn valid(&self) -> bool;
}

pub trait LevelDBIteratorSeekToFirst {

    /**
      | Position at the first key in the source.
      | The iterator is Valid() after this call
      | iff the source is not empty.
      |
      */
    fn seek_to_first(&mut self);
}

pub trait LevelDBIteratorSeekToLast {

    /**
      | Position at the last key in the source.
      | The iterator is Valid() after this call
      | iff the source is not empty.
      |
      */
    fn seek_to_last(&mut self);
}

pub trait LevelDBIteratorSeek {

    /**
      | Position at the first key in the source that
      | is at or past target.  The iterator is
      | Valid() after this call iff the source
      | contains an entry that comes at or past
      | target.
      */
    fn seek(&mut self, target: &Slice);
}

pub trait LevelDBIteratorNext {

    /**
      | Moves to the next entry in the source.  After
      | this call, Valid() is true iff the iterator
      | was not positioned at the last entry in the
      | source.
      |
      | REQUIRES: Valid()
      */
    fn next(&mut self);
}

pub trait LevelDBIteratorPrev {

    /**
      | Moves to the previous entry in the source.
      | After this call, Valid() is true iff the
      | iterator was not positioned at the first
      | entry in source.
      |
      | REQUIRES: Valid()
      */
    fn prev(&mut self);
}

pub trait LevelDBIteratorStatus {

    /**
      | If an error has occurred, return it.
      | Else return an ok status.
      |
      */
    fn status(&self) -> crate::Status;
}

pub trait LevelDBIteratorKey {

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

pub trait LevelDBIteratorValue {

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

pub trait RegisterCleanup {
    fn register_cleanup(
        &self,
        func:  LevelDBIteratorCleanupFunction,
        arg1:  *mut c_void,
        arg2:  *mut c_void
    );
}

#[cfg(test)]
mod tests_iterator_interface_object_safety {
    use super::*;

    #[traced_test]
    fn iterator_interface_trait_object_can_be_named() {
        // We only care that the trait object compiles and can appear in
        // a type like Option; we do not need any concrete implementor
        // here.
        let opt: Option<&dyn LevelDBIteratorInterface> = None;
        assert!(opt.is_none());
    }
}
