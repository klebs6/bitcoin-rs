// ---------------- [ File: bitcoin-tx/src/txin.rs ]
crate::ix!();

/**
  | An input of a transaction. It contains
  | the location of the previous transaction's
  | output that it claims and a signature
  | that matches the output's public key.
  |
  */
#[derive(Clone,Serialize,Deserialize)]
pub struct TxIn {
    pub prevout:        OutPoint,
    pub script_sig:     Script,
    pub n_sequence:     u32,

    /**
      | Only serialized through CTransaction
      |
      */
    pub script_witness: ScriptWitness,
}

impl RecursiveDynamicUsage for TxIn {

    fn recursive_dynamic_usage(&self) -> usize {
        
        todo!();
            /*
                size_t mem = RecursiveDynamicUsage(in.scriptSig) + RecursiveDynamicUsage(in.prevout) + memusage::DynamicUsage(in.scriptWitness.stack);
            for (std::vector<std::vector<unsigned char> >::const_iterator it = in.scriptWitness.stack.begin(); it != in.scriptWitness.stack.end(); it++) {
                 mem += memusage::DynamicUsage(*it);
            }
            return mem;
            */
    }
}

pub mod tx_in {

    /**
      | Setting nSequence to this value for
      | every input in a transaction disables
      | nLockTime.
      |
      */
    pub const SEQUENCE_FINAL: u32 = 0xffffffff;

    /**
      | Below flags apply in the context of BIP
      | 68
      | 
      | If this flag set, CTxIn::nSequence
      | is NOT interpreted as a relative lock-time.
      |
      */
    pub const SEQUENCE_LOCKTIME_DISABLE_FLAG: u32 = 1 << 31;

    /**
      | If CTxIn::nSequence encodes a relative
      | lock-time and this flag is set, the relative
      | lock-time has units of 512 seconds,
      | otherwise it specifies blocks with
      | a granularity of 1.
      |
      */
    pub const SEQUENCE_LOCKTIME_TYPE_FLAG: u32 = 1 << 22;

    /**
      | If CTxIn::nSequence encodes a relative
      | lock-time, this mask is applied to extract
      | that lock-time from the sequence field.
      |
      */
    pub const SEQUENCE_LOCKTIME_MASK: u32 = 0x0000ffff;

    /**
      | In order to use the same number of bits
      | to encode roughly the same wall-clock
      | duration, and because blocks are naturally
      | limited to occur every 600s on average,
      | the minimum granularity for time-based
      | relative lock-time is fixed at 512 seconds.
      | 
      | Converting from CTxIn::nSequence
      | to seconds is performed by multiplying
      | by 512 = 2^9, or equivalently shifting
      | up by 9 bits.
      |
      */
    pub const SEQUENCE_LOCKTIME_GRANULARITY: i32 = 9;
}

impl Default for TxIn {
    
    fn default() -> Self {
        todo!();
        /*


            nSequence = SEQUENCE_FINAL;
        */
    }
}

lazy_static!{
    /*
    SERIALIZE_METHODS(CTxIn, obj) { 
        READWRITE(obj.prevout, obj.scriptSig, obj.nSequence); 
    }
    */
}

impl PartialEq<TxIn> for TxIn {
    
    #[inline] fn eq(&self, other: &TxIn) -> bool {
        todo!();
        /*
            return (a.prevout   == b.prevout &&
                    a.scriptSig == b.scriptSig &&
                    a.nSequence == b.nSequence);
        */
    }
}

impl Eq for TxIn {}

impl TxIn {
    
    pub fn new_from_outpoint(
        prevout_in:    OutPoint,
        script_sig_in: Option<Script>,
        n_sequence_in: Option<u32>) -> Self {
    
        let script_sig_in: Script =
                 script_sig_in.unwrap_or(Script::default());

        let n_sequence_in: u32 =
                 n_sequence_in.unwrap_or(tx_in::SEQUENCE_FINAL);

        todo!();
        /*


            prevout = prevoutIn;
        scriptSig = scriptSigIn;
        nSequence = nSequenceIn;
        */
    }
    
    pub fn new(
        hash_prev_tx:  u256,
        n_out:         u32,
        script_sig_in: Option<Script>,
        n_sequence_in: Option<u32>) -> Self {
    
        let script_sig_in: Script =
                 script_sig_in.unwrap_or(Script::default());

        let n_sequence_in: u32 =
                 n_sequence_in.unwrap_or(tx_in::SEQUENCE_FINAL);

        todo!();
        /*


            prevout = OutPoint(hashPrevTx, nOut);
        scriptSig = scriptSigIn;
        nSequence = nSequenceIn;
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            std::string str;
        str += "CTxIn(";
        str += prevout.ToString();
        if (prevout.IsNull())
            str += strprintf(", coinbase %s", HexStr(scriptSig));
        else
            str += strprintf(", scriptSig=%s", HexStr(scriptSig).substr(0, 24));
        if (nSequence != tx_in::SEQUENCE_FINAL)
            str += strprintf(", nSequence=%u", nSequence);
        str += ")";
        return str;
        */
    }
}
