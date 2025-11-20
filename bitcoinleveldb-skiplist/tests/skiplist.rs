// ---------------- [ File: bitcoinleveldb-skiplist/tests/skiplist.rs ]
use bitcoinleveldb_skiplist::*;
use bitcoinleveldb_arena::*;
use bitcoinleveldb_rand::*;
use bitcoinleveldb_key::*;
use bitcoinleveldb_hash::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/skiplist_test.cc]

type TestKey = u64;

/// Simple comparator for `u64` keys.
#[derive(Clone, Copy, Default, Debug)]
struct TestComparator;

impl SkipListComparator<TestKey> for TestComparator {
    fn compare(&self, a: &TestKey, b: &TestKey) -> i32 {
        if a < b {
            -1
        } else if a > b {
            1
        } else {
            0
        }
    }
}

/// Basic empty-skiplist tests, translated from LevelDB's `SkipTest::Empty`.
#[traced_test]
fn skiplist_empty_behaviour() {
    info!("skiplist_empty_behaviour: start");

    let mut arena = Arena::default();
    let cmp = TestComparator::default();
    let list: SkipList<TestKey, TestComparator> =
        SkipList::new(cmp, &mut arena as *mut Arena);

    assert!(
        !list.contains(&10),
        "Empty skiplist must not contain arbitrary key"
    );

    let mut iter = SkipListIterator::new(&list);

    assert!(!iter.valid(), "Fresh iterator should be invalid");

    iter.seek_to_first();
    assert!(!iter.valid(), "seek_to_first on empty list must be invalid");

    iter.seek(&100);
    assert!(!iter.valid(), "seek on empty list must be invalid");

    iter.seek_to_last();
    assert!(!iter.valid(), "seek_to_last on empty list must be invalid");

    info!("skiplist_empty_behaviour: done");
}

/// Insert a bunch of keys and verify lookup and iterator behaviour.
#[traced_test]
fn skiplist_insert_and_lookup() {
    info!("skiplist_insert_and_lookup: start");

    const N: i32 = 2000;
    const R: u64 = 5000;

    let mut rnd = Random::new(1000);
    let mut keys = BTreeSet::<TestKey>::new();

    let mut arena = Arena::default();
    let cmp = TestComparator::default();
    let mut list: SkipList<TestKey, TestComparator> =
        SkipList::new(cmp, &mut arena as *mut Arena);

    // Populate skiplist with random keys.
    for _ in 0..N {
        let raw = rnd.next() as u64;
        let key = raw % R;
        if keys.insert(key) {
            list.insert(key);
        }
    }

    // Verify `contains` matches model set.
    for i in 0..R {
        let in_list = list.contains(&i);
        let in_set = keys.contains(&i);
        assert_eq!(
            in_list, in_set,
            "Mismatch for key {}: list={}, set={}",
            i, in_list, in_set
        );
    }

    // Simple iterator tests.
    {
        let mut iter = SkipListIterator::new(&list);
        assert!(!iter.valid(), "Fresh iterator must be invalid");

        if keys.is_empty() {
            iter.seek(&0);
            assert!(
                !iter.valid(),
                "seek on empty key-set must leave iterator invalid"
            );
            iter.seek_to_first();
            assert!(
                !iter.valid(),
                "seek_to_first on empty key-set must be invalid"
            );
            iter.seek_to_last();
            assert!(
                !iter.valid(),
                "seek_to_last on empty key-set must be invalid"
            );
        } else {
            let first = *keys.iter().next().unwrap();
            let last = *keys.iter().next_back().unwrap();

            iter.seek(&0);
            assert!(iter.valid(), "seek(0) should be valid");
            assert_eq!(
                first,
                iter.key(),
                "seek(0) must land on smallest key"
            );

            iter.seek_to_first();
            assert!(iter.valid(), "seek_to_first should be valid");
            assert_eq!(
                first,
                iter.key(),
                "seek_to_first must land on smallest key"
            );

            iter.seek_to_last();
            assert!(iter.valid(), "seek_to_last should be valid");
            assert_eq!(
                last,
                iter.key(),
                "seek_to_last must land on largest key"
            );
        }
    }

    // Forward iteration tests: for each possible starting key i,
    // check the first three entries against the model set.
    for i in 0..R {
        let mut iter = SkipListIterator::new(&list);
        iter.seek(&i);

        let mut model_iter = keys.range(i..);
        for _ in 0..3 {
            match model_iter.next() {
                None => {
                    assert!(
                        !iter.valid(),
                        "Iterator should be invalid once model_iter is exhausted (start i={})",
                        i
                    );
                    break;
                }
                Some(&model_key) => {
                    assert!(
                        iter.valid(),
                        "Iterator should be valid when model iterator has a key (start i={})",
                        i
                    );
                    assert_eq!(
                        model_key,
                        iter.key(),
                        "Iterator key mismatch (start i={})",
                        i
                    );
                    iter.next();
                }
            }
        }
    }

    // Backward iteration test.
    if !keys.is_empty() {
        let mut iter = SkipListIterator::new(&list);
        iter.seek_to_last();

        for &model_key in keys.iter().rev() {
            assert!(
                iter.valid(),
                "Iterator should be valid while model reverse-iter has entries"
            );
            assert_eq!(
                model_key,
                iter.key(),
                "Backward iteration mismatch"
            );
            iter.prev();
        }
        assert!(
            !iter.valid(),
            "Iterator must be invalid after stepping back past first element"
        );
    }

    info!("skiplist_insert_and_lookup: done");
}

