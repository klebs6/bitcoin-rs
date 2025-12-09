// ---------------- [ File: bitcoinleveldb-versionset/tests/add_boundary_inputs.rs ]
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_set_test.cc]
struct AddBoundaryInputsTest {
    level_files:      Vec<*mut FileMetaData>,
    compaction_files: Vec<*mut FileMetaData>,
    all_files:        Vec<*mut FileMetaData>,
    icmp:             InternalKeyComparator,
}

impl Default for AddBoundaryInputsTest {
    
    fn default() -> Self {
        todo!();
        /*
        : icmp(BytewiseComparator()),

        
        */
    }
}

impl Drop for AddBoundaryInputsTest {
    fn drop(&mut self) {
        todo!();
        /*
            for (size_t i = 0; i < all_files_.size(); ++i) {
          delete all_files_[i];
        }
        all_files_.clear();
        */
    }
}

impl AddBoundaryInputsTest {

    pub fn create_file_meta_data(&mut self, 
        number:   u64,
        smallest: InternalKey,
        largest:  InternalKey) -> *mut FileMetaData {
        
        todo!();
        /*
            FileMetaData* f = new FileMetaData();
        f->number = number;
        f->smallest = smallest;
        f->largest = largest;
        all_files_.push_back(f);
        return f;
        */
    }
}

#[test] fn add_boundary_inputs_test_empty_file_sets() {
    todo!();
    /*
    
      AddBoundaryInputs(icmp_, level_files_, &compaction_files_);
      ASSERT_TRUE(compaction_files_.empty());
      ASSERT_TRUE(level_files_.empty());

    */
}

#[test] fn add_boundary_inputs_test_empty_level_files() {
    todo!();
    /*
    
      FileMetaData* f1 =
          CreateFileMetaData(1, InternalKey("100", 2, kTypeValue),
                             InternalKey(InternalKey("100", 1, kTypeValue)));
      compaction_files_.push_back(f1);

      AddBoundaryInputs(icmp_, level_files_, &compaction_files_);
      ASSERT_EQ(1, compaction_files_.size());
      ASSERT_EQ(f1, compaction_files_[0]);
      ASSERT_TRUE(level_files_.empty());

    */
}

#[test] fn add_boundary_inputs_test_empty_compaction_files() {
    todo!();
    /*
    
      FileMetaData* f1 =
          CreateFileMetaData(1, InternalKey("100", 2, kTypeValue),
                             InternalKey(InternalKey("100", 1, kTypeValue)));
      level_files_.push_back(f1);

      AddBoundaryInputs(icmp_, level_files_, &compaction_files_);
      ASSERT_TRUE(compaction_files_.empty());
      ASSERT_EQ(1, level_files_.size());
      ASSERT_EQ(f1, level_files_[0]);

    */
}

#[test] fn add_boundary_inputs_test_no_files() {
    todo!();
    /*
    
      FileMetaData* f1 =
          CreateFileMetaData(1, InternalKey("100", 2, kTypeValue),
                             InternalKey(InternalKey("100", 1, kTypeValue)));
      FileMetaData* f2 =
          CreateFileMetaData(1, InternalKey("200", 2, kTypeValue),
                             InternalKey(InternalKey("200", 1, kTypeValue)));
      FileMetaData* f3 =
          CreateFileMetaData(1, InternalKey("300", 2, kTypeValue),
                             InternalKey(InternalKey("300", 1, kTypeValue)));

      level_files_.push_back(f3);
      level_files_.push_back(f2);
      level_files_.push_back(f1);
      compaction_files_.push_back(f2);
      compaction_files_.push_back(f3);

      AddBoundaryInputs(icmp_, level_files_, &compaction_files_);
      ASSERT_EQ(2, compaction_files_.size());

    */
}

#[test] fn add_boundary_inputs_test_one_files() {
    todo!();
    /*
    
      FileMetaData* f1 =
          CreateFileMetaData(1, InternalKey("100", 3, kTypeValue),
                             InternalKey(InternalKey("100", 2, kTypeValue)));
      FileMetaData* f2 =
          CreateFileMetaData(1, InternalKey("100", 1, kTypeValue),
                             InternalKey(InternalKey("200", 3, kTypeValue)));
      FileMetaData* f3 =
          CreateFileMetaData(1, InternalKey("300", 2, kTypeValue),
                             InternalKey(InternalKey("300", 1, kTypeValue)));

      level_files_.push_back(f3);
      level_files_.push_back(f2);
      level_files_.push_back(f1);
      compaction_files_.push_back(f1);

      AddBoundaryInputs(icmp_, level_files_, &compaction_files_);
      ASSERT_EQ(2, compaction_files_.size());
      ASSERT_EQ(f1, compaction_files_[0]);
      ASSERT_EQ(f2, compaction_files_[1]);

    */
}

#[test] fn add_boundary_inputs_test_two_files() {
    todo!();
    /*
    
      FileMetaData* f1 =
          CreateFileMetaData(1, InternalKey("100", 6, kTypeValue),
                             InternalKey(InternalKey("100", 5, kTypeValue)));
      FileMetaData* f2 =
          CreateFileMetaData(1, InternalKey("100", 2, kTypeValue),
                             InternalKey(InternalKey("300", 1, kTypeValue)));
      FileMetaData* f3 =
          CreateFileMetaData(1, InternalKey("100", 4, kTypeValue),
                             InternalKey(InternalKey("100", 3, kTypeValue)));

      level_files_.push_back(f2);
      level_files_.push_back(f3);
      level_files_.push_back(f1);
      compaction_files_.push_back(f1);

      AddBoundaryInputs(icmp_, level_files_, &compaction_files_);
      ASSERT_EQ(3, compaction_files_.size());
      ASSERT_EQ(f1, compaction_files_[0]);
      ASSERT_EQ(f3, compaction_files_[1]);
      ASSERT_EQ(f2, compaction_files_[2]);

    */
}

#[test] fn add_boundary_inputs_test_disjoin_file_pointers() {
    todo!();
    /*
    
      FileMetaData* f1 =
          CreateFileMetaData(1, InternalKey("100", 6, kTypeValue),
                             InternalKey(InternalKey("100", 5, kTypeValue)));
      FileMetaData* f2 =
          CreateFileMetaData(1, InternalKey("100", 6, kTypeValue),
                             InternalKey(InternalKey("100", 5, kTypeValue)));
      FileMetaData* f3 =
          CreateFileMetaData(1, InternalKey("100", 2, kTypeValue),
                             InternalKey(InternalKey("300", 1, kTypeValue)));
      FileMetaData* f4 =
          CreateFileMetaData(1, InternalKey("100", 4, kTypeValue),
                             InternalKey(InternalKey("100", 3, kTypeValue)));

      level_files_.push_back(f2);
      level_files_.push_back(f3);
      level_files_.push_back(f4);

      compaction_files_.push_back(f1);

      AddBoundaryInputs(icmp_, level_files_, &compaction_files_);
      ASSERT_EQ(3, compaction_files_.size());
      ASSERT_EQ(f1, compaction_files_[0]);
      ASSERT_EQ(f4, compaction_files_[1]);
      ASSERT_EQ(f3, compaction_files_[2]);

    */
}
