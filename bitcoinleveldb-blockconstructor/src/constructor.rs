// ---------------- [ File: bitcoinleveldb-blockconstructor/src/constructor.rs ]
crate::ix!();

/// Scoped byte fixture map; the unrelated global text KVMap stays unchanged.
pub type ConstructorByteMap = std::collections::BTreeMap<Vec<u8>, Vec<u8>>;

pub fn constructor_bytes_from_text(data: &KVMap) -> ConstructorByteMap {
    data.iter().map(|(key, value)| (key.as_bytes().to_vec(), value.as_bytes().to_vec())).collect()
}

/// Sort by the active comparator and reject ambiguous equivalent input keys.
/// A Constructor already coalesces such keys in insertion order in add_bytes.
pub fn constructor_ordered_byte_entries<'a>(
    data: &'a ConstructorByteMap, comparator: &dyn SliceComparator,
) -> Result<Vec<(&'a Vec<u8>, &'a Vec<u8>)>, Status> {
    let mut entries: Vec<_> = data.iter().collect();
    entries.sort_by(|a, b| comparator.compare(&Slice::from(a.0.as_slice()), &Slice::from(b.0.as_slice())).cmp(&0));
    if entries.windows(2).any(|pair| comparator.compare(
        &Slice::from(pair[0].0.as_slice()), &Slice::from(pair[1].0.as_slice())) == 0) {
        return Err(Status::invalid_argument(&Slice::from("constructor input has equivalent duplicate keys"), None));
    }
    Ok(entries)
}

pub struct Constructor {
    data: ConstructorByteMap,
    comparator: std::sync::Arc<Box<dyn SliceComparator>>,
    text_view: std::sync::OnceLock<Option<KVMap>>,
}

// Retained legacy text interface for external implementations.
pub trait ConstructorInterface: ConstructorFinishImpl + ConstructorNewIterator {}
pub trait ConstructorFinishImpl {
    fn finish_impl(&mut self, options: &Options, data: &KVMap) -> Status;
}
pub trait ConstructorNewIterator { fn new_iterator(&self) -> *mut LevelDBIterator; }

impl Constructor {
    pub fn new(cmp: Box<dyn SliceComparator>) -> Self {
        Self::with_comparator_owner(std::sync::Arc::new(cmp))
    }
    /// Share an owner while keeping the original boxed comparator address stable.
    pub fn with_comparator_owner(comparator: std::sync::Arc<Box<dyn SliceComparator>>) -> Self {
        Self { data: ConstructorByteMap::new(), comparator, text_view: std::sync::OnceLock::new() }
    }
    pub fn with_default() -> Self { Self::new(Box::new(BytewiseComparatorImpl::default())) }
    pub fn comparator_ref(&self) -> &dyn SliceComparator { &**self.comparator }

