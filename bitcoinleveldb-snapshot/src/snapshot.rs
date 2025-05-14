// ---------------- [ File: bitcoinleveldb-snapshot/src/snapshot.rs ]
crate::ix!();

/**
  | Abstract handle to particular state of a DB.
  |
  | A Snapshot is an immutable object and can
  | therefore be safely accessed from multiple
  | threads without any external synchronization.
  */
pub trait Snapshot {

}

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/snapshot.h]

/**
  | Snapshots are kept in a doubly-linked list in
  | the DB.
  |
  | Each SnapshotImpl corresponds to a particular
  | sequence number.
  */
pub struct SnapshotImpl {

    /**
      | SnapshotImpl is kept in a doubly-linked
      | circular list. The SnapshotList
      | implementation operates on the
      | next/previous fields direcly.
      */
    prev:            *mut SnapshotImpl,

    next:            *mut SnapshotImpl,
    sequence_number: SequenceNumber,

    #[cfg(not(NDEBUG))]
    list:            *mut SnapshotList, // default = nullptr
}

impl Snapshot for SnapshotImpl {

}

impl SnapshotImpl {
    
    pub fn new(sequence_number: SequenceNumber) -> Self {
    
        todo!();
        /*
        : sequence_number(sequence_number),

        
        */
    }
    
    pub fn sequence_number(&self) -> SequenceNumber {
        
        todo!();
        /*
            return sequence_number_;
        */
    }
}

///---------------------
pub struct SnapshotList {

    /**
      | Dummy head of doubly-linked list of
      | snapshots
      |
      */
    head: SnapshotImpl,
}

impl Default for SnapshotList {
    
    fn default() -> Self {
        todo!();
        /*
        : head(0),

            head_.prev_ = &head_;
        head_.next_ = &head_;
        */
    }
}

impl SnapshotList {
    
    pub fn empty(&self) -> bool {
        
        todo!();
        /*
            return head_.next_ == &head_;
        */
    }
    
    pub fn oldest(&self) -> *mut SnapshotImpl {
        
        todo!();
        /*
            assert(!empty());
        return head_.next_;
        */
    }
    
    pub fn newest(&self) -> *mut SnapshotImpl {
        
        todo!();
        /*
            assert(!empty());
        return head_.prev_;
        */
    }

    /**
      | Creates a SnapshotImpl and appends
      | it to the end of the list.
      |
      */
    pub fn new(&mut self, sequence_number: SequenceNumber) -> *mut SnapshotImpl {
        
        todo!();
        /*
            assert(empty() || newest()->sequence_number_ <= sequence_number);

        SnapshotImpl* snapshot = new SnapshotImpl(sequence_number);

    #if !defined(NDEBUG)
        snapshot->list_ = this;
    #endif  // !defined(NDEBUG)
        snapshot->next_ = &head_;
        snapshot->prev_ = head_.prev_;
        snapshot->prev_->next_ = snapshot;
        snapshot->next_->prev_ = snapshot;
        return snapshot;
        */
    }

    /**
      | Removes a SnapshotImpl from this list.
      |
      | The snapshot must have been created by
      | calling New() on this list.
      |
      | The snapshot pointer should not be const,
      | because its memory is deallocated. However,
      | that would force us to change
      | DB::ReleaseSnapshot(), which is in the API,
      | and currently takes a const Snapshot.
      */
    pub fn delete(&mut self, snapshot: *const SnapshotImpl)  {
        
        todo!();
        /*
            #if !defined(NDEBUG)
        assert(snapshot->list_ == this);
    #endif  // !defined(NDEBUG)
        snapshot->prev_->next_ = snapshot->next_;
        snapshot->next_->prev_ = snapshot->prev_;
        delete snapshot;
        */
    }
}
