// ---------------- [ File: bitcoinleveldb-meta/src/version_set_test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_set_test.cc]

struct FindFileTest {
    disjoint_sorted_files: bool,
    files:                 Vec<*mut FileMetaData>,
}

impl Default for FindFileTest {
    
    fn default() -> Self {
        todo!();
        /*
        : disjoint_sorted_files(true),

        
        */
    }
}

impl Drop for FindFileTest {
    fn drop(&mut self) {
        todo!();
        /*
            for (int i = 0; i < files_.size(); i++) {
          delete files_[i];
        }
        */
    }
}

impl FindFileTest {

    pub fn add(&mut self, 
        smallest:     *const u8,
        largest:      *const u8,
        smallest_seq: Option<SequenceNumber>,
        largest_seq:  Option<SequenceNumber>)  {

        let smallest_seq: SequenceNumber = smallest_seq.unwrap_or(100);
        let largest_seq: SequenceNumber = largest_seq.unwrap_or(100);

        todo!();
        /*
            FileMetaData* f = new FileMetaData;
        f->number = files_.size() + 1;
        f->smallest = InternalKey(smallest, smallest_seq, kTypeValue);
        f->largest = InternalKey(largest, largest_seq, kTypeValue);
        files_.push_back(f);
        */
    }
    
    pub fn find(&mut self, key_: *const u8) -> i32 {
        
        todo!();
        /*
            InternalKey target(key, 100, kTypeValue);
        InternalKeyComparator cmp(BytewiseComparator());
        return FindFile(cmp, files_, target.Encode());
        */
    }
    
    pub fn overlaps(&mut self, 
        smallest: *const u8,
        largest:  *const u8) -> bool {
        
        todo!();
        /*
            InternalKeyComparator cmp(BytewiseComparator());
        Slice s(smallest != nullptr ? smallest : "");
        Slice l(largest != nullptr ? largest : "");
        return SomeFileOverlapsRange(cmp, disjoint_sorted_files_, files_,
                                     (smallest != nullptr ? &s : nullptr),
                                     (largest != nullptr ? &l : nullptr));
        */
    }
}

#[test] fn find_file_test_empty() {
    todo!();
    /*
    
      ASSERT_EQ(0, Find("foo"));
      ASSERT_TRUE(!Overlaps("a", "z"));
      ASSERT_TRUE(!Overlaps(nullptr, "z"));
      ASSERT_TRUE(!Overlaps("a", nullptr));
      ASSERT_TRUE(!Overlaps(nullptr, nullptr));

    */
}

#[test] fn find_file_test_single() {
    todo!();
    /*
    
      Add("p", "q");
      ASSERT_EQ(0, Find("a"));
      ASSERT_EQ(0, Find("p"));
      ASSERT_EQ(0, Find("p1"));
      ASSERT_EQ(0, Find("q"));
      ASSERT_EQ(1, Find("q1"));
      ASSERT_EQ(1, Find("z"));

      ASSERT_TRUE(!Overlaps("a", "b"));
      ASSERT_TRUE(!Overlaps("z1", "z2"));
      ASSERT_TRUE(Overlaps("a", "p"));
      ASSERT_TRUE(Overlaps("a", "q"));
      ASSERT_TRUE(Overlaps("a", "z"));
      ASSERT_TRUE(Overlaps("p", "p1"));
      ASSERT_TRUE(Overlaps("p", "q"));
      ASSERT_TRUE(Overlaps("p", "z"));
      ASSERT_TRUE(Overlaps("p1", "p2"));
      ASSERT_TRUE(Overlaps("p1", "z"));
      ASSERT_TRUE(Overlaps("q", "q"));
      ASSERT_TRUE(Overlaps("q", "q1"));

      ASSERT_TRUE(!Overlaps(nullptr, "j"));
      ASSERT_TRUE(!Overlaps("r", nullptr));
      ASSERT_TRUE(Overlaps(nullptr, "p"));
      ASSERT_TRUE(Overlaps(nullptr, "p1"));
      ASSERT_TRUE(Overlaps("q", nullptr));
      ASSERT_TRUE(Overlaps(nullptr, nullptr));

    */
}

