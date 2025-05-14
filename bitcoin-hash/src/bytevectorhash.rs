// ---------------- [ File: bitcoin-hash/src/bytevectorhash.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/bytevectorhash.h]
//-------------------------------------------[.cpp/bitcoin/src/util/bytevectorhash.cpp]

/**
  | Implementation of Hash named requirement
  | for types that internally store a byte
  | array. This may be used as the hash function
  | in std::unordered_set or std::unordered_map
  | over such types.
  | 
  | Internally, this uses a random instance
  | of SipHash-2-4.
  |
  */
pub struct ByteVectorHash {
    k0: u64,
    k1: u64,
}

impl Default for ByteVectorHash {
    
    fn default() -> Self {
        todo!();
        /*


            GetRandBytes(reinterpret_cast<unsigned char*>(&m_k0), sizeof(m_k0));
        GetRandBytes(reinterpret_cast<unsigned char*>(&m_k1), sizeof(m_k1));
        */
    }
}

impl BuildHasher for ByteVectorHash {

    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        todo!();
    }
}

impl Hasher for ByteVectorHash {

    fn finish(&self) -> u64 {
        todo!();
    }

    fn write(&mut self, bytes: &[u8]) {
        todo!();
    }
}

impl ByteVectorHash {
    
    pub fn invoke(&self, input: &Vec<u8>) -> usize {
        
        todo!();
        /*
            return CSipHasher(m_k0, m_k1).Write(input.data(), input.size()).Finalize();
        */
    }
}
