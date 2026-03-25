// ---------------- [ File: bitcoinleveldbt-snapshot/src/collect_all_kvs.rs ]
crate::ix!();

pub fn snapshot_suite_collect_all_kvs_from_db_with_optional_snapshot(
    db: &mut dyn DBNewIterator,
    snapshot: Option<std::sync::Arc<dyn Snapshot>>,
) -> Vec<(String, String)> {
    let mut ro: ReadOptions = ReadOptions::default();
    ro.set_snapshot(snapshot);

    let it_ptr: *mut LevelDBIterator = DBNewIterator::new_iterator(db, &ro);
    assert!(!it_ptr.is_null(), "new_iterator returned null");

    let mut it_box: Box<LevelDBIterator> = unsafe { Box::from_raw(it_ptr) };
    <LevelDBIterator as LevelDBIteratorSeekToFirst>::seek_to_first(&mut *it_box);

    let mut out: Vec<(String, String)> = Vec::new();
    while <LevelDBIterator as LevelDBIteratorValid>::valid(&*it_box) {
        let k: String = <LevelDBIterator as LevelDBIteratorKey>::key(&*it_box).to_string();
        let v: String = <LevelDBIterator as LevelDBIteratorValue>::value(&*it_box).to_string();
        out.push((k, v));
        <LevelDBIterator as LevelDBIteratorNext>::next(&mut *it_box);
    }

    out
}
