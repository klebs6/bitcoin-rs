// ---------------- [ File: bitcoinleveldb-key/src/dbformat_test.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/dbformat_test.cc]

fn key(
        user_key_: &String,
        seq:      u64,
        vt:       ValueType) -> String {
    
    todo!();
        /*
            std::string encoded;
      AppendInternalKey(&encoded, ParsedInternalKey(user_key, seq, vt));
      return encoded;
        */
}

fn shorten(
        s: &String,
        l: &String) -> String {
    
    todo!();
        /*
            std::string result = s;
      InternalKeyComparator(BytewiseComparator()).FindShortestSeparator(&result, l);
      return result;
        */
}

fn short_successor(s: &String) -> String {
    
    todo!();
        /*
            std::string result = s;
      InternalKeyComparator(BytewiseComparator()).FindShortSuccessor(&result);
      return result;
        */
}

fn test_key(
        key_: &String,
        seq: u64,
        vt:  ValueType)  {
    
    todo!();
        /*
            std::string encoded = IKey(key, seq, vt);

      Slice in(encoded);
      ParsedInternalKey decoded("", 0, kTypeValue);

      ASSERT_TRUE(ParseInternalKey(in, &decoded));
      ASSERT_EQ(key, decoded.user_key.ToString());
      ASSERT_EQ(seq, decoded.sequence);
      ASSERT_EQ(vt, decoded.type);

      ASSERT_TRUE(!ParseInternalKey(Slice("bar"), &decoded));
        */
}

struct FormatTest {}

#[test] fn format_test_internal_key_encode_decode() {
    todo!();
    /*
    
      const char* keys[] = {"", "k", "hello", "longggggggggggggggggggggg"};
      const uint64_t seq[] = {1,
                              2,
                              3,
                              (1ull << 8) - 1,
                              1ull << 8,
                              (1ull << 8) + 1,
                              (1ull << 16) - 1,
                              1ull << 16,
                              (1ull << 16) + 1,
                              (1ull << 32) - 1,
                              1ull << 32,
                              (1ull << 32) + 1};
      for (int k = 0; k < sizeof(keys) / sizeof(keys[0]); k++) {
        for (int s = 0; s < sizeof(seq) / sizeof(seq[0]); s++) {
          TestKey(keys[k], seq[s], kTypeValue);
          TestKey("hello", 1, kTypeDeletion);
        }
      }

    */
}

#[test] fn format_test_internal_key_decode_from_empty() {
    todo!();
    /*
    
      InternalKey internal_key;

      ASSERT_TRUE(!internal_key.DecodeFrom(""));

    */
}

#[test] fn format_test_internal_key_short_separator() {
    todo!();
    /*
    
      // When user keys are same
      ASSERT_EQ(IKey("foo", 100, kTypeValue),
                Shorten(IKey("foo", 100, kTypeValue), IKey("foo", 99, kTypeValue)));
      ASSERT_EQ(
          IKey("foo", 100, kTypeValue),
          Shorten(IKey("foo", 100, kTypeValue), IKey("foo", 101, kTypeValue)));
      ASSERT_EQ(
          IKey("foo", 100, kTypeValue),
          Shorten(IKey("foo", 100, kTypeValue), IKey("foo", 100, kTypeValue)));
      ASSERT_EQ(
          IKey("foo", 100, kTypeValue),
          Shorten(IKey("foo", 100, kTypeValue), IKey("foo", 100, kTypeDeletion)));

      // When user keys are misordered
      ASSERT_EQ(IKey("foo", 100, kTypeValue),
                Shorten(IKey("foo", 100, kTypeValue), IKey("bar", 99, kTypeValue)));

      // When user keys are different, but correctly ordered
      ASSERT_EQ(
          IKey("g", kMaxSequenceNumber, kValueTypeForSeek),
          Shorten(IKey("foo", 100, kTypeValue), IKey("hello", 200, kTypeValue)));

      // When start user key is prefix of limit user key
      ASSERT_EQ(
          IKey("foo", 100, kTypeValue),
          Shorten(IKey("foo", 100, kTypeValue), IKey("foobar", 200, kTypeValue)));

      // When limit user key is prefix of start user key
      ASSERT_EQ(
          IKey("foobar", 100, kTypeValue),
          Shorten(IKey("foobar", 100, kTypeValue), IKey("foo", 200, kTypeValue)));

    */
}

#[test] fn format_test_internal_key_shortest_successor() {
    todo!();
    /*
    
      ASSERT_EQ(IKey("g", kMaxSequenceNumber, kValueTypeForSeek),
                ShortSuccessor(IKey("foo", 100, kTypeValue)));
      ASSERT_EQ(IKey("\xff\xff", 100, kTypeValue),
                ShortSuccessor(IKey("\xff\xff", 100, kTypeValue)));

    */
}

#[test] fn format_test_parsed_internal_key_debug_string() {
    todo!();
    /*
    
      ParsedInternalKey key("The \"key\" in 'single quotes'", 42, kTypeValue);

      ASSERT_EQ("'The \"key\" in 'single quotes'' @ 42 : 1", key.DebugString());

    */
}

#[test] fn format_test_internal_key_debug_string() {
    todo!();
    /*
    
      InternalKey key("The \"key\" in 'single quotes'", 42, kTypeValue);
      ASSERT_EQ("'The \"key\" in 'single quotes'' @ 42 : 1", key.DebugString());

      InternalKey invalid_key;
      ASSERT_EQ("(bad)", invalid_key.DebugString());

    */
}

fn dbdbformat_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
