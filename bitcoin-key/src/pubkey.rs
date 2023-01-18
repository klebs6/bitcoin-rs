crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/pubkey.h]

pub type ChainCode = u256;

pub fn bip32hash(
        chain_code: &ChainCode,
        n_child:    u32,
        header:     u8,
        data:       [u8; 32],
        output:     [u8; 64])  {
    
    todo!();
        /*
            unsigned char num[4];
        WriteBE32(num, nChild);
        CHMAC_SHA512(chainCode.begin(), chainCode.size()).Write(&header, 1).Write(data, 32).Write(num, 4).Finalize(output);
        */
}

pub const BIP32_EXTKEY_SIZE: usize = 74;

/**
  | A reference to a CKey: the Hash160 of
  | its serialized public key
  |
  */
#[derive(Clone,PartialEq,Eq,Hash)]
pub struct KeyID {
    base: u160,
}

impl Default for KeyID {
    
    fn default() -> Self {
        todo!();
        /*
        : u160(),

        
        */
    }
}

impl From<&u160> for KeyID {
    
    fn from(in_: &u160) -> Self {
    
        todo!();
        /*
        : u160(in),
        */
    }
}

/**
  | An encapsulated public key.
  |
  | Opaque data structure that holds a parsed
  | and valid public key.
  | 
  | The exact representation of data inside
  | is implementation defined and not guaranteed
  | to be portable between different platforms
  | or versions. It is however guaranteed
  | to be 64 bytes in size, and can be safely
  | copied/moved.
  | 
  | If you need to convert to a format suitable
  | for storage or transmission, use ec_pubkey_serialize
  | and ec_pubkey_parse. To compare keys,
  | use ec_pubkey_cmp.
  |
  */
#[derive(Clone,Hash)]
pub struct PubKey {

    /**
      | Just store the serialized data.
      | 
      | Its length can very cheaply be computed
      | from the first byte.
      |
      */
    vch: [u8; PUB_KEY_SIZE],
}

/**
  | secp256k1:
  |
  */
pub const PUB_KEY_SIZE:                   usize = 65;
pub const PUB_KEY_COMPRESSED_SIZE:        usize = 33;
pub const PUB_KEY_SIGNATURE_SIZE:         usize = 72;
pub const PUB_KEY_COMPACT_SIGNATURE_SIZE: usize = 65;

/**
  | see www.keylength.com script supports
  | up to 75 for single byte push
  |
  |
  */
const_assert!{
    PUB_KEY_SIZE >= PUB_KEY_COMPRESSED_SIZE
} // "COMPRESSED_SIZE is larger than SIZE"

impl Default for PubKey {
    
    /**
      | Construct an invalid public key.
      |
      */
    fn default() -> Self {
        todo!();
        /*


            Invalidate();
        */
    }
}

impl Index<u32> for PubKey {

    type Output = u8;
    
    #[inline] fn index(&self, pos: u32) -> &Self::Output {
        todo!();
        /*
            return vch[pos];
        */
    }
}

impl PartialEq<PubKey> for PubKey {
    
    /**
       Comparator implementation.
      */
    #[inline] fn eq(&self, other: &PubKey) -> bool {
        todo!();
        /*
            return a.vch[0] == b.vch[0] &&
                   memcmp(a.vch, b.vch, a.size()) == 0;
        */
    }
}

impl Eq for PubKey {}

impl Ord for PubKey {
    
    #[inline] fn cmp(&self, other: &PubKey) -> Ordering {
        todo!();
        /*
            return a.vch[0] < b.vch[0] ||
                   (a.vch[0] == b.vch[0] && memcmp(a.vch, b.vch, a.size()) < 0);
        */
    }
}

impl PartialOrd<PubKey> for PubKey {
    #[inline] fn partial_cmp(&self, other: &PubKey) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&[u8]> for PubKey {

    /**
      | Construct a public key from a byte vector.
      |
      */
    fn from(vch: &[u8]) -> Self {
    
        todo!();
        /*
            Set(_vch.begin(), _vch.end());
        */
    }
}

impl PubKey {

    /**
      | Compute the length of a pubkey with a
      | given first byte.
      |
      */
    pub fn get_len(ch_header: u8) -> u32 {
        
        todo!();
        /*
            if (chHeader == 2 || chHeader == 3)
                return COMPRESSED_SIZE;
            if (chHeader == 4 || chHeader == 6 || chHeader == 7)
                return SIZE;
            return 0;
        */
    }

