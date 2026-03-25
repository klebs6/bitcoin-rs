// ---------------- [ File: bitcoinleveldb-snapshot/src/snapshot_impl.rs ]
crate::ix!();

/**
  | Snapshots are kept in a doubly-linked list in
  | the DB.
  |
  | Each SnapshotImpl corresponds to a particular
  | sequence number.
  */
#[derive(Getters,Setters)]
#[getset(get="pub",set="pub")]
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

    fn snapshot_runtime_implementation_kind(&self) -> SnapshotDispatchConcreteImplementationKind {
        trace!(
            target: "bitcoinleveldb_snapshot::snapshot_impl",
            event = "snapshot_impl_snapshot_runtime_implementation_kind_entry",
            sequence_number = *self.sequence_number()
        );

        let implementation_kind = SnapshotDispatchConcreteImplementationKind::SnapshotImpl;

        trace!(
            target: "bitcoinleveldb_snapshot::snapshot_impl",
            event = "snapshot_impl_snapshot_runtime_implementation_kind_exit",
            implementation_kind = ?implementation_kind,
            sequence_number = *self.sequence_number()
        );

        implementation_kind
    }

    fn snapshot_sequence_number_for_read_reconstruction(&self) -> Option<SequenceNumber> {
        trace!(
            target: "bitcoinleveldb_snapshot::snapshot_impl",
            event = "snapshot_impl_snapshot_sequence_number_for_read_reconstruction_entry",
            sequence_number = *self.sequence_number()
        );

        let sequence_number = Some(*self.sequence_number());

        trace!(
            target: "bitcoinleveldb_snapshot::snapshot_impl",
            event = "snapshot_impl_snapshot_sequence_number_for_read_reconstruction_exit",
            sequence_number = *self.sequence_number()
        );

        sequence_number
    }

    fn snapshot_read_arc_clone(&self) -> Option<Arc<dyn Snapshot>> {
        trace!(
            target: "bitcoinleveldb_snapshot::snapshot_impl",
            event = "snapshot_impl_snapshot_read_arc_clone_entry",
            sequence_number = *self.sequence_number()
        );

        let sequence_number = *self.sequence_number();
        let snapshot_arc: Arc<dyn Snapshot> = Arc::new(SnapshotImpl::new(sequence_number));

        trace!(
            target: "bitcoinleveldb_snapshot::snapshot_impl",
            event = "snapshot_impl_snapshot_read_arc_clone_exit",
            sequence_number = sequence_number
        );

        Some(snapshot_arc)
    }
}

impl SnapshotImpl {
    pub fn new(sequence_number: SequenceNumber) -> Self {
        debug!(
            "creating SnapshotImpl with sequence_number={}",
            sequence_number
        );

        SnapshotImpl {
            prev: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
            sequence_number,
            #[cfg(not(NDEBUG))]
            list: core::ptr::null_mut(),
        }
    }
}

#[cfg(test)]
mod snapshot_impl_construction_and_access_spec {
    use super::*;

    #[traced_test]
    fn constructing_snapshot_impl_initializes_links_to_null() {
        debug!("constructing SnapshotImpl and validating initial link pointers are null");

        let s = SnapshotImpl::new(7 as SequenceNumber);

        let prev_is_null = unsafe { (*s.prev()).is_null() };
        let next_is_null = unsafe { (*s.next()).is_null() };

        trace!(
            prev_ptr = ?s.prev(),
            next_ptr = ?s.next(),
            prev_is_null,
            next_is_null,
            "validated initial raw pointers on SnapshotImpl"
        );

        assert!(prev_is_null, "new() must initialize prev to null");
        assert!(next_is_null, "new() must initialize next to null");
    }

    #[traced_test]
    fn sequence_number_getter_exposes_value_passed_to_constructor() {
        debug!("verifying sequence_number getter");

        let s = SnapshotImpl::new(12345 as SequenceNumber);
        assert_eq!(
            *s.sequence_number(),
            12345,
            "expected the same sequence number to be accessible via getter"
        );
    }
}
