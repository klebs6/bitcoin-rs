// ---------------- [ File: bitcoinleveldb-modeldb/src/byte_model.rs ]
crate::ix!();

/// Owned reference-model payload. BTreeMap supplies storage only; ModelDB uses
/// Options' comparator for key equivalence and visible iteration order.
pub type ModelByteMap = std::collections::BTreeMap<Vec<u8>, Vec<u8>>;

pub(crate) type ModelByteCompare = Box<dyn Fn(&[u8], &[u8]) -> i32>;

pub(crate) fn model_byte_comparator(options: &Options) -> ModelByteCompare {
    // Keep only the comparator Arc, not the environment-bearing Options.
    let comparator = std::sync::Arc::clone(options.comparator());
    Box::new(move |a, b| comparator.compare(&Slice::from(a), &Slice::from(b)))
}

pub(crate) fn unsupported_model_snapshot() -> Status {
    Status::invalid_argument(&Slice::from("snapshot has no model byte-map capability"), None)
}

pub(crate) fn model_byte_lookup<'a>(
    map: &'a ModelByteMap, key: &[u8], compare: &dyn Fn(&[u8], &[u8]) -> i32,
) -> Option<&'a Vec<u8>> {
    map.iter().find(|(stored, _)| compare(stored, key) == 0).map(|(_, value)| value)
}

pub(crate) fn model_byte_put(
    map: &mut ModelByteMap, key: &[u8], value: &[u8],
    compare: &dyn Fn(&[u8], &[u8]) -> i32,
) {
    // Match std::map semantics: equivalent replacement retains the stored key.
    let stored = map.keys().find(|stored| compare(stored, key) == 0).cloned();
    map.insert(stored.unwrap_or_else(|| key.to_vec()), value.to_vec());
}

pub(crate) fn model_byte_delete(
    map: &mut ModelByteMap, key: &[u8], compare: &dyn Fn(&[u8], &[u8]) -> i32,
) {
    let stored = map.keys().find(|stored| compare(stored, key) == 0).cloned();
    if let Some(stored) = stored { map.remove(&stored); }
}

/// Data and comparison policy are owned. No ReadOptions, ModelDB or snapshot
/// needs to remain alive after this iterator is constructed.
pub(crate) struct ByteModelIter {
    rows: Vec<(Vec<u8>, Vec<u8>)>,
    compare: ModelByteCompare,
    pos: isize,
    status: Status,
}

impl ByteModelIter {
    pub(crate) fn new(map: &ModelByteMap, compare: ModelByteCompare) -> Self {
        let mut rows: Vec<_> = map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
        rows.sort_by(|a, b| compare(&a.0, &b.0).cmp(&0));
        Self { pos: rows.len() as isize, rows, compare, status: Status::ok() }
    }

    pub(crate) fn error(status: Status) -> Self {
        Self { rows: Vec::new(), compare: Box::new(|_, _| 0), pos: 0, status }
    }
}

impl LevelDBIteratorInterface for ByteModelIter {}
impl LevelDBIteratorValid for ByteModelIter {
    fn valid(&self) -> bool { self.pos >= 0 && (self.pos as usize) < self.rows.len() }
}
impl LevelDBIteratorSeekToFirst for ByteModelIter {
    fn seek_to_first(&mut self) { self.pos = 0; }
}
impl LevelDBIteratorSeekToLast for ByteModelIter {
    fn seek_to_last(&mut self) { self.pos = self.rows.len() as isize - 1; }
}
impl LevelDBIteratorSeek for ByteModelIter {
    fn seek(&mut self, target: &Slice) {
        self.pos = self.rows.binary_search_by(|row|
            (self.compare)(&row.0, target.as_bytes()).cmp(&0))
            .unwrap_or_else(|position| position) as isize;
    }
}
impl LevelDBIteratorNext for ByteModelIter {
    fn next(&mut self) { if self.valid() { self.pos += 1; } }
}
impl LevelDBIteratorPrev for ByteModelIter {
    fn prev(&mut self) {
        if self.pos == self.rows.len() as isize || self.valid() { self.pos -= 1; }
    }
}
impl LevelDBIteratorKey for ByteModelIter {
    fn key(&self) -> Slice {
        assert!(self.valid(), "ByteModelIter::key requires a valid position");
        Slice::from(self.rows[self.pos as usize].0.as_slice())
    }
}
impl LevelDBIteratorValue for ByteModelIter {
    fn value(&self) -> Slice {
        assert!(self.valid(), "ByteModelIter::value requires a valid position");
        Slice::from(self.rows[self.pos as usize].1.as_slice())
    }
}
impl LevelDBIteratorStatus for ByteModelIter {
    fn status(&self) -> Status { Status::new_from_other_copy(&self.status) }
}

#[cfg(test)]
mod byte_model_contracts {
    use super::*;

