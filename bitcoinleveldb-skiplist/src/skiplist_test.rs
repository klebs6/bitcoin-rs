// ---------------- [ File: bitcoinleveldb-skiplist/src/skiplist_test.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/skiplist_test.cc]

type Key = u64;

struct TComparator { }

impl TComparator {

    pub fn invoke(&self, a: &Key, b: &Key) -> i32 {
        
        todo!();
        /*
            if (a < b) {
          return -1;
        } else if (a > b) {
          return +1;
        } else {
          return 0;
        }
        */
    }
}

struct SkipTest {}

#[test] fn skip_test_empty() {
    todo!();
    /*
    
      Arena arena;
      Comparator cmp;
      SkipList<Key, Comparator> list(cmp, &arena);
      ASSERT_TRUE(!list.Contains(10));

      SkipList<Key, Comparator>::Iterator iter(&list);
      ASSERT_TRUE(!iter.Valid());
      iter.SeekToFirst();
      ASSERT_TRUE(!iter.Valid());
      iter.Seek(100);
      ASSERT_TRUE(!iter.Valid());
      iter.SeekToLast();
      ASSERT_TRUE(!iter.Valid());

    */
}

#[test] fn skip_test_insert_and_lookup() {
    todo!();
    /*
    
      const int N = 2000;
      const int R = 5000;
      Random rnd(1000);
      std::set<Key> keys;
      Arena arena;
      Comparator cmp;
      SkipList<Key, Comparator> list(cmp, &arena);
      for (int i = 0; i < N; i++) {
        Key key = rnd.Next() % R;
        if (keys.insert(key).second) {
          list.Insert(key);
        }
      }

      for (int i = 0; i < R; i++) {
        if (list.Contains(i)) {
          ASSERT_EQ(keys.count(i), 1);
        } else {
          ASSERT_EQ(keys.count(i), 0);
        }
      }

      // Simple iterator tests
      {
        SkipList<Key, Comparator>::Iterator iter(&list);
        ASSERT_TRUE(!iter.Valid());

        iter.Seek(0);
        ASSERT_TRUE(iter.Valid());
        ASSERT_EQ(*(keys.begin()), iter.key());

        iter.SeekToFirst();
        ASSERT_TRUE(iter.Valid());
        ASSERT_EQ(*(keys.begin()), iter.key());

        iter.SeekToLast();
        ASSERT_TRUE(iter.Valid());
        ASSERT_EQ(*(keys.rbegin()), iter.key());
      }

      // Forward iteration test
      for (int i = 0; i < R; i++) {
        SkipList<Key, Comparator>::Iterator iter(&list);
        iter.Seek(i);

        // Compare against model iterator
        std::set<Key>::iterator model_iter = keys.lower_bound(i);
        for (int j = 0; j < 3; j++) {
          if (model_iter == keys.end()) {
            ASSERT_TRUE(!iter.Valid());
            break;
          } else {
            ASSERT_TRUE(iter.Valid());
            ASSERT_EQ(*model_iter, iter.key());
            ++model_iter;
            iter.Next();
          }
        }
      }

      // Backward iteration test
      {
        SkipList<Key, Comparator>::Iterator iter(&list);
        iter.SeekToLast();

        // Compare against model iterator
        for (std::set<Key>::reverse_iterator model_iter = keys.rbegin();
             model_iter != keys.rend(); ++model_iter) {
          ASSERT_TRUE(iter.Valid());
          ASSERT_EQ(*model_iter, iter.key());
          iter.Prev();
        }
        ASSERT_TRUE(!iter.Valid());
      }

    */
}

/**
  | We want to make sure that with a single writer
  | and multiple concurrent readers (with no
  | synchronization other than when a reader's
  | iterator is created), the reader always
  | observes all the data that was present in the
  | skip list when the iterator was constructed.
  | Because insertions are happening concurrently,
  | we may also observe new values that were
  | inserted since the iterator was constructed,
  | but we should never miss any values that were
  | present at iterator construction time.
  |
  | We generate multi-part keys:
  |     <key,gen,hash>
  | where:
  |     key is in range [0..K-1]
  |     gen is a generation number for key
  |     hash is hash(key,gen)
  |
  | The insertion code picks a random key, sets gen
  | to be 1 + the last generation number inserted
  | for that key, and sets hash to Hash(key,gen).
  |
  | At the beginning of a read, we snapshot the
  | last inserted generation number for each key.
  | We then iterate, including random calls to
  | Next() and Seek().  For every key we encounter,
  | we check that it is either expected given the
  | initial snapshot or has been concurrently added
  | since the iterator started.
  */
