// ---------------- [ File: bitcoin-tx/src/txout.rs ]
crate::ix!();

/**
  | An output of a transaction. It contains
  | the public key that the next input must
  | be able to sign with to claim it.
  |
  */
#[derive(Clone,Serialize,Deserialize)]
pub struct TxOut {
    pub n_value:        Amount,
    pub script_pub_key: Script,
}

pub const DEFAULT_TX_OUT: TxOut = TxOut::new();

impl RecursiveDynamicUsage for TxOut {

    fn recursive_dynamic_usage(&self) -> usize {
        
        todo!();
            /*
                return RecursiveDynamicUsage(out.scriptPubKey);
            */
    }
}

impl Default for TxOut {
    
    fn default() -> Self {
        todo!();
        /*
            SetNull();
        */
    }
}

impl TxOut {
    pub const fn new() -> Self {
        Self {
            n_value: 0,
            script_pub_key: Script::new(),
        }
    }
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CTxOut, obj) { 
        READWRITE(obj.nValue, obj.scriptPubKey); 
    }
    */
}

impl PartialEq<TxOut> for TxOut {
    
    #[inline] fn eq(&self, other: &TxOut) -> bool {
        todo!();
        /*
            return (a.nValue       == b.nValue &&
                    a.scriptPubKey == b.scriptPubKey);
        */
    }
}

impl Eq for TxOut {}

impl TxOut {
    
    pub fn set_null(&mut self)  {
        
        todo!();
        /*
            nValue = -1;
            scriptPubKey.clear();
        */
    }
    
    pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            return (nValue == -1);
        */
    }
    
    pub fn new_from_amount_and_script(
        n_value_in:        &Amount,
        script_pub_key_in: Script) -> Self {
    
        todo!();
        /*


            nValue = nValueIn;
        scriptPubKey = scriptPubKeyIn;
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            return strprintf("CTxOut(nValue=%d.%08d, scriptPubKey=%s)", nValue / COIN, nValue % COIN, HexStr(scriptPubKey).substr(0, 30));
        */
    }
}
