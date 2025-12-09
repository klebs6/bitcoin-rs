// ---------------- [ File: bitcoinleveldb-modeldb/src/snapshot.rs ]
crate::ix!();

pub struct ModelSnapshot {
    map:  KVMap,
}

impl Snapshot for ModelSnapshot {

}