    /**
      | Set this key data to be invalid
      |
      */
    pub fn invalidate(&mut self)  {
        
        todo!();
        /*
            vch[0] = 0xFF;
        */
    }
    
    pub fn valid_size(vch: &Vec<u8>) -> bool {
        
        todo!();
        /*
            return vch.size() > 0 && GetLen(vch[0]) == vch.size();
        */
    }

    /**
      | Initialize a public key using begin/end
      | iterators to byte data.
      |
      */
    pub fn set<T>(&mut self, 
        pbegin: T,
        pend:   T)  {
    
        todo!();
        /*
            int len = pend == pbegin ? 0 : GetLen(pbegin[0]);
            if (len && len == (pend - pbegin))
                memcpy(vch, (unsigned char*)&pbegin[0], len);
            else
                Invalidate();
        */
    }

    /**
      | Construct a public key using begin/end
      | iterators to byte data.
      |
      */
    pub fn new_with_iter<T>(
        pbegin: T,
        pend:   T) -> Self {
    
        todo!();
        /*


            Set(pbegin, pend);
        */
    }

    /**
      | Construct a public key using begin/end
      | iterators to byte data.
      |
      */
    pub fn new(slice: &[u8]) -> Self {
    
        todo!();
        /*
            Set(pbegin, pend);
        */
    }

    /**
      | Simple read-only vector-like interface
      | to the pubkey data.
      |
      */
    pub fn size(&self) -> u32 {
        
        todo!();
        /*
            return GetLen(vch[0]);
        */
    }
    
    pub fn data(&self) -> *const u8 {
        
        todo!();
        /*
            return vch;
        */
    }
    
    pub fn begin(&self) -> *const u8 {
        
        todo!();
        /*
            return vch;
        */
    }
    
    pub fn end(&self) -> *const u8 {
        
        todo!();
        /*
            return vch + size();
        */
    }

