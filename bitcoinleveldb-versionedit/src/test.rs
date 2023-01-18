crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/version_edit_test.cc]

fn test_encode_decode(edit: &VersionEdit)  {
    
    todo!();
        /*
            std::string encoded, encoded2;
      edit.EncodeTo(&encoded);
      VersionEdit parsed;
      Status s = parsed.DecodeFrom(encoded);
      ASSERT_TRUE(s.ok()) << s.ToString();
      parsed.EncodeTo(&encoded2);
      ASSERT_EQ(encoded, encoded2);
        */
}

struct VersionEditTest {}

#[test] fn version_edit_test_encode_decode() {
    todo!();
    /*
    
      static const uint64_t kBig = 1ull << 50;

      VersionEdit edit;
      for (int i = 0; i < 4; i++) {
        TestEncodeDecode(edit);
        edit.AddFile(3, kBig + 300 + i, kBig + 400 + i,
                     InternalKey("foo", kBig + 500 + i, kTypeValue),
                     InternalKey("zoo", kBig + 600 + i, kTypeDeletion));
        edit.DeleteFile(4, kBig + 700 + i);
        edit.SetCompactPointer(i, InternalKey("x", kBig + 900 + i, kTypeValue));
      }

      edit.SetComparatorName("foo");
      edit.SetLogNumber(kBig + 100);
      edit.SetNextFile(kBig + 200);
      edit.SetLastSequence(kBig + 1000);
      TestEncodeDecode(edit);

    */
}

fn dbversion_edit_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}

