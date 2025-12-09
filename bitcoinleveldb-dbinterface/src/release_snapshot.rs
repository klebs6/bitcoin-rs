// ---------------- [ File: bitcoinleveldb-dbinterface/src/release_snapshot.rs ]
crate::ix!();

pub trait ReleaseSnapshot {

    /**
      | Release a previously acquired snapshot.
      | The caller must not use "snapshot" after
      | this call.
      |
      */
    fn release_snapshot(&mut self, snapshot: Box<dyn Snapshot>);
}
