// ---------------- [ File: bitcoinleveldb-compaction/src/interface.rs ]
crate::ix!();

pub trait CompactionInterface
: CompactRange
+ NeedsCompaction
+ MakeInputIteratorOverCompactionInputs
+ PickCompaction
+ CompactionSetupOtherInputs
{}

pub trait CompactRange {

    /**
      | Return a compaction object for compacting the
      | range [begin,end] in the specified level.
      | Returns nullptr if there is nothing in that
      | level that overlaps the specified range.
      | Caller should delete the result.
      */
    fn compact_range(
        &mut self, 
        level: i32,
        begin: *const InternalKey,
        end:   *const InternalKey) -> *mut Compaction;
}

pub trait NeedsCompaction {

    /**
      | Returns true iff some level needs a compaction.
      |
      */
    fn needs_compaction(&self) -> bool;
}

pub trait MakeInputIteratorOverCompactionInputs {

    /**
      | Create an iterator that reads over the
      | compaction inputs for "*c".
      |
      | The caller should delete the iterator when no
      | longer needed.
      */
    fn make_input_iterator(&mut self, c: *mut Compaction) -> *mut LevelDBIterator;
}

pub trait PickCompaction {

    /**
      | Pick level and inputs for a new compaction.
      |
      | Returns nullptr if there is no compaction to
      | be done.
      |
      | Otherwise returns a pointer to
      | a heap-allocated object that describes the
      | compaction.  Caller should delete the result.
      */
    fn pick_compaction(&mut self) -> *mut Compaction;
}

pub trait CompactionSetupOtherInputs {
    
    fn setup_other_inputs(&mut self, c: *mut Compaction);
}

