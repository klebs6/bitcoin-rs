// ---------------- [ File: bitcoinleveldb-iterator/src/iterator.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_iter.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/db_iter.cc]

pub struct LevelDBIterator {
    rep: Rc<RefCell<LevelDBIteratorInner>>,
}

impl LevelDBIterator {

    pub fn new() -> Self {
        trace!("LevelDBIterator::new");
        Self {
            rep: Rc::new(RefCell::new(LevelDBIteratorInner::new())),
        }
    }

    pub fn inner(&self) -> Rc<RefCell<LevelDBIteratorInner>> {
        trace!(
            "LevelDBIterator::inner: cloning Rc to inner; strong_count={}",
            Rc::strong_count(&self.rep)
        );
        self.rep.clone()
    }

    /**
      | Register a cleanup function that will be
      | invoked when the iterator is eventually
      | destroyed.
      |
      */
    pub fn register_cleanup(
        &self,
        func: LevelDBIteratorCleanupFunction,
        arg1: *mut c_void,
        arg2: *mut c_void,
    ) {
        trace!(
            "LevelDBIterator::register_cleanup: delegating to inner; func={:p}, arg1={:?}, arg2={:?}",
            func as *const (),
            arg1,
            arg2
        );
        self.rep.borrow_mut().register_cleanup(func, arg1, arg2);
    }
}

#[cfg(test)]
mod tests_leveldb_iterator_lifecycle {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static ITERATOR_CLEANUP_CALLED: AtomicUsize = AtomicUsize::new(0);

    fn iterator_cleanup(_arg1: *mut c_void, _arg2: *mut c_void) -> c_void {
        debug!("iterator_cleanup invoked");
        ITERATOR_CLEANUP_CALLED.fetch_add(1, Ordering::SeqCst);
        unsafe { std::mem::zeroed() }
    }

    #[traced_test]
    fn leveldb_iterator_new_creates_inner_without_cleanup() {
        let iter = LevelDBIterator::new();
        let inner_rc = iter.inner();

        // We have two references now: `iter` and `inner_rc`.
        assert_eq!(Rc::strong_count(&inner_rc), 2);

        // Dropping `iter` should not panic, and the inner will be dropped
        // once the last Rc goes away.
        drop(iter);
        drop(inner_rc);
    }

    #[traced_test]
    fn leveldb_iterator_register_cleanup_delegates_to_inner() {
        ITERATOR_CLEANUP_CALLED.store(0, Ordering::SeqCst);

        {
            let iter = LevelDBIterator::new();
            iter.register_cleanup(
                iterator_cleanup,
                core::ptr::null_mut(),
                core::ptr::null_mut(),
            );
        } // iterator (and its inner) dropped here

        assert_eq!(
            ITERATOR_CLEANUP_CALLED.load(Ordering::SeqCst),
            1,
            "Cleanup registered via LevelDBIterator::register_cleanup must run exactly once on drop"
        );
    }
}