    /**
      | Implement serialization, as if this
      | was a byte vector.
      |
      */
    pub fn serialize<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            unsigned int len = size();
            ::WriteCompactSize(s, len);
            s.write((char*)vch, len);
        */
    }
    
    
    pub fn unserialize<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            unsigned int len = ::ReadCompactSize(s);
            if (len <= SIZE) {
                s.read((char*)vch, len);
                if (len != size()) {
                    Invalidate();
                }
            } else {
                // invalid pubkey, skip available data
                char dummy;
                while (len--)
                    s.read(&dummy, 1);
                Invalidate();
            }
        */
    }

    /**
      | Get the KeyID of this public key (hash
      | of its serialization)
      |
      */
    pub fn getid(&self) -> KeyID {
        
        todo!();
        /*
            return CKeyID(Hash160(MakeSpan(vch).first(size())));
        */
    }

    /**
      | Get the 256-bit hash of this public key.
      |
      */
    pub fn get_hash(&self) -> u256 {
        
        todo!();
        /*
            return Hash(MakeSpan(vch).first(size()));
        */
    }

    /**
      | Check syntactic correctness.
      | 
      | When setting a pubkey (Set()) or deserializing
      | fails (its header bytes don't match
      | the length of the data), the size is set
      | to 0. Thus, by checking size, one can
      | observe whether Set() or deserialization
      | has failed.
      | 
      | This does not check for more than that.
      | In particular, it does not verify that
      | the coordinates correspond to a point
      | on the curve (see IsFullyValid() for
      | that instead).
      | 
      | -----------
      | @note
      | 
      | this is consensus critical as
      | 
      | CheckECDSASignature() calls it!
      |
      */
    pub fn is_valid(&self) -> bool {
        
        todo!();
        /*
            return size() > 0;
        */
    }

    /**
      | Check whether this is a compressed public
      | key.
      |
      */
    pub fn is_compressed(&self) -> bool {
        
        todo!();
        /*
            return size() == COMPRESSED_SIZE;
        */
    }

    /**
      | Check whether a signature is normalized
      | (lower-S).
      |
      */
    pub fn check_lows(&mut self, vch_sig: &Vec<u8>) -> bool {
        
        todo!();
        /*
            secp256k1_ecdsa_signature sig;
        assert(secp256k1_context_verify && "secp256k1_context_verify must be initialized to use CPubKey.");
        if (!ecdsa_signature_parse_der_lax(secp256k1_context_verify, &sig, vchSig.data(), vchSig.size())) {
            return false;
        }
        return (!secp256k1_ecdsa_signature_normalize(secp256k1_context_verify, nullptr, &sig));
        */
    }

    /**
      | Verify a DER signature (~72 bytes).
      | 
      | If this public key is not fully valid,
      | the return value will be false.
      |
      */
    pub fn verify(&self, 
        hash:    &u256,
        vch_sig: &Vec<u8>) -> bool {
        
        todo!();
        /*
            if (!IsValid())
            return false;
        secp256k1_pubkey pubkey;
        secp256k1_ecdsa_signature sig;
        assert(secp256k1_context_verify && "secp256k1_context_verify must be initialized to use CPubKey.");
        if (!secp256k1_ec_pubkey_parse(secp256k1_context_verify, &pubkey, vch, size())) {
            return false;
        }
        if (!ecdsa_signature_parse_der_lax(secp256k1_context_verify, &sig, vchSig.data(), vchSig.size())) {
            return false;
        }
        /* libsecp256k1's ECDSA verification requires lower-S signatures, which have
         * not historically been enforced in Bitcoin, so normalize them first. */
        secp256k1_ecdsa_signature_normalize(secp256k1_context_verify, &sig, &sig);
        return secp256k1_ecdsa_verify(secp256k1_context_verify, &sig, hash.begin(), &pubkey);
        */
    }
    
    /**
      | Recover a public key from a compact signature.
      |
      */
    pub fn recover_compact(&mut self, 
        hash:    &u256,
        vch_sig: &Vec<u8>) -> bool {
        
        todo!();
        /*
            if (vchSig.size() != COMPACT_SIGNATURE_SIZE)
            return false;
        int recid = (vchSig[0] - 27) & 3;
        bool fComp = ((vchSig[0] - 27) & 4) != 0;
        secp256k1_pubkey pubkey;
        secp256k1_ecdsa_recoverable_signature sig;
        assert(secp256k1_context_verify && "secp256k1_context_verify must be initialized to use CPubKey.");
        if (!secp256k1_ecdsa_recoverable_signature_parse_compact(secp256k1_context_verify, &sig, &vchSig[1], recid)) {
            return false;
        }
        if (!secp256k1_ecdsa_recover(secp256k1_context_verify, &pubkey, &sig, hash.begin())) {
            return false;
        }
        unsigned char pub[SIZE];
        size_t publen = SIZE;
        secp256k1_ec_pubkey_serialize(secp256k1_context_verify, pub, &publen, &pubkey, fComp ? SECP256K1_EC_COMPRESSED : SECP256K1_EC_UNCOMPRESSED);
        Set(pub, pub + publen);
        return true;
        */
    }
    
    /**
      | fully validate whether this is a valid
      | public key (more expensive than IsValid())
      |
      */
    pub fn is_fully_valid(&self) -> bool {
        
        todo!();
        /*
            if (!IsValid())
            return false;
        secp256k1_pubkey pubkey;
        assert(secp256k1_context_verify && "secp256k1_context_verify must be initialized to use CPubKey.");
        return secp256k1_ec_pubkey_parse(secp256k1_context_verify, &pubkey, vch, size());
        */
    }
    
    /**
      | Turn this public key into an uncompressed
      | public key.
      |
      */
    pub fn decompress(&mut self) -> bool {
        
        todo!();
        /*
            if (!IsValid())
            return false;
        secp256k1_pubkey pubkey;
        assert(secp256k1_context_verify && "secp256k1_context_verify must be initialized to use CPubKey.");
        if (!secp256k1_ec_pubkey_parse(secp256k1_context_verify, &pubkey, vch, size())) {
            return false;
        }
        unsigned char pub[SIZE];
        size_t publen = SIZE;
        secp256k1_ec_pubkey_serialize(secp256k1_context_verify, pub, &publen, &pubkey, SECP256K1_EC_UNCOMPRESSED);
        Set(pub, pub + publen);
        return true;
        */
    }
    
    /**
      | Derive BIP32 child pubkey.
      |
      */
    pub fn derive(&self, 
        pubkey_child: &mut PubKey,
        cc_child:     &mut ChainCode,
        n_child:      u32,
        cc:           &ChainCode) -> bool {
        
        todo!();
        /*
            assert(IsValid());
        assert((nChild >> 31) == 0);
        assert(size() == COMPRESSED_SIZE);
        unsigned char out[64];
        BIP32Hash(cc, nChild, *begin(), begin()+1, out);
        memcpy(ccChild.begin(), out+32, 32);
        secp256k1_pubkey pubkey;
        assert(secp256k1_context_verify && "secp256k1_context_verify must be initialized to use CPubKey.");
        if (!secp256k1_ec_pubkey_parse(secp256k1_context_verify, &pubkey, vch, size())) {
            return false;
        }
        if (!secp256k1_ec_pubkey_tweak_add(secp256k1_context_verify, &pubkey, out)) {
            return false;
        }
        unsigned char pub[COMPRESSED_SIZE];
        size_t publen = COMPRESSED_SIZE;
        secp256k1_ec_pubkey_serialize(secp256k1_context_verify, pub, &publen, &pubkey, SECP256K1_EC_COMPRESSED);
        pubkeyChild.Set(pub, pub + publen);
        return true;
        */
    }
}


