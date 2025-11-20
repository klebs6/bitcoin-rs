// ---------------- [ File: bitcoinleveldb-skiplist/src/skip_list.rs ]
crate::ix!();

/// Arena-backed skiplist, translated from LevelDB's `SkipList<Key, Comparator>`.
///
/// - `K` is the key type. It must be `Copy + Default` so that we can
///   cheaply pass keys by value and create the dummy head key.
/// - `C` is a comparator implementing `SkipListComparator<K>`.
#[derive(Getters,MutGetters)]
#[getset(get="pub",get_mut="pub")]
pub struct SkipList<K, C>
where
    K: Debug + Copy + Default,
    C: SkipListComparator<K>,
{
    /// Immutable after construction
    ///
    compare:    C,

    /// Arena used for allocations of nodes
    ///
    arena:      *mut Arena,

    head:       *mut SkipListNode<K>,

    /// Modified only by Insert(). 
    ///
    /// Read racily by readers, but stale values are ok.
    /// 
    /// Height of the entire list
    /// 
    max_height: AtomicI32,

    /// Read/written only by Insert().
    /// 
    rnd:        Random,
}

/// Maximum height of the skiplist towers.
///
/// This matches LevelDB's kMaxHeight constant.
pub const SkipListMaxHeight: usize = 12;