    pub fn add_bytes(&mut self, key: &[u8], value: &[u8]) {
        let stored = self.data.keys().find(|stored| self.comparator.compare(
            &Slice::from(stored.as_slice()), &Slice::from(key)) == 0).cloned();
        // Equivalent replacement preserves the first stored key and last value.
        self.data.insert(stored.unwrap_or_else(|| key.to_vec()), value.to_vec());
        self.text_view.take();
    }
    pub fn add(&mut self, key: &String, value: &Slice) {
        self.add_bytes(key.as_bytes(), value.as_bytes());
    }
    pub fn data_bytes(&self) -> &ConstructorByteMap { &self.data }
    pub fn try_data(&self) -> Option<&KVMap> {
        self.text_view.get_or_init(|| {
            let mut text = KVMap::default();
            for (key, value) in &self.data {
                text.insert(String::from_utf8(key.clone()).ok()?, String::from_utf8(value.clone()).ok()?);
            }
            Some(text)
        }).as_ref()
    }
    /// Legacy view; panics for non-UTF-8 data. Use data_bytes/try_data instead.
    pub fn data(&self) -> &KVMap {
        self.try_data().expect("Constructor contains binary data; use data_bytes")
    }
    pub fn finish_bytes(&mut self, options: &Options, keys: *mut Vec<Vec<u8>>, map: *mut ConstructorByteMap) -> Status {
        if keys.is_null() || map.is_null() {
            return Status::invalid_argument(&Slice::from("null constructor byte output"), None);
        }
        if self.comparator.name() != options.comparator().name() {
            return Status::invalid_argument(&Slice::from("constructor comparator differs from Options"), None);
        }
        let entries = match constructor_ordered_byte_entries(&self.data, options.comparator().as_ref()) {
            Ok(entries) => entries, Err(status) => return status,
        };
        let sorted = entries.into_iter().map(|(key, _)| key.clone()).collect();
        let saved = self.data.clone();
        unsafe { *keys = sorted; *map = saved; }
        self.data.clear(); self.text_view.take();
        Status::ok()
    }
    /// Checked legacy adapter. Binary data or mismatched comparators panic
    /// before caller outputs change and before this constructor is cleared.
    pub fn finish(&mut self, options: &Options, keys: *mut Vec<String>, kvmap: *mut KVMap) {
        assert!(!keys.is_null(), "Constructor::finish: keys pointer is null");
        assert!(!kvmap.is_null(), "Constructor::finish: kvmap pointer is null");
        assert!(self.try_data().is_some(), "Constructor contains binary data; use finish_bytes");
        assert!(self.comparator.name() == options.comparator().name(), "Constructor::finish comparator mismatch");
        let entries = constructor_ordered_byte_entries(&self.data, options.comparator().as_ref())
            .expect("Constructor equivalent-key invariant");
        let text_keys = entries.into_iter().map(|(key, _)|
            String::from_utf8(key.clone()).expect("validated text key")).collect();
        let text = self.text_view.take().expect("validated text cache").expect("validated text view");
        self.data.clear();
        unsafe { *keys = text_keys; *kvmap = text; }
    }
}

#[cfg(test)]
mod constructor_kvmap_and_finish_tests {
    use super::*;

    #[traced_test]
    fn constructor_add_populates_internal_map() {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(bitcoinleveldb_comparator::BytewiseComparatorImpl::default());
        let mut ctor = Constructor::new(cmp_box);

        let value_slice = Slice::from("value-1".as_bytes());
        let key = "k1".to_string();

        trace!("calling Constructor::add");
        ctor.add(&key, &value_slice);

        let data = ctor.data();
        debug!("Constructor::data size after single add = {}", data.len());
        assert_eq!(data.len(), 1);
        assert_eq!(data.get("k1").unwrap(), "value-1");
    }

    #[traced_test]
    fn constructor_finish_clones_map_and_collects_keys() {
        let cmp_box: Box<dyn SliceComparator> =
            Box::new(bitcoinleveldb_comparator::BytewiseComparatorImpl::default());
        let mut ctor = Constructor::new(cmp_box);

        let v1 = Slice::from("value1".as_bytes());
        let v2 = Slice::from("value2".as_bytes());

        ctor.add(&"b".to_string(), &v1);
        ctor.add(&"a".to_string(), &v2);

        let mut keys  = Vec::<String>::new();
        let mut kvmap = KVMap::default();
        let options   = Options::default();

        trace!("calling Constructor::finish");
        ctor.finish(
            &options,
            &mut keys as *mut Vec<String>,
            &mut kvmap as *mut KVMap,
        );

        debug!("after finish: keys={:?}, kvmap={:?}", keys, kvmap);
        assert_eq!(kvmap.len(), 2);
        assert!(kvmap.contains_key("a"));
        assert!(kvmap.contains_key("b"));

        assert!(ctor.data().is_empty());

        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
    }
}