///--------------------------
#[derive(Default)]
pub struct XOnlyPubKey {
    keydata: u256,
}

impl Index<i32> for XOnlyPubKey {
    type Output = u8;
    
    #[inline] fn index(&self, pos: i32) -> &Self::Output {
        todo!();
        /*
            return *(m_keydata.begin() + pos);
        */
    }
}

impl PartialEq<XOnlyPubKey> for XOnlyPubKey {
    
    #[inline] fn eq(&self, other: &XOnlyPubKey) -> bool {
        todo!();
        /*
            return m_keydata == other.m_keydata;
        */
    }
}

impl Eq for XOnlyPubKey {}

impl Ord for XOnlyPubKey {
    
    #[inline] fn cmp(&self, other: &XOnlyPubKey) -> Ordering {
        todo!();
        /*
            return m_keydata < other.m_keydata;
        */
    }
}

impl PartialOrd<XOnlyPubKey> for XOnlyPubKey {
    #[inline] fn partial_cmp(&self, other: &XOnlyPubKey) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&PubKey> for XOnlyPubKey {

    /**
      | Construct an x-only pubkey from a normal
      | pubkey.
      |
      */
    fn from(pubkey: &PubKey) -> Self {
    
        todo!();
        /*
            : XOnlyPubKey(Span<const unsigned char>(pubkey.begin() + 1, pubkey.begin() + 33))
        */
    }
}

impl From<&[u8]> for XOnlyPubKey {

    /**
      | Construct an x-only pubkey from exactly
      | 32 bytes.
      |
      */
    fn from(bytes: &[u8]) -> Self {
    
        todo!();
        /*


            assert(bytes.size() == 32);
        std::copy(bytes.begin(), bytes.end(), m_keydata.begin());
        */
    }
}

impl XOnlyPubKey {

