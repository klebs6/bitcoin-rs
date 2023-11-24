crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/compilerbug_tests.cpp]

#[cfg(__GNUC__)]
#[cfg(test)]
pub mod compilerbug_tests {

    /**
      | This block will also be built under clang,
      | which is fine (as it supports noinline)
      |
      */
    #[inline(never)] pub fn set_one(ptr: *mut u8)  {
        
        todo!();
            /*
                *ptr = 1;
            */
    }

    #[inline(never)] pub fn check_zero(
            in_: *const u8,
            len: u32) -> i32 {
        
        todo!();
            /*
                for (unsigned int i = 0; i < len; ++i) {
                if (in[i] != 0) return 0;
            }
            return 1;
            */
    }

    pub fn set_one_on_stack()  {
        
        todo!();
            /*
                unsigned char buf[1];
            set_one(buf);
            */
    }

    #[test] fn gccbug_90348() {
        todo!();
        /*
        
            // Test for GCC bug 90348. See https://gcc.gnu.org/bugzilla/show_bug.cgi?id=90348
            for (int i = 0; i <= 4; ++i) {
                unsigned char in[4];
                for (int j = 0; j < i; ++j) {
                    in[j] = 0;
                    set_one_on_stack(); // Apparently modifies in[0]
                }
                BOOST_CHECK(check_zero(in, i));
            }

        */
    }
}

