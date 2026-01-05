// ---------------- [ File: bitcoinleveldb-dbinterface/src/get_snapshot.rs ]
crate::ix!();

pub trait DBGetSnapshot {

    /// Return a handle to the current DB state.
    ///
    /// Iterators created with this handle will all observe a stable snapshot of the current DB
    /// state.
    /// 
    /// The caller must call ReleaseSnapshot(result) when the snapshot is no longer needed.
    ///
    fn get_snapshot(&mut self) -> Box<dyn Snapshot>;
}

#[cfg(test)]
mod get_snapshot_lifecycle_suite {
    use super::*;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };
    use tracing::{debug, error, info, trace, warn};

    struct DropCountingSnapshot {
        drops: Arc<AtomicUsize>,
    }

    impl Snapshot for DropCountingSnapshot {}

    impl Drop for DropCountingSnapshot {
        fn drop(&mut self) {
            let n = self.drops.fetch_add(1, Ordering::SeqCst) + 1;
            trace!(drop_count = n, "snapshot dropped");
        }
    }

    struct SnapshotFactory {
        drops: Arc<AtomicUsize>,
    }

    impl SnapshotFactory {
        fn new(drops: Arc<AtomicUsize>) -> Self {
            Self { drops }
        }
    }

    impl GetSnapshot for SnapshotFactory {
        fn get_snapshot(&mut self) -> Box<dyn Snapshot> {
            trace!("creating snapshot");
            Box::new(DropCountingSnapshot {
                drops: self.drops.clone(),
            })
        }
    }

    #[traced_test]
    fn get_snapshot_returns_owned_handle_that_drops_when_box_is_dropped() {
        let drops = Arc::new(AtomicUsize::new(0));
        let mut db = SnapshotFactory::new(drops.clone());

        trace!("acquiring snapshot");
        let snap = db.get_snapshot();

        assert_eq!(drops.load(Ordering::SeqCst), 0);

        trace!("dropping snapshot handle without calling release");
        drop(snap);

        assert_eq!(drops.load(Ordering::SeqCst), 1);

        info!("verified get_snapshot returns an owned handle that is dropped with Box");
    }

    #[traced_test]
    fn get_snapshot_can_be_called_multiple_times_producing_multiple_owned_handles() {
        let drops = Arc::new(AtomicUsize::new(0));
        let mut db = SnapshotFactory::new(drops.clone());

        let s1 = db.get_snapshot();
        let s2 = db.get_snapshot();

        assert_eq!(drops.load(Ordering::SeqCst), 0);

        drop(s1);
        assert_eq!(drops.load(Ordering::SeqCst), 1);

        drop(s2);
        assert_eq!(drops.load(Ordering::SeqCst), 2);

        info!("verified repeated get_snapshot calls yield independently owned handles");
    }
}
