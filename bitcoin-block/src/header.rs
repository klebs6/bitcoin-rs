// ---------------- [ File: bitcoin-block/src/header.rs ]
crate::ix!();

/**
  | Nodes collect new transactions into a block,
  | hash them into a hash tree, and scan through
  | nonce values to make the block's hash satisfy
  | proof-of-work requirements. 
  |
  | When they solve the proof-of-work, they
  | broadcast the block to everyone and the block
  | is added to the block chain.
  |
  | The first transaction in the block is
  | a special one that creates a new coin owned by
  | the creator of the block.
  |
  */
#[derive(PartialEq,Eq,Serialize,Deserialize,Clone)]
pub struct BlockHeader {

    /** --- header --- */
    pub n_version:        i32,
    pub hash_prev_block:  u256,
    pub hash_merkle_root: u256,
    pub n_time:           u32,
    pub n_bits:           u32,
    pub n_nonce:          u32,
}

impl Default for BlockHeader {
    
    fn default() -> Self {
        let mut x: Self = unsafe { std::mem::zeroed() };
        x.set_null();
        x
    }
}

impl BlockHeader {

    pub fn set_null(&mut self)  {
        
        self.n_version = 0;
        self.hash_prev_block.set_null();
        self.hash_merkle_root.set_null();
        self.n_time  = 0;
        self.n_bits  = 0;
        self.n_nonce = 0;
    }
    
    pub fn is_null(&self) -> bool {
        
        self.n_bits == 0
    }
    
    pub fn get_block_time(&self) -> i64 {
        
        self.n_time as i64
    }
    
    pub fn get_hash(&self) -> u256 {
        
        serialize_hash(self, None, None)
    }
}

pub fn decode_hex_block_header(
        _0:         &mut BlockHeader,
        hex_header: &String) -> bool {
    
    todo!();
        /*
        
        */
}
