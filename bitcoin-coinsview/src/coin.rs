// ---------------- [ File: bitcoin-coinsview/src/coin.rs ]
crate::ix!();

/**
  | A UTXO entry.
  | 
  | Serialized format:
  | 
  | - VARINT((coinbase ? 1 : 0) | (height << 1))
  | 
  | - the non-spent CTxOut (via TxOutCompression)
  |
  */
pub struct Coin {

    /**
      | unspent transaction output
      |
      */
    pub out:  TxOut,
    pub bits: CoinBitfield,
}

lazy_static!{
    pub static ref COIN_EMPTY: Coin = Coin::empty();
}

impl PartialEq<Coin> for Coin {
    
    #[inline] fn eq(&self, other: &Coin) -> bool {

        /*
           | Empty Coin objects are always equal.
           |
           */
        if self.is_spent() && other.is_spent() {
            return true;
        }

        self.bits.coinbase()        
            == other.bits.coinbase() 

            && self.bits.n_height() 
            == other.bits.n_height() 

            && self.out      
            == other.out
    }
}

impl Eq for Coin {}

impl Default for Coin {
    
    /**
      | empty constructor
      |
      */
    fn default() -> Self {
        Self::empty()
    }
}

impl Clone for Coin {
    
    fn clone(&self) -> Self {
    
        todo!();
        /*
        : n_height(in.nHeight),
        : out(std::move(in.out)),
        */
    }
}

impl Coin {

    fn empty() -> Self {
        Self {
            out:  TxOut::new(),
            bits: CoinBitfield::from_fields(0,false),
        }
    }
    
    /**
      | construct a Coin from a TxOut and height/coinbase
      | information.
      |
      */
    pub fn new(
        out_in:       &TxOut,
        n_height_in:  i32,
        coin_base_in: bool) -> Self {
    
        todo!();
        /*
        : out(outIn),
        : coin_base(fCoinBaseIn),
        : n_height(nHeightIn),
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            out.SetNull();
            fCoinBase = false;
            nHeight = 0;
        */
    }
    
    pub fn is_coinbase(&self) -> bool {
        
        todo!();
        /*
            return fCoinBase;
        */
    }
    
    pub fn serialize<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            assert(!IsSpent());
                uint32_t code = nHeight * uint32_t{2} + fCoinBase;
                ::Serialize(s, VARINT(code));
                ::Serialize(s, Using<TxOutCompression>(out));
        */
    }
    
    pub fn unserialize<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            uint32_t code = 0;
                ::Unserialize(s, VARINT(code));
                nHeight = code >> 1;
                fCoinBase = code & 1;
                ::Unserialize(s, Using<TxOutCompression>(out));
        */
    }

    /**
      | Either this coin never existed (see
      | e.g. coinEmpty in coins.cpp), or it
      | did exist and has been spent.
      |
      */
    pub fn is_spent(&self) -> bool {
        
        todo!();
        /*
            return out.IsNull();
        */
    }
    
    pub fn dynamic_memory_usage(&self) -> usize {
        
        todo!();
        /*
            return memusage::DynamicUsage(out.scriptPubKey);
        */
    }
}