#[cfg(test)]
mod byte_constructor_contracts {
    use super::*;
    struct Folded;
    impl Compare for Folded {
        fn compare(&self, a: &Slice, b: &Slice) -> i32 {
            match a.as_bytes().to_ascii_lowercase().cmp(&b.as_bytes().to_ascii_lowercase()) {
                std::cmp::Ordering::Less => -1, std::cmp::Ordering::Equal => 0, std::cmp::Ordering::Greater => 1,
            }
        }
    }
    impl Named for Folded { fn name(&self) -> Cow<'_, str> { Cow::Borrowed("test.ConstructorFolded086") } }
    impl FindShortestSeparator for Folded { fn find_shortest_separator(&self, _: &mut Vec<u8>, _: &[u8]) {} }
    impl FindShortSuccessor for Folded { fn find_short_successor(&self, _: &mut Vec<u8>) {} }
    impl SliceComparator for Folded { fn bytewise_comparator(&self) -> *const dyn SliceComparator { self } }

    #[test]
    fn constructor_equivalent_keys_keep_first_key_last_value_and_sorted_output() {
        let mut constructor = Constructor::new(Box::new(Folded));
        constructor.add_bytes(b"Key", &[0xff, 0]);
        constructor.add_bytes(b"kEY", &[0x80]);
        constructor.add_bytes(b"Key", &[0xfe, 0]);
        constructor.add_bytes(b"Zoo", b"last");
        assert_eq!(constructor.data_bytes().len(), 2);
        assert!(constructor.data_bytes().contains_key(b"Key".as_slice()));
        assert_eq!(constructor.data_bytes()[b"Key".as_slice()], vec![0xfe, 0]);
        let mut options = Options::default(); options.set_comparator(std::sync::Arc::new(Folded));
        let mut keys = Vec::new(); let mut map = ConstructorByteMap::new();
        assert!(constructor.finish_bytes(&options, &mut keys, &mut map).is_ok());
        assert_eq!(keys, vec![b"Key".to_vec(), b"Zoo".to_vec()]);
        assert_eq!(map[b"Key".as_slice()], vec![0xfe, 0]);
        assert!(constructor.data_bytes().is_empty());
        let mut ambiguous = ConstructorByteMap::new();
        ambiguous.insert(b"a".to_vec(), vec![]); ambiguous.insert(b"A".to_vec(), vec![]);
        assert!(constructor_ordered_byte_entries(&ambiguous, &Folded).is_err());
    }

    #[test]
    fn constructor_binary_legacy_view_fails_before_outputs_or_state_change() {
        let mut constructor = Constructor::with_default();
        let payload: Vec<u8> = (0u8..=255).collect();
        constructor.add(&String::from("text-key"), &Slice::from(payload.as_slice()));
        assert!(constructor.try_data().is_none());
        let mut keys = vec![String::from("sentinel")]; let mut text = KVMap::default();
        text.insert(String::from("sentinel"), String::from("kept")); let prior = text.clone();
        let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            constructor.finish(&Options::default(), &mut keys, &mut text);
        }));
        assert!(caught.is_err()); assert_eq!(keys, vec![String::from("sentinel")]); assert_eq!(text, prior);
        assert_eq!(constructor.data_bytes()[b"text-key".as_slice()], payload);
        let mut byte_keys = vec![]; let mut bytes = ConstructorByteMap::new();
        assert!(constructor.finish_bytes(&Options::default(), &mut byte_keys, &mut bytes).is_ok());
        assert_eq!(bytes[b"text-key".as_slice()], payload);
        assert!(constructor.data().is_empty());
    }

    #[test]
    fn constructor_output_and_comparator_errors_preserve_owned_state() {
        let mut constructor = Constructor::new(Box::new(Folded)); constructor.add_bytes(b"a", b"v");
        let mut keys = vec![b"keep".to_vec()]; let mut map = ConstructorByteMap::new();
        map.insert(b"keep".to_vec(), b"value".to_vec()); let prior = map.clone();
        assert!(constructor.finish_bytes(&Options::default(), &mut keys, &mut map).is_invalid_argument());
        assert_eq!(map, prior); assert_eq!(keys, vec![b"keep".to_vec()]);
        assert_eq!(constructor.data_bytes().len(), 1);
        assert!(constructor.finish_bytes(&Options::default(), std::ptr::null_mut(), &mut map).is_invalid_argument());
        assert_eq!(map, prior); assert_eq!(constructor.data_bytes().len(), 1);
    }
}
