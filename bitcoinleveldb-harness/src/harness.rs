// ---------------- [ File: bitcoinleveldb-harness/src/harness.rs ]
crate::ix!();

pub struct Harness {
    options:     Options,
    constructor: *mut Constructor,
}

impl Default for Harness {
    fn default() -> Self {
        Self {
            options:     Options::default(),
            constructor: std::ptr::null_mut(),
        }
    }
}

impl Drop for Harness {
    fn drop(&mut self) {
        /*
            delete constructor_;
        */
        const TAG_MASK: usize = 0b11;

        let tagged: usize = self.constructor as usize;
        let tag: usize = tagged & TAG_MASK;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        if !raw.is_null() {
            unsafe {
                match tag {
                    0 => {
                        drop(Box::from_raw(raw as *mut TableConstructor));
                    }
                    1 => {
                        drop(Box::from_raw(raw as *mut BlockConstructor));
                    }
                    2 => {
                        drop(Box::from_raw(raw as *mut MemTableConstructor));
                    }
                    3 => {
                        drop(Box::from_raw(raw as *mut DBConstructor));
                    }
                    _ => {
                        unreachable!();
                    }
                }
            }
        }

        self.constructor = std::ptr::null_mut();
    }
}

impl Harness {
    pub fn init(&mut self, args: &TestArgs) {
        /*
            delete constructor_;
        constructor_ = nullptr;
        options_ = Options();

        options_.block_restart_interval = args.restart_interval;
        // Use shorter block size for tests to exercise block boundary
        // conditions more.
        options_.block_size = 256;
        if (args.reverse_compare) {
          options_.comparator = &reverse_key_comparator;
        }
        switch (args.type) {
          case TABLE_TEST:
            constructor_ = new TableConstructor(options_.comparator);
            break;
          case BLOCK_TEST:
            constructor_ = new BlockConstructor(options_.comparator);
            break;
          case MEMTABLE_TEST:
            constructor_ = new MemTableConstructor(options_.comparator);
            break;
          case DB_TEST:
            constructor_ = new DBConstructor(options_.comparator);
            break;
        }
        */
        trace!(
            target: "bitcoinleveldb_harness",
            ty = ?args.ty,
            reverse_compare = args.reverse_compare,
            restart_interval = args.restart_interval,
            "Harness::init"
        );

        const TAG_MASK: usize = 0b11;

        // delete constructor_;
        {
            let tagged: usize = self.constructor as usize;
            let tag: usize = tagged & TAG_MASK;
            let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

            if !raw.is_null() {
                unsafe {
                    match tag {
                        0 => {
                            drop(Box::from_raw(raw as *mut TableConstructor));
                        }
                        1 => {
                            drop(Box::from_raw(raw as *mut BlockConstructor));
                        }
                        2 => {
                            drop(Box::from_raw(raw as *mut MemTableConstructor));
                        }
                        3 => {
                            drop(Box::from_raw(raw as *mut DBConstructor));
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                }
            }
        }

        // constructor_ = nullptr;
        self.constructor = std::ptr::null_mut();

        // options_ = Options();
        self.options = Options::default();

        // options_.block_restart_interval = args.restart_interval;
        self.options.set_block_restart_interval(args.restart_interval);

        // Use shorter block size for tests to exercise block boundary
        // conditions more.
        // options_.block_size = 256;
        self.options.set_block_size(256);

        // if (args.reverse_compare) { options_.comparator = &reverse_key_comparator; }
        if args.reverse_compare {
            self.options.set_comparator(reverse_key_comparator.clone());
        }

        // switch (args.type) { ... }
        match args.ty {
            TestType::TABLE_TEST => {
                let cmp_box: Box<dyn SliceComparator> = Box::new(
                    ArcSliceComparatorAdapter::new(self.options.comparator().clone()),
                );

                let ctor: Box<TableConstructor> = Box::new(TableConstructor::new(cmp_box));
                let raw_ctor: *mut TableConstructor = Box::into_raw(ctor);

                let base_ptr: *mut Constructor = raw_ctor as *mut Constructor;
                self.constructor = base_ptr;
            }

            TestType::BLOCK_TEST => {
                let cmp_box: Box<dyn SliceComparator> = Box::new(
                    ArcSliceComparatorAdapter::new(self.options.comparator().clone()),
                );

                let ctor: Box<BlockConstructor> = Box::new(BlockConstructor::new(cmp_box));
                let raw_ctor: *mut BlockConstructor = Box::into_raw(ctor);

                let base_ptr: *mut Constructor = raw_ctor as *mut Constructor;
                self.constructor = ((base_ptr as usize) | 1usize) as *mut Constructor;
            }

            TestType::MEMTABLE_TEST => {
                let cmp_box: Box<dyn SliceComparator> = Box::new(
                    ArcSliceComparatorAdapter::new(self.options.comparator().clone()),
                );

                let ctor: Box<MemTableConstructor> = Box::new(MemTableConstructor::new(cmp_box));
                let raw_ctor: *mut MemTableConstructor = Box::into_raw(ctor);

                let base_ptr: *mut Constructor = raw_ctor as *mut Constructor;
                self.constructor = ((base_ptr as usize) | 2usize) as *mut Constructor;
            }

            TestType::DB_TEST => {
                let cmp_box: Box<dyn SliceComparator> = Box::new(
                    ArcSliceComparatorAdapter::new(self.options.comparator().clone()),
                );

                let ctor: Box<DBConstructor> = Box::new(DBConstructor::new(cmp_box));
                let raw_ctor: *mut DBConstructor = Box::into_raw(ctor);

                let base_ptr: *mut Constructor = raw_ctor as *mut Constructor;
                self.constructor = ((base_ptr as usize) | 3usize) as *mut Constructor;
            }
        }
    }

    pub fn add(&mut self, key_: &String, value: &String) {
        /*
            constructor_->Add(key, value);
        */
        trace!(
            target: "bitcoinleveldb_harness",
            key_len = key_.len(),
            value_len = value.len(),
            "Harness::add"
        );

        const TAG_MASK: usize = 0b11;

        let tagged: usize = self.constructor as usize;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        unsafe {
            let v = Slice::from(value.as_slice());
            (&mut *raw).add(key_, &v);
        }
    }

    pub fn test(&mut self, rnd: *mut Random) {
        /*
            std::vector<std::string> keys;
        KVMap data;
        constructor_->Finish(options_, &keys, &data);

        TestForwardScan(keys, data);
        TestBackwardScan(keys, data);
        TestRandomAccess(rnd, keys, data);
        */
        trace!(target: "bitcoinleveldb_harness", "Harness::test");

        const TAG_MASK: usize = 0b11;

        let tagged: usize = self.constructor as usize;
        let tag: usize = tagged & TAG_MASK;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        let mut keys: Vec<String> = Vec::new();
        let mut data: KVMap = KVMap::default();

        unsafe {
            (&mut *raw).finish(&self.options, &mut keys as *mut Vec<String>, &mut data as *mut KVMap);

            match tag {
                0 => {
                    let s = (&mut *(raw as *mut TableConstructor)).finish_impl(&self.options, &data);
                    assert!(s.ok());
                }
                1 => {
                    let s = (&mut *(raw as *mut BlockConstructor)).finish_impl(&self.options, &data);
                    assert!(s.ok());
                }
                2 => {
                    let s = (&mut *(raw as *mut MemTableConstructor)).finish_impl(&self.options, &data);
                    assert!(s.ok());
                }
                3 => {
                    let s = (&mut *(raw as *mut DBConstructor)).finish_impl(&self.options, &data);
                    assert!(s.ok());
                }
                _ => {
                    unreachable!();
                }
            }
        }

        self.test_forward_scan(&keys, &data);
        self.test_backward_scan(&keys, &data);
        self.test_random_access(rnd, &keys, &data);
    }

    pub fn test_forward_scan(&mut self, keys: &Vec<String>, data: &KVMap) {
        /*
            Iterator* iter = constructor_->NewIterator();
        ASSERT_TRUE(!iter->Valid());
        iter->SeekToFirst();
        for (KVMap::const_iterator model_iter = data.begin();
             model_iter != data.end(); ++model_iter) {
          ASSERT_EQ(ToString(data, model_iter), ToString(iter));
          iter->Next();
        }
        ASSERT_TRUE(!iter->Valid());
        delete iter;
        */
        trace!(
            target: "bitcoinleveldb_harness",
            num_keys = keys.len(),
            "Harness::test_forward_scan"
        );

        const TAG_MASK: usize = 0b11;

        let tagged: usize = self.constructor as usize;
        let tag: usize = tagged & TAG_MASK;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        let iter: *mut LevelDBIterator = unsafe {
            match tag {
                0 => (&*(raw as *mut TableConstructor)).new_iterator(),
                1 => (&*(raw as *mut BlockConstructor)).new_iterator(),
                2 => (&*(raw as *mut MemTableConstructor)).new_iterator(),
                3 => (&*(raw as *mut DBConstructor)).new_iterator(),
                _ => unreachable!(),
            }
        };

        unsafe {
            assert!(!(&*iter).valid());
            (&mut *iter).seek_to_first();

            for k in keys.iter() {
                let v = data.get(k).unwrap();
                assert_eq!(
                    self.to_string_with_data(data, Some((k, v))),
                    self.to_string(iter as *const LevelDBIterator)
                );
                (&mut *iter).next();
            }

            assert!(!(&*iter).valid());
            drop(Box::from_raw(iter));
        }
    }

    pub fn test_backward_scan(&mut self, keys: &Vec<String>, data: &KVMap) {
        /*
            Iterator* iter = constructor_->NewIterator();
        ASSERT_TRUE(!iter->Valid());
        iter->SeekToLast();
        for (KVMap::const_reverse_iterator model_iter = data.rbegin();
             model_iter != data.rend(); ++model_iter) {
          ASSERT_EQ(ToString(data, model_iter), ToString(iter));
          iter->Prev();
        }
        ASSERT_TRUE(!iter->Valid());
        delete iter;
        */
        trace!(
            target: "bitcoinleveldb_harness",
            num_keys = keys.len(),
            "Harness::test_backward_scan"
        );

        const TAG_MASK: usize = 0b11;

        let tagged: usize = self.constructor as usize;
        let tag: usize = tagged & TAG_MASK;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        let iter: *mut LevelDBIterator = unsafe {
            match tag {
                0 => (&*(raw as *mut TableConstructor)).new_iterator(),
                1 => (&*(raw as *mut BlockConstructor)).new_iterator(),
                2 => (&*(raw as *mut MemTableConstructor)).new_iterator(),
                3 => (&*(raw as *mut DBConstructor)).new_iterator(),
                _ => unreachable!(),
            }
        };

        unsafe {
            assert!(!(&*iter).valid());
            (&mut *iter).seek_to_last();

            for k in keys.iter().rev() {
                let v = data.get(k).unwrap();
                assert_eq!(
                    self.to_string_rev(data, Some((k, v))),
                    self.to_string(iter as *const LevelDBIterator)
                );
                (&mut *iter).prev();
            }

            assert!(!(&*iter).valid());
            drop(Box::from_raw(iter));
        }
    }

    pub fn test_random_access(&mut self, rnd: *mut Random, keys: &Vec<String>, data: &KVMap) {
        /*
            static const bool kVerbose = false;
        Iterator* iter = constructor_->NewIterator();
        ASSERT_TRUE(!iter->Valid());
        KVMap::const_iterator model_iter = data.begin();
        if (kVerbose) fprintf(stderr, "---\n");
        for (int i = 0; i < 200; i++) {
          const int toss = rnd->Uniform(5);
          switch (toss) {
            case 0: {
              if (iter->Valid()) {
                if (kVerbose) fprintf(stderr, "Next\n");
                iter->Next();
                ++model_iter;
                ASSERT_EQ(ToString(data, model_iter), ToString(iter));
              }
              break;
            }

            case 1: {
              if (kVerbose) fprintf(stderr, "SeekToFirst\n");
              iter->SeekToFirst();
              model_iter = data.begin();
              ASSERT_EQ(ToString(data, model_iter), ToString(iter));
              break;
            }

            case 2: {
              std::string key = PickRandomKey(rnd, keys);
              model_iter = data.lower_bound(key);
              if (kVerbose)
                fprintf(stderr, "Seek '%s'\n", EscapeString(key).c_str());
              iter->Seek(Slice(key));
              ASSERT_EQ(ToString(data, model_iter), ToString(iter));
              break;
            }

            case 3: {
              if (iter->Valid()) {
                if (kVerbose) fprintf(stderr, "Prev\n");
                iter->Prev();
                if (model_iter == data.begin()) {
                  model_iter = data.end();  // Wrap around to invalid value
                } else {
                  --model_iter;
                }
                ASSERT_EQ(ToString(data, model_iter), ToString(iter));
              }
              break;
            }

            case 4: {
              if (kVerbose) fprintf(stderr, "SeekToLast\n");
              iter->SeekToLast();
              if (keys.empty()) {
                model_iter = data.end();
              } else {
                std::string last = data.rbegin()->first;
                model_iter = data.lower_bound(last);
              }
              ASSERT_EQ(ToString(data, model_iter), ToString(iter));
              break;
            }
          }
        }
        delete iter;
        */
        trace!(
            target: "bitcoinleveldb_harness",
            num_keys = keys.len(),
            "Harness::test_random_access"
        );

        static K_VERBOSE: bool = false;

        const TAG_MASK: usize = 0b11;

        let tagged: usize = self.constructor as usize;
        let tag: usize = tagged & TAG_MASK;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        let iter: *mut LevelDBIterator = unsafe {
            match tag {
                0 => (&*(raw as *mut TableConstructor)).new_iterator(),
                1 => (&*(raw as *mut BlockConstructor)).new_iterator(),
                2 => (&*(raw as *mut MemTableConstructor)).new_iterator(),
                3 => (&*(raw as *mut DBConstructor)).new_iterator(),
                _ => unreachable!(),
            }
        };

        unsafe {
            assert!(!(&*iter).valid());
        }

        let mut model_index: usize = 0;

        if K_VERBOSE {
            // fprintf(stderr, "---\n");
            debug!(target: "bitcoinleveldb_harness", "---");
        }

        for _i in 0..200 {
            let toss: i32 = unsafe { (*rnd).uniform(5) as i32 };

            match toss {
                0 => {
                    if unsafe { (&*iter).valid() } {
                        if K_VERBOSE {
                            // fprintf(stderr, "Next\n");
                            debug!(target: "bitcoinleveldb_harness", "Next");
                        }
                        unsafe {
                            (&mut *iter).next();
                        }
                        model_index += 1;

                        let expected = if model_index == keys.len() {
                            self.to_string_with_data(data, None)
                        } else {
                            let k = &keys[model_index];
                            let v = data.get(k).unwrap();
                            self.to_string_with_data(data, Some((k, v)))
                        };

                        assert_eq!(expected, self.to_string(iter as *const LevelDBIterator));
                    }
                }

                1 => {
                    if K_VERBOSE {
                        // fprintf(stderr, "SeekToFirst\n");
                        debug!(target: "bitcoinleveldb_harness", "SeekToFirst");
                    }
                    unsafe {
                        (&mut *iter).seek_to_first();
                    }
                    model_index = 0;

                    let expected = if model_index == keys.len() {
                        self.to_string_with_data(data, None)
                    } else {
                        let k = &keys[model_index];
                        let v = data.get(k).unwrap();
                        self.to_string_with_data(data, Some((k, v)))
                    };

                    assert_eq!(expected, self.to_string(iter as *const LevelDBIterator));
                }

                2 => {
                    let key: String = self.pick_random_key(rnd, keys);

                    let mut left: usize = 0;
                    let mut right: usize = keys.len();
                    while left < right {
                        let mid: usize = left + ((right - left) / 2);

                        let a = Slice::from(keys[mid].as_slice());
                        let b = Slice::from(key.as_slice());

                        let c = self.options.comparator().compare(&a, &b);
                        if c < 0 {
                            left = mid + 1;
                        } else {
                            right = mid;
                        }
                    }
                    model_index = left;

                    if K_VERBOSE {
                        // fprintf(stderr, "Seek '%s'\n", EscapeString(key).c_str());
                        debug!(target: "bitcoinleveldb_harness", "Seek '{:?}'", key);
                    }

                    let target = Slice::from(key.as_slice());
                    unsafe {
                        (&mut *iter).seek(&target);
                    }

                    let expected = if model_index == keys.len() {
                        self.to_string_with_data(data, None)
                    } else {
                        let k = &keys[model_index];
                        let v = data.get(k).unwrap();
                        self.to_string_with_data(data, Some((k, v)))
                    };

                    assert_eq!(expected, self.to_string(iter as *const LevelDBIterator));
                }

                3 => {
                    if unsafe { (&*iter).valid() } {
                        if K_VERBOSE {
                            // fprintf(stderr, "Prev\n");
                            debug!(target: "bitcoinleveldb_harness", "Prev");
                        }
                        unsafe {
                            (&mut *iter).prev();
                        }
                        if model_index == 0 {
                            model_index = keys.len(); // Wrap around to invalid value
                        } else {
                            model_index -= 1;
                        }

                        let expected = if model_index == keys.len() {
                            self.to_string_with_data(data, None)
                        } else {
                            let k = &keys[model_index];
                            let v = data.get(k).unwrap();
                            self.to_string_with_data(data, Some((k, v)))
                        };

                        assert_eq!(expected, self.to_string(iter as *const LevelDBIterator));
                    }
                }

                4 => {
                    if K_VERBOSE {
                        // fprintf(stderr, "SeekToLast\n");
                        debug!(target: "bitcoinleveldb_harness", "SeekToLast");
                    }
                    unsafe {
                        (&mut *iter).seek_to_last();
                    }
                    if keys.is_empty() {
                        model_index = keys.len();
                    } else {
                        let last: &String = &keys[keys.len() - 1];

                        let mut left: usize = 0;
                        let mut right: usize = keys.len();
                        while left < right {
                            let mid: usize = left + ((right - left) / 2);

                            let a = Slice::from(keys[mid].as_slice());
                            let b = Slice::from(last.as_slice());

                            let c = self.options.comparator().compare(&a, &b);
                            if c < 0 {
                                left = mid + 1;
                            } else {
                                right = mid;
                            }
                        }
                        model_index = left;
                    }

                    let expected = if model_index == keys.len() {
                        self.to_string_with_data(data, None)
                    } else {
                        let k = &keys[model_index];
                        let v = data.get(k).unwrap();
                        self.to_string_with_data(data, Some((k, v)))
                    };

                    assert_eq!(expected, self.to_string(iter as *const LevelDBIterator));
                }

                _ => {
                    unreachable!();
                }
            }
        }

        unsafe {
            drop(Box::from_raw(iter));
        }
    }

    pub fn to_string_with_data<'a>(
        &mut self,
        _data: &KVMap,
        it: Option<(&'a String, &'a String)>,
    ) -> String {
        /*
            if (it == data.end()) {
          return "END";
        } else {
          return "'" + it->first + "->" + it->second + "'";
        }
        */
        match it {
            None => b"END".to_vec(),
            Some((k, v)) => {
                let mut out: String = Vec::with_capacity(1 + k.len() + 2 + v.len() + 1);
                out.push(b'\'');
                out.extend_from_slice(k.as_slice());
                out.extend_from_slice(b"->");
                out.extend_from_slice(v.as_slice());
                out.push(b'\'');
                out
            }
        }
    }

    pub fn to_string_rev<'a>(
        &mut self,
        data: &KVMap,
        it: Option<(&'a String, &'a String)>,
    ) -> String {
        /*
            if (it == data.rend()) {
          return "END";
        } else {
          return "'" + it->first + "->" + it->second + "'";
        }
        */
        self.to_string_with_data(data, it)
    }

