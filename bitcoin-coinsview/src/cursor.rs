// ---------------- [ File: bitcoin-coinsview/src/cursor.rs ]
crate::ix!();

/**
  | Cursor for iterating over CoinsView
  | state
  |
  */
pub struct CoinsViewCursor {
    hash_block: u256,
}

pub mod coins_view_cursor {
    use super::*;

    pub trait Interface {
        fn get_key(&self, key: &mut OutPoint) -> bool;
        fn get_value(&self, coin: &mut Coin) -> bool;
        fn get_value_size(&self) -> u32;
        fn valid(&self) -> bool;
        fn next(&mut self);
    }
}

impl From<&u256> for CoinsViewCursor {

    fn from(hash_block_in: &u256) -> Self {
    
        todo!();
        /*
        : hash_block(hashBlockIn),
        */
    }
}

impl CoinsViewCursor {

    /**
      | Get best block at the time this cursor
      | was created
      |
      */
    pub fn get_best_block(&self) -> &u256 {
        
        todo!();
        /*
            return hashBlock;
        */
    }
}