    /**
      | Test whether this is the 0 key (the result
      | of default construction). This implies
      | !IsFullyValid().
      |
      */
    pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            return m_keydata.IsNull();
        */
    }

    pub fn data(&self) -> *const u8 {
        
        todo!();
        /*
            return m_keydata.begin();
        */
    }
    
    pub fn size() -> usize {
        
        todo!();
        /*
            return decltype(m_keydata)::size();
        */
    }
    
    pub fn begin(&self) -> *const u8 {
        
        todo!();
        /*
            return m_keydata.begin();
        */
    }
    
    pub fn end(&self) -> *const u8 {
        
        todo!();
        /*
            return m_keydata.end();
        */
    }
    
    pub fn begin_mut(&mut self) -> *mut u8 {
        
        todo!();
        /*
            return m_keydata.begin();
        */
    }
    
    pub fn end_mut(&mut self) -> *mut u8 {
        
        todo!();
        /*
            return m_keydata.end();
        */
    }

    /**
      | Returns a list of CKeyIDs for the CPubKeys
      | that could have been used to create this
      | 
      | XOnlyPubKey.
      | 
      | This is needed for key lookups since
      | keys are indexed by CKeyID.
      |
      */
    pub fn get_key_ids(&self) -> Vec<KeyID> {
        
        todo!();
        /*
            std::vector<CKeyID> out;
        // For now, use the old full pubkey-based key derivation logic. As it is indexed by
        // Hash160(full pubkey), we need to return both a version prefixed with 0x02, and one
        // with 0x03.
        unsigned char b[33] = {0x02};
        std::copy(m_keydata.begin(), m_keydata.end(), b + 1);
        CPubKey fullpubkey;
        fullpubkey.Set(b, b + 33);
        out.push_back(fullpubkey.GetID());
        b[0] = 0x03;
        fullpubkey.Set(b, b + 33);
        out.push_back(fullpubkey.GetID());
        return out;
        */
    }
    
    /**
      | Determine if this pubkey is fully valid.
      | This is true for approximately 50% of
      | all possible 32-byte arrays. If false,
      | 
      | VerifySchnorr and CreatePayToContract
      | will always fail.
      |
      */
    pub fn is_fully_valid(&self) -> bool {
        
        todo!();
        /*
            secp256k1_xonly_pubkey pubkey;
        return secp256k1_xonly_pubkey_parse(secp256k1_context_verify, &pubkey, m_keydata.data());
        */
    }
    
    /**
      | Verify a Schnorr signature against
      | this public key. sigbytes must be exactly
      | 64 bytes.
      |
      */
    pub fn verify_schnorr(&self, 
        msg:      &u256,
        sigbytes: &[u8]) -> bool {
        
        todo!();
        /*
            assert(sigbytes.size() == 64);
        secp256k1_xonly_pubkey pubkey;
        if (!secp256k1_xonly_pubkey_parse(secp256k1_context_verify, &pubkey, m_keydata.data())) return false;
        return secp256k1_schnorrsig_verify(secp256k1_context_verify, sigbytes.data(), msg.begin(), 32, &pubkey);
        */
    }
    
    /**
      | Compute the Taproot tweak as specified
      | in BIP341, with *this as internal key:
      | 
      | - if merkle_root == nullptr: H_TapTweak(xonly_pubkey)
      | 
      | - otherwise: H_TapTweak(xonly_pubkey || *merkle_root)
      | 
      | -----------
      | @note
      | 
      | the behavior of this function with merkle_root
      | != nullptr is consensus critical.
      |
      */
    pub fn compute_tap_tweak_hash(&self, merkle_root: *const u256) -> u256 {
        
        todo!();
        /*
            if (merkle_root == nullptr) {
            // We have no scripts. The actual tweak does not matter, but follow BIP341 here to
            // allow for reproducible tweaking.
            return (CHashWriter(HASHER_TAPTWEAK) << m_keydata).GetSHA256();
        } else {
            return (CHashWriter(HASHER_TAPTWEAK) << m_keydata << *merkle_root).GetSHA256();
        }
        */
    }
    
    /**
      | Verify that this is a Taproot tweaked
      | output point, against a specified internal
      | key,
      | 
      | Merkle root, and parity.
      |
      */
    pub fn check_tap_tweak(&self, 
        internal:    &XOnlyPubKey,
        merkle_root: &u256,
        parity:      bool) -> bool {
        
        todo!();
        /*
            secp256k1_xonly_pubkey internal_key;
        if (!secp256k1_xonly_pubkey_parse(secp256k1_context_verify, &internal_key, internal.data())) return false;
        uint256 tweak = internal.ComputeTapTweakHash(&merkle_root);
        return secp256k1_xonly_pubkey_tweak_add_check(secp256k1_context_verify, m_keydata.begin(), parity, &internal_key, tweak.begin());
        */
    }
    
    /**
      | Construct a Taproot tweaked output
      | point with this point as internal key.
      |
      */
    pub fn create_tap_tweak(&self, merkle_root: *const u256) -> Option<(XOnlyPubKey,bool)> {
        
        todo!();
        /*
            secp256k1_xonly_pubkey base_point;
        if (!secp256k1_xonly_pubkey_parse(secp256k1_context_verify, &base_point, data())) return std::nullopt;
        secp256k1_pubkey out;
        uint256 tweak = ComputeTapTweakHash(merkle_root);
        if (!secp256k1_xonly_pubkey_tweak_add(secp256k1_context_verify, &out, &base_point, tweak.data())) return std::nullopt;
        int parity = -1;
        std::pair<XOnlyPubKey, bool> ret;
        secp256k1_xonly_pubkey out_xonly;
        if (!secp256k1_xonly_pubkey_from_pubkey(secp256k1_context_verify, &out_xonly, &parity, &out)) return std::nullopt;
        secp256k1_xonly_pubkey_serialize(secp256k1_context_verify, ret.first.begin(), &out_xonly);
        assert(parity == 0 || parity == 1);
        ret.second = parity;
        return ret;
        */
    }
}

