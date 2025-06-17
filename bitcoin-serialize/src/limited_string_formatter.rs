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
