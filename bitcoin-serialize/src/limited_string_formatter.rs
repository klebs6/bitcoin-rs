// ---------------- [ File: bitcoin-serialize/src/limited_string_formatter.rs ]
crate::ix!();

///-------------------------------
pub struct LimitedStringFormatter<'a, const Limit: usize> { 
    pub item: &'a mut String,
}

impl<'a, const Limit: usize> LimitedStringFormatter<'a, Limit> {
    
    pub fn unser<Stream>(&mut self, 
        s: &mut Stream,
        v: &mut String)  {
    
        todo!();
        /*
            size_t size = ReadCompactSize(s);
            if (size > Limit) {
                throw std::ios_base::failure("String length limit exceeded");
            }
            v.resize(size);
            if (size != 0) s.read((char*)v.data(), size);
        */
    }
    
    
    pub fn ser<Stream>(&mut self, 
        s: &mut Stream,
        v: &String)  {
    
        todo!();
        /*
            s << v;
        */
    }
}
