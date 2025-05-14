// ---------------- [ File: bitcoin-scripting/src/sigcache.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/script/sigcache.h]

/**
  | DoS prevention: limit cache size to 32MB (over
  | 1000000 entries on 64-bit systems). Due to how
  | we count cache size, actual memory usage is
  | slightly more (~32.25 MB)
  */
pub const DEFAULT_MAX_SIG_CACHE_SIZE: u32 = 32;

/**
  | Maximum sig cache size allowed
  |
  */
pub const MAX_MAX_SIG_CACHE_SIZE: i64 = 16384;


//-------------------------------------------[.cpp/bitcoin/src/script/sigcache.cpp]

/**
  | Valid signature cache, to avoid doing
  | expensive ECDSA signature checking
  | twice for every transaction (once when
  | accepted into memory pool, and again
  | when accepted into the block chain)
  |
  */
pub struct SignatureCache {

    /**
      | Entries are SHA256(nonce || 'E' or 'S'
      | || 31 zero bytes || signature hash ||
      | public key | signature):
      |
      */
    salted_hasher_ecdsa:   Sha256,
    salted_hasher_schnorr: Sha256,
    set_valid:             SignatureCacheMap,
    cs_sigcache:           RawSharedMutex,
}

pub type SignatureCacheMap = cuckoo_cache::Cache<u256, SignatureCacheHasher>;

impl Default for SignatureCache {
    
    fn default() -> Self {
        todo!();
        /*


            uint256 nonce = GetRandHash();
            // We want the nonce to be 64 bytes long to force the hasher to process
            // this chunk, which makes later hash computations more efficient. We
            // just write our 32-byte entropy, and then pad with 'E' for ECDSA and
            // 'S' for Schnorr (followed by 0 bytes).
            static constexpr unsigned char PADDING_ECDSA[32] = {'E'};
            static constexpr unsigned char PADDING_SCHNORR[32] = {'S'};
            m_salted_hasher_ecdsa.Write(nonce.begin(), 32);
            m_salted_hasher_ecdsa.Write(PADDING_ECDSA, 32);
            m_salted_hasher_schnorr.Write(nonce.begin(), 32);
            m_salted_hasher_schnorr.Write(PADDING_SCHNORR, 32);
        */
    }
}

impl SignatureCache {

    pub fn compute_entryecdsa(&self, 
        entry:   &mut u256,
        hash:    &u256,
        vch_sig: &Vec<u8>,
        pubkey:  &PubKey)  {
        
        todo!();
        /*
            CSHA256 hasher = m_salted_hasher_ecdsa;
            hasher.Write(hash.begin(), 32).Write(pubkey.data(), pubkey.size()).Write(vchSig.data(), vchSig.size()).Finalize(entry.begin());
        */
    }
    
    pub fn compute_entry_schnorr(&self, 
        entry:  &mut u256,
        hash:   &u256,
        sig:    &[u8],
        pubkey: &XOnlyPubKey)  {
        
        todo!();
        /*
            CSHA256 hasher = m_salted_hasher_schnorr;
            hasher.Write(hash.begin(), 32).Write(pubkey.data(), pubkey.size()).Write(sig.data(), sig.size()).Finalize(entry.begin());
        */
    }
    
    pub fn get(&mut self, 
        entry: &u256,
        erase: bool) -> bool {
        
        todo!();
        /*
            std::shared_lock<std::shared_mutex> lock(cs_sigcache);
            return setValid.contains(entry, erase);
        */
    }
    
    pub fn set(&mut self, entry: &u256)  {
        
        todo!();
        /*
            std::unique_lock<std::shared_mutex> lock(cs_sigcache);
            setValid.insert(entry);
        */
    }
    
    pub fn setup_bytes(&mut self, n: usize) -> u32 {
        
        todo!();
        /*
            return setValid.setup_bytes(n);
        */
    }
}

/**
  | In previous versions of this code, signatureCache
  | was a local static variable in CachingTransactionSignatureChecker::VerifySignature.
  | We initialize signatureCache outside
  | of VerifySignature to avoid the atomic
  | operation per call overhead associated
  | with local static variables even though
  | signatureCache could be made local
  | to VerifySignature.
  |
  */
lazy_static!{
    /*
    static CSignatureCache signatureCache;
    */
}

/**
  | To be called once in
  | 
  | AppInitMain/BasicTestingSetup to
  | initialize the signatureCache.
  |
  */
pub fn init_signature_cache()  {
    
    todo!();
        /*
            // nMaxCacheSize is unsigned. If -maxsigcachesize is set to zero,
        // setup_bytes creates the minimum possible cache (2 elements).
        size_t nMaxCacheSize = std::min(std::max((int64_t)0, gArgs.GetIntArg("-maxsigcachesize", DEFAULT_MAX_SIG_CACHE_SIZE) / 2), MAX_MAX_SIG_CACHE_SIZE) * ((size_t) 1 << 20);
        size_t nElems = signatureCache.setup_bytes(nMaxCacheSize);
        LogPrintf("Using %zu MiB out of %zu/2 requested for signature cache, able to store %zu elements\n",
                (nElems*sizeof(uint256)) >>20, (nMaxCacheSize*2)>>20, nElems);
        */
}
