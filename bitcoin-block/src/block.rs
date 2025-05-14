// ---------------- [ File: bitcoin-block/src/block.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/primitives/block.h]
//-------------------------------------------[.cpp/bitcoin/src/primitives/block.cpp]

pub type BlockMap = HashMap<u256,*mut BlockIndex,BlockHasher>;

#[derive(Serialize,Deserialize)]
pub struct Block {

    pub header: BlockHeader,

    /**
      | network and disk
      |
      */
    pub vtx:     Vec<TransactionRef>,

    /**
      | memory only
      |
      */
    #[serde(skip)]
    pub checked: RefCell<bool>,
}

impl RecursiveDynamicUsage for Block {

    fn recursive_dynamic_usage(&self) -> usize {
        
        todo!();
            /*
            size_t mem = memusage::DynamicUsage(block.vtx);
            for (const auto& tx : block.vtx) {
                mem += memusage::DynamicUsage(tx) + RecursiveDynamicUsage(*tx);
            }
            return mem;
            */
    }
}

unsafe impl Send for Block {}
unsafe impl Sync for Block {}

impl Default for Block {
    
    fn default() -> Self {

        let mut x: Self = unsafe { std::mem::zeroed() };

        x.set_null();

        x
    }
}

impl Block {
    
    pub fn new(header: &BlockHeader) -> Self {
    
        let mut x: Self = unsafe { std::mem::zeroed() };

        x.set_null();
        x.header = header.clone();

        x
    }

    delegate!{
        to self.header {
            pub fn get_hash(&self) -> u256;
        }
    }

    pub fn set_null(&mut self)  {
        
        self.header.set_null();
        self.vtx.clear();
        self.checked.replace(false);
    }
    
    pub fn get_block_header<'a>(&'a self) -> &'a BlockHeader {
        &self.header
    }
    
    pub fn to_string(&self) -> String {
        
        let mut s = String::default();

        let mut s = format!{
            "CBlock(hash={}, ver=0x{:08x}, hashPrevBlock={}, hashMerkleRoot={}, nTime={}, nBits={:08x}, nNonce={}, vtx={})\n",
            self.header.get_hash().to_string(),
            self.header.n_version,
            self.header.hash_prev_block.to_string(),
            self.header.hash_merkle_root.to_string(),
            self.header.n_time,
            self.header.n_bits,
            self.header.n_nonce,
            self.vtx.len()
        };

        for tx in self.vtx.iter() {
            s.push_str("  ");
            s.push_str(&tx.get().to_string());
            s.push_str("\n");
        }

         s
    }
}

pub fn decode_hex_blk(
        _0:          &mut Block,
        str_hex_blk: &String) -> bool {
    
    todo!();
        /*
        
        */
}
