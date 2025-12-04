// ---------------- [ File: bitcoinleveldb-iterator/src/iterator_cleanup.rs ]
crate::ix!();

pub type LevelDBIteratorCleanupFunction =
    fn(arg1: *mut c_void, arg2: *mut c_void) -> c_void;

/**
  | Cleanup functions are stored in
  | a single-linked list.
  |
  | The list's head node is inlined in the
  | iterator.
  */
#[derive(Builder,Getters, MutGetters)]
#[getset(get = "pub(crate)", get_mut = "pub(crate)")]
#[builder(setter(into))]
pub struct LevelDBIteratorCleanupNode {
    /**
      | The head node is used if the function
      | pointer is not null.
      |
      */
    function: Option<LevelDBIteratorCleanupFunction>,

    arg1: *mut c_void,
    arg2: *mut c_void,
    next: *mut LevelDBIteratorCleanupNode,
}

impl LevelDBIteratorCleanupNode {

    pub fn new(
        function: Option<LevelDBIteratorCleanupFunction>,
        arg1: *mut c_void,
        arg2: *mut c_void,
        next: *mut LevelDBIteratorCleanupNode,
    ) -> Self {
        trace!(
            "LevelDBIteratorCleanupNode::new: function_present={}, arg1={:?}, arg2={:?}, next={:?}",
            function.is_some(),
            arg1,
            arg2,
            next
        );
        LevelDBIteratorCleanupNode {
            function,
            arg1,
            arg2,
            next,
        }
    }

    /**
      | True if the node is not used. Only head
      | nodes might be unused.
      */
    pub fn is_empty(&self) -> bool {
        let is_empty = self.function.is_none();
        trace!(
            "LevelDBIteratorCleanupNode::is_empty called; is_empty={}, function_present={}, arg1={:?}, arg2={:?}, next={:?}",
            is_empty,
            self.function.is_some(),
            self.arg1,
            self.arg2,
            self.next
        );
        is_empty
    }

    /**
      | Invokes the cleanup function.
      |
      */
    pub fn run(&mut self) {
        match self.function {
            None => {
                warn!(
                    "LevelDBIteratorCleanupNode::run called on empty node; arg1={:?}, arg2={:?}, next={:?}",
                    self.arg1,
                    self.arg2,
                    self.next
                );
            }
            Some(func) => {
                trace!(
                    "LevelDBIteratorCleanupNode::run: invoking cleanup function {:p} with arg1={:?}, arg2={:?}",
                    func as *const (),
                    self.arg1,
                    self.arg2
                );
                func(self.arg1, self.arg2);
            }
        }
    }

    /**
      | Accessor for the next pointer in the cleanup
      | singly‑linked list.
      |
      */
    pub fn next_ptr(&self) -> *mut LevelDBIteratorCleanupNode {
        trace!(
            "LevelDBIteratorCleanupNode::next_ptr called; self={:p}, next={:?}",
            self as *const _,
            self.next
        );
        self.next
    }

    /**
      | Mutator for the next pointer in the cleanup
      | singly‑linked list.
      |
      */
    pub fn set_next_ptr(&mut self, next: *mut LevelDBIteratorCleanupNode) {
        trace!(
            "LevelDBIteratorCleanupNode::set_next_ptr called; self={:p}, old_next={:?}, new_next={:?}",
            self as *const _,
            self.next,
            next
        );
        self.next = next;
    }
}

#[cfg(test)]
mod tests_iterator_cleanup_node {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn increment_node_run_count(arg1: *mut c_void, arg2: *mut c_void) -> c_void {
        debug!(
            "increment_node_run_count invoked with arg1={:?}, arg2={:?}",
            arg1,
            arg2
        );
        assert!(
            !arg1.is_null(),
            "increment_node_run_count received null counter pointer"
        );

        unsafe {
            let counter: &AtomicUsize = &*(arg1 as *const AtomicUsize);
            counter.fetch_add(1, Ordering::SeqCst);
        }

        unsafe { core::mem::zeroed() }
    }

    #[traced_test]
    fn cleanup_node_is_empty_when_no_function() {
        let run_count = AtomicUsize::new(0);

        let node = LevelDBIteratorCleanupNode::new(
            None,
            core::ptr::null_mut(),
            (&run_count as *const AtomicUsize as *mut AtomicUsize).cast(),
            core::ptr::null_mut(),
        );

        assert!(
            node.is_empty(),
            "Node with no function must report is_empty()"
        );
        assert_eq!(
            run_count.load(Ordering::SeqCst),
            0,
            "Run count must remain zero when no cleanup function is configured"
        );
    }

    #[traced_test]
    fn cleanup_node_run_invokes_function() {
        let run_count = AtomicUsize::new(0);

        let mut node = LevelDBIteratorCleanupNode::new(
            Some(increment_node_run_count),
            (&run_count as *const AtomicUsize as *mut AtomicUsize).cast(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );

        assert!(
            !node.is_empty(),
            "Node with a function must not report is_empty()"
        );

        node.run();

        assert_eq!(
            run_count.load(Ordering::SeqCst),
            1,
            "cleanup function must be invoked exactly once"
        );
    }

    #[traced_test]
    fn cleanup_node_next_accessors_round_trip() {
        let dummy_next: *mut LevelDBIteratorCleanupNode = 0x1 as *mut _;

        let mut node = LevelDBIteratorCleanupNode::new(
            Some(increment_node_run_count),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        );

        node.set_next_ptr(dummy_next);

        assert_eq!(
            node.next_ptr(),
            dummy_next,
            "set_next_ptr and next_ptr must round‑trip the same pointer"
        );
    }
}

#[cfg(test)]
mod iterator_cleanup_node_behavior_tests {
    use super::*;

    fn test_cleanup_flag(arg1: *mut c_void, _arg2: *mut c_void) -> c_void {
        debug!("test_cleanup_flag invoked");
        unsafe {
            let flag_ptr = arg1 as *mut bool;
            assert!(!flag_ptr.is_null(), "flag pointer must not be null");
            *flag_ptr = true;
        }
        // Return value is unused; we just satisfy the function type.
        unsafe { core::mem::zeroed() }
    }

    #[traced_test]
    fn cleanup_node_reports_empty_correctly() {
        let node_empty = LevelDBIteratorCleanupNode {
            function: None,
            arg1: core::ptr::null_mut(),
            arg2: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
        };

        assert!(
            node_empty.is_empty(),
            "Node with no function should report empty"
        );

        let node_non_empty = LevelDBIteratorCleanupNode {
            function: Some(test_cleanup_flag),
            arg1: core::ptr::null_mut(),
            arg2: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
        };

        assert!(
            !node_non_empty.is_empty(),
            "Node with a function should not report empty"
        );
    }

    #[traced_test]
    fn cleanup_node_run_invokes_function_when_present() {
        let mut flag = false;

        let mut node = LevelDBIteratorCleanupNode {
            function: Some(test_cleanup_flag),
            arg1: (&mut flag as *mut bool).cast(),
            arg2: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
        };

        assert!(!flag, "flag should start as false");
        node.run();
        assert!(flag, "cleanup function should have set the flag to true");
    }

    #[traced_test]
    fn cleanup_node_run_is_noop_on_empty_node() {
        let mut node = LevelDBIteratorCleanupNode {
            function: None,
            arg1: core::ptr::null_mut(),
            arg2: core::ptr::null_mut(),
            next: core::ptr::null_mut(),
        };

        // This should not panic even though no function is registered.
        node.run();
    }
}
