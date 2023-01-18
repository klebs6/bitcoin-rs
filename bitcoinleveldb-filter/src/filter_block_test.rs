crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/table/filter_block_test.cc]

/**
  | For testing: emit an array with one hash
  | value per key
  |
  */
struct TestHashFilter {

}

impl FilterPolicy for TestHashFilter {

}

impl Name for TestHashFilter {

    fn name(&self) -> *const u8 {
        
        todo!();
        /*
            return "TestHashFilter";
        */
    }
}
    
impl CreateFilter for TestHashFilter {

    fn create_filter(&self, 
        keys: *const Slice,
        n:    i32,
        dst:  *mut String)  {
        
        todo!();
        /*
            for (int i = 0; i < n; i++) {
          uint32_t h = Hash(keys[i].data(), keys[i].size(), 1);
          PutFixed32(dst, h);
        }
        */
    }
}

impl KeyMayMatch for TestHashFilter {

    fn key_may_match(&self, 
        key_:    &Slice,
        filter: &Slice) -> bool {
        
        todo!();
        /*
            uint32_t h = Hash(key.data(), key.size(), 1);
        for (size_t i = 0; i + 4 <= filter.size(); i += 4) {
          if (h == DecodeFixed32(filter.data() + i)) {
            return true;
          }
        }
        return false;
        */
    }
}

struct FilterBlockTest {
    policy: TestHashFilter,
}

#[test] fn filter_block_test_empty_builder() {
    todo!();
    /*
    
      FilterBlockBuilder builder(&policy_);
      Slice block = builder.Finish();
      ASSERT_EQ("\\x00\\x00\\x00\\x00\\x0b", EscapeString(block));
      FilterBlockReader reader(&policy_, block);
      ASSERT_TRUE(reader.KeyMayMatch(0, "foo"));
      ASSERT_TRUE(reader.KeyMayMatch(100000, "foo"));

    */
}

#[test] fn filter_block_test_single_chunk() {
    todo!();
    /*
    
      FilterBlockBuilder builder(&policy_);
      builder.StartBlock(100);
      builder.AddKey("foo");
      builder.AddKey("bar");
      builder.AddKey("box");
      builder.StartBlock(200);
      builder.AddKey("box");
      builder.StartBlock(300);
      builder.AddKey("hello");
      Slice block = builder.Finish();
      FilterBlockReader reader(&policy_, block);
      ASSERT_TRUE(reader.KeyMayMatch(100, "foo"));
      ASSERT_TRUE(reader.KeyMayMatch(100, "bar"));
      ASSERT_TRUE(reader.KeyMayMatch(100, "box"));
      ASSERT_TRUE(reader.KeyMayMatch(100, "hello"));
      ASSERT_TRUE(reader.KeyMayMatch(100, "foo"));
      ASSERT_TRUE(!reader.KeyMayMatch(100, "missing"));
      ASSERT_TRUE(!reader.KeyMayMatch(100, "other"));

    */
}

#[test] fn filter_block_test_multi_chunk() {
    todo!();
    /*
    
      FilterBlockBuilder builder(&policy_);

      // First filter
      builder.StartBlock(0);
      builder.AddKey("foo");
      builder.StartBlock(2000);
      builder.AddKey("bar");

      // Second filter
      builder.StartBlock(3100);
      builder.AddKey("box");

      // Third filter is empty

      // Last filter
      builder.StartBlock(9000);
      builder.AddKey("box");
      builder.AddKey("hello");

      Slice block = builder.Finish();
      FilterBlockReader reader(&policy_, block);

      // Check first filter
      ASSERT_TRUE(reader.KeyMayMatch(0, "foo"));
      ASSERT_TRUE(reader.KeyMayMatch(2000, "bar"));
      ASSERT_TRUE(!reader.KeyMayMatch(0, "box"));
      ASSERT_TRUE(!reader.KeyMayMatch(0, "hello"));

      // Check second filter
      ASSERT_TRUE(reader.KeyMayMatch(3100, "box"));
      ASSERT_TRUE(!reader.KeyMayMatch(3100, "foo"));
      ASSERT_TRUE(!reader.KeyMayMatch(3100, "bar"));
      ASSERT_TRUE(!reader.KeyMayMatch(3100, "hello"));

      // Check third filter (empty)
      ASSERT_TRUE(!reader.KeyMayMatch(4100, "foo"));
      ASSERT_TRUE(!reader.KeyMayMatch(4100, "bar"));
      ASSERT_TRUE(!reader.KeyMayMatch(4100, "box"));
      ASSERT_TRUE(!reader.KeyMayMatch(4100, "hello"));

      // Check last filter
      ASSERT_TRUE(reader.KeyMayMatch(9000, "box"));
      ASSERT_TRUE(reader.KeyMayMatch(9000, "hello"));
      ASSERT_TRUE(!reader.KeyMayMatch(9000, "foo"));
      ASSERT_TRUE(!reader.KeyMayMatch(9000, "bar"));

    */
}

fn tablefilter_block_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
