// ---------------- [ File: bitcoin-txmempool/src/txundo.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/undo.h]

/**
  | Formatter for undo information for
  | a CTxIn
  | 
  | Contains the prevout's CTxOut being
  | spent, and its metadata as well (coinbase
  | or not, height). The serialization
  | contains a dummy value of zero. This
  | is compatible with older versions which
  | expect to see the transaction version
  | there.
  |
  */
pub struct TxInUndoFormatter {

}

impl TxInUndoFormatter {
    
    pub fn ser<Stream>(&mut self, 
        s:     &mut Stream,
        txout: &Coin)  {
    
        todo!();
        /*
            ::Serialize(s, VARINT(txout.nHeight * uint32_t{2} + txout.fCoinBase ));
            if (txout.nHeight > 0) {
                // Required to maintain compatibility with older undo format.
                ::Serialize(s, (unsigned char)0);
            }
            ::Serialize(s, Using<TxOutCompression>(txout.out));
        */
    }
    
    
    pub fn unser<Stream>(&mut self, 
        s:     &mut Stream,
        txout: &mut Coin)  {
    
        todo!();
        /*
            uint32_t nCode = 0;
            ::Unserialize(s, VARINT(nCode));
            txout.nHeight = nCode >> 1;
            txout.fCoinBase = nCode & 1;
            if (txout.nHeight > 0) {
                // Old versions stored the version number for the last spend of
                // a transaction's outputs. Non-final spends were indicated with
                // height = 0.
                unsigned int nVersionDummy;
                ::Unserialize(s, VARINT(nVersionDummy));
            }
            ::Unserialize(s, Using<TxOutCompression>(txout.out));
        */
    }
}


lazy_static!{
    /*
    SERIALIZE_METHODS(CTxUndo, obj) { 
        READWRITE(Using<VectorFormatter<TxInUndoFormatter>>(obj.vprevout)); 
    }
    */
}

/**
  | Undo information for a CBlock
  |
  */
pub struct BlockUndo {

    /**
      | for all but the coinbase
      |
      */
    pub vtxundo: Vec<TxUndo>,
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CBlockUndo, obj) { 
        READWRITE(obj.vtxundo); 
    }
    */
}
