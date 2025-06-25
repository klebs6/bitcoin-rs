use bitcoin_crc32c::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_unittest.cc]

#[test] fn crc_32c_test() {
    todo!();
    /*
    
  // From rfc3720 section B.4.
  uint8_t buf[32];

  std::memset(buf, 0, sizeof(buf));
  EXPECT_EQ(static_cast<uint32_t>(0x8a9136aa),
            crc32c::Crc32c(buf, sizeof(buf)));

  std::memset(buf, 0xff, sizeof(buf));
  EXPECT_EQ(static_cast<uint32_t>(0x62a8ab43),
            crc32c::Crc32c(buf, sizeof(buf)));

  for (size_t i = 0; i < 32; ++i)
    buf[i] = static_cast<uint8_t>(i);
  EXPECT_EQ(static_cast<uint32_t>(0x46dd794e),
            crc32c::Crc32c(buf, sizeof(buf)));

  for (size_t i = 0; i < 32; ++i)
    buf[i] = static_cast<uint8_t>(31 - i);
  EXPECT_EQ(static_cast<uint32_t>(0x113fdb5c),
            crc32c::Crc32c(buf, sizeof(buf)));

  uint8_t data[48] = {
      0x01, 0xc0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
      0x00, 0x00, 0x00, 0x14, 0x00, 0x00, 0x00, 0x18, 0x28, 0x00, 0x00, 0x00,
      0x00, 0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
  };
  EXPECT_EQ(static_cast<uint32_t>(0xd9963a56),
            crc32c::Crc32c(data, sizeof(data)));

    */
}

pub mod crc32c {

    pub struct ApiTestTraits { }

    impl ApiTestTraits {
        
        pub fn extend(
            crc:   u32,
            data:  *const u8,
            count: usize) -> u32 {
            
            todo!();
            /*
                return ::crc32c::Extend(crc, data, count);
            */
        }
    }
}

#[test] fn crc32c_test_crc_32c_char_pointer() {
    todo!();
    /*
    
  char buf[32];

  std::memset(buf, 0, sizeof(buf));
  EXPECT_EQ(static_cast<uint32_t>(0x8a9136aa),
            crc32c::Crc32c(buf, sizeof(buf)));

  std::memset(buf, 0xff, sizeof(buf));
  EXPECT_EQ(static_cast<uint32_t>(0x62a8ab43),
            crc32c::Crc32c(buf, sizeof(buf)));

  for (size_t i = 0; i < 32; ++i)
    buf[i] = static_cast<char>(i);
  EXPECT_EQ(static_cast<uint32_t>(0x46dd794e),
            crc32c::Crc32c(buf, sizeof(buf)));

  for (size_t i = 0; i < 32; ++i)
    buf[i] = static_cast<char>(31 - i);
  EXPECT_EQ(static_cast<uint32_t>(0x113fdb5c),
            crc32c::Crc32c(buf, sizeof(buf)));

    */
}

#[test] fn crc32c_test_crc_32c_std_string() {
    todo!();
    /*
    
  std::string buf;
  buf.resize(32);

  for (size_t i = 0; i < 32; ++i)
    buf[i] = static_cast<char>(0x00);
  EXPECT_EQ(static_cast<uint32_t>(0x8a9136aa), crc32c::Crc32c(buf));

  for (size_t i = 0; i < 32; ++i)
    buf[i] = '\xff';
  EXPECT_EQ(static_cast<uint32_t>(0x62a8ab43), crc32c::Crc32c(buf));

  for (size_t i = 0; i < 32; ++i)
    buf[i] = static_cast<char>(i);
  EXPECT_EQ(static_cast<uint32_t>(0x46dd794e), crc32c::Crc32c(buf));

  for (size_t i = 0; i < 32; ++i)
    buf[i] = static_cast<char>(31 - i);
  EXPECT_EQ(static_cast<uint32_t>(0x113fdb5c), crc32c::Crc32c(buf));

    */
}

#[test] fn crc32c_test_crc_32c_std_string_view() {
    todo!();
    /*
    
  uint8_t buf[32];
  std::string_view view(reinterpret_cast<const char*>(buf), sizeof(buf));

  std::memset(buf, 0, sizeof(buf));
  EXPECT_EQ(static_cast<uint32_t>(0x8a9136aa), crc32c::Crc32c(view));

  std::memset(buf, 0xff, sizeof(buf));
  EXPECT_EQ(static_cast<uint32_t>(0x62a8ab43), crc32c::Crc32c(view));

  for (size_t i = 0; i < 32; ++i)
    buf[i] = static_cast<uint8_t>(i);
  EXPECT_EQ(static_cast<uint32_t>(0x46dd794e), crc32c::Crc32c(view));

  for (size_t i = 0; i < 32; ++i)
    buf[i] = static_cast<uint8_t>(31 - i);
  EXPECT_EQ(static_cast<uint32_t>(0x113fdb5c), crc32c::Crc32c(view));

    */
}
