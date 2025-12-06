crate::ix!();

/* ------------- Multi-threaded test:  ------------- */

pub const NUM_THREADS:  usize = 4;
pub const TEST_SECONDS: usize = 10;
pub const NUM_KEYS:     usize = 1000;

pub struct MTState {
    test:        *mut DBTest,
    stop:        AtomicBool,
    counter:     [AtomicI32;  NUM_THREADS],
    thread_done: [AtomicBool; NUM_THREADS],
}

pub struct MTThread {
    state: *mut MTState,
    id:    i32,
}

pub fn mt_thread_body(arg: *mut c_void)  {
    
    todo!();
        /*
            MTThread* t = reinterpret_cast<MTThread*>(arg);
      int id = t->id;
      DB* db = t->state->test->db_;
      int counter = 0;
      fprintf(stderr, "... starting thread %d\n", id);
      Random rnd(1000 + id);
      std::string value;
      char valbuf[1500];
      while (!t->state->stop.load(std::memory_order_acquire)) {
        t->state->counter[id].store(counter, std::memory_order_release);

        int key = rnd.Uniform(kNumKeys);
        char keybuf[20];
        snprintf(keybuf, sizeof(keybuf), "%016d", key);

        if (rnd.OneIn(2)) {
          // Write values of the form <key, my id, counter>.
          // We add some padding for force compactions.
          snprintf(valbuf, sizeof(valbuf), "%d.%d.%-1000d", key, id,
                   static_cast<int>(counter));
          ASSERT_OK(db->Put(WriteOptions(), Slice(keybuf), Slice(valbuf)));
        } else {
          // Read a value and verify that it matches the pattern written above.
          Status s = db->Get(ReadOptions(), Slice(keybuf), &value);
          if (s.IsNotFound()) {
            // Key has not yet been written
          } else {
            // Check that the writer thread counter is >= the counter in the value
            ASSERT_OK(s);
            int k, w, c;
            ASSERT_EQ(3, sscanf(value.c_str(), "%d.%d.%d", &k, &w, &c)) << value;
            ASSERT_EQ(k, key);
            ASSERT_GE(w, 0);
            ASSERT_LT(w, kNumThreads);
            ASSERT_LE(c, t->state->counter[w].load(std::memory_order_acquire));
          }
        }
        counter++;
      }
      t->state->thread_done[id].store(true, std::memory_order_release);
      fprintf(stderr, "... stopping thread %d after %d ops\n", id, counter);
        */
}

#[test] fn db_test_multi_threaded() {
    todo!();
    /*
    
      do {
        // Initialize state
        MTState mt;
        mt.test = this;
        mt.stop.store(false, std::memory_order_release);
        for (int id = 0; id < kNumThreads; id++) {
          mt.counter[id].store(false, std::memory_order_release);
          mt.thread_done[id].store(false, std::memory_order_release);
        }

        // Start threads
        MTThread thread[kNumThreads];
        for (int id = 0; id < kNumThreads; id++) {
          thread[id].state = &mt;
          thread[id].id = id;
          env_->StartThread(MTThreadBody, &thread[id]);
        }

        // Let them run for a while
        DelayMilliseconds(kTestSeconds * 1000);

        // Stop the threads and wait for them to finish
        mt.stop.store(true, std::memory_order_release);
        for (int id = 0; id < kNumThreads; id++) {
          while (!mt.thread_done[id].load(std::memory_order_acquire)) {
            DelayMilliseconds(100);
          }
        }
      } while (ChangeOptions());

    */
}