#[test] fn find_file_test_multiple() {
    todo!();
    /*
    
      Add("150", "200");
      Add("200", "250");
      Add("300", "350");
      Add("400", "450");
      ASSERT_EQ(0, Find("100"));
      ASSERT_EQ(0, Find("150"));
      ASSERT_EQ(0, Find("151"));
      ASSERT_EQ(0, Find("199"));
      ASSERT_EQ(0, Find("200"));
      ASSERT_EQ(1, Find("201"));
      ASSERT_EQ(1, Find("249"));
      ASSERT_EQ(1, Find("250"));
      ASSERT_EQ(2, Find("251"));
      ASSERT_EQ(2, Find("299"));
      ASSERT_EQ(2, Find("300"));
      ASSERT_EQ(2, Find("349"));
      ASSERT_EQ(2, Find("350"));
      ASSERT_EQ(3, Find("351"));
      ASSERT_EQ(3, Find("400"));
      ASSERT_EQ(3, Find("450"));
      ASSERT_EQ(4, Find("451"));

      ASSERT_TRUE(!Overlaps("100", "149"));
      ASSERT_TRUE(!Overlaps("251", "299"));
      ASSERT_TRUE(!Overlaps("451", "500"));
      ASSERT_TRUE(!Overlaps("351", "399"));

      ASSERT_TRUE(Overlaps("100", "150"));
      ASSERT_TRUE(Overlaps("100", "200"));
      ASSERT_TRUE(Overlaps("100", "300"));
      ASSERT_TRUE(Overlaps("100", "400"));
      ASSERT_TRUE(Overlaps("100", "500"));
      ASSERT_TRUE(Overlaps("375", "400"));
      ASSERT_TRUE(Overlaps("450", "450"));
      ASSERT_TRUE(Overlaps("450", "500"));

    */
}

#[test] fn find_file_test_multiple_null_boundaries() {
    todo!();
    /*
    
      Add("150", "200");
      Add("200", "250");
      Add("300", "350");
      Add("400", "450");
      ASSERT_TRUE(!Overlaps(nullptr, "149"));
      ASSERT_TRUE(!Overlaps("451", nullptr));
      ASSERT_TRUE(Overlaps(nullptr, nullptr));
      ASSERT_TRUE(Overlaps(nullptr, "150"));
      ASSERT_TRUE(Overlaps(nullptr, "199"));
      ASSERT_TRUE(Overlaps(nullptr, "200"));
      ASSERT_TRUE(Overlaps(nullptr, "201"));
      ASSERT_TRUE(Overlaps(nullptr, "400"));
      ASSERT_TRUE(Overlaps(nullptr, "800"));
      ASSERT_TRUE(Overlaps("100", nullptr));
      ASSERT_TRUE(Overlaps("200", nullptr));
      ASSERT_TRUE(Overlaps("449", nullptr));
      ASSERT_TRUE(Overlaps("450", nullptr));

    */
}

#[test] fn find_file_test_overlap_sequence_checks() {
    todo!();
    /*
    
      Add("200", "200", 5000, 3000);
      ASSERT_TRUE(!Overlaps("199", "199"));
      ASSERT_TRUE(!Overlaps("201", "300"));
      ASSERT_TRUE(Overlaps("200", "200"));
      ASSERT_TRUE(Overlaps("190", "200"));
      ASSERT_TRUE(Overlaps("200", "210"));

    */
}

#[test] fn find_file_test_overlapping_files() {
    todo!();
    /*
    
      Add("150", "600");
      Add("400", "500");
      disjoint_sorted_files_ = false;
      ASSERT_TRUE(!Overlaps("100", "149"));
      ASSERT_TRUE(!Overlaps("601", "700"));
      ASSERT_TRUE(Overlaps("100", "150"));
      ASSERT_TRUE(Overlaps("100", "200"));
      ASSERT_TRUE(Overlaps("100", "300"));
      ASSERT_TRUE(Overlaps("100", "400"));
      ASSERT_TRUE(Overlaps("100", "500"));
      ASSERT_TRUE(Overlaps("375", "400"));
      ASSERT_TRUE(Overlaps("450", "450"));
      ASSERT_TRUE(Overlaps("450", "500"));
      ASSERT_TRUE(Overlaps("450", "700"));
      ASSERT_TRUE(Overlaps("600", "700"));

    */
}

fn add_boundary_inputs(
        icmp:             &InternalKeyComparator,
        level_files:      &Vec<*mut FileMetaData>,
        compaction_files: *mut Vec<*mut FileMetaData>)  {
    
    todo!();
        /*
        
        */
}

///------------------------
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

fn dbversion_set_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
