// ---------------- [ File: bitcoinleveldb-dbinterface/src/compact_range.rs ]
crate::ix!();

pub trait DBCompactRange {

    /// Compact the underlying storage for the key range [*begin,*end].  
    ///
    /// In particular, deleted and overwritten versions are discarded, and the data is rearranged
    /// to reduce the cost of operations needed to access the data.  
    ///
    /// This operation should typically only be invoked by users who understand the underlying
    /// implementation.
    /// 
    /// begin==nullptr is treated as a key before all keys in the database.  end==nullptr is
    /// treated as a key after all keys in the database.  
    ///
    /// Therefore the following call will compact the entire database: db->CompactRange(nullptr,
    /// nullptr);
    ///
    fn compact_range(&mut self, 
            begin: *const Slice,
            end:   *const Slice);
}

#[cfg(test)]
mod compact_range_contract_suite {
    use super::*;
    use core::ptr;
    use tracing::{debug, error, info, trace, warn};

    #[derive(Default)]
    struct RecordingCompactionCall {
        calls: usize,
        last_begin: *const Slice,
        last_end: *const Slice,
    }

    impl CompactRange for RecordingCompactionCall {
        fn compact_range(&mut self, begin: *const Slice, end: *const Slice) {
            trace!(?begin, ?end, "compact_range invoked");
            self.calls += 1;
            self.last_begin = begin;
            self.last_end = end;
        }
    }

    #[traced_test]
    fn compact_range_accepts_null_boundaries_and_passes_pointers_verbatim() {
        let mut recorder = RecordingCompactionCall::default();

        let compactor: &mut dyn CompactRange = &mut recorder;

        trace!("invoking compact_range with nullptr begin/end");
        compactor.compact_range(ptr::null::<Slice>(), ptr::null::<Slice>());

        assert_eq!(recorder.calls, 1);
        assert!(recorder.last_begin.is_null());
        assert!(recorder.last_end.is_null());

        info!("verified nullptr boundary semantics are representable and forwarded verbatim");
    }

    #[traced_test]
    fn compact_range_passes_non_null_slice_pointers_verbatim() {
        let mut recorder = RecordingCompactionCall::default();

        let begin = Slice::from("begin");
        let end = Slice::from("end");

        let begin_ptr: *const Slice = &begin as *const Slice;
        let end_ptr: *const Slice = &end as *const Slice;

        trace!(?begin_ptr, ?end_ptr, "invoking compact_range with non-null boundaries");
        recorder.compact_range(begin_ptr, end_ptr);

        assert_eq!(recorder.calls, 1);
        assert_eq!(recorder.last_begin, begin_ptr);
        assert_eq!(recorder.last_end, end_ptr);

        info!("verified non-null boundary pointers are forwarded verbatim");
    }

    #[traced_test]
    fn compact_range_can_be_invoked_through_trait_object_reference() {
        let mut recorder = RecordingCompactionCall::default();

        let begin = Slice::from("a");
        let end = Slice::from("z");

        let begin_ptr: *const Slice = &begin as *const Slice;
        let end_ptr: *const Slice = &end as *const Slice;

        let compactor: &mut dyn CompactRange = &mut recorder;

        debug!("calling through &mut dyn CompactRange");
        compactor.compact_range(begin_ptr, end_ptr);

        assert_eq!(recorder.calls, 1);
        assert_eq!(recorder.last_begin, begin_ptr);
        assert_eq!(recorder.last_end, end_ptr);

        info!("verified dispatch through trait object reference");
    }
}