// -----------------------------------------------------------------------------
// Concurrent tests (single writer, concurrent readers)
// -----------------------------------------------------------------------------

type Key = u64;

mod concurrent_test {
    use super::*;

    pub const K: usize = 4;

    /// Per-key generation state, stored atomically.
    #[derive(Debug)]
    pub struct State {
        generation: [AtomicI32; K],
    }

    impl Default for State {
        fn default() -> Self {
            trace!("concurrent_test::State::default");
            State {
                generation: [
                    AtomicI32::new(0),
                    AtomicI32::new(0),
                    AtomicI32::new(0),
                    AtomicI32::new(0),
                ],
            }
        }
    }

    impl State {
        pub fn set(&self, k: i32, v: i32) {
            trace!("State::set: k={}, v={}", k, v);
            assert!(
                (k as usize) < K,
                "State::set: key index {} out of range",
                k
            );
            self.generation[k as usize].store(v, atomic::Ordering::Release);
        }

        pub fn get(&self, k: i32) -> i32 {
            assert!(
                (k as usize) < K,
                "State::get: key index {} out of range",
                k
            );
            let v = self.generation[k as usize].load(atomic::Ordering::Acquire);
            trace!("State::get: k={}, v={}", k, v);
            v
        }
    }
}

/// We want to make sure that with a single writer and multiple concurrent
/// readers (with no synchronization other than when a reader's iterator is
/// created), the reader always observes all the data that was present in the
/// skip list when the iterator was constructed.
///
/// Because insertions are happening concurrently, we may also observe new
/// values that were inserted since the iterator was constructed, but we should
/// never miss any values that were present at iterator construction time.
/// 
/// We generate multi-part keys:
///     <key,gen,hash>
/// where:
///     key is in range [0..K-1]
///     gen is a generation number for key
///     hash is hash(key,gen)
/// 
/// The insertion code picks a random key, sets gen to be 1 + the last
/// generation number inserted for that key, and sets hash to Hash(key,gen).
/// 
/// At the beginning of a read, we snapshot the last inserted generation number
/// for each key. We then iterate, including random calls to Next() and Seek().  
///
/// For every key we encounter, we check that it is either expected given the
/// initial snapshot or has been concurrently added since the iterator started.
///
struct ConcurrentTest {

    /// Current state of the test
    current: concurrent_test::State,
    arena:   Box<Arena>,

    /// SkipList is not protected by mu_. 
    ///
    /// We just use a single writer thread to modify it.
    list:    SkipList<Key, TestComparator>,
}

impl Default for ConcurrentTest {
    fn default() -> Self {
        info!("ConcurrentTest::default: constructing");

        let mut arena = Box::new(Arena::default());
        let cmp = TestComparator::default();
        let arena_ptr: *mut Arena = &mut *arena;

        let list: SkipList<Key, TestComparator> =
            SkipList::new(cmp, arena_ptr);

        ConcurrentTest {
            current: concurrent_test::State::default(),
            arena,
            list,
        }
    }
}

