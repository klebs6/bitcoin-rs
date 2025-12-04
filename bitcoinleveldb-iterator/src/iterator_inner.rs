// ---------------- [ File: bitcoinleveldb-iterator/src/iterator_inner.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/iterator.h]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/iterator.cc]
/// An iterator yields a sequence of key/value pairs from a source.  The
/// following class defines the interface.  Multiple implementations are
/// provided by this library. In particular, iterators are provided to access
/// the contents of a Table or a DB.
/// 
/// Multiple threads can invoke const methods on an Iterator without external
/// synchronization, but if any of the threads may call a non-const method, all
/// threads accessing the same Iterator must use external synchronization.
///
#[derive(Default)]
pub(crate) struct LevelDBIteratorInner {
    cleanup_head: Option<LevelDBIteratorCleanupNode>,
}

impl Drop for LevelDBIteratorInner {
    fn drop(&mut self) {
        trace!("LevelDBIteratorInner::drop: begin");

        // Take ownership of the head node (if any) so we can safely
        // walk and free the rest of the list without borrowing self.
        if let Some(mut head) = self.cleanup_head.take() {
            let head_next = *head.next();
            trace!(
                "LevelDBIteratorInner::drop: head node present (is_empty={}, next={:?})",
                head.is_empty(),
                head_next
            );

            if !head.is_empty() {
                head.run();
            } else {
                trace!(
                    "LevelDBIteratorInner::drop: head node logically empty; no cleanup invoked"
                );
            }

            let mut node_ptr: *mut LevelDBIteratorCleanupNode = head_next;

            unsafe {
                while !node_ptr.is_null() {
                    // Reconstruct the Box so Rust can free this node once
                    // we are done with it.
                    let mut boxed: Box<LevelDBIteratorCleanupNode> =
                        Box::from_raw(node_ptr);
                    let next_ptr = *boxed.next();

                    trace!(
                        "LevelDBIteratorInner::drop: running cleanup node {:p} (next={:?}, is_empty={})",
                        node_ptr,
                        next_ptr,
                        boxed.is_empty()
                    );

                    if !boxed.is_empty() {
                        boxed.run();
                    } else {
                        trace!(
                            "LevelDBIteratorInner::drop: encountered logically empty cleanup node {:p}",
                            node_ptr
                        );
                    }

                    // Move to the next node; `boxed` is dropped here and
                    // its memory reclaimed.
                    node_ptr = next_ptr;
                }
            }

            trace!("LevelDBIteratorInner::drop: all cleanup nodes processed");
        } else {
            trace!("LevelDBIteratorInner::drop: no cleanup handlers registered");
        }
    }
}

impl LevelDBIteratorInner {

    pub fn new() -> Self {
        trace!("LevelDBIteratorInner::new");
        LevelDBIteratorInner {
            cleanup_head: None,
        }
    }