impl<K, C> SkipList<K, C>
where
    K: Debug + Copy + Default,
    C: SkipListComparator<K>,
{
    /// Return the height of the skiplist (maximum tower height used so far).
    #[inline]
    pub fn get_max_height(&self) -> i32 {
        let h = self.max_height.load(atomic::Ordering::Relaxed);
        trace!("SkipList::get_max_height -> {}", h);
        h
    }
    
    #[inline]
    pub fn equal(&self, a: &K, b: &K) -> bool {
        let r = self.compare.compare(a, b) == 0;
        trace!("SkipList::equal({:?}, {:?}) -> {}", a, b, r);
        r
    }

    /// Allocate and initialize a new node with the given key and height.
    ///
    /// Memory is taken from the arena, matching the C++ implementation:
    ///   sizeof(Node) + sizeof(AtomicPtr<Node>) * (height - 1)
    pub fn new_node(&mut self, key: K, height: i32) -> *mut SkipListNode<K> {
        debug!("SkipList::new_node: key={:?}, height={}", key, height);
        assert!(
            height >= 1 && height <= SkipListMaxHeight as i32,
            "SkipList::new_node: invalid height {}",
            height
        );

        let extra = (height as usize - 1)
            * core::mem::size_of::<core::sync::atomic::AtomicPtr<SkipListNode<K>>>();
        let node_size = core::mem::size_of::<SkipListNode<K>>() + extra;

        unsafe {
            let arena_ref: &mut Arena = &mut *self.arena;
            let mem = arena_ref.allocate_aligned(node_size);
            let node = mem as *mut SkipListNode<K>;

            SkipListNode::<K>::write_key_at(node, key);

            for level in 0..height {
                let slot = SkipListNode::<K>::next_slot(node, level);
                core::ptr::write(
                    slot,
                    core::sync::atomic::AtomicPtr::new(core::ptr::null_mut()),
                );
            }

            node
        }
    }

    /// Randomly choose a height for a new node.
    ///
    /// This matches LevelDB's RandomHeight() with kBranching=4.
    pub fn random_height(&mut self) -> i32 {
        const K_BRANCHING: u32 = 4;

        let mut height: i32 = 1;
        while height < SkipListMaxHeight as i32
            && (self.rnd.next() % K_BRANCHING) == 0
        {
            height += 1;
        }

        trace!("SkipList::random_height -> {}", height);

        assert!(height >= 1);
        assert!(height <= SkipListMaxHeight as i32);
        height
    }

    /// Return true if `key` is greater than the key stored in node `n`.
    ///
    /// A null node is considered to be "infinite".
    pub fn key_is_after_node(&self, key: &K, n: *mut SkipListNode<K>) -> bool {
        if n.is_null() {
            return false;
        }
        unsafe {
            let n_key = (*n).key_ref();
            let cmp = self.compare.compare(n_key, key);
            let result = cmp < 0;
            trace!(
                "SkipList::key_is_after_node: n_key={:?}, key={:?}, cmp={}, result={}",
                n_key,
                key,
                cmp,
                result
            );
            result
        }
    }

    /// Return the earliest node whose key is >= `key`.
    ///
    /// If `prev` is provided, it is filled with the previous node at
    /// each level.
    pub fn find_greater_or_equal(
        &self,
        key:  &K,
        mut prev: Option<&mut [*mut SkipListNode<K>]>,
    ) -> *mut SkipListNode<K> {
        trace!("SkipList::find_greater_or_equal: key={:?}", key);

        let mut x = self.head;
        let mut level = self.get_max_height() - 1;

        loop {
            let next = unsafe { (*x).next(level) };
            if self.key_is_after_node(key, next) {
                // Keep searching in this level.
                x = next;
            } else {
                if let Some(ref mut prev_slice) = prev {
                    prev_slice[level as usize] = x;
                }
                if level == 0 {
                    trace!(
                        "SkipList::find_greater_or_equal: found node={:p}",
                        next
                    );
                    return next;
                } else {
                    level -= 1;
                }
            }
        }
    }

    /// Return the latest node with key < `key`.
    ///
    /// Returns `head_` if there is no such node.
    pub fn find_less_than(&self, key: &K) -> *mut SkipListNode<K> {
        trace!("SkipList::find_less_than: key={:?}", key);

        let mut x = self.head;
        let mut level = self.get_max_height() - 1;

        loop {
            debug_assert!(
                x == self.head
                    || self.compare.compare(
                        unsafe { (*x).key_ref() },
                        key
                    ) < 0
            );
            let next = unsafe { (*x).next(level) };
            let done = if next.is_null() {
                true
            } else {
                let next_key = unsafe { (*next).key_ref() };
                self.compare.compare(next_key, key) >= 0
            };

            if done {
                if level == 0 {
                    trace!("SkipList::find_less_than -> {:p}", x);
                    return x;
                } else {
                    level -= 1;
                }
            } else {
                x = next;
            }
        }
    }

    /// Return the last node in the list, or `head_` if empty.
    pub fn find_last(&self) -> *mut SkipListNode<K> {
        trace!("SkipList::find_last");

        let mut x = self.head;
        let mut level = self.get_max_height() - 1;

        loop {
            let next = unsafe { (*x).next(level) };
            if next.is_null() {
                if level == 0 {
                    trace!("SkipList::find_last -> {:p}", x);
                    return x;
                } else {
                    level -= 1;
                }
            } else {
                x = next;
            }
        }
    }

    /// Create a new SkipList object that will use "compare" for comparing keys,
    /// and will allocate memory using "*arena".
    ///
    /// Objects allocated in the arena must remain allocated for the lifetime of
    /// the skiplist object.
    pub fn new(compare: C, arena: *mut Arena) -> Self {
        info!(
            "SkipList::new: arena={:p}, max_height={}",
            arena, SkipListMaxHeight
        );
        assert!(
            !arena.is_null(),
            "SkipList::new: arena pointer must not be null"
        );

        let mut list = SkipList {
            compare,
            arena,
            head: ptr::null_mut(),
            max_height: AtomicI32::new(1),
            rnd: Random::new(0xdeadbeef),
        };

        // Create the dummy head node with maximum height.
        let head = list.new_node(K::default(), SkipListMaxHeight as i32);
        list.head = head;

        for level in 0..SkipListMaxHeight as i32 {
            unsafe {
                (*head).no_barrier_set_next(level, ptr::null_mut());
            }
        }

        list
    }
 
    /// Insert `key` into the skiplist.
    ///
    /// REQUIRES: no existing entry compares equal to `key`.
    pub fn insert(&mut self, key: K) {
        trace!("SkipList::insert: key={:?}", key);

        // Find the insertion position and populate `prev`.
        let mut prev: [*mut SkipListNode<K>; SkipListMaxHeight] =
            [ptr::null_mut(); SkipListMaxHeight];
        let x = self.find_greater_or_equal(&key, Some(&mut prev[..]));

        // Our data structure does not allow duplicate insertion.
        if !x.is_null() {
            let x_key = unsafe { (*x).key_ref() };
            assert!(
                !self.equal(&key, x_key),
                "SkipList::insert: duplicate key inserted: {:?}",
                key
            );
        }

        // Choose a random height for the new node.
        let height = self.random_height();
        let current_max = self.get_max_height();

        if height > current_max {
            for level in current_max..height {
                prev[level as usize] = self.head;
            }

            // See comments in the C++ code for why this relaxed store is safe.
            self.max_height
                .store(height, atomic::Ordering::Relaxed);
        }

        let node = self.new_node(key, height);
        for level in 0..height {
            unsafe {
                let prev_node = prev[level as usize];
                let next = (*prev_node).no_barrier_next(level);
                (*node).no_barrier_set_next(level, next);
                (*prev_node).set_next(level, node);
            }
        }
    }

    /// Return true iff the list contains an entry equal to `key`.
    pub fn contains(&self, key: &K) -> bool {
        trace!("SkipList::contains: key={:?}", key);
        let x = self.find_greater_or_equal(key, None);
        if x.is_null() {
            false
        } else {
            let x_key = unsafe { (*x).key_ref() };
            self.equal(key, x_key)
        }
    }
}