struct ConcurrentTest {

    /**
       Current state of the test
      */
    current: concurrent_test::State,

    arena:   Arena,

    /**
      | SkipList is not protected by mu_. We
      | just use a single writer thread to modify
      | it.
      |
      */
    list:    SkipList<TComparator>,
}

mod concurrent_test {
    use super::*;

    pub const K: usize = 4;

    /**
      | Per-key generation
      |
      */
    pub struct State {
        generation: [AtomicI32; K],
    }

    impl Default for State {
        
        fn default() -> Self {
            todo!();
            /*


                for (int k = 0; k < K; k++) {
                Set(k, 0);
              }
            */
        }
    }

    impl State {
        
        pub fn set(&mut self, k: i32, v: i32)  {
            
            todo!();
            /*
                generation[k].store(v, std::memory_order_release);
            */
        }
        
        pub fn get(&mut self, k: i32) -> i32 {
            
            todo!();
            /*
                return generation[k].load(std::memory_order_acquire);
            */
        }
    }
}

impl Default for ConcurrentTest {
    
    fn default() -> Self {
        todo!();
        /*


            : list_(Comparator(), &arena_)
        */
    }
}

impl ConcurrentTest {

    pub fn key(key_: Key) -> u64 {
        
        todo!();
        /*
            return (key >> 40);
        */
    }
    
    pub fn gen(key_: Key) -> u64 {
        
        todo!();
        /*
            return (key >> 8) & 0xffffffffu;
        */
    }
    
    pub fn hash(key_: Key) -> u64 {
        
        todo!();
        /*
            return key & 0xff;
        */
    }
    
    pub fn hash_numbers(k: u64, g: u64) -> u64 {
        
        todo!();
        /*
            uint64_t data[2] = {k, g};
        return Hash(reinterpret_cast<char*>(data), sizeof(data), 0);
        */
    }
    
    pub fn make_key(k: u64, g: u64) -> Key {
        
        todo!();
        /*
            const_assert(sizeof(Key) == sizeof(uint64_t), "");
        assert(k <= K);  // We sometimes pass K to seek to the end of the skiplist
        assert(g <= 0xffffffffu);
        return ((k << 40) | (g << 8) | (HashNumbers(k, g) & 0xff));
        */
    }
    
    pub fn is_valid_key(k: Key) -> bool {
        
        todo!();
        /*
            return hash(k) == (HashNumbers(key(k), gen(k)) & 0xff);
        */
    }
    
    pub fn random_target(rnd: *mut Random) -> Key {
        
        todo!();
        /*
            switch (rnd->Next() % 10) {
          case 0:
            // Seek to beginning
            return MakeKey(0, 0);
          case 1:
            // Seek to end
            return MakeKey(K, 0);
          default:
            // Seek to middle
            return MakeKey(rnd->Next() % K, 0);
        }
        */
    }

    /**
      | REQUIRES: External synchronization
      |
      */
    pub fn write_step(&mut self, rnd: *mut Random)  {
        
        todo!();
        /*
            const uint32_t k = rnd->Next() % K;
        const intptr_t g = current_.Get(k) + 1;
        const Key key = MakeKey(k, g);
        list_.Insert(key);
        current_.Set(k, g);
        */
    }
    
