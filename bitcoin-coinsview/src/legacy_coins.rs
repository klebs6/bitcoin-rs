crate::ix!();

/**
  | Legacy class to deserialize pre-pertxout
  | database entries without reindex.
  |
  */
pub struct LegacyCoins {

    /**
      | whether transaction is a coinbase
      |
      */
    coin_base: bool,

    /**
      | unspent transaction outputs; spent
      | outputs are .IsNull(); spent outputs
      | at the end of the array are dropped
      |
      */
    vout:      Vec<TxOut>,

    /**
      | at which height this transaction was
      | included in the active block chain
      |
      */
    n_height:  i32,
}

impl Default for LegacyCoins {
    
    /**
      | empty constructor
      |
      */
    fn default() -> Self {
        todo!();
        /*
        : coin_base(false),
        : vout(0),
        : n_height(0),

        
        */
    }
}

impl LegacyCoins {
    
    pub fn unserialize<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            unsigned int nCode = 0;
            // version
            unsigned int nVersionDummy;
            ::Unserialize(s, VARINT(nVersionDummy));
            // header code
            ::Unserialize(s, VARINT(nCode));
            fCoinBase = nCode & 1;
            std::vector<bool> vAvail(2, false);
            vAvail[0] = (nCode & 2) != 0;
            vAvail[1] = (nCode & 4) != 0;
            unsigned int nMaskCode = (nCode / 8) + ((nCode & 6) != 0 ? 0 : 1);
            // spentness bitmask
            while (nMaskCode > 0) {
                unsigned char chAvail = 0;
                ::Unserialize(s, chAvail);
                for (unsigned int p = 0; p < 8; p++) {
                    bool f = (chAvail & (1 << p)) != 0;
                    vAvail.push_back(f);
                }
                if (chAvail != 0)
                    nMaskCode--;
            }
            // txouts themself
            vout.assign(vAvail.size(), CTxOut());
            for (unsigned int i = 0; i < vAvail.size(); i++) {
                if (vAvail[i])
                    ::Unserialize(s, Using<TxOutCompression>(vout[i]));
            }
            // coinbase height
            ::Unserialize(s, VARINT_MODE(nHeight, VarIntMode::NONNEGATIVE_SIGNED));
        */
    }
}