    fn byte_order(a: &[u8], b: &[u8]) -> i32 {
        match a.cmp(b) { std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1 }
    }
    fn collect_rows(iter: &mut LevelDBIterator) -> Vec<(Vec<u8>, Vec<u8>)> {
        let mut rows = Vec::new();
        iter.seek_to_first();
        while iter.valid() {
            rows.push((iter.key().as_bytes().to_vec(), iter.value().as_bytes().to_vec()));
            iter.next();
        }
        assert!(iter.status().is_ok()); rows
    }
    fn model_iter(db: &mut ModelDB, opts: &ReadOptions) -> Box<LevelDBIterator> {
        unsafe { Box::from_raw(db.new_iterator(opts)) }
    }

    #[test]
    fn model_binary_reads_keep_distinct_keys_empty_values_and_checked_text() {
        let mut db = ModelDB::new(&Options::default());
        let wo = WriteOptions::default(); let ro = ReadOptions::default();
        let all: Vec<u8> = (0u8..=255).collect();
        let keys = [vec![0xff], vec![0xfe], "�".as_bytes().to_vec(), vec![0, 0x80]];
        for (index, key) in keys.iter().enumerate() {
            let payload = [all.as_slice(), &[index as u8]].concat();
            assert!(db.put(&wo, &Slice::from(key.as_slice()), &Slice::from(payload.as_slice())).is_ok());
        }
        let mut output = vec![42];
        for (index, key) in keys.iter().enumerate() {
            assert!(db.get_bytes(&ro, &Slice::from(key.as_slice()), &mut output).is_ok());
            assert_eq!(output, [all.as_slice(), &[index as u8]].concat());
        }
        let mut text = String::from("preserve");
        assert!(db.get(&ro, &Slice::from(keys[0].as_slice()), &mut text).is_corruption());
        assert_eq!(text, "preserve");
        let previous = output.clone();
        assert!(db.get_bytes(&ro, &Slice::from("absent"), &mut output).is_not_found());
        assert_eq!(output, previous);
        assert!(db.put(&wo, &Slice::default(), &Slice::default()).is_ok());
        assert!(db.get_bytes(&ro, &Slice::default(), &mut output).is_ok());
        assert!(output.is_empty());
        assert!(db.put(&wo, &Slice::from("text"), &Slice::from("é\0水")).is_ok());
        assert!(db.get(&ro, &Slice::from("text"), &mut text).is_ok());
        assert_eq!(text, "é\0水");
    }

    #[test]
    fn model_binary_batch_order_and_bidirectional_seek_match_byte_order() {
        let mut db = ModelDB::new(&Options::default());
        let mut batch = WriteBatch::new();
        let key = [0xff];
        batch.put(&Slice::from(&key[..]), &Slice::from("old"));
        batch.delete(&Slice::from(&key[..]));
        batch.put(&Slice::from(&key[..]), &Slice::from("new"));
        batch.put(&Slice::from(&[0xfe][..]), &Slice::from("other"));
        batch.put(&Slice::default(), &Slice::default());
        batch.put(&Slice::from(&[0, 0x80][..]), &Slice::from(&[0xff, 0][..]));
        assert!(db.write(&WriteOptions::default(), &mut batch).is_ok());
        let mut iter = model_iter(&mut db, &ReadOptions::default());
        let expected = vec![(vec![], vec![]), (vec![0, 0x80], vec![0xff, 0]),
            (vec![0xfe], b"other".to_vec()), (vec![0xff], b"new".to_vec())];
        assert_eq!(collect_rows(&mut iter), expected);
        for target in [vec![], vec![0], vec![0, 0x81], vec![0xfe], vec![0xff], vec![0xff, 0]] {
            iter.seek(&Slice::from(target.as_slice()));
            let wanted = expected.iter().find(|row| row.0 >= target);
            assert_eq!(iter.valid(), wanted.is_some());
            if let Some(row) = wanted { assert_eq!(iter.key().as_bytes(), row.0.as_slice()); }
        }
        let mut backwards = Vec::new(); iter.seek_to_last();
        while iter.valid() { backwards.push(iter.key().as_bytes().to_vec()); iter.prev(); }
        assert_eq!(backwards, expected.iter().rev().map(|r| r.0.clone()).collect::<Vec<_>>());
    }