    pub fn to_string(&mut self, it: *const LevelDBIterator) -> String {
        /*
            if (!it->Valid()) {
          return "END";
        } else {
          return "'" + it->key().ToString() + "->" + it->value().ToString() + "'";
        }
        */
        unsafe {
            if !(&*it).valid() {
                b"END".to_vec()
            } else {
                let k: String = (&*it).key().to_string();
                let v: String = (&*it).value().to_string();

                let mut out: String = Vec::with_capacity(1 + k.len() + 2 + v.len() + 1);
                out.push(b'\'');
                out.extend_from_slice(k.as_slice());
                out.extend_from_slice(b"->");
                out.extend_from_slice(v.as_slice());
                out.push(b'\'');
                out
            }
        }
    }

    pub fn pick_random_key(&mut self, rnd: *mut Random, keys: &Vec<String>) -> String {
        /*
            if (keys.empty()) {
          return "foo";
        } else {
          const int index = rnd->Uniform(keys.size());
          std::string result = keys[index];
          switch (rnd->Uniform(3)) {
            case 0:
              // Return an existing key
              break;
            case 1: {
              // Attempt to return something smaller than an existing key
              if (!result.empty() && result[result.size() - 1] > '\0') {
                result[result.size() - 1]--;
              }
              break;
            }
            case 2: {
              // Return something larger than an existing key
              Increment(options_.comparator, &result);
              break;
            }
          }
          return result;
        }
        */
        if keys.is_empty() {
            b"foo".to_vec()
        } else {
            let index: usize = unsafe { (*rnd).uniform(keys.len() as i32) as usize };
            let mut result: String = keys[index].clone();

            match unsafe { (*rnd).uniform(3) as i32 } {
                0 => {
                    // Return an existing key
                }

                1 => {
                    // Attempt to return something smaller than an existing key
                    if !result.is_empty() && result[result.len() - 1] > b'\0' {
                        let last = result.len() - 1;
                        result[last] = result[last].wrapping_sub(1);
                    }
                }

                2 => {
                    // Return something larger than an existing key
                    const BYTEWISE: &str = "leveldb.BytewiseComparator";
                    const REVERSE: &str = "leveldb.ReverseBytewiseComparator";

                    let name = self.options.comparator().name();

                    if name == BYTEWISE {
                        result.push(b'\0');
                    } else {
                        // This is the case used by the C++ test harness.
                        // (ReverseKeyComparator compares keys by comparing their reversed bytes.)
                        debug_assert_eq!(name, REVERSE);

                        let mut rev: String = result.iter().copied().rev().collect();
                        rev.push(b'\0');
                        result = rev.iter().copied().rev().collect();
                    }
                }

                _ => {
                    unreachable!();
                }
            }

            result
        }
    }

    /**
       Returns nullptr if not running against a DB
      */
    pub fn db(&self) -> *mut dyn DB {
        /*
            return constructor_->db();
        */
        const TAG_MASK: usize = 0b11;

        let tagged: usize = self.constructor as usize;
        let tag: usize = tagged & TAG_MASK;
        let raw: *mut Constructor = (tagged & !TAG_MASK) as *mut Constructor;

        if raw.is_null() {
            return std::ptr::null_mut();
        }

        unsafe {
            match tag {
                3 => (&*(raw as *mut DBConstructor)).db(),
                _ => std::ptr::null_mut(),
            }
        }
    }
}
