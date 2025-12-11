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
        trace!(
            "CompactionState::current_output: outputs_len={}",
            self.outputs.len()
        );

        let len = self.outputs.len();
        assert!(
            len > 0,
            "CompactionState::current_output called with empty outputs vector"
        );

        let ptr: *mut compaction_state::CompactionStateOutput =
            &mut self.outputs[len - 1];

        trace!(
            "CompactionState::current_output: returning pointer {:p} to index {}",
            ptr,
            len - 1
        );

        ptr
    }

    pub fn new(c: *mut Compaction) -> Self {
        trace!(
            "CompactionState::new: creating CompactionState for compaction_ptr={:p}",
            c
        );

        let outfile: Rc<RefCell<dyn WritableFile>> =
            Rc::new(RefCell::new(StdoutPrinter {}));

        CompactionState {
            compaction: c as *const Compaction,
            smallest_snapshot: 0,
            outputs: Vec::new(),
            outfile,
            builder: core::ptr::null_mut(),
            total_bytes: 0,
        }
    }
}

#[cfg(test)]
mod compaction_state_output_tracking_tests {
    use super::*;

    #[traced_test]
    fn new_initializes_internal_fields_consistently() {
        let compaction_ptr = core::ptr::null_mut::<Compaction>();
        let state = CompactionState::new(compaction_ptr);

        assert_eq!(state.compaction, compaction_ptr as *const Compaction);
        assert_eq!(state.smallest_snapshot, 0);
        assert!(state.outputs.is_empty());
        assert!(state.builder.is_null());
        assert_eq!(state.total_bytes, 0);
    }

    #[traced_test]
    fn current_output_returns_last_element() {
        let mut state = CompactionState::new(core::ptr::null_mut());

        let o1 = CompactionStateOutput {
            number: 1,
            file_size: 10,
            smallest: InternalKey::default(),
            largest: InternalKey::default(),
        };
        let o2 = CompactionStateOutput {
            number: 2,
            file_size: 20,
            smallest: InternalKey::default(),
            largest: InternalKey::default(),
        };

        state.outputs.push(o1);
        state.outputs.push(o2);

        let ptr = state.current_output();
        assert!(!ptr.is_null());

        unsafe {
            let last = &*ptr;
            assert_eq!(last.number, 2);
            assert_eq!(last.file_size, 20);
        }
    }

    #[traced_test]
    fn current_output_pointer_can_mutate_last_element() {
        let mut state = CompactionState::new(core::ptr::null_mut());

        state.outputs.push(CompactionStateOutput {
            number: 7,
            file_size: 50,
            smallest: InternalKey::default(),
            largest: InternalKey::default(),
        });

        let ptr = state.current_output();
        assert!(!ptr.is_null());

        unsafe {
            (*ptr).file_size = 99;
        }

        assert_eq!(state.outputs.last().unwrap().file_size, 99);
    }

    #[test]
    #[should_panic(expected = "CompactionState::current_output called with empty outputs vector")]
    fn current_output_panics_when_no_outputs_present() {
        let mut state = CompactionState::new(core::ptr::null_mut());
        let _ = state.current_output();
    }
}
