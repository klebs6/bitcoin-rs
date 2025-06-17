// ---------------- [ File: bitcoin-serialize/src/vector_formatter.rs ]
crate::ix!();

/**
  | Formatter to serialize/deserialize
  | vector elements using another formatter
  | 
  | Example:
  | 
  | -----------
  | @code
  | 
  | struct X {
  |   std::vector<uint64_t> v;
  |   SERIALIZE_METHODS(X, obj) { READWRITE(Using<VectorFormatter<VarInt>>(obj.v)); }
  | };
  |
  | will define a struct that contains a
  | vector of uint64_t, which is serialized
  | as a vector of VarInt-encoded integers.
  | 
  | V is not required to be an std::vector
  | type. It works for any class that exposes
  | a value_type, size, reserve, emplace_back,
  | back, and const iterators.
  |
  */
pub struct VectorFormatter<'a,T> {
    item: &'a mut T,
}

impl<'a,T> VectorFormatter<'a,T> {
    
    pub fn ser<Formatter,Stream, V>(&mut self, 
        s: &mut Stream,
        v: &V)  {
    
        todo!();
        /*
            Formatter formatter;
            WriteCompactSize(s, v.size());
            for (const typename V::value_type& elem : v) {
                formatter.Ser(s, elem);
            }
        */
    }
    
    pub fn unser<Formatter,Stream, V>(&mut self, 
        s: &mut Stream,
        v: &mut V)  {
    
        todo!();
        /*
            Formatter formatter;
            v.clear();
            size_t size = ReadCompactSize(s);
            size_t allocated = 0;
            while (allocated < size) {
                // For DoS prevention, do not blindly allocate as much as the stream claims to contain.
                // Instead, allocate in 5MiB batches, so that an attacker actually needs to provide
                // X MiB of data to make us allocate X+5 Mib.
                const_assert(sizeof(typename V::value_type) <= MAX_VECTOR_ALLOCATE, "Vector element size too large");
                allocated = std::min(size, allocated + MAX_VECTOR_ALLOCATE / sizeof(typename V::value_type));
                v.reserve(allocated);
                while (v.size() < allocated) {
                    v.emplace_back();
                    formatter.Unser(s, v.back());
                }
            }
        }{
        */
    }
}