impl ConcurrentTest {
    #[inline]
    fn key_from_composite(k: Key) -> u64 {
        k >> 40
    }

    #[inline]
    fn gen_from_composite(k: Key) -> u64 {
        (k >> 8) & 0xffff_ffff
    }

    #[inline]
    fn hash_from_composite(k: Key) -> u64 {
        k & 0xff
    }

    fn hash_numbers(k: u64, g: u64) -> u64 {
        let mut buf = [0u8; 16];
        let k_bytes = encode_fixed64_le(k);
        let g_bytes = encode_fixed64_le(g);
        buf[..8].copy_from_slice(&k_bytes);
        buf[8..].copy_from_slice(&g_bytes);

        // Use the LevelDB hash with explicit pointer, length, and seed.
        let h = leveldb_hash(buf.as_ptr(), buf.len(), 0);
        trace!(
            "ConcurrentTest::hash_numbers: k={}, g={}, hash=0x{:08x}",
            k, g, h
        );
        h as u64
    }

    fn make_key(k: u64, g: u64) -> Key {
        const_assert!(std::mem::size_of::<Key>() == std::mem::size_of::<u64>());

        assert!(
            k <= concurrent_test::K as u64,
            "make_key: k={} out of range",
            k
        );
        assert!(
            g <= 0xffff_ffff,
            "make_key: g={} out of range",
            g
        );

        let hash = Self::hash_numbers(k, g) & 0xff;
        (k << 40) | (g << 8) | hash
    }

    fn is_valid_key(k: Key) -> bool {
        let key_part = Self::key_from_composite(k);
        let gen_part = Self::gen_from_composite(k);
        let hash = Self::hash_from_composite(k);
        let expected = Self::hash_numbers(key_part, gen_part) & 0xff;
        hash == expected
    }

    fn random_target(rnd: &mut Random) -> Key {
        match rnd.next() % 10 {
            0 => Self::make_key(0, 0),
            1 => Self::make_key(concurrent_test::K as u64, 0),
            _ => {
                let k = (rnd.next() as usize % concurrent_test::K) as u64;
                Self::make_key(k, 0)
            }
        }
    }

    /// Single-writer step: insert a new generation for a random key slot.
    fn write_step(&mut self, rnd: &mut Random) {
        let k = (rnd.next() as usize % concurrent_test::K) as i32;
        let g = self.current.get(k) + 1;
        let key = Self::make_key(k as u64, g as u64);

        self.list.insert(key);
        self.current.set(k, g);
    }

    /// Reader step: verify that all visible keys respect snapshot semantics.
    fn read_step(&self, rnd: &mut Random) {
        // Snapshot the current generation state.
        let mut initial_state = concurrent_test::State::default();
        for k in 0..concurrent_test::K as i32 {
            let v = self.current.get(k);
            initial_state.set(k, v);
        }

        let mut pos = Self::random_target(rnd);
        let mut iter = SkipListIterator::new(&self.list);
        iter.seek(&pos);

        loop {
            let current = if iter.valid() {
                let c = iter.key();
                assert!(
                    Self::is_valid_key(c),
                    "read_step: observed invalid key {}",
                    c
                );
                c
            } else {
                Self::make_key(concurrent_test::K as u64, 0)
            };

            assert!(
                pos <= current,
                "read_step: iterator went backwards (pos={}, current={})",
                pos,
                current
            );

            // Verify that everything in [pos, current) was not present in
            // initial_state at iterator construction time.
            while pos < current {
                let key_slot = Self::key_from_composite(pos);
                assert!(
                    key_slot < concurrent_test::K as u64,
                    "read_step: key slot {} out of range (pos={})",
                    key_slot,
                    pos
                );

                let gen = Self::gen_from_composite(pos);
                if gen != 0 {
                    let snap_gen =
                        initial_state.get(key_slot as i32) as u64;
                    assert!(
                        gen > snap_gen,
                        "read_step: missing key at pos={}, key_slot={}, gen={}, snap_gen={}",
                        pos,
                        key_slot,
                        gen,
                        snap_gen
                    );
                }

                // Advance pos to next key in valid key space.
                if Self::key_from_composite(pos)
                    < Self::key_from_composite(current)
                {
                    pos = Self::make_key(
                        Self::key_from_composite(pos) + 1,
                        0,
                    );
                } else {
                    pos = Self::make_key(
                        Self::key_from_composite(pos),
                        Self::gen_from_composite(pos) + 1,
                    );
                }
            }

            if !iter.valid() {
                break;
            }

            if (rnd.next() % 2) == 0 {
                iter.next();
                pos = Self::make_key(
                    Self::key_from_composite(pos),
                    Self::gen_from_composite(pos) + 1,
                );
            } else {
                let new_target = Self::random_target(rnd);
                if new_target > pos {
                    pos = new_target;
                    iter.seek(&new_target);
                }
            }
        }
    }
}

