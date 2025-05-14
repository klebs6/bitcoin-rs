// ---------------- [ File: bitcoin-crc32c/src/round_up.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/crc32c/src/crc32c_round_up.h]

pub mod crc32c {

    /**
      | Returns the smallest number >= the given number
      | that is evenly divided by N.
      |
      | N must be a power of two.
      */
    #[inline] pub fn round_up_with_uintptr<const N: i32>(pointer: libc::uintptr_t) -> libc::uintptr_t {

        todo!();
            /*
                const_assert((N & (N - 1)) == 0, "N must be a power of two");
          return (pointer + (N - 1)) & ~(N - 1);
            */
    }

    /**
      | Returns the smallest address >= the given
      | address that is aligned to N bytes.
      |
      | N must be a power of two.
      */
    #[inline] pub fn round_up<const N: i32>(pointer: *const u8) -> *const u8 {

        todo!();
            /*
                const_assert((N & (N - 1)) == 0, "N must be a power of two");
          return reinterpret_cast<uint8_t*>(
              RoundUp<N>(reinterpret_cast<uintptr_t>(pointer)));
            */
    }
}
