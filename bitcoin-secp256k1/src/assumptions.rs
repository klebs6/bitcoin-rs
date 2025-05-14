// ---------------- [ File: bitcoin-secp256k1/src/assumptions.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/secp256k1/src/assumptions.h]

/**
  | This library, like most software, relies
  | on a number of compiler implementation
  | defined (but not undefined) behaviours.
  | 
  | Although the behaviours we require
  | are essentially universal we test them
  | specifically here to reduce the odds
  | of experiencing an unwelcome surprise.
  |
  */
pub mod assumption_checker {

    /*
      | Conversions from unsigned to signed
      | outside of the bounds of the signed type
      | are implementation-defined. Verify
      | that they function as reinterpreting
      | the lower bits of the input in two's complement
      | notation. Do this for conversions:
      | 
      | - from uint(N)_t to int(N)_t with negative
      | result
      | 
      | - from uint(2N)_t to int(N)_t with negative
      | result
      | 
      | - from int(2N)_t to int(N)_t with negative
      | result
      | 
      | - from int(2N)_t to int(N)_t with positive
      | result
      |
      */

    /**
      | To int8_t.
      |
      */
    const_assert_eq!{
        (0xAB as u8) as i8,  
        -(0x55 as i8) as i8
    }

    const_assert_eq!{
        (0xABCD as u16) as i8,  
        -(0x33 as i8) as i8
    }

    const_assert_eq!{
        ((0xCDEF as u16) as i16) as i8, 
        (0xEF as u8) as i8 
    }

    const_assert_eq!{
        ((0x9234 as u16) as i16) as i8, 
        (0x34 as u8) as i8 
    }

    /**
      | To int16_t.
      |
      */
    const_assert_eq!{
         (0xBCDE as u16) as i16,
        -(0x4322 as i16) as i16 
    }

    const_assert_eq!{
        (0xA1B2C3D4 as u32) as i16,
        -(0x3C2C as i16) as i16
    }

    const_assert_eq!{
        ((0xC1D2E3F4 as u32) as i32) as i16,  
        (0xE3F4 as u16) as i16
    }

    const_assert_eq!{
        ((0x92345678 as u32) as i32) as i16, 
        (0x5678 as u16) as i16
    }

    /**
      | To int32_t.
      |
      */
    const_assert_eq!{
        (0xB2C3D4E5 as u32) as i32, 
        -(0x4D3C2B1B as i32) as i32
    }

    const_assert_eq!{
        (0xA123B456C789D012 as u64) as i32, 
        -(0x38762FEE as i32) as i32
    }

    const_assert_eq!{
        ((0xC1D2E3F4A5B6C7D8 as u64) as i64) as i32, 
        (0xA5B6C7D8 as u32) as i32
    }

    const_assert_eq!{
        ((0xABCDEF0123456789 as u64) as i64) as i32, 
        (0x23456789 as u32) as i32 
    }

    /**
      | To int64_t.
      |
      */
    const_assert_eq!{
        (0xB123C456D789E012 as u64) as i64, 
        -(0x4EDC3BA928761FEE as i64) as i64
    }

    #[cfg(WIDEMUL_INT128)]
    const_assert_eq!{
        (((0xA1234567B8901234 as u128) << 64) + 0xC5678901D2345678) as i64, 
        -(0x3A9876FE2DCBA988 as i64) as i64 
    }

    #[cfg(WIDEMUL_INT128)]
    const_assert_eq!{
        ((((0xB1C2D3E4F5A6B7C8 as u128) << 64) + 0xD9E0F1A2B3C4D5E6) as i128) as i64, 
        (0xD9E0F1A2B3C4D5E6 as u64) as i64
    }

    #[cfg(WIDEMUL_INT128)]
    const_assert_eq!{
        ((((0xABCDEF0123456789 as u128) << 64) + 0x0123456789ABCDEF) as i128) as i64, 
        (0x0123456789ABCDEF as u64) as i64 
    }

    /**
      | To int128_t.
      |
      */
    #[cfg(WIDEMUL_INT128)]
    const_assert_eq!{
        (((0xB1234567C8901234 as u128) << 64) + 0xD5678901E2345678) as i128, 
        (-(0x8E1648B3F50E80DC as i128) * 0x8E1648B3F50E80DD + 0x5EA688D5482F9464) as i128 
    }

    /**
      | Right shift on negative signed values
      | is implementation defined. Verify
      | that it acts as a right shift in two's
      | complement with sign extension (i.e
      | duplicating the top bit into newly added
      | bits).
      |
      */
    const_assert_eq!{
        unsafe{ std::mem::transmute::<u8,i8>(0xE8) } >> 2, 
        (0xFA as u8) as i8 
    }

    const_assert_eq!{
        unsafe { std::mem::transmute::<u16,i16>(0xE9AC) } >> 4, 
        (0xFE9A as u16) as i16 
    }

    const_assert_eq!{
        unsafe { std::mem::transmute::<u32,i32>(0x937C918A) } >> 9, 
        (0xFFC9BE48 as u32) as i32
    }

    const_assert_eq!{
        unsafe { std::mem::transmute::<u64,i64>(0xA8B72231DF9CF4B9) } >> 19, 
        (0xFFFFF516E4463BF3 as u64) as i64
    }

    #[cfg(WIDEMUL_INT128)]
    const_assert_eq!{
        ((((0xCD833A65684A0DBC as u128) << 64) + 0xB349312F71EA7637) as i128) >> 39, 
        (((0xFFFFFFFFFF9B0674 as u128) << 64) + 0xCAD0941B79669262) as i128
    }
}
