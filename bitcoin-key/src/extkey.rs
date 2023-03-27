crate::ix!();

pub struct ExtKey {
    n_depth:         u8,
    vch_fingerprint: [u8; 4],
    n_child:         u32,
    chaincode:       ChainCode,
    key:             Key,
}

impl PartialEq<ExtKey> for ExtKey {
    
    #[inline] fn eq(&self, other: &ExtKey) -> bool {
        todo!();
        /*
            return a.nDepth == b.nDepth &&
                memcmp(a.vchFingerprint, b.vchFingerprint, sizeof(vchFingerprint)) == 0 &&
                a.nChild == b.nChild &&
                a.chaincode == b.chaincode &&
                a.key == b.key;
        */
    }
}

impl Eq for ExtKey {}

impl ExtKey {

    pub fn derive(&self, 
        out:     &mut ExtKey,
        n_child: u32) -> bool {
        
        todo!();
        /*
            out.nDepth = nDepth + 1;
        CKeyID id = key.GetPubKey().GetID();
        memcpy(out.vchFingerprint, &id, 4);
        out.nChild = _nChild;
        return key.Derive(out.key, out.chaincode, _nChild, chaincode);
        */
    }
    
    pub fn set_seed(&mut self, 
        seed:       *const u8,
        n_seed_len: u32)  {
        
        todo!();
        /*
            static const unsigned char hashkey[] = {'B','i','t','c','o','i','n',' ','s','e','e','d'};
        std::vector<unsigned char, secure_allocator<unsigned char>> vout(64);
        CHMAC_SHA512(hashkey, sizeof(hashkey)).Write(seed, nSeedLen).Finalize(vout.data());
        key.Set(vout.data(), vout.data() + 32, true);
        memcpy(chaincode.begin(), vout.data() + 32, 32);
        nDepth = 0;
        nChild = 0;
        memset(vchFingerprint, 0, sizeof(vchFingerprint));
        */
    }
    
    pub fn neuter(&self) -> ExtPubKey {
        
        todo!();
        /*
            CExtPubKey ret;
        ret.nDepth = nDepth;
        memcpy(ret.vchFingerprint, vchFingerprint, 4);
        ret.nChild = nChild;
        ret.pubkey = key.GetPubKey();
        ret.chaincode = chaincode;
        return ret;
        */
    }
    
    pub fn encode(&self, code: [u8; BIP32_EXTKEY_SIZE])  {
        
        todo!();
        /*
            code[0] = nDepth;
        memcpy(code+1, vchFingerprint, 4);
        WriteBE32(code+5, nChild);
        memcpy(code+9, chaincode.begin(), 32);
        code[41] = 0;
        assert(key.size() == 32);
        memcpy(code+42, key.begin(), 32);
        */
    }
    
    pub fn decode(&mut self, code: [u8; BIP32_EXTKEY_SIZE])  {
        
        todo!();
        /*
            nDepth = code[0];
        memcpy(vchFingerprint, code+1, 4);
        nChild = ReadBE32(code+5);
        memcpy(chaincode.begin(), code+9, 32);
        key.Set(code+42, code+BIP32_EXTKEY_SIZE, true);
        if ((nDepth == 0 && (nChild != 0 || ReadLE32(vchFingerprint) != 0)) || code[41] != 0) key = CKey();
        */
    }
}