    #[test]
    fn model_snapshot_clone_and_iterator_own_binary_state_after_owners_drop() {
        let mut db = ModelDB::new(&Options::default()); let wo = WriteOptions::default();
        let key = [0xff, 0]; let old = [0x80, 0, 0xfe];
        assert!(db.put(&wo, &Slice::from(&key[..]), &Slice::from(&old[..])).is_ok());
        let original = db.get_snapshot();
        let cloned = original.snapshot_read_arc_clone().expect("model clone");
        assert_eq!(cloned.snapshot_model_byte_map().unwrap().get(key.as_slice()).unwrap(), &old);
        db.release_snapshot(original);
        assert!(db.delete(&wo, &Slice::from(&key[..])).is_ok());
        let mut ro = ReadOptions::default(); ro.set_snapshot(Some(cloned.clone()));
        let mut output = Vec::new();
        assert!(db.get_bytes(&ro, &Slice::from(&key[..]), &mut output).is_ok());
        assert_eq!(output, old);
        let mut iter = model_iter(&mut db, &ro);
        drop(ro); drop(cloned); drop(db);
        assert_eq!(collect_rows(&mut iter), vec![(key.to_vec(), old.to_vec())]);
    }

    #[test]
    fn model_rejects_foreign_snapshots_without_live_fallback_or_output_mutation() {
        struct OtherSnapshot; impl Snapshot for OtherSnapshot {}
        let mut db = ModelDB::new(&Options::default());
        assert!(db.put(&WriteOptions::default(), &Slice::from("k"), &Slice::from("live")).is_ok());
        let mut ro = ReadOptions::default(); ro.set_snapshot(Some(std::sync::Arc::new(OtherSnapshot)));
        let mut bytes = vec![0x80, 0]; let prior = bytes.clone();
        assert!(db.get_bytes(&ro, &Slice::from("k"), &mut bytes).is_invalid_argument());
        assert_eq!(bytes, prior);
        let mut iter = model_iter(&mut db, &ro); iter.seek_to_first();
        assert!(!iter.valid()); assert!(iter.status().is_invalid_argument());
    }

    #[test]
    fn model_comparator_equivalence_controls_put_get_delete_and_seek() {
        let compare = |a: &[u8], b: &[u8]| byte_order(&a.to_ascii_lowercase(), &b.to_ascii_lowercase());
        let mut map = ModelByteMap::default();
        // Exercise the same handler and lookup functions used by ModelDB.
        let mut handler = ModelDBWriteBatchHandler { map: &mut map, compare: Box::new(compare) };
        handler.put(&Slice::from("Key"), &Slice::from("old"));
        handler.put(&Slice::from("kEY"), &Slice::from(&[0xff][..]));
        assert_eq!(map.len(), 1);
        assert_eq!(model_byte_lookup(&map, b"KEY", &compare), Some(&vec![0xff]));
        assert!(map.contains_key(b"Key".as_slice()));
        handler.put(&Slice::from("Zoo"), &Slice::from("z"));
        let mut iter = ByteModelIter::new(&map, Box::new(compare));
        iter.seek(&Slice::from("KEY")); assert!(iter.valid()); assert_eq!(iter.key().as_bytes(), b"Key");
        handler.delete(&Slice::from("keY"));
        assert!(model_byte_lookup(&map, b"KEY", &compare).is_none());
        assert_eq!(map.len(), 1);
    }

    #[test]
    fn model_reverse_comparator_owns_policy_and_orders_seek_in_both_directions() {
        let mut map = ModelByteMap::default();
        for key in [vec![0], vec![0x80], vec![0xff]] { map.insert(key.clone(), key); }
        let mut iter = ByteModelIter::new(&map, Box::new(|a, b| byte_order(b, a)));
        drop(map); iter.seek_to_first(); assert_eq!(iter.key().as_bytes(), &[0xff]);
        iter.next(); assert_eq!(iter.key().as_bytes(), &[0x80]);
        iter.seek(&Slice::from(&[0x81][..])); assert_eq!(iter.key().as_bytes(), &[0x80]);
        iter.seek_to_last(); assert_eq!(iter.key().as_bytes(), &[0]);
        iter.prev(); assert_eq!(iter.key().as_bytes(), &[0x80]);
    }

    #[test]
    fn snapshot_text_view_is_complete_strict_and_legacy_iterator_seeks_raw_bytes() {
        let mut bytes = ModelByteMap::default(); bytes.insert(vec![0xff], vec![0x80]);
        let snapshot = ModelSnapshot::new_from_byte_map(&bytes);
        assert!(snapshot.try_map_ref().is_none()); assert_eq!(snapshot.byte_map_ref(), &bytes);
        assert_eq!(snapshot.snapshot_read_arc_clone().unwrap().snapshot_model_byte_map(), Some(&bytes));
        let mut text = KVMap::default(); text.insert("�".to_string(), "ok".to_string());
        let text_snapshot = ModelSnapshot::new_from_map(&text);
        assert_eq!(text_snapshot.try_map_ref(), Some(&text));
        let mut legacy = ModelIter::new(&text, false);
        legacy.seek(&Slice::from(&[0xff][..])); assert!(!legacy.valid());
        legacy.seek(&Slice::from(&[0xee][..])); assert!(legacy.valid());
        assert_eq!(legacy.key().as_bytes(), "�".as_bytes());
    }
}
