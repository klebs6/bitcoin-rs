crate::ix!();

/**
  | CoinsView backed by another CoinsView
  |
  */
pub struct CoinsViewBacked {
    impls: Box<dyn CoinsView>,
    base: *mut dyn CoinsView,
}

impl From<*mut dyn CoinsView> for CoinsViewBacked {

    fn from(view_in: *mut dyn CoinsView) -> Self {
    
        todo!();
        /*
        : base(viewIn),
        */
    }
}

impl CoinsViewBacked {

    fn set_backend(&mut self, view_in: &mut dyn CoinsView)  {
        
        todo!();
        /*
            base = &viewIn;
        */
    }
}
    
impl GetCoin for CoinsViewBacked {

    fn get_coin(&self, 
        outpoint: &OutPoint,
        coin:     &mut Coin) -> bool {
        
        todo!();
        /*
            return base->GetCoin(outpoint, coin);
        */
    }
}
    
impl HaveCoin for CoinsViewBacked {
    fn have_coin(&self, outpoint: &OutPoint) -> bool {
        
        todo!();
        /*
            return base->HaveCoin(outpoint);
        */
    }
}
    
impl GetBestBlock for CoinsViewBacked {
    fn get_best_block(&self) -> u256 {
        
        todo!();
        /*
            return base->GetBestBlock();
        */
    }
}
    
impl GetHeadBlocks for CoinsViewBacked {
    fn get_head_blocks(&self) -> Vec<u256> {
        
        todo!();
        /*
            return base->GetHeadBlocks();
        */
    }
}
    
impl BatchWrite for CoinsViewBacked {
    fn batch_write(&mut self, 
        map_coins:  &mut CoinsMap,
        hash_block: &u256) -> bool {
        
        todo!();
        /*
            return base->BatchWrite(mapCoins, hashBlock);
        */
    }
}
    
impl Cursor for CoinsViewBacked {
    fn cursor(&self) -> Option<Box<CoinsViewCursor>> {
        
        todo!();
        /*
            return base->Cursor();
        */
    }
}
    
impl EstimateSize for CoinsViewBacked {
    fn estimate_size(&self) -> usize {
        
        todo!();
        /*
            return base->EstimateSize();
        */
    }
}

