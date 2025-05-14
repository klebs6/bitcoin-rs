// ---------------- [ File: bitcoin-coinsview/src/bitfield.rs ]
crate::ix!();

#[bitfield]
pub struct CoinBitfield {

    /**
      | whether containing transaction was
      | a coinbase
      |
      */
    pub coinbase: B1,

    /**
      | at which height this containing transaction
      | was included in the active block chain
      |
      */
    pub n_height:  B31,
}

impl CoinBitfield {

    pub fn from_fields(
        n_height_in:  i32,
        coinbase_in: bool) -> Self {

        CoinBitfield::new()
            .with_coinbase(coinbase_in as u8)
            .with_n_height(n_height_in as u32)
    }
}
