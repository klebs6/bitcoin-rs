crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_sse42_unittest.cc]

#[cfg(all(HAVE_SSE42,any(_M_X64,__x86_64__)))]
pub mod crc32c {

    pub struct Sse42TestTraits {

    }

    impl Sse42TestTraits {

        pub fn extend(
            crc:   u32,
            data:  *const u8,
            count: usize) -> u32 {
            
            todo!();
            /*
                return ExtendSse42(crc, data, count);
            */
        }
    }

    instantiate_typed_test_suite_p!{
        Sse42, 
        ExtendTest, 
        Sse42TestTraits
    }
}
