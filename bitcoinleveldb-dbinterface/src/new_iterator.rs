// ---------------- [ File: bitcoinleveldb-dbinterface/src/new_iterator.rs ]
crate::ix!();

pub trait DBNewIterator {

    /// Return a heap-allocated iterator over the contents of the database.  
    ///
    /// The result of NewIterator() is initially invalid (caller must call one of the Seek methods
    /// on the iterator before using it).
    /// 
    /// Caller should delete the iterator when it is no longer needed.  
    ///
    /// The returned iterator should be deleted before this db is deleted.
    ///
    fn new_iterator(&mut self, options: &ReadOptions) -> *mut LevelDBIterator;
}

#[cfg(test)]
mod new_iterator_lifecycle_suite {
    use super::*;
    use tracing::{debug, info, trace};

    struct IteratorProvider {
        entries: Vec<(Vec<u8>, Vec<u8>)>,
    }

    impl IteratorProvider {
        fn new(entries: &[(&[u8], &[u8])]) -> Self {
            Self {
                entries: entries
                    .iter()
                    .map(|(k, v)| (k.to_vec(), v.to_vec()))
                    .collect(),
            }
        }

        fn to_pairs(&self) -> Vec<(&[u8], &[u8])> {
            self.entries
                .iter()
                .map(|(k, v)| (k.as_slice(), v.as_slice()))
                .collect()
        }
    }

    impl DBNewIterator for IteratorProvider {
        fn new_iterator(&mut self, _options: &ReadOptions) -> *mut LevelDBIterator {
            let pairs = self.to_pairs();
            let stub = MockStubIterator::new_with_entries(pairs.as_slice());
            Box::into_raw(Box::new(LevelDBIterator::new(Some(Box::new(stub)))))
        }
    }

    fn slice_to_vec(s: &Slice) -> Vec<u8> {
        let p = *s.data();
        let n = *s.size();
        unsafe { core::slice::from_raw_parts(p, n) }.to_vec()
    }

    #[traced_test]
    fn new_iterator_starts_invalid_until_seek_is_called() {
        let mut provider = IteratorProvider::new(&[(b"k", b"v")]);

        let ro = ReadOptions::default();

        trace!("creating iterator");
        let it_ptr = provider.new_iterator(&ro);
        assert!(!it_ptr.is_null());

        let it = unsafe { &mut *it_ptr };

        debug!("verifying iterator is initially invalid");
        assert!(!it.valid());

        trace!("seeking to first");
        it.seek_to_first();

        assert!(it.valid());
        assert_eq!(slice_to_vec(&it.key()), b"k".to_vec());
        assert_eq!(slice_to_vec(&it.value()), b"v".to_vec());

        unsafe {
            drop(Box::from_raw(it_ptr));
        }

        info!("verified NewIterator contract: invalid until seek, then yields first entry");
    }

    #[traced_test]
    fn new_iterator_seek_on_empty_iterator_remains_invalid() {
        let mut provider = IteratorProvider::new(&[]);
        let ro = ReadOptions::default();

        trace!("creating empty iterator");
        let it_ptr = provider.new_iterator(&ro);
        assert!(!it_ptr.is_null());

        let it = unsafe { &mut *it_ptr };

        assert!(!it.valid());

        trace!("seeking to first on empty");
        it.seek_to_first();

        assert!(!it.valid());

        unsafe {
            drop(Box::from_raw(it_ptr));
        }

        info!("verified seek_to_first on empty iterator keeps it invalid");
    }

    #[traced_test]
    fn new_iterator_is_heap_allocated_and_can_be_dropped_by_caller() {
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };

        let drops = Arc::new(AtomicUsize::new(0));
        let tracked = MockTrackedIterator::new(drops.clone());

        let it_ptr = Box::into_raw(Box::new(LevelDBIterator::new(Some(Box::new(tracked)))));

        assert!(!it_ptr.is_null());
        assert_eq!(drops.load(Ordering::SeqCst), 0);

        trace!("dropping iterator");
        unsafe {
            drop(Box::from_raw(it_ptr));
        }

        assert_eq!(drops.load(Ordering::SeqCst), 1);

        info!("verified caller drop releases underlying iterator resources");
    }
}