    /**
      | Clients are allowed to register
      | function/arg1/arg2 triples that will be
      | invoked when this iterator is destroyed.
      |
      | Note that unlike all of the preceding
      | methods, this method is not abstract and
      | therefore clients should not override it.
      */
    pub fn register_cleanup(
        &mut self,
        func: LevelDBIteratorCleanupFunction,
        arg1: *mut c_void,
        arg2: *mut c_void,
    ) {
        trace!(
            "LevelDBIteratorInner::register_cleanup: func={:p}, arg1={:?}, arg2={:?}",
            func as *const (),
            arg1,
            arg2
        );

        // In Rust, function pointers are non‑null, but we keep this
        // assertion to mirror the original intent.
        assert!(
            (func as *const ()) as usize != 0,
            "LevelDBIteratorInner::register_cleanup: null function pointer"
        );

        match self.cleanup_head.as_mut() {
            None => {
                // First cleanup node becomes the inlined "head".
                let head = LevelDBIteratorCleanupNodeBuilder::default()
                    .function(Some(func))
                    .arg1(arg1)
                    .arg2(arg2)
                    .next(core::ptr::null_mut())
                    .build()
                    .unwrap();

                trace!(
                    "LevelDBIteratorInner::register_cleanup: created head node {:p}",
                    &head as *const _
                );

                self.cleanup_head = Some(head);
            }
            Some(head) => {
                // Subsequent nodes are heap allocated and linked off head.
                let current_next = *head.next();

                unsafe {
                    let node = Box::new(LevelDBIteratorCleanupNodeBuilder::default()
                        .function(Some(func))
                        .arg1(arg1)
                        .arg2(arg2)
                        .next(current_next)
                        .build()
                        .unwrap()
                    );

                    let node_ptr: *mut LevelDBIteratorCleanupNode =
                        Box::into_raw(node);

                    trace!(
                        "LevelDBIteratorInner::register_cleanup: inserting node {:p} after head {:p} (old_next={:?})",
                        node_ptr,
                        head as *mut _,
                        current_next
                    );

                    *head.next_mut() = node_ptr;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests_iterator_inner_cleanup_chain {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn increment_cleanup_counter(arg1: *mut c_void, _arg2: *mut c_void) -> c_void {
        debug!(
            "increment_cleanup_counter invoked with arg1={:?}, arg2={:?}",
            arg1,
            _arg2
        );
        assert!(
            !arg1.is_null(),
            "increment_cleanup_counter received null counter pointer"
        );

        unsafe {
            let counter: &AtomicUsize = &*(arg1 as *const AtomicUsize);
            counter.fetch_add(1, Ordering::SeqCst);
        }

        unsafe { core::mem::zeroed() }
    }

    #[traced_test]
    fn iterator_inner_runs_single_cleanup_on_drop() {
        let first_called = AtomicUsize::new(0);

        {
            let mut inner = LevelDBIteratorInner::new();

            let counter_ptr: *mut AtomicUsize =
                &first_called as *const AtomicUsize as *mut AtomicUsize;

            inner.register_cleanup(
                increment_cleanup_counter,
                counter_ptr.cast(),
                core::ptr::null_mut(),
            );
        } // `inner` dropped here; cleanup must run exactly once.

        assert_eq!(
            first_called.load(Ordering::SeqCst),
            1,
            "Drop must run exactly one registered cleanup function"
        );
    }

    #[traced_test]
    fn iterator_inner_runs_all_cleanup_nodes_in_chain() {
        let first_called = AtomicUsize::new(0);
        let second_called = AtomicUsize::new(0);

        {
            let mut inner = LevelDBIteratorInner::new();

            let first_ptr: *mut AtomicUsize =
                &first_called as *const AtomicUsize as *mut AtomicUsize;
            let second_ptr: *mut AtomicUsize =
                &second_called as *const AtomicUsize as *mut AtomicUsize;

            inner.register_cleanup(
                increment_cleanup_counter,
                first_ptr.cast(),
                core::ptr::null_mut(),
            );
            inner.register_cleanup(
                increment_cleanup_counter,
                second_ptr.cast(),
                core::ptr::null_mut(),
            );
        } // `inner` dropped here; both cleanup nodes must run exactly once.

        assert_eq!(
            first_called.load(Ordering::SeqCst),
            1,
            "First cleanup function must be invoked exactly once"
        );
        assert_eq!(
            second_called.load(Ordering::SeqCst),
            1,
            "Second cleanup function must be invoked exactly once"
        );
    }

    #[traced_test]
    fn iterator_inner_handles_drop_without_registered_cleanup() {
        {
            let _inner = LevelDBIteratorInner::new();
            // No cleanup handlers registered; drop must be a no‑op w.r.t. callbacks.
        }

        // Reaching here without panic is sufficient; there are no callbacks to
        // observe when none are registered.
    }
}

#[cfg(test)]
mod iterator_inner_cleanup_chain_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn increment_cleanup_counter(arg1: *mut c_void, _arg2: *mut c_void) -> c_void {
        debug!(
            "increment_cleanup_counter invoked with arg1={:?}, arg2={:?}",
            arg1,
            _arg2
        );
        assert!(
            !arg1.is_null(),
            "increment_cleanup_counter received null counter pointer"
        );

        unsafe {
            let counter: &AtomicUsize = &*(arg1 as *const AtomicUsize);
            counter.fetch_add(1, Ordering::SeqCst);
        }

        unsafe { core::mem::zeroed() }
    }

    #[traced_test]
    fn iterator_inner_runs_single_cleanup_on_drop() {
        let first_called = AtomicUsize::new(0);

        {
            let mut inner = LevelDBIteratorInner::new();

            let counter_ptr: *mut AtomicUsize =
                &first_called as *const AtomicUsize as *mut AtomicUsize;

            inner.register_cleanup(
                increment_cleanup_counter,
                counter_ptr.cast(),
                core::ptr::null_mut(),
            );
        } // `inner` dropped here; cleanup must run exactly once.

        assert_eq!(
            first_called.load(Ordering::SeqCst),
            1,
            "Drop must run exactly one registered cleanup function"
        );
    }

    #[traced_test]
    fn iterator_inner_runs_all_cleanup_nodes_in_chain() {
        let first_called = AtomicUsize::new(0);
        let second_called = AtomicUsize::new(0);

        {
            let mut inner = LevelDBIteratorInner::new();

            let first_ptr: *mut AtomicUsize =
                &first_called as *const AtomicUsize as *mut AtomicUsize;
            let second_ptr: *mut AtomicUsize =
                &second_called as *const AtomicUsize as *mut AtomicUsize;

            inner.register_cleanup(
                increment_cleanup_counter,
                first_ptr.cast(),
                core::ptr::null_mut(),
            );
            inner.register_cleanup(
                increment_cleanup_counter,
                second_ptr.cast(),
                core::ptr::null_mut(),
            );
        } // `inner` dropped here; both cleanup nodes must run exactly once.

        assert_eq!(
            first_called.load(Ordering::SeqCst),
            1,
            "First cleanup function must be invoked exactly once"
        );
        assert_eq!(
            second_called.load(Ordering::SeqCst),
            1,
            "Second cleanup function must be invoked exactly once"
        );
    }

    #[traced_test]
    fn iterator_inner_handles_drop_without_registered_cleanup() {
        {
            let _inner = LevelDBIteratorInner::new();
            // No cleanup handlers registered; drop must be a no‑op w.r.t. callbacks.
        }

        // Reaching here without panic is sufficient; there are no callbacks to
        // observe when none are registered.
    }
}
