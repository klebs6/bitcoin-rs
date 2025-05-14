// ---------------- [ File: bitcoinleveldb-table/src/iterator.rs ]
/*!
  | An iterator yields a sequence of key/value
  | pairs from a source.  The following class
  | defines the interface.  Multiple
  | implementations are provided by this library.
  | In particular, iterators are provided to access
  | the contents of a Table or a DB.
  |
  | Multiple threads can invoke const methods on an
  | Iterator without external synchronization, but
  | if any of the threads may call a non-const
  | method, all threads accessing the same Iterator
  | must use external synchronization.
  */

crate::ix!();

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

/**
  | Cleanup functions are stored in
  | a single-linked list.
  |
  | The list's head node is inlined in the
  | iterator.
  */
pub struct LevelDBIteratorCleanupNode {

    /**
      | The head node is used if the function
      | pointer is not null.
      |
      */
    function: LevelDBIteratorCleanupFunction,

    arg1:     *mut c_void,
    arg2:     *mut c_void,
    next:     *mut LevelDBIteratorCleanupNode,
}

impl LevelDBIteratorCleanupNode {

    /**
      | True if the node is not used. Only head
      | nodes might be unused.
      |
      */
    pub fn is_empty(&self) -> bool {
        
        todo!();
        /*
            return function == nullptr;
        */
    }

    /**
      | Invokes the cleanup function.
      |
      */
    pub fn run(&mut self)  {
        
        todo!();
        /*
            assert(function != nullptr);
          (*function)(arg1, arg2);
        */
    }
}

pub type LevelDBIteratorCleanupFunction = fn(arg1: *mut c_void, arg2: *mut c_void) -> c_void;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/iterator.h]

#[derive(Default)]
pub struct LevelDBIteratorInner {
    cleanup_head: Option<LevelDBIteratorCleanupNode>,
}

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

/**
  | Return an empty iterator (yields nothing).
  |
  */
pub fn new_empty_iterator() -> *mut LevelDBIteratorInner {
    
    todo!();
        /*
        
        */
}

/**
  | Return an empty iterator with the specified
  | status.
  |
  */
pub fn new_error_iterator(status: &Status) -> *mut LevelDBIterator {
    
    todo!();
        /*
        
        */
}
