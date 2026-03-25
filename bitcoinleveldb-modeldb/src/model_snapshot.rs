// ---------------- [ File: bitcoinleveldb-modeldb/src/model_snapshot.rs ]
crate::ix!();

pub struct ModelSnapshot {
    map:  KVMap,
}

impl Snapshot for ModelSnapshot {

    fn snapshot_runtime_implementation_kind(&self) -> SnapshotDispatchConcreteImplementationKind {
        trace!(
            target: "bitcoinleveldb_modeldb::snapshot",
            event = "model_snapshot_runtime_implementation_kind_entry",
            map_len = self.map_ref().len()
        );

        let implementation_kind = SnapshotDispatchConcreteImplementationKind::ModelSnapshot;

        trace!(
            target: "bitcoinleveldb_modeldb::snapshot",
            event = "model_snapshot_runtime_implementation_kind_exit",
            implementation_kind = ?implementation_kind,
            map_len = self.map_ref().len()
        );

        implementation_kind
    }

    fn snapshot_read_arc_clone(&self) -> Option<Arc<dyn Snapshot>> {
        trace!(
            target: "bitcoinleveldb_modeldb::snapshot",
            event = "model_snapshot_read_arc_clone_entry",
            map_len = self.map_ref().len()
        );

        let snapshot_arc: Arc<dyn Snapshot> = Arc::new(ModelSnapshot::new_from_map(self.map_ref()));

        trace!(
            target: "bitcoinleveldb_modeldb::snapshot",
            event = "model_snapshot_read_arc_clone_exit",
            map_len = self.map_ref().len()
        );

        Some(snapshot_arc)
    }
}

impl ModelSnapshot {
    pub fn new_from_map(map: &KVMap) -> Self {
        tracing::debug!("ModelSnapshot::new_from_map");
        Self { map: map.clone() }
    }

    pub fn map_ref(&self) -> &KVMap {
        tracing::trace!("ModelSnapshot::map_ref");
        &self.map
    }
}

#[cfg(test)]
mod model_snapshot_clone_and_iteration_suite {
    use super::*;

    #[traced_test]
    fn model_snapshot_clones_map_and_is_immutable_from_original() {
        tracing::info!("starting model_snapshot_clones_map_and_is_immutable_from_original");

        let mut original: KVMap = KVMap::default();
        original.insert(String::from("a"), String::from("va"));
        original.insert(String::from("b"), String::from("vb"));

        let snap: ModelSnapshot = ModelSnapshot::new_from_map(&original);

        original.insert(String::from("c"), String::from("vc"));
        original.insert(String::from("a"), String::from("va2"));

        let snap_map: &KVMap = snap.map_ref();

        assert_eq!(snap_map.get("a").cloned(), Some(String::from("va")));
        assert_eq!(snap_map.get("b").cloned(), Some(String::from("vb")));
        assert!(snap_map.get("c").is_none());
    }

    #[traced_test]
    fn model_snapshot_arc_in_read_options_produces_iterator_over_snapshot_state() {
        tracing::info!("starting model_snapshot_arc_in_read_options_produces_iterator_over_snapshot_state");

        let options: Options = Options::default();
        let mut db: crate::ModelDB = crate::ModelDB::new(&options);

        let wo: WriteOptions = WriteOptions::default();

        assert!(DBPut::put(&mut db, &wo, &Slice::from("a"), &Slice::from("va")).is_ok());
        assert!(DBPut::put(&mut db, &wo, &Slice::from("b"), &Slice::from("vb")).is_ok());

        let snap_box: Box<dyn Snapshot> = DBGetSnapshot::get_snapshot(&mut db);
        let snap_arc: std::sync::Arc<dyn Snapshot> = std::sync::Arc::from(snap_box);

        assert!(DBPut::put(&mut db, &wo, &Slice::from("c"), &Slice::from("vc")).is_ok());
        assert!(DBDelete::delete(&mut db, &wo, &Slice::from("a")).is_ok());

        let mut ro: ReadOptions = ReadOptions::default();
        ro.set_snapshot(Some(snap_arc.clone()));

        let it_ptr: *mut LevelDBIterator = DBNewIterator::new_iterator(&mut db, &ro);
        assert!(!it_ptr.is_null());

        let mut it_box: Box<LevelDBIterator> = unsafe { Box::from_raw(it_ptr) };

        <LevelDBIterator as LevelDBIteratorSeekToFirst>::seek_to_first(&mut *it_box);

        let mut out: Vec<(String, String)> = Vec::new();
        while <LevelDBIterator as LevelDBIteratorValid>::valid(&*it_box) {
            let k: String = <LevelDBIterator as LevelDBIteratorKey>::key(&*it_box).to_string();
            let v: String = <LevelDBIterator as LevelDBIteratorValue>::value(&*it_box).to_string();
            out.push((k, v));
            <LevelDBIterator as LevelDBIteratorNext>::next(&mut *it_box);
        }

        assert_eq!(
            out,
            vec![(String::from("a"), String::from("va")), (String::from("b"), String::from("vb"))]
        );
    }
}
