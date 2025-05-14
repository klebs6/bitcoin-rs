// ---------------- [ File: bitcoinleveldb-bloom/src/bloom_test.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/bloom_test.cc]

const VERBOSE: i32 = 1;

fn key(
        i:      i32,
        buffer: *mut u8) -> Slice {
    
    todo!();
        /*
            EncodeFixed32(buffer, i);
      return Slice(buffer, sizeof(uint32_t));
        */
}

///-------------------
struct BloomTest {
    policy: Box<dyn FilterPolicy>,
    filter: String,
    keys:   Vec<String>,
}

impl Default for BloomTest {
    
    fn default() -> Self {
        todo!();
        /*
        : policy(NewBloomFilterPolicy(10)),

        
        */
    }
}

impl Drop for BloomTest {
    fn drop(&mut self) {
        todo!();
        /*
            delete policy_;
        */
    }
}

impl BloomTest {
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            keys_.clear();
        filter_.clear();
        */
    }
    
    pub fn add(&mut self, s: &Slice)  {
        
        todo!();
        /*
            keys_.push_back(s.ToString());
        */
    }
    
    pub fn build(&mut self)  {
        
        todo!();
        /*
            std::vector<Slice> key_slices;
        for (size_t i = 0; i < keys_.size(); i++) {
          key_slices.push_back(Slice(keys_[i]));
        }
        filter_.clear();
        policy_->CreateFilter(&key_slices[0], static_cast<int>(key_slices.size()),
                              &filter_);
        keys_.clear();
        if (kVerbose >= 2) DumpFilter();
        */
    }
    
    pub fn filter_size(&self) -> usize {
        
        todo!();
        /*
            return filter_.size();
        */
    }
    
    pub fn dump_filter(&mut self)  {
        
        todo!();
        /*
            fprintf(stderr, "F(");
        for (size_t i = 0; i + 1 < filter_.size(); i++) {
          const unsigned int c = static_cast<unsigned int>(filter_[i]);
          for (int j = 0; j < 8; j++) {
            fprintf(stderr, "%c", (c & (1 << j)) ? '1' : '.');
          }
        }
        fprintf(stderr, ")\n");
        */
    }
    
    pub fn matches(&mut self, s: &Slice) -> bool {
        
        todo!();
        /*
            if (!keys_.empty()) {
          Build();
        }
        return policy_->KeyMayMatch(s, filter_);
        */
    }
    
    pub fn false_positive_rate(&mut self) -> f64 {
        
        todo!();
        /*
            char buffer[sizeof(int)];
        int result = 0;
        for (int i = 0; i < 10000; i++) {
          if (Matches(Key(i + 1000000000, buffer))) {
            result++;
          }
        }
        return result / 10000.0;
        */
    }
}

#[test] fn bloom_test_empty_filter() {
    todo!();
    /*
    
      ASSERT_TRUE(!Matches("hello"));
      ASSERT_TRUE(!Matches("world"));

    */
}

#[test] fn bloom_test_small() {
    todo!();
    /*
    
      Add("hello");
      Add("world");
      ASSERT_TRUE(Matches("hello"));
      ASSERT_TRUE(Matches("world"));
      ASSERT_TRUE(!Matches("x"));
      ASSERT_TRUE(!Matches("foo"));

    */
}

fn next_length(length: i32) -> i32 {
    
    todo!();
        /*
            if (length < 10) {
        length += 1;
      } else if (length < 100) {
        length += 10;
      } else if (length < 1000) {
        length += 100;
      } else {
        length += 1000;
      }
      return length;
        */
}

#[test] fn bloom_test_varying_lengths() {
    todo!();
    /*
    
      char buffer[sizeof(int)];

      // Count number of filters that significantly exceed the false positive rate
      int mediocre_filters = 0;
      int good_filters = 0;

      for (int length = 1; length <= 10000; length = NextLength(length)) {
        Reset();
        for (int i = 0; i < length; i++) {
          Add(Key(i, buffer));
        }
        Build();

        ASSERT_LE(FilterSize(), static_cast<size_t>((length * 10 / 8) + 40))
            << length;

        // All added keys must match
        for (int i = 0; i < length; i++) {
          ASSERT_TRUE(Matches(Key(i, buffer)))
              << "Length " << length << "; key " << i;
        }

        // Check false positive rate
        double rate = FalsePositiveRate();
        if (kVerbose >= 1) {
          fprintf(stderr, "False positives: %5.2f%% @ length = %6d ; bytes = %6d\n",
                  rate * 100.0, length, static_cast<int>(FilterSize()));
        }
        ASSERT_LE(rate, 0.02);  // Must not be over 2%
        if (rate > 0.0125)
          mediocre_filters++;  // Allowed, but not too often
        else
          good_filters++;
      }
      if (kVerbose >= 1) {
        fprintf(stderr, "Filters: %d good, %d mediocre\n", good_filters,
                mediocre_filters);
      }
      ASSERT_LE(mediocre_filters, good_filters / 5);

    */
}

/**
  | Different bits-per-byte
  |
  */
fn testbloom_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
