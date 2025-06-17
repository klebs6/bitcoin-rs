// ---------------- [ File: bitcoin-serialize/src/compact_size_formatter.rs ]
crate::ix!();

/**
  | Formatter for integers in CompactSize
  | format.
  |
  */
pub struct CompactSizeFormatter<const RangeCheck: bool> { }

impl<const RangeCheck: bool> CompactSizeFormatter<RangeCheck> {
    
    pub fn unser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: &mut I)  {
    
        todo!();
        /*
            uint64_t n = ReadCompactSize<Stream>(s, RangeCheck);
            if (n < std::numeric_limits<I>::min() || n > std::numeric_limits<I>::max()) {
                throw std::ios_base::failure("CompactSize exceeds limit of type");
            }
            v = n;
        */
    }
    
    
    pub fn ser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: I)  {
    
        todo!();
        /*
            const_assert(std::is_unsigned<I>::value, "CompactSize only supported for unsigned integers");
            const_assert(std::numeric_limits<I>::max() <= std::numeric_limits<uint64_t>::max(), "CompactSize only supports 64-bit integers and below");

            WriteCompactSize<Stream>(s, v);
        */
    }
}