    pub fn read_step(&mut self, rnd: *mut Random)  {
        
        todo!();
        /*
            // Remember the initial committed state of the skiplist.
        State initial_state;
        for (int k = 0; k < K; k++) {
          initial_state.Set(k, current_.Get(k));
        }

        Key pos = RandomTarget(rnd);
        SkipList<Key, Comparator>::Iterator iter(&list_);
        iter.Seek(pos);
        while (true) {
          Key current;
          if (!iter.Valid()) {
            current = MakeKey(K, 0);
          } else {
            current = iter.key();
            ASSERT_TRUE(IsValidKey(current)) << current;
          }
          ASSERT_LE(pos, current) << "should not go backwards";

          // Verify that everything in [pos,current) was not present in
          // initial_state.
          while (pos < current) {
            ASSERT_LT(key(pos), K) << pos;

            // Note that generation 0 is never inserted, so it is ok if
            // <*,0,*> is missing.
            ASSERT_TRUE((gen(pos) == 0) ||
                        (gen(pos) > static_cast<Key>(initial_state.Get(key(pos)))))
                << "key_: " << key(pos) << "; gen: " << gen(pos)
                << "; initgen: " << initial_state.Get(key(pos));

            // Advance to next key in the valid key space
            if (key(pos) < key(current)) {
              pos = MakeKey(key(pos) + 1, 0);
            } else {
              pos = MakeKey(key(pos), gen(pos) + 1);
            }
          }

          if (!iter.Valid()) {
            break;
          }

          if (rnd->Next() % 2) {
            iter.Next();
            pos = MakeKey(key(pos), gen(pos) + 1);
          } else {
            Key new_target = RandomTarget(rnd);
            if (new_target > pos) {
              pos = new_target;
              iter.Seek(new_target);
            }
          }
        }
        */
    }
}

/**
  | Simple test that does single-threaded
  | testing of the ConcurrentTest scaffolding.
  |
  */
#[test] fn skip_test_concurrent_without_threads() {
    todo!();
    /*
    
      ConcurrentTest test;
      Random rnd(test::RandomSeed());
      for (int i = 0; i < 10000; i++) {
        test.ReadStep(&rnd);
        test.WriteStep(&rnd);
      }

    */
}

struct TestState {
    t:         ConcurrentTest,
    seed:      i32,
    quit_flag: AtomicBool,
    mu:        Mutex<test_state::Inner>,
}

mod test_state {

    use super::*;

    pub struct Inner {
        state:    ReaderState,
        state_cv: Condvar,
    }

    pub enum ReaderState { 
        STARTING, 
        RUNNING, 
        DONE 
    }
}

impl TestState {
    
    pub fn new(s: i32) -> Self {
    
        todo!();
        /*
        : seed(s),
        : quit_flag(false),
        : state(STARTING),
        : state_cv(&mu_),

        
        */
    }

    #[LOCKS_EXCLUDED(mu_)]
    pub fn wait(&mut self, s: test_state::ReaderState)  {
        
        todo!();
        /*
            mu_.Lock();
        while (state_ != s) {
          state_cv_.Wait();
        }
        mu_.Unlock();
        */
    }

    #[LOCKS_EXCLUDED(mu_)]
    pub fn change(&mut self, s: test_state::ReaderState)  {
        
        todo!();
        /*
            mu_.Lock();
        state_ = s;
        state_cv_.Signal();
        mu_.Unlock();
        */
    }
}

fn concurrent_reader(arg: *mut c_void)  {
    
    todo!();
        /*
            TestState* state = reinterpret_cast<TestState*>(arg);
      Random rnd(state->seed_);
      int64_t reads = 0;
      state->Change(TestState::RUNNING);
      while (!state->quit_flag_.load(std::memory_order_acquire)) {
        state->t_.ReadStep(&rnd);
        ++reads;
      }
      state->Change(TestState::DONE);
        */
}

fn run_concurrent(run: i32)  {
    
    todo!();
        /*
            const int seed = test::RandomSeed() + (run * 100);
      Random rnd(seed);
      const int N = 1000;
      const int kSize = 1000;
      for (int i = 0; i < N; i++) {
        if ((i % 100) == 0) {
          fprintf(stderr, "Run %d of %d\n", i, N);
        }
        TestState state(seed + 1);
        Env::Default()->Schedule(ConcurrentReader, &state);
        state.Wait(TestState::RUNNING);
        for (int i = 0; i < kSize; i++) {
          state.t_.WriteStep(&rnd);
        }
        state.quit_flag_.store(true, std::memory_order_release);
        state.Wait(TestState::DONE);
      }
        */
}

#[test] fn skip_test_concurrent1() {
    todo!();
    /*
         RunConcurrent(1); 
    */
}

#[test] fn skip_test_concurrent2() {
    todo!();
    /*
         RunConcurrent(2); 
    */
}

#[test] fn skip_test_concurrent3() {
    todo!();
    /*
         RunConcurrent(3); 
    */
}

#[test] fn skip_test_concurrent4() {
    todo!();
    /*
         RunConcurrent(4); 
    */
}

#[test] fn skip_test_concurrent5() {
    todo!();
    /*
         RunConcurrent(5); 
    */
}

fn skiplist_test_main(
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
