crate::ix!();

pub trait NewIterator {

    /**
      | Return a heap-allocated iterator over the
      | contents of the database.  The result of
      | NewIterator() is initially invalid (caller
      | must call one of the Seek methods on the
      | iterator before using it).
      |
      | Caller should delete the iterator when it is
      | no longer needed.  The returned iterator
      | should be deleted before this db is deleted.
      */
    fn new_iterator(&mut self, options: &ReadOptions) -> *mut LevelDBIterator;
}
