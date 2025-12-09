// ---------------- [ File: bitcoinleveldb-compaction/src/compaction_state.rs ]
crate::ix!();

/**
  | Files produced by compaction
  |
  */
pub struct CompactionStateOutput {
    number:    u64,
    file_size: u64,
    smallest:  InternalKey,
    largest:   InternalKey,
}

pub struct CompactionState {

    compaction:        *const Compaction,

    /**
      | Sequence numbers < smallest_snapshot
      | are not significant since we will never
      | have to service a snapshot below smallest_snapshot.
      | 
      | Therefore if we have seen a sequence
      | number S <= smallest_snapshot, we can
      | drop all entries for the same key with
      | sequence numbers < S.
      |
      */
    smallest_snapshot: SequenceNumber,

    outputs:           Vec<compaction_state::CompactionStateOutput>,

    /**
      | State kept for output being generated
      |
      */
    outfile:           Rc<RefCell<dyn WritableFile>>,

    builder:           *mut TableBuilder,
    total_bytes:       u64,
}

impl CompactionState {

    pub fn current_output(&mut self) -> *mut compaction_state::CompactionStateOutput {
        
        todo!();
        /*
            return &outputs[outputs.size() - 1];
        */
    }
    
    pub fn new(c: *mut Compaction) -> Self {
    
        todo!();
        /*
        : compaction(c),
        : smallest_snapshot(0),
        : outfile(nullptr),
        : builder(nullptr),
        : total_bytes(0),
        */
    }
}
