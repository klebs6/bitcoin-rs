// ---------------- [ File: bitcoinleveldb-iterator/src/iterator_interface.rs ]
crate::ix!();

pub trait LevelDBIteratorInterface:
    Valid
    + SeekToFirst
    + SeekToLast
    + Seek
    + Next
    + Prev
    + Key
    + Value
    + LevelDBIteratorStatus { }

pub trait Valid {

    /**
      | An iterator is either positioned at
      | a key/value pair, or not valid. This
      | method returns true iff the iterator
      | is valid.
      |
      */
    fn valid(&self) -> bool;
}

pub trait SeekToFirst {

    /**
      | Position at the first key in the source.
      | The iterator is Valid() after this call
      | iff the source is not empty.
      |
      */
    fn seek_to_first(&mut self);
}

pub trait SeekToLast {

    /**
      | Position at the last key in the source.
      | The iterator is Valid() after this call
      | iff the source is not empty.
      |
      */
    fn seek_to_last(&mut self);
}

pub trait Seek {

    /**
      | Position at the first key in the source that
      | is at or past target.  The iterator is
      | Valid() after this call iff the source
      | contains an entry that comes at or past
      | target.
      */
    fn seek(&mut self, target: &Slice);
}

pub trait Next {

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

pub trait Prev {

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
