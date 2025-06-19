// ---------------- [ File: bitcoin-serialize/src/limited_string_formatter.rs ]
crate::ix!();

///-------------------------------
pub struct LimitedStringFormatter<'a, const Limit: usize> { 
    pub item: &'a mut String,
}

impl<'a, const Limit: usize> LimitedStringFormatter<'a, Limit> {
    pub fn unser<Stream>(&mut self, s: &mut Stream, v: &mut String)
    where
        Stream: Read,
    {
        let size = read_compact_size(s, Some(true)) as usize;
        if size > Limit {
            panic!("String length limit exceeded ({} > {})", size, Limit);
        }
        v.clear();
        v.reserve(size);
        if size != 0 {
            // SAFETY: we allocated exactly `size` bytes, read will fill them.
            unsafe {
                v.as_mut_vec().set_len(size);
            }
            s.read_exact(unsafe { v.as_mut_vec() })
                .expect("I/O error while reading limited string");
        }
        trace!(len = size, "LimitedStringFormatter::unser");
    }

    pub fn ser<Stream>(&mut self, s: &mut Stream, v: &String)
    where
        Stream: Write,
    {
        write_compact_size(s, v.len() as u64);
        if !v.is_empty() {
            s.write_all(v.as_bytes())
                .expect("I/O error while writing limited string");
        }
        trace!(len = v.len(), "LimitedStringFormatter::ser");
    }
}

#[cfg(test)]
mod limited_string_formatter_tests {
    use super::*;
    use std::io::Cursor;

    const LIMIT: usize = 16;

    /// Round‑trip strings whose length is *strictly below* the limit.
    #[traced_test]
    fn roundtrip_short_string() {
        let original = "hello".to_string();
        let mut cur = Cursor::new(Vec::<u8>::new());

        // Serialise
        let mut scratch = String::new();
        let mut fmt_ser = LimitedStringFormatter::<LIMIT> { item: &mut scratch };
        fmt_ser.ser(&mut cur, &original);

        // Deserialise
        let mut decoded = String::new();
        let mut fmt_de = LimitedStringFormatter::<LIMIT> { item: &mut decoded };
        cur.set_position(0);
        fmt_de.unser(&mut cur, &mut decoded);

        assert_eq!(decoded, original);
    }

    /// Boundary‑value: a string whose length is *exactly* the allowed
    /// limit must succeed.
    #[traced_test]
    fn exactly_at_limit_is_ok() {
        let original = "X".repeat(LIMIT);
        let mut buf = Cursor::new(Vec::<u8>::new());
        let mut tmp = String::new();
        let mut fmt = LimitedStringFormatter::<LIMIT> { item: &mut tmp };

        fmt.ser(&mut buf, &original);

        buf.set_position(0);
        let mut decoded = String::new();
        let mut fmt2 = LimitedStringFormatter::<LIMIT> { item: &mut decoded };
        fmt2.unser(&mut buf, &mut decoded);

        assert_eq!(decoded, original);
    }

    /// Attempting to decode a string that *exceeds* the configured limit
    /// must panic.  We do not match the exact message.
    #[test]
    #[should_panic]
    fn oversize_panics() {
        let original = "Y".repeat(LIMIT + 1);
        let mut buf = Cursor::new(Vec::<u8>::new());
        let mut tmp = String::new();
        let mut fmt = LimitedStringFormatter::<LIMIT> { item: &mut tmp };
        fmt.ser(&mut buf, &original);

        buf.set_position(0);
        let mut decoded = String::new();
        let mut fmt2 = LimitedStringFormatter::<LIMIT> { item: &mut decoded };
        fmt2.unser(&mut buf, &mut decoded);
    }

    /// Empty strings are handled correctly.
    #[traced_test]
    fn empty_string_roundtrip() {
        let original = String::new();
        let mut buf = Cursor::new(Vec::<u8>::new());
        let mut tmp = String::new();
        let mut fmt = LimitedStringFormatter::<LIMIT> { item: &mut tmp };
        fmt.ser(&mut buf, &original);

        buf.set_position(0);
        let mut decoded = "non‑empty".to_string();
        let mut fmt2 = LimitedStringFormatter::<LIMIT> { item: &mut decoded };
        fmt2.unser(&mut buf, &mut decoded);
        assert_eq!(decoded, original);
    }
}