///------------------------
pub struct ExtPubKey {
    n_depth:         u8,
    vch_fingerprint: [u8; 4],
    n_child:         u32,
    chaincode:       ChainCode,
    pubkey:          PubKey,
}

impl PartialEq<ExtPubKey> for ExtPubKey {
    
    #[inline] fn eq(&self, other: &ExtPubKey) -> bool {
        todo!();
        /*
            return a.nDepth == b.nDepth &&
                memcmp(a.vchFingerprint, b.vchFingerprint, sizeof(vchFingerprint)) == 0 &&
                a.nChild == b.nChild &&
                a.chaincode == b.chaincode &&
                a.pubkey == b.pubkey;
        */
    }
}

impl Eq for ExtPubKey {}

impl ExtPubKey {

    pub fn encode(&self, code: [u8; BIP32_EXTKEY_SIZE])  {
        
        todo!();
        /*
            code[0] = nDepth;
        memcpy(code+1, vchFingerprint, 4);
        WriteBE32(code+5, nChild);
        memcpy(code+9, chaincode.begin(), 32);
        assert(pubkey.size() == CPubKey::COMPRESSED_SIZE);
        memcpy(code+41, pubkey.begin(), CPubKey::COMPRESSED_SIZE);
        */
    }
    
    pub fn decode(&mut self, code: [u8; BIP32_EXTKEY_SIZE])  {
        
        todo!();
        /*
            nDepth = code[0];
        memcpy(vchFingerprint, code+1, 4);
        nChild = ReadBE32(code+5);
        memcpy(chaincode.begin(), code+9, 32);
        pubkey.Set(code+41, code+BIP32_EXTKEY_SIZE);
        if ((nDepth == 0 && (nChild != 0 || ReadLE32(vchFingerprint) != 0)) || !pubkey.IsFullyValid()) pubkey = CPubKey();
        */
    }
    
    pub fn derive(&self, 
        out:     &mut ExtPubKey,
        n_child: u32) -> bool {
        
        todo!();
        /*
            out.nDepth = nDepth + 1;
        CKeyID id = pubkey.GetID();
        memcpy(out.vchFingerprint, &id, 4);
        out.nChild = _nChild;
        return pubkey.Derive(out.pubkey, out.chaincode, _nChild, chaincode);
        */
    }
}

/**
  | Users of this module must hold an
  | 
  | ECCVerifyHandle. The constructor
  | and destructor of these are not allowed
  | to run in parallel, though.
  |
  */
pub struct ECCVerifyHandle {}

pub mod ecc_verify_handle {

    use super::*;

    lazy_static!{
        /*
        static int refcount;
        int ECCVerifyHandle::refcount = 0;
        */
    }
}

//-------------------------------------------[.cpp/bitcoin/src/pubkey.cpp]

/**
  | Global secp256k1_context object used
  | for verification.
  |
  */
lazy_static!{
    /*
    secp256k1_context* secp256k1_context_verify = nullptr;
    */
}

///-------------------------
impl Default for ECCVerifyHandle {
    
    fn default() -> Self {
    
        todo!();
        /*
        if (refcount == 0) {
            assert(secp256k1_context_verify == nullptr);
            secp256k1_context_verify = secp256k1_context_create(SECP256K1_CONTEXT_VERIFY);
            assert(secp256k1_context_verify != nullptr);
        }
        refcount++;
        */
    }
}

impl Drop for ECCVerifyHandle {

    fn drop(&mut self) {
        todo!();
        /*
            refcount--;
        if (refcount == 0) {
            assert(secp256k1_context_verify != nullptr);
            secp256k1_context_destroy(secp256k1_context_verify);
            secp256k1_context_verify = nullptr;
        }
        */
    }
}
