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
