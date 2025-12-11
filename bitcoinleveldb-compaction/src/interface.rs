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

#[cfg(test)]
mod compaction_interface_dummy_impl_tests {
    use super::*;

    struct DummyCompactionOwner;

    impl CompactRange for DummyCompactionOwner {
        fn compact_range(
            &mut self,
            _level: i32,
            _begin: *const InternalKey,
            _end:   *const InternalKey,
        ) -> *mut Compaction {
            trace!(
                "DummyCompactionOwner::compact_range: invoked (no-op dummy implementation)"
            );
            core::ptr::null_mut()
        }
    }

    impl NeedsCompaction for DummyCompactionOwner {
        fn needs_compaction(&self) -> bool {
            trace!(
                "DummyCompactionOwner::needs_compaction: returning false (dummy)"
            );
            false
        }
    }

    impl MakeInputIteratorOverCompactionInputs for DummyCompactionOwner {
        fn make_input_iterator(&mut self, _c: *mut Compaction) -> *mut LevelDBIterator {
            trace!(
                "DummyCompactionOwner::make_input_iterator: returning null iterator (dummy)"
            );
            core::ptr::null_mut()
        }
    }

    impl PickCompaction for DummyCompactionOwner {
        fn pick_compaction(&mut self) -> *mut Compaction {
            trace!(
                "DummyCompactionOwner::pick_compaction: returning null compaction (dummy)"
            );
            core::ptr::null_mut()
        }
    }

    impl CompactionSetupOtherInputs for DummyCompactionOwner {
        fn setup_other_inputs(&mut self, _c: *mut Compaction) {
            trace!(
                "DummyCompactionOwner::setup_other_inputs: no-op (dummy)"
            );
        }
    }

    impl CompactionInterface for DummyCompactionOwner {}

    #[traced_test]
    fn dummy_compaction_interface_impl_is_callable() {
        let mut owner = DummyCompactionOwner;

        let needs = owner.needs_compaction();
        assert!(!needs);

        let picked = owner.pick_compaction();
        assert!(picked.is_null());

        let begin = InternalKey::default();
        let end   = InternalKey::default();
        let cptr  = owner.compact_range(
            0,
            &begin as *const InternalKey,
            &end as *const InternalKey,
        );
        assert!(cptr.is_null());
    }

    #[traced_test]
    fn dummy_make_input_iterator_returns_null_pointer() {
        let mut owner = DummyCompactionOwner;
        let compaction_ptr = core::ptr::null_mut::<Compaction>();

        let iter = owner.make_input_iterator(compaction_ptr);
        assert!(iter.is_null());
    }

    #[traced_test]
    fn dummy_compaction_interface_object_safe_through_trait_object() {
        let mut owner = DummyCompactionOwner;
        let iface: &mut dyn CompactionInterface = &mut owner;

        assert!(!iface.needs_compaction());

        let picked = iface.pick_compaction();
        assert!(picked.is_null());

        let begin = InternalKey::default();
        let end   = InternalKey::default();

        let range_ptr = iface.compact_range(
            1,
            &begin as *const InternalKey,
            &end as *const InternalKey,
        );
        assert!(range_ptr.is_null());

        let iter = iface.make_input_iterator(core::ptr::null_mut());
        assert!(iter.is_null());

        iface.setup_other_inputs(core::ptr::null_mut());
    }
}
