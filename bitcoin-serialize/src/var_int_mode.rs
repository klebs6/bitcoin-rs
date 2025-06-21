// ---------------- [ File: bitcoin-serialize/src/var_int_mode.rs ]
crate::ix!();

/**
  | Mode for encoding VarInts.
  | 
  | Currently there is no support for signed
  | encodings. The default mode will not
  | compile with signed values, and the
  | legacy "nonnegative signed" mode will
  | accept signed values, but improperly
  | encode and decode them if they are negative.
  | In the future, the Default mode could
  | be extended to support negative numbers
  | in a backwards compatible way, and additional
  | modes could be added to support different
  | varint formats (e.g. zigzag encoding).
  |
  */
#[derive(Clone,Debug,ConstParamTy,PartialEq,Eq)]
pub enum VarIntMode { 
    Default, 
    NonNegativeSigned 
}

#[inline]
pub fn get_size_of_var_int<I, const Mode: VarIntMode>(mut n: I) -> u32
where
    (): ModeConstraint<Mode, I>,
    I: Into<u128> + Copy,
{
    let mut n128: u128 = n.into();
    let mut count = 1;
    while n128 > 0x7F {
        n128 = (n128 >> 7) - 1;
        count += 1;
    }
    count
}

pub fn write_var_int<Stream, I, const Mode: VarIntMode>(os: &mut Stream, n: I)
where
    (): ModeConstraint<Mode, I>,
    Stream: Write,
    I: Into<u128> + Copy,
{
    let mut val: u128 = n.into();
    let mut bytes = Vec::<u8>::new();

    loop {
        bytes.push(((val & 0x7F) as u8) | if bytes.is_empty() { 0 } else { 0x80 });
        if val <= 0x7F { break; }
        val = (val >> 7) - 1;
    }
    for b in bytes.iter().rev() { ser_writedata8(os, *b); }
}

pub fn read_var_int<Stream, I, const Mode: VarIntMode>(is: &mut Stream) -> I
where
    (): ModeConstraint<Mode, I>,
    Stream: Read,
    I: TryFrom<u128>,
    <I as TryFrom<u128>>::Error: core::fmt::Debug,
{
    let mut n: u128 = 0;
    loop {
        let ch = ser_readdata8(is);
        if n > (u128::MAX >> 7) { panic!("ReadVarInt(): size too large"); }
        n = (n << 7) | u128::from(ch & 0x7F);
        if ch & 0x80 != 0 {
            if n == u128::MAX { panic!("ReadVarInt(): size too large"); }
            n += 1;
        } else {
            break;
        }
    }
    I::try_from(n).expect("ReadVarInt(): value does not fit target type")
}

#[cfg(test)]
mod var_int_algo_tests {
    use super::*;
    use std::io::Cursor;

    /* ---------- common fixtures ---------- */

    // Representative boundary values plus a few big numbers.
    const UNSIGNED_SAMPLES: &[u64] = &[
        0, 1, 0x7F, 0x80, 0x3FFF, 0x4000,
        0xFFFF_FFFF, (u32::MAX as u64) + 1, u64::MAX / 2,
    ];
    const SIGNED_SAMPLES: &[i64] = &[
        0, 1, 0x7F, 0x1000, 0x1_FFFF, 0x7FFF_FFFF,
    ];

    /* ---------- Default mode (unsigned) ---------- */

    #[traced_test]
    fn roundtrip_default_mode() {
        for &n in UNSIGNED_SAMPLES {
            let mut buf = Cursor::new(Vec::<u8>::new());
            write_var_int::<_, u64, { VarIntMode::Default }>(&mut buf, n);
            buf.set_position(0);
            let out: u64 =
                read_var_int::<_, u64, { VarIntMode::Default }>(&mut buf);
            assert_eq!(out, n, "round‑trip failed for {n}");
        }
    }

    #[traced_test]
    fn size_fn_matches_bytes_written() {
        for &n in UNSIGNED_SAMPLES {
            let mut buf = Cursor::new(Vec::<u8>::new());
            write_var_int::<_, u64, { VarIntMode::Default }>(&mut buf, n);
            let bytes = buf.get_ref().len();
            let spec  =
                get_size_of_var_int::<u64, { VarIntMode::Default }>(n);
            assert_eq!(bytes, spec as usize, "size mismatch for {n}");
        }
    }

    /* ---------- Formatter wrapper ---------- */

    #[traced_test]
    fn formatter_wrapper_roundtrip() {
        let original = 300u64;
        let mut buf  = Cursor::new(Vec::<u8>::new());

        // serialize
        VarIntFormatter::<{ VarIntMode::Default }>::default()
            .ser(&mut buf, &original);

        // deserialize
        buf.set_position(0);
        let mut decoded = 0u64;
        VarIntFormatter::<{ VarIntMode::Default }>::default()
            .unser(&mut buf, &mut decoded);

        assert_eq!(decoded, original);
    }

    /* ---------- Error handling ---------- */

    /// Encoding of 2 ⁶⁴ − 1 does **not** fit in a `u32`.
    #[test]
    #[should_panic]   // exact message is implementation detail
    fn read_into_too_small_type_panics() {
        let mut buf = Cursor::new(Vec::<u8>::new());
        write_var_int::<_, u64, { VarIntMode::Default }>(
            &mut buf,
            u64::MAX,
        );
        buf.set_position(0);
        let _: u32 =
            read_var_int::<_, u32, { VarIntMode::Default }>(&mut buf);
    }

    /// Malformed byte stream with the continuation‑bit set in every byte.
    #[test]
    #[should_panic]
    fn decode_overflow_panics() {
        let garbage = [0x80u8; 20];           // will overflow u128
        let mut buf = Cursor::new(garbage.as_slice());
        let _: u64 =
            read_var_int::<_, u64, { VarIntMode::Default }>(&mut buf);
    }

    /* ---------- Compile‑time assertions ---------- */

    // This function *never* runs – it only verifies that the `ModeConstraint`
    // accepts (u64, Default) and (i64, NonNegativeSigned).
    #[allow(dead_code)]
    fn _compile_time_constraints()
    where
        (): crate::check_var_int_mode::ModeConstraint<
            { VarIntMode::Default },
            u64,
        >,
        (): crate::check_var_int_mode::ModeConstraint<
            { VarIntMode::NonNegativeSigned },
            i64,
        >,
    {}
}
