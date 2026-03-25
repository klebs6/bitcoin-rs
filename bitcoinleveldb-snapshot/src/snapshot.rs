// ---------------- [ File: bitcoinleveldb-snapshot/src/snapshot.rs ]
crate::ix!();

pub trait WriteSnapshot {
    
    /**
      | Save current contents to *log
      |
      */
    fn write_snapshot(&mut self, log: &mut LogWriter) -> Status;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotDispatchConcreteImplementationKind {
    ModelSnapshot,
    SnapshotImpl,
    Unsupported,
}

/// Abstract handle to particular state of a DB.
/// 
/// A Snapshot is an immutable object and can therefore be safely accessed from multiple threads
/// without any external synchronization.
///
pub trait Snapshot {

    /// Invariant: runtime classification is semantic and must remain stable for
    /// the lifetime of the snapshot object. Callers must not infer this from
    /// trait-object metadata addresses.
    fn snapshot_runtime_implementation_kind(&self) -> SnapshotDispatchConcreteImplementationKind {
        trace!(
            target: "bitcoinleveldb_snapshot::snapshot",
            event = "snapshot_runtime_implementation_kind_default_entry"
        );

        let implementation_kind = SnapshotDispatchConcreteImplementationKind::Unsupported;

        trace!(
            target: "bitcoinleveldb_snapshot::snapshot",
            event = "snapshot_runtime_implementation_kind_default_exit",
            implementation_kind = ?implementation_kind
        );

        implementation_kind
    }

    /// Invariant: a returned sequence number is valid only when sequence-only
    /// reconstruction preserves the original read boundary exactly.
    fn snapshot_sequence_number_for_read_reconstruction(&self) -> Option<SequenceNumber> {
        trace!(
            target: "bitcoinleveldb_snapshot::snapshot",
            event = "snapshot_sequence_number_for_read_reconstruction_default_entry"
        );

        let sequence_number = None;

        trace!(
            target: "bitcoinleveldb_snapshot::snapshot",
            event = "snapshot_sequence_number_for_read_reconstruction_default_exit",
            has_sequence_number = sequence_number.is_some()
        );

        sequence_number
    }

    /// Invariant: a returned Arc must preserve the original snapshot's read
    /// semantics exactly without consulting trait-object metadata identity.
    fn snapshot_read_arc_clone(&self) -> Option<Arc<dyn Snapshot>> {
        trace!(
            target: "bitcoinleveldb_snapshot::snapshot",
            event = "snapshot_read_arc_clone_default_entry"
        );

        let snapshot_arc = None;

        trace!(
            target: "bitcoinleveldb_snapshot::snapshot",
            event = "snapshot_read_arc_clone_default_exit",
            produced_snapshot_arc = snapshot_arc.is_some()
        );

        snapshot_arc
    }
}
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/snapshot.h]
#[cfg(test)]
mod snapshot_trait_contract_spec {
    use super::*;

    #[traced_test]
    fn snapshot_impl_converts_to_trait_object_and_is_stable() {
        debug!("verifying SnapshotImpl conforms to Snapshot trait object contract");

        let impl_obj = SnapshotImpl::new(42 as SequenceNumber);
        let dyn_view: &dyn Snapshot = &impl_obj;

        trace!(
            impl_addr = ?(&impl_obj as *const SnapshotImpl),
            dyn_addr  = ?(dyn_view as *const dyn Snapshot as *const ()),
            "constructed &dyn Snapshot view over SnapshotImpl"
        );

        assert_eq!(
            *impl_obj.sequence_number(),
            42,
            "sequence_number getter must expose the value passed to new()"
        );
    }

    #[allow(dead_code)]
    fn _compile_time_assert_impl<T: Snapshot>() {}
    
    #[traced_test]
    fn snapshot_impl_satisfies_snapshot_bound() {
        debug!("ensuring SnapshotImpl satisfies Snapshot via generic bound");
        _compile_time_assert_impl::<SnapshotImpl>();
        info!("SnapshotImpl satisfies Snapshot");
    }
}
