crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/hasher.h]

pub struct SaltedTxidHasher {

    /**
      | Salt
      |
      */
    k0: u64,
    k1: u64,
}

impl SaltedTxidHasher {

    pub fn invoke(&self, txid: &u256) -> usize {
        
        todo!();
        /*
            return SipHashUint256(k0, k1, txid);
        */
    }
}

///------------------------
pub struct SaltedOutpointHasher {

    /**
      | Salt
      |
      */
    k0: u64,
    k1: u64,
}

impl BuildHasher for SaltedOutpointHasher {

    type Hasher = Self;

    fn build_hasher(&self) -> Self::Hasher {
        todo!();
    }
}

impl Hasher for SaltedOutpointHasher {

    fn finish(&self) -> u64 {
        todo!();
    }

    fn write(&mut self, bytes: &[u8]) {
        todo!();
    }
}

impl SaltedOutpointHasher {

    /**
      | Having the hash allows libstdc++'s
      | unordered_map to recalculate the hash
      | during rehash, so it does not have to
      | cache the value. This reduces node's
      | memory by sizeof(size_t). The required
      | recalculation has a slight performance
      | penalty (around 1.6%), but this is compensated
      | by memory savings of about 9% which allow
      | for a larger dbcache setting.
      | 
      | -----------
      | @note
      | 
      | see https://gcc.gnu.org/onlinedocs/gcc-9.2.0/libstdc++/manual/manual/unordered_associative.html
      |
      */
    pub fn invoke(&self, id: &OutPoint) -> usize {
        
        todo!();
        /*
            return SipHashUint256Extra(k0, k1, id.hash, id.n);
        */
    }
}

pub struct FilterHeaderHasher { }

impl FilterHeaderHasher {
    
    pub fn invoke(&self, hash: &u256) -> usize {
        
        todo!();
        /*
            return ReadLE64(hash.begin());
        */
    }
}

/**
  | We're hashing a nonce into the entries
  | themselves, so we don't need extra blinding
  | in the set hash computation.
  | 
  | This may exhibit platform endian dependent
  | behavior but because these are nonced
  | hashes (random) and this state is only
  | ever used locally it is safe.
  | 
  | All that matters is local consistency.
  |
  */
pub struct SignatureCacheHasher { }

impl SignatureCacheHasher {
    
    pub fn invoke<const hash_select: u8>(&self, key: &u256) -> u32 {
    
        todo!();
        /*
            const_assert(hash_select <8, "SignatureCacheHasher only has 8 hashes available.");
            uint32_t u;
            std::memcpy(&u, key.begin()+4*hash_select, 4);
            return u;
        */
    }
}

pub struct BlockHasher { }

impl BlockHasher {

    /**
      | this used to call `GetCheapHash()` in
      | uint256, which was later moved; the cheap
      | hash function simply calls ReadLE64()
      | however, so the end result is identical
      */
    pub fn invoke(&self, hash: &u256) -> usize {
        
        todo!();
        /*
            return ReadLE64(hash.begin());
        */
    }
}

///--------------------
pub struct SaltedSipHasher {

    /**
      | Salt
      |
      */
    k0: u64,
    k1: u64,
}

impl SaltedSipHasher {
    
    pub fn invoke(&self, script: &[u8]) -> usize {
        
        todo!();
        /*
            return CSipHasher(m_k0, m_k1).Write(script.data(), script.size()).Finalize();
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/util/hasher.cpp]
impl Default for SaltedTxidHasher {

    fn default() -> Self {
    
        todo!();
        /*
            : k0(GetRand(std::numeric_limits<uint64_t>::max())), k1(GetRand(std::numeric_limits<uint64_t>::max()))
        */
    }
}

impl Default for SaltedOutpointHasher {
    
    fn default() -> Self {
        todo!();
        /*


            : k0(GetRand(std::numeric_limits<uint64_t>::max())), k1(GetRand(std::numeric_limits<uint64_t>::max()))
        */
    }
}

impl Default for SaltedSipHasher {
    
    fn default() -> Self {
        todo!();
        /*


            : m_k0(GetRand(std::numeric_limits<uint64_t>::max())), m_k1(GetRand(std::numeric_limits<uint64_t>::max()))
        */
    }
}