/// Reader thread state machine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReaderState {
    Starting,
    Running,
    Done,
}

/// Shared state used by writer and reader threads in the concurrent tests.
struct TestState {
    t:         Mutex<ConcurrentTest>,
    seed:      i32,
    quit_flag: AtomicBool,
    mu:        Mutex<ReaderState>,
    cv:        Condvar,
}

unsafe impl Send for TestState {}
unsafe impl Sync for TestState {}

impl TestState {

    fn new(seed: i32) -> Self {
        info!("TestState::new: seed={}", seed);
        TestState {
            t:         Mutex::new(ConcurrentTest::default()),
            seed,
            quit_flag: AtomicBool::new(false),
            mu:        Mutex::new(ReaderState::Starting),
            cv:        Condvar::new(),
        }
    }

    fn wait_for_state(&self, desired: ReaderState) {
        let mut state = self.mu.lock();
        while *state != desired {
            self.cv.wait(&mut state);
        }
        trace!(
            "TestState::wait_for_state: reached desired state: {:?}",
            desired
        );
    }

    fn change_state(&self, new_state: ReaderState) {
        {
            let mut state = self.mu.lock();
            *state = new_state;
            trace!(
                "TestState::change_state: updated state to {:?}",
                new_state
            );
        }
        self.cv.notify_all();
    }
}

/// Reader thread body.
fn concurrent_reader(state: Arc<TestState>) {
    info!("concurrent_reader: starting");
    let mut rnd = Random::new(state.seed as u32);
    let mut reads: i64 = 0;

    state.change_state(ReaderState::Running);
    while !state.quit_flag.load(atomic::Ordering::Acquire) {
        {
            let guard = state.t.lock();
            guard.read_step(&mut rnd);
        }
        reads += 1;
    }

    info!("concurrent_reader: completed {} reads", reads);
    state.change_state(ReaderState::Done);
}

/// Run the concurrent test with a particular run id, matching LevelDB's
/// skiplist_test.cc structure.
fn run_concurrent(run: i32) {
    info!("run_concurrent: run={}", run);

    let seed = 1_000_000 + run * 100;
    let mut rnd = Random::new(seed as u32);

    const N: i32 = 1000;
    const K_SIZE: i32 = 1000;

    for i in 0..N {
        if i % 100 == 0 {
            info!("run_concurrent: run {} progress {}/{}", run, i, N);
        }

        let state = Arc::new(TestState::new((rnd.next() as i32) + 1));

        let reader_state = state.clone();
        let handle = std::thread::spawn(move || {
            concurrent_reader(reader_state);
        });

        state.wait_for_state(ReaderState::Running);

        for _ in 0..K_SIZE {
            let mut guard = state.t.lock();
            guard.write_step(&mut rnd);
        }

        state
            .quit_flag
            .store(true, atomic::Ordering::Release);
        state.wait_for_state(ReaderState::Done);

        handle.join().expect("reader thread panicked");
    }
}

/// Single-threaded shakeout of the concurrent test harness.
#[traced_test]
fn skiplist_concurrent_without_threads() {
    info!("skiplist_concurrent_without_threads: start");
    let mut test = ConcurrentTest::default();
    let mut rnd = Random::new(12345);

    for _ in 0..10_000 {
        test.read_step(&mut rnd);
        test.write_step(&mut rnd);
    }

    info!("skiplist_concurrent_without_threads: done");
}
