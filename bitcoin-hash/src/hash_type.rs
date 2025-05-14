// ---------------- [ File: bitcoin-hash/src/hash_type.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/hash_type.h]

pub struct BaseHash<HashType> {
    hash: HashType,
}

impl<HashType> Default for BaseHash<HashType> {
    
    fn default() -> Self {
        todo!();
        /*
        : hash(),

        
        */
    }
}

impl<HashType> Into<Vec<u8>> for BaseHash<HashType> {

    #[inline] fn into(self) -> Vec<u8> {
        todo!();
        /*
            return std::vector<unsigned char>{m_hash.begin(), m_hash.end()};
        */
    }
}

impl<HashType> PartialEq<BaseHash<HashType>> for BaseHash<HashType> {
    
    #[inline] fn eq(&self, other: &BaseHash<HashType>) -> bool {
        todo!();
        /*
            return m_hash == other.m_hash;
        */
    }
}

impl<HashType> Eq for BaseHash<HashType> {}

impl<HashType> Ord for BaseHash<HashType> {
    
    #[inline] fn cmp(&self, other: &BaseHash<HashType>) -> Ordering {
        todo!();
        /*
            return m_hash < other.m_hash;
        */
    }
}

impl<HashType> PartialOrd<BaseHash<HashType>> for BaseHash<HashType> {
    #[inline] fn partial_cmp(&self, other: &BaseHash<HashType>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<HashType> BaseHash<HashType> {

    pub fn new(in_: &HashType) -> Self {
    
        todo!();
        /*
        : hash(in),

        
        */
    }
    
    pub fn begin_mut(&mut self) -> *mut u8 {
        
        todo!();
        /*
            return m_hash.begin();
        */
    }
    
    pub fn begin(&self) -> *const u8 {
        
        todo!();
        /*
            return m_hash.begin();
        */
    }
    
    pub fn end_mut(&mut self) -> *mut u8 {
        
        todo!();
        /*
            return m_hash.end();
        */
    }
    
    pub fn end(&self) -> *const u8 {
        
        todo!();
        /*
            return m_hash.end();
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return m_hash.ToString();
        */
    }
    
    pub fn size(&self) -> usize {
        
        todo!();
        /*
            return m_hash.size();
        */
    }
    
    pub fn data_mut(&mut self) -> *mut u8 {
        
        todo!();
        /*
            return m_hash.data();
        */
    }
    
    pub fn data(&self) -> *const u8 {
        
        todo!();
        /*
            return m_hash.data();
        */
    }
}
