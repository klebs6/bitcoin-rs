// ---------------- [ File: bitcoin-blockencoding/src/difference_formatter.rs ]
crate::ix!();

#[derive(Default)]
pub struct DifferenceFormatter {
    pub shift: u64, // default = 0
}

impl DifferenceFormatter {

    pub fn ser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: I)  {
    
        todo!();
        /*
            if (v < m_shift || v >= std::numeric_limits<uint64_t>::max()) throw std::ios_base::failure("differential value overflow");
            WriteCompactSize(s, v - m_shift);
            m_shift = uint64_t(v) + 1;
        */
    }
    
    pub fn unser<Stream, I>(&mut self, 
        s: &mut Stream,
        v: &mut I)  {
    
        todo!();
        /*
            uint64_t n = ReadCompactSize(s);
            m_shift += n;
            if (m_shift < n || m_shift >= std::numeric_limits<uint64_t>::max() || m_shift < std::numeric_limits<I>::min() || m_shift > std::numeric_limits<I>::max()) throw std::ios_base::failure("differential value overflow");
            v = I(m_shift++);
        */
    }
}
