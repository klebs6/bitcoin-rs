crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/leveldb/util/crc32c_test.cc]

mod crc32c {

    pub struct CRC {}

    #[test] fn crc_standard_results() {
        todo!();
        /*
        
          // From rfc3720 section B.4.
          char buf[32];

          memset(buf, 0, sizeof(buf));
          ASSERT_EQ(0x8a9136aa, Value(buf, sizeof(buf)));

          memset(buf, 0xff, sizeof(buf));
          ASSERT_EQ(0x62a8ab43, Value(buf, sizeof(buf)));

          for (int i = 0; i < 32; i++) {
            buf[i] = i;
          }
          ASSERT_EQ(0x46dd794e, Value(buf, sizeof(buf)));

          for (int i = 0; i < 32; i++) {
            buf[i] = 31 - i;
          }
          ASSERT_EQ(0x113fdb5c, Value(buf, sizeof(buf)));

          uint8_t data[48] = {
              0x01, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
              0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00,
              0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
          };
          ASSERT_EQ(0xd9963a56, Value(reinterpret_cast<char*>(data), sizeof(data)));

        */
    }

    #[test] fn crc_values() {
        todo!();
        /*
             ASSERT_NE(Value("a", 1), Value("foo", 3)); 
        */
    }

    #[test] fn crc_extend() {
        todo!();
        /*
        
          ASSERT_EQ(Value("hello world", 11), Extend(Value("hello ", 6), "world", 5));

        */
    }

    #[test] fn crc_mask() {
        todo!();
        /*
        
          uint32_t crc = Value("foo", 3);
          ASSERT_NE(crc, Mask(crc));
          ASSERT_NE(crc, Mask(Mask(crc)));
          ASSERT_EQ(crc, Unmask(Mask(crc)));
          ASSERT_EQ(crc, Unmask(Unmask(Mask(Mask(crc)))));

        */
    }
}

fn testcrc32c_test_main (
        argc: i32,
        argv: *mut *mut u8) -> i32 {
    
    todo!();
        /*
            return leveldb::test::RunAllTests();
        */
}
