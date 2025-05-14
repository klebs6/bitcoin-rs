// ---------------- [ File: bitcoin-crc32c/src/read_le_unittest.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_read_le_unittest.cc]

pub mod crc32c {

    #[test] fn crc_32c_read_le_test_uint32le() {
        todo!();
        /*
        
      // little-endian 0x12345678
      alignas(4) uint8_t bytes[] = {0x78, 0x56, 0x34, 0x12};

      ASSERT_EQ(RoundUp<4>(bytes), bytes) << "Stack array is not aligned";
      EXPECT_EQ(static_cast<uint32_t>(0x12345678), ReadUint32LE(bytes));

        */
    }

    #[test] fn crc_32c_read_le_test_uint64le() {
        todo!();
        /*
        
      // little-endian 0x123456789ABCDEF0
      alignas(8) uint8_t bytes[] = {0xF0, 0xDE, 0xBC, 0x9A, 0x78, 0x56, 0x34, 0x12};

      ASSERT_EQ(RoundUp<8>(bytes), bytes) << "Stack array is not aligned";
      EXPECT_EQ(static_cast<uint64_t>(0x123456789ABCDEF0), ReadUint64LE(bytes));

        */
    }
}
