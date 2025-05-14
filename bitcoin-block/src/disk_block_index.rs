// ---------------- [ File: bitcoin-block/src/disk_block_index.rs ]
crate::ix!();

/**
  | Used to marshal pointers into hashes
  | for db storage.
  |
  */
pub struct DiskBlockIndex {
    pub index:     BlockIndex,
    pub hash_prev: u256,
}

impl Default for DiskBlockIndex {
    
    fn default() -> Self {
        Self {
            index:     Default::default(),
            hash_prev: u256::default(),
        }
    }
}

impl DiskBlockIndex {

    lazy_static!{
        /*
        SERIALIZE_METHODS(CDiskBlockIndex, obj)
            {
                int _nVersion = s.GetVersion();
                if (!(s.GetType() & SER_GETHASH)) READWRITE(VARINT_MODE(_nVersion, VarIntMode::NONNEGATIVE_SIGNED));

                READWRITE(VARINT_MODE(obj.nHeight, VarIntMode::NONNEGATIVE_SIGNED));
                READWRITE(VARINT(obj.nStatus));
                READWRITE(VARINT(obj.nTx));
                if (obj.nStatus & (BLOCK_HAVE_DATA | BLOCK_HAVE_UNDO)) READWRITE(VARINT_MODE(obj.nFile, VarIntMode::NONNEGATIVE_SIGNED));
                if (obj.nStatus & BLOCK_HAVE_DATA) READWRITE(VARINT(obj.nDataPos));
                if (obj.nStatus & BLOCK_HAVE_UNDO) READWRITE(VARINT(obj.nUndoPos));

                // block header
                READWRITE(obj.nVersion);
                READWRITE(obj.hashPrev);
                READWRITE(obj.hashMerkleRoot);
                READWRITE(obj.nTime);
                READWRITE(obj.nBits);
                READWRITE(obj.nNonce);
            }
        */
    }

    pub fn new(pindex: *const BlockIndex) -> Self {

        unsafe {
            let mut x: Self = std::mem::zeroed();

            x.index = (*pindex).clone();

            x.hash_prev = match x.index.pprev.is_some() {
                true   => x.index.pprev.as_ref().unwrap().get_block_hash(),
                false  => u256::default()
            };

            x
        }
    }
    
    pub fn get_block_hash(&self) -> u256 {
        
        let mut block = BlockHeader::default();

        block.n_version        = self.index.n_version;
        block.hash_prev_block  = self.hash_prev.clone();
        block.hash_merkle_root = self.index.hash_merkle_root.clone();
        block.n_time           = self.index.n_time;
        block.n_bits           = self.index.n_bits;
        block.n_nonce          = self.index.n_nonce;

        block.get_hash()
    }

    pub fn to_string(&self) -> String {

        let mut str_: String = "CDiskBlockIndex(".to_string();
        str_.push_str(&self.index.to_string());

        str_.push_str(
            &format!{
                "\n                hashBlock={}, hashPrev={})",
                self.get_block_hash().to_string(),
                self.hash_prev.to_string()
            }
        );

        str_
    }
}
