crate::ix!();

/**
  | Specialization of CCoinsViewCursor
  | to iterate over a CCoinsViewDB
  |
  */
pub struct CoinsViewDBCursor<'db> {
    base:    CoinsViewCursor,
    pcursor: Box<DBIterator<'db>>,
    key_tmp: (u8,OutPoint),
}

impl<'db> CoinsViewDBCursor<'db> {

    /**
      | Prefer using CCoinsViewDB::Cursor()
      | since we want to perform some cache warmup
      | on instantiation.
      |
      */
    pub fn new(
        pcursor_in:    *mut DBIterator<'db>,
        hash_block_in: &u256) -> Self {
    
        todo!();
        /*
        : coins_view_cursor(hashBlockIn),
        : pcursor(pcursorIn),

        
        */
    }
    
    pub fn get_key(&self, key: &mut OutPoint) -> bool {
        
        todo!();
        /*
            // Return cached key
        if (keyTmp.first == DB_COIN) {
            key = keyTmp.second;
            return true;
        }
        return false;
        */
    }
    
    pub fn get_value(&self, coin: &mut Coin) -> bool {
        
        todo!();
        /*
            return pcursor->GetValue(coin);
        */
    }
    
    pub fn get_value_size(&self) -> u32 {
        
        todo!();
        /*
            return pcursor->GetValueSize();
        */
    }
    
    pub fn valid(&self) -> bool {
        
        todo!();
        /*
            return keyTmp.first == DB_COIN;
        */
    }
    
    pub fn next(&mut self)  {
        
        todo!();
        /*
            pcursor->Next();
        CoinEntry entry(&keyTmp.second);
        if (!pcursor->Valid() || !pcursor->GetKey(entry)) {
            keyTmp.first = 0; // Invalidate cached key after last record so that Valid() and GetKey() return false
        } else {
            keyTmp.first = entry.key;
        }
        */
    }
}