#[cfg(test)]
mod skip_list_contract_suite {
    use super::*;
    use bitcoin_imports::*;
    use bitcoinleveldb_arena::Arena;

    struct U64Cmp;
    impl SkipListComparator<u64> for U64Cmp {
        #[inline]
        fn compare(&self, a: &u64, b: &u64) -> i32 {
            if a < b { -1 } else if a > b { 1 } else { 0 }
        }
    }

    #[traced_test]
    fn new_list_exposes_empty_invariants() {
        info!("new_list_exposes_empty_invariants: start");

        let mut arena = Arena::default();
        let list: SkipList<u64, U64Cmp> = SkipList::new(U64Cmp, &mut arena as *mut Arena);

        // ---------------------------------------------
        // Extract the raw head pointer correctly
        // ---------------------------------------------
        let head_ref = list.head();           // &*mut SkipListNode<u64>
        let head_ptr: *mut SkipListNode<u64> = *head_ref;

        // ---------------------------------------------
        // head->next(0) must be NULL on an empty list
        // ---------------------------------------------
        unsafe {
            let next0 = (&*head_ptr).no_barrier_next(0);
            assert!(
                next0.is_null(),
                "head.next(0) must be null for an empty skiplist"
            );
        }

        // ---------------------------------------------
        // find_less_than must return head
        // ---------------------------------------------
        let lt = list.find_less_than(&0);
        assert!(
            lt == head_ptr,
            "find_less_than must return head on an empty list"
        );

        // ---------------------------------------------
        // find_last must return head
        // ---------------------------------------------
        let last = list.find_last();
        assert!(
            last == head_ptr,
            "find_last must return head on an empty list"
        );

        // ---------------------------------------------
        // find_greater_or_equal must yield null
        // ---------------------------------------------
        let ge = list.find_greater_or_equal(&0, None);
        assert!(
            ge.is_null(),
            "find_greater_or_equal must return null on empty list"
        );

        info!("new_list_exposes_empty_invariants: done");
    }

    #[traced_test]
    fn insert_and_query_basic_ordering() {
        info!("insert_and_query_basic_ordering: start");

        let mut arena = Arena::default();
        let mut list: SkipList<u64, U64Cmp> = SkipList::new(U64Cmp, &mut arena as *mut Arena);

        for k in [10_u64, 20, 30] {
            list.insert(k);
        }

        assert!(list.contains(&10), "list must contain inserted key 10");
        assert!(list.contains(&20), "list must contain inserted key 20");
        assert!(list.contains(&30), "list must contain inserted key 30");
        assert!(!list.contains(&25), "list must not contain absent key 25");

        let p = list.find_less_than(&15);
        unsafe {
            assert_eq!(*(&*p).key_ref(), 10_u64, "LT(15) must be 10");
        }

        let q = list.find_greater_or_equal(&15, None);
        unsafe {
            assert_eq!(*(&*q).key_ref(), 20_u64, "GE(15) must be 20");
        }

        let last = list.find_last();
        unsafe {
            assert_eq!(*(&*last).key_ref(), 30_u64, "find_last must yield 30");
        }

        info!("insert_and_query_basic_ordering: done");
    }

    #[traced_test]
    fn random_height_is_within_bounds() {
        info!("random_height_is_within_bounds: start");

        let mut arena = Arena::default();
        let mut list: SkipList<u64, U64Cmp> = SkipList::new(U64Cmp, &mut arena as *mut Arena);

        for _ in 0..10_000 {
            let h = list.random_height();
            assert!(h >= 1, "height must be at least 1");
            assert!(h <= SkipListMaxHeight as i32, "height must be at most SkipListMaxHeight");
        }

        info!("random_height_is_within_bounds: done");
    }

    #[traced_test]
    fn duplicate_insertion_panics() {
        info!("duplicate_insertion_panics: start");

        let mut arena = Arena::default();
        let mut list: SkipList<u64, U64Cmp> = SkipList::new(U64Cmp, &mut arena as *mut Arena);

        list.insert(7);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            list.insert(7);
        }));

        assert!(result.is_err(), "inserting duplicate key must panic");
        info!("duplicate_insertion_panics: done");
    }
}
