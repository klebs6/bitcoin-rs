// ---------------- [ File: bitcoin-serialize/src/custom_uint_formatter.rs ]
crate::ix!();

/**
  | **CustomUintFormatter**
  |
  | A generic formatter for fixed‑width unsigned
  | integers and C‑style enums.  It supports:
  |
  | * **BYTES** – number of encoded bytes
  |   (compile‑time range‑checked: 1 – 8);
  | * **BIG_ENDIAN** – endianness (default:
  |   little‑endian = `false`; set `true`
  |   for network / big‑endian order).
  |
  | The value is range‑checked against the
  | maximum representable with the chosen
  | width before serialisation and after
  | deserialisation.
  |
  */
pub struct CustomUintFormatter<const BYTES: i32, const BIG_ENDIAN: bool = false>;

impl<const BYTES: i32, const BIG_ENDIAN: bool> Default
    for CustomUintFormatter<BYTES, BIG_ENDIAN>
{
    #[inline]
    fn default() -> Self {
        Self
    }
}

impl<I, const BYTES: i32, const BIG_ENDIAN: bool> ValueFormatter<I>
    for CustomUintFormatter<BYTES, BIG_ENDIAN>
where
    /* Compile‑time guarantee: 1 ≤ BYTES ≤ 8 */
    crate::meta::If<{ crate::meta::inclusive_range_1_to_8::<BYTES>() }>: crate::meta::True,
    /* Conversion traits */
    I: Copy + core::fmt::Debug + TryInto<u64> + TryFrom<u64>,
    <I as TryInto<u64>>::Error: core::fmt::Debug,
    <I as TryFrom<u64>>::Error: core::fmt::Debug,
{
    #[inline]
    fn ser<S: Write>(&mut self, s: &mut S, v: &I) {

        let max: u64 = u64::MAX >> (8 * (8 - BYTES) as usize);/* width mask */

        let raw: u64 = (*v)
            .try_into()
            .expect("CustomUintFormatter: failed to convert value to u64");

        if raw > max {
            error!(value = raw, max = max, "CustomUintFormatter value out of range");
            panic!("CustomUintFormatter value out of range");
        }

        let full_bytes = if BIG_ENDIAN {
            raw.to_be_bytes()
        } else {
            raw.to_le_bytes()
        };

        let slice = if BIG_ENDIAN {
            &full_bytes[8 - BYTES as usize..]
        } else {
            &full_bytes[..BYTES as usize]
        };

        s.write_all(slice)
            .expect("I/O error while writing CustomUintFormatter");

        trace!(
            bytes = BYTES,
            big_endian = BIG_ENDIAN,
            value = raw,
            "CustomUintFormatter::ser"
        );
    }

    #[inline]
    fn unser<S: Read>(&mut self, s: &mut S, v: &mut I) {

        let max: u64 = u64::MAX >> (8 * (8 - BYTES) as usize); /* width mask */

        let mut buf = [0u8; 8];
        if BIG_ENDIAN {
            s.read_exact(&mut buf[8 - BYTES as usize..])
        } else {
            s.read_exact(&mut buf[..BYTES as usize])
        }
        .expect("I/O error while reading CustomUintFormatter");

        let raw = if BIG_ENDIAN {
            u64::from_be_bytes(buf)
        } else {
            u64::from_le_bytes(buf)
        };

        if raw > max {
            error!(value = raw, max = max, "CustomUintFormatter value out of range");
            panic!("CustomUintFormatter value out of range");
        }

        *v = I::try_from(raw)
            .expect("CustomUintFormatter: target type cannot represent value");

        trace!(
            bytes = BYTES,
            big_endian = BIG_ENDIAN,
            value = raw,
            "CustomUintFormatter::unser"
        );
    }
}

/// Convenience alias: always big‑endian.
pub type BigEndianFormatter<const BYTES: i32> = CustomUintFormatter<BYTES, true>;

#[cfg(test)]
mod custom_uint_formatter_tests {
    use super::*;
    use std::io::Cursor;

    #[traced_test]
    fn roundtrip_little_endian_u32_3bytes() {
        type Fmt = CustomUintFormatter<3, false>;
        let mut fmt = Fmt::default();

        let value: u32 = 0x00_12_34_56;
        let mut cur = Cursor::new(Vec::<u8>::new());

        fmt.ser(&mut cur, &value);
        assert_eq!(cur.get_ref().as_slice(), &[0x56, 0x34, 0x12]);

        cur.set_position(0);
        let mut decoded = 0u32;
        fmt.unser(&mut cur, &mut decoded);
        assert_eq!(decoded, value);
    }

    #[traced_test]
    fn roundtrip_big_endian_u16_2bytes() {
        type Fmt = CustomUintFormatter<2, true>;
        let mut fmt = Fmt::default();

        let value: u16 = 0xABCD;
        let mut cur = Cursor::new(Vec::<u8>::new());

        fmt.ser(&mut cur, &value);
        assert_eq!(cur.get_ref().as_slice(), &[0xAB, 0xCD]);

        cur.set_position(0);
        let mut decoded = 0u16;
        fmt.unser(&mut cur, &mut decoded);
        assert_eq!(decoded, value);
    }
}
