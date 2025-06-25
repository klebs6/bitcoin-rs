use bitcoin_crc32c::*;
use bitcoin_imports::*;

//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_arm64_unittest.cc]

pub mod crc32c {

    #[cfg(HAVE_ARM64_CRC32C)]
    pub struct Arm64TestTraits {

    }

    #[cfg(HAVE_ARM64_CRC32C)]
    impl Arm64TestTraits {

        pub fn extend(
            crc:   u32,
            data:  *const u8,
            count: usize) -> u32 {
            
            todo!();
            /*
                return ExtendArm64(crc, data, count);
            */
        }
    }

    #[cfg(HAVE_ARM64_CRC32C)]
    instantiate_typed_test_suite_p!{
        Arm64, ExtendTest, Arm64TestTraits
    }
}
