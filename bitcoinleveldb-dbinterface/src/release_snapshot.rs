// ---------------- [ File: bitcoinleveldb-dbinterface/src/release_snapshot.rs ]
crate::ix!();

pub trait DBReleaseSnapshot {

    /// Release a previously acquired snapshot.
    ///
    /// The caller must not use "snapshot" after this call.
    /// 
    fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>);
}

#[cfg(test)]
mod release_snapshot_lifecycle_suite {
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

    struct SnapshotReleaser {
        drops: Arc<AtomicUsize>,
        releases: Arc<AtomicUsize>,
    }

    impl SnapshotReleaser {
        fn new(drops: Arc<AtomicUsize>, releases: Arc<AtomicUsize>) -> Self {
            Self { drops, releases }
        }

        fn make_snapshot(&self) -> Box<dyn Snapshot> {
            Box::new(DropCountingSnapshot {
                drops: self.drops.clone(),
            })
        }
    }

    impl ReleaseSnapshot for SnapshotReleaser {
        fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>) {
            let n = self.releases.fetch_add(1, Ordering::SeqCst) + 1;
            trace!(release_calls = n, "release_snapshot invoked");
            drop(snapshot);
        }
    }

    #[traced_test]
    fn release_snapshot_drops_snapshot_handle() {
        let drops = Arc::new(AtomicUsize::new(0));
        let releases = Arc::new(AtomicUsize::new(0));

        let mut db = SnapshotReleaser::new(drops.clone(), releases.clone());

        let snap = db.make_snapshot();

        assert_eq!(drops.load(Ordering::SeqCst), 0);
        assert_eq!(releases.load(Ordering::SeqCst), 0);

        trace!("releasing snapshot");
        db.release_snapshot(snap);

        assert_eq!(releases.load(Ordering::SeqCst), 1);
        assert_eq!(drops.load(Ordering::SeqCst), 1);

        info!("verified release_snapshot consumes the handle and drops snapshot");
    }

    #[traced_test]
    fn release_snapshot_can_be_called_multiple_times() {
        let drops = Arc::new(AtomicUsize::new(0));
        let releases = Arc::new(AtomicUsize::new(0));

        let mut db = SnapshotReleaser::new(drops.clone(), releases.clone());

        let s1 = db.make_snapshot();
        let s2 = db.make_snapshot();

        db.release_snapshot(s1);
        db.release_snapshot(s2);

        assert_eq!(releases.load(Ordering::SeqCst), 2);
        assert_eq!(drops.load(Ordering::SeqCst), 2);

        info!("verified multiple snapshot releases are supported");
    }
}
