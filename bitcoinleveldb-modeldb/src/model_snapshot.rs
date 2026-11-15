// ---------------- [ File: bitcoinleveldb-modeldb/src/model_snapshot.rs ]
crate::ix!();

pub struct ModelSnapshot {
    map: ModelByteMap,
    text_view: std::sync::OnceLock<Option<KVMap>>,
}

impl Snapshot for ModelSnapshot {
    fn snapshot_model_byte_map(&self) -> Option<&ModelByteMap> { Some(&self.map) }


    fn snapshot_runtime_implementation_kind(&self) -> SnapshotDispatchConcreteImplementationKind {
        trace!(
            target: "bitcoinleveldb_modeldb::snapshot",
            event = "model_snapshot_runtime_implementation_kind_entry",
            map_len = self.map.len()
        );

        let implementation_kind = SnapshotDispatchConcreteImplementationKind::ModelSnapshot;

        trace!(
            target: "bitcoinleveldb_modeldb::snapshot",
            event = "model_snapshot_runtime_implementation_kind_exit",
            implementation_kind = ?implementation_kind,
            map_len = self.map.len()
        );

        implementation_kind
    }

    fn snapshot_read_arc_clone(&self) -> Option<Arc<dyn Snapshot>> {
        trace!(
            target: "bitcoinleveldb_modeldb::snapshot",
            event = "model_snapshot_read_arc_clone_entry",
            map_len = self.map.len()
        );

        let snapshot_arc: Arc<dyn Snapshot> = Arc::new(ModelSnapshot::new_from_byte_map(&self.map));

        trace!(
            target: "bitcoinleveldb_modeldb::snapshot",
            event = "model_snapshot_read_arc_clone_exit",
            map_len = self.map.len()
        );

        Some(snapshot_arc)
    }
}

impl ModelSnapshot {
    /// Compatibility constructor: UTF-8 inputs are copied without transcoding.
    pub fn new_from_map(map: &KVMap) -> Self {
        let bytes = map.iter().map(|(key, value)|
            (key.as_bytes().to_vec(), value.as_bytes().to_vec())).collect();
        Self::new_from_byte_map(&bytes)
    }

    pub fn new_from_byte_map(map: &ModelByteMap) -> Self {
        Self { map: map.clone(), text_view: std::sync::OnceLock::new() }
    }

    pub fn byte_map_ref(&self) -> &ModelByteMap { &self.map }

    /// A complete strict UTF-8 view, or None if any key/value is non-text.
    /// Conversion is lazy; binary reads and snapshot clones never need this cache.
    pub fn try_map_ref(&self) -> Option<&KVMap> {
        self.text_view.get_or_init(|| {
            let mut text = KVMap::default();
            for (key, value) in &self.map {
                text.insert(String::from_utf8(key.clone()).ok()?,
                    String::from_utf8(value.clone()).ok()?);
            }
            Some(text)
        }).as_ref()
    }

    /// Legacy text view. Use byte_map_ref for arbitrary LevelDB bytes.
    /// Panics if this snapshot contains a non-UTF-8 key or value.
    pub fn map_ref(&self) -> &KVMap {
        self.try_map_ref().expect("ModelSnapshot contains binary data; use byte_map_ref")
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
