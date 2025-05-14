// ---------------- [ File: bitcoin-coinsview/src/interface.rs ]
crate::ix!();

/**
  | Abstract view on the open txout dataset.
  |
  */
pub trait CoinsView:
GetCoin
+ HaveCoin
+ GetBestBlock
+ GetHeadBlocks
+ BatchWrite
+ Cursor
+ EstimateSize { }

//-------------------------------------------[.cpp/bitcoin/src/coins.cpp]

pub trait GetCoin {

    /**
      | Retrieve the Coin (unspent transaction
      | output) for a given outpoint.
      | 
      | Returns true only when an unspent coin
      | was found, which is returned in coin.
      | 
      | When false is returned, coin's value
      | is unspecified.
      |
      */
    fn get_coin(&self, 
        outpoint: &OutPoint,
        coin:     &mut Coin) -> bool { false }
}

pub trait HaveCoin: GetCoin {

    /**
      | Just check whether a given outpoint
      | is unspent.
      |
      */
    fn have_coin(&self, outpoint: &OutPoint) -> bool {

        let mut coin = Coin::default();
        self.get_coin(outpoint, &mut coin)
    }
}

pub trait GetBestBlock {

    /**
      | Retrieve the block hash whose state
      | this
      | 
      | CoinsView currently represents
      |
      */
    fn get_best_block(&self) -> u256 {
        u256::ZERO
    }
}

pub trait GetHeadBlocks {

    /**
      | Retrieve the range of blocks that may have
      | been only partially written.
      |
      | If the database is in a consistent state,
      | the result is the empty vector.
      |
      | Otherwise, a two-element vector is returned
      | consisting of the new and the old block
      | hash, in that order.
      */
    fn get_head_blocks(&self) -> Vec<u256> {
        vec!{}
    }
}

pub trait BatchWrite {

    /**
      | Do a bulk modification (multiple Coin
      | changes + BestBlock change).
      |
      | The passed mapCoins can be modified.
      */
    fn batch_write(&mut self, 
            map_coins:  &mut CoinsMap,
            hash_block: &u256) -> bool { false }
}

pub trait Cursor {

    /**
      | Get a cursor to iterate over the whole
      | state
      |
      */
    fn cursor(&self) -> Option<Box<CoinsViewCursor>> {
        None
    }
}

pub trait EstimateSize {

    /**
      | Estimate database size (0 if not implemented)
      |
      */
    fn estimate_size(&self) -> usize { 0 }
}
