crate::ix!();

/**
  | A writer stream (for serialization)
  | that computes a 256-bit hash.
  |
  */
pub struct HashWriter {
    ctx:       Sha256,
    n_type:    i32,
    n_version: i32,
}

impl<T> Shl<&T> for HashWriter {
    type Output = HashWriter;
    
    #[inline] fn shl(self, rhs: &T) -> Self::Output {
        todo!();
        /*
            // Serialize to this stream
            ::Serialize(*this, obj);
            return (*this);
        */
    }
}

impl HashWriter {

    pub fn new(
        n_type_in:    i32,
        n_version_in: i32) -> Self {
    
        todo!();
        /*
        : n_type(nTypeIn),
        : n_version(nVersionIn),

        
        */
    }
    
    pub fn get_type(&self) -> i32 {
        
        todo!();
        /*
            return nType;
        */
    }
    
    pub fn get_version(&self) -> i32 {
        
        todo!();
        /*
            return nVersion;
        */
    }
    
    pub fn write(&mut self, 
        pch:  *const u8,
        size: usize)  {
        
        todo!();
        /*
            ctx.Write((const unsigned char*)pch, size);
        */
    }

    /**
      | Compute the double-SHA256 hash of all
      | data written to this object.
      | 
      | Invalidates this object.
      |
      */
    pub fn get_hash(&mut self) -> u256 {
        
        todo!();
        /*
            uint256 result;
            ctx.Finalize(result.begin());
            ctx.Reset().Write(result.begin(), CSHA256::OUTPUT_SIZE).Finalize(result.begin());
            return result;
        */
    }

    /**
      | Compute the SHA256 hash of all data written
      | to this object.
      | 
      | Invalidates this object.
      |
      */
    pub fn getsha256(&mut self) -> u256 {
        
        todo!();
        /*
            uint256 result;
            ctx.Finalize(result.begin());
            return result;
        */
    }

    /**
      | Returns the first 64 bits from the resulting
      | hash.
      |
      */
    #[inline] pub fn get_cheap_hash(&mut self) -> u64 {
        
        todo!();
        /*
            uint256 result = GetHash();
            return ReadLE64(result.begin());
        */
    }
}

lazy_static!{
    /*
    extern const CHashWriter HASHER_TAPLEAF;    /// Hasher with tag "TapLeaf" pre-fed to it.
    extern const CHashWriter HASHER_TAPBRANCH;  /// Hasher with tag "TapBranch" pre-fed to it.
    */
}

/**
  | Return a CHashWriter primed for tagged
  | hashes (as specified in BIP 340).
  | 
  | The returned object will have SHA256(tag)
  | written to it twice (= 64 bytes).
  | 
  | A tagged hash can be computed by feeding
  | the message into this object, and then
  | calling CHashWriter::GetSHA256().
  |
  */
pub fn tagged_hash(tag: &str) -> HashWriter {
    
    todo!();
        /*
            CHashWriter writer(SER_GETHASH, 0);
        uint256 taghash;
        CSHA256().Write((const unsigned char*)tag.data(), tag.size()).Finalize(taghash.begin());
        writer << taghash << taghash;
        return writer;
        */
}

lazy_static!{
    static ref HASHER_TAPSIGHASH: HashWriter = tagged_hash("TapSighash");
    static ref HASHER_TAPLEAF:    HashWriter = tagged_hash("TapLeaf");
    static ref HASHER_TAPBRANCH:  HashWriter = tagged_hash("TapBranch");
    static ref HASHER_TAPTWEAK:   HashWriter = tagged_hash("TapTweak");
}

