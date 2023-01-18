crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/primitives/transaction.h]

/**
  | A flag that is ORed into the protocol
  | version to designate that a transaction
  | should be (un)serialized without witness
  | data.
  | 
  | Make sure that this does not collide
  | with any of the values in `version.h`
  | or with `ADDRV2_FORMAT`.
  |
  */
pub const SERIALIZE_TRANSACTION_NO_WITNESS: i32 = 0x40000000;

/**
  | Basic transaction serialization format:
  | 
  | - int32_t nVersion
  | - std::vector<CTxIn> vin
  | - std::vector<CTxOut> vout
  | - uint32_t nLockTime
  | 
  | Extended transaction serialization
  | format:
  | 
  | - int32_t nVersion
  | - unsigned char dummy = 0x00
  | - unsigned char flags (!= 0)
  | - std::vector<CTxIn> vin
  | - std::vector<CTxOut> vout
  | - if (flags & 1):
  | - CTxWitness wit;
  | - uint32_t nLockTime
  |
  */
#[inline] pub fn unserialize_transaction<Stream, TxType>(
        tx: &mut TxType,
        s:  &mut Stream)  {

    todo!();
        /*
            const bool fAllowWitness = !(s.GetVersion() & SERIALIZE_TRANSACTION_NO_WITNESS);

        s >> tx.nVersion;
        unsigned char flags = 0;
        tx.vin.clear();
        tx.vout.clear();
        /* Try to read the vin. In case the dummy is there, this will be read as an empty vector. */
        s >> tx.vin;
        if (tx.vin.size() == 0 && fAllowWitness) {
            /* We read a dummy or an empty vin. */
            s >> flags;
            if (flags != 0) {
                s >> tx.vin;
                s >> tx.vout;
            }
        } else {
            /* We read a non-empty vin. Assume a normal vout follows. */
            s >> tx.vout;
        }
        if ((flags & 1) && fAllowWitness) {
            /* The witness flag is present, and we support witnesses. */
            flags ^= 1;
            for (size_t i = 0; i < tx.vin.size(); i++) {
                s >> tx.vin[i].scriptWitness.stack;
            }
            if (!tx.HasWitness()) {
                /* It's illegal to encode witnesses when all witness stacks are empty. */
                throw std::ios_base::failure("Superfluous witness record");
            }
        }
        if (flags) {
            /* Unknown flag in the serialization */
            throw std::ios_base::failure("Unknown transaction optional data");
        }
        s >> tx.nLockTime;
        */
}

#[inline] pub fn serialize_transaction<Stream, TxType>(
        tx: &TxType,
        s:  &mut Stream)  {

    todo!();
        /*
            const bool fAllowWitness = !(s.GetVersion() & SERIALIZE_TRANSACTION_NO_WITNESS);

        s << tx.nVersion;
        unsigned char flags = 0;
        // Consistency check
        if (fAllowWitness) {
            /* Check whether witnesses need to be serialized. */
            if (tx.HasWitness()) {
                flags |= 1;
            }
        }
        if (flags) {
            /* Use extended format in case witnesses are to be serialized. */
            std::vector<CTxIn> vinDummy;
            s << vinDummy;
            s << flags;
        }
        s << tx.vin;
        s << tx.vout;
        if (flags & 1) {
            for (size_t i = 0; i < tx.vin.size(); i++) {
                s << tx.vin[i].scriptWitness.stack;
            }
        }
        s << tx.nLockTime;
        */
}

/**
  | The basic transaction that is broadcasted
  | on the network and contained in blocks.
  | A transaction can contain multiple
  | inputs and outputs.
  |
  */
#[derive(Default,Serialize,Deserialize)]
pub struct Transaction {

    /**
      | The local variables are made const to
      | prevent unintended modification without
      | updating the cached hash value. 
      |
      | However, CTransaction is not actually
      | immutable; deserialization and assignment
      | are implemented, and bypass the constness. 
      |
      | This is safe, as they update the entire
      | structure, including the hash.
      */
    pub vin:          Vec<TxIn>,
    pub vout:         Vec<TxOut>,

    pub n_version:    i32,
    pub n_lock_time:  u32,

    /**
      | Memory only.
      |
      */
    pub hash:         u256,

    pub witness_hash: u256,
}

impl RecursiveDynamicUsage for Transaction {

    fn recursive_dynamic_usage(&self) -> usize {
        
        todo!();
            /*
                size_t mem = memusage::DynamicUsage(tx.vin) + memusage::DynamicUsage(tx.vout);
            for (std::vector<CTxIn>::const_iterator it = tx.vin.begin(); it != tx.vin.end(); it++) {
                mem += RecursiveDynamicUsage(*it);
            }
            for (std::vector<CTxOut>::const_iterator it = tx.vout.begin(); it != tx.vout.end(); it++) {
                mem += RecursiveDynamicUsage(*it);
            }
            return mem;
            */
    }
}

/**
  | Default transaction version.
  |
  */
pub const TRANSACTION_CURRENT_VERSION: i32 = 2;

impl PartialEq<Transaction> for Transaction {
    
    #[inline] fn eq(&self, other: &Transaction) -> bool {
        todo!();
        /*
            return a.hash == b.hash;
        */
    }
}

impl Eq for Transaction {}

impl Transaction {

    #[inline] pub fn serialize<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            SerializeTransaction(*this, s);
        */
    }

    /**
      | This deserializing constructor is
      | provided instead of an Unserialize
      | method.
      | 
      | Unserialize is not possible, since
      | it would require overwriting const
      | fields.
      |
      */
    pub fn new<Stream>(
        _0: DeserializeType,
        s:  &mut Stream) -> Self {
    
        todo!();
        /*
        : transaction(CMutableTransaction(deserialize, s)),
        */
    }
    
    pub fn is_null(&self) -> bool {
        
        todo!();
        /*
            return vin.empty() && vout.empty();
        */
    }
    
    pub fn get_hash(&self) -> &u256 {
        
        todo!();
        /*
            return hash;
        */
    }
    
    pub fn get_witness_hash(&self) -> &u256 {
        
        todo!();
        /*
            return m_witness_hash; }{
        */
    }

    pub fn is_coinbase(&self) -> bool {
        
        todo!();
        /*
            return (vin.size() == 1 && vin[0].prevout.IsNull());
        */
    }
    
    pub fn has_witness(&self) -> bool {
        
        todo!();
        /*
            for (size_t i = 0; i < vin.size(); i++) {
                if (!vin[i].scriptWitness.IsNull()) {
                    return true;
                }
            }
            return false;
        */
    }
    
    pub fn compute_hash(&self) -> u256 {
        
        todo!();
        /*
            return SerializeHash(*this, SER_GETHASH, SERIALIZE_TRANSACTION_NO_WITNESS);
        */
    }
    
    pub fn compute_witness_hash(&self) -> u256 {
        
        todo!();
        /*
            if (!HasWitness()) {
            return hash;
        }
        return SerializeHash(*this, SER_GETHASH, 0);
        */
    }
    
    /**
      | Return sum of txouts.
      |
      */
    pub fn get_value_out(&self) -> Amount {
        
        todo!();
        /*
            CAmount nValueOut = 0;
        for (const auto& tx_out : vout) {
            if (!MoneyRange(tx_out.nValue) || !MoneyRange(nValueOut + tx_out.nValue))
                throw std::runtime_error(std::string(__func__) + ": value out of range");
            nValueOut += tx_out.nValue;
        }
        assert(MoneyRange(nValueOut));
        return nValueOut;
        */
    }
    
    /**
      | Get the total transaction size in bytes,
      | including witness data. "Total Size"
      | defined in BIP141 and BIP144.
      | 
      | -----------
      | @return
      | 
      | Total transaction size in bytes
      |
      */
    pub fn get_total_size(&self) -> u32 {
        
        todo!();
        /*
            return ::GetSerializeSize(*this, PROTOCOL_VERSION);
        */
    }
    
    pub fn to_string(&self) -> String {
        
        todo!();
        /*
            std::string str;
        str += strprintf("CTransaction(hash=%s, ver=%d, vin.size=%u, vout.size=%u, nLockTime=%u)\n",
            GetHash().ToString().substr(0,10),
            nVersion,
            vin.size(),
            vout.size(),
            nLockTime);
        for (const auto& tx_in : vin)
            str += "    " + tx_in.ToString() + "\n";
        for (const auto& tx_in : vin)
            str += "    " + tx_in.scriptWitness.ToString() + "\n";
        for (const auto& tx_out : vout)
            str += "    " + tx_out.ToString() + "\n";
        return str;
        */
    }
}

impl From<&MutableTransaction> for Transaction {

    /**
      | Convert a MutableTransaction into
      | a Transaction.
      |
      */
    fn from(tx: &MutableTransaction) -> Self {
    
        todo!();
        /*
            : vin(tx.vin), vout(tx.vout), nVersion(tx.nVersion), nLockTime(tx.nLockTime), hash{ComputeHash()}, m_witness_hash{ComputeWitnessHash()}
        */
    }
}

/**
  | A mutable version of CTransaction.
  |
  */
pub struct MutableTransaction {
    pub vin:         Vec<TxIn>,
    pub vout:        Vec<TxOut>,
    pub n_version:   i32,
    pub n_lock_time: u32,
}

impl RecursiveDynamicUsage for MutableTransaction {

    fn recursive_dynamic_usage(&self) -> usize {
        
        todo!();
            /*
            size_t mem = memusage::DynamicUsage(tx.vin) + memusage::DynamicUsage(tx.vout);
            for (std::vector<CTxIn>::const_iterator it = tx.vin.begin(); it != tx.vin.end(); it++) {
                mem += RecursiveDynamicUsage(*it);
            }
            for (std::vector<CTxOut>::const_iterator it = tx.vout.begin(); it != tx.vout.end(); it++) {
                mem += RecursiveDynamicUsage(*it);
            }
            return mem;
            */
    }
}

impl Default for MutableTransaction {

    fn default() -> Self {
    
        todo!();
        /*
        : n_version(TRANSACTION_CURRENT_VERSION),
        : n_lock_time(0),
        */
    }
}

impl From<&Transaction> for MutableTransaction {

    fn from(tx: &Transaction) -> Self {
    
        todo!();
        /*
        : vin(tx.vin),
        : vout(tx.vout),
        : n_version(tx.nVersion),
        : n_lock_time(tx.nLockTime),
        */
    }
}
    
impl MutableTransaction {
    
    #[inline] pub fn serialize<Stream>(&self, s: &mut Stream)  {
    
        todo!();
        /*
            SerializeTransaction(*this, s);
        */
    }
    
    #[inline] pub fn unserialize<Stream>(&mut self, s: &mut Stream)  {
    
        todo!();
        /*
            UnserializeTransaction(*this, s);
        */
    }
    
    pub fn new<Stream>(
        _0: DeserializeType,
        s:  &mut Stream) -> Self {
    
        todo!();
        /*
            Unserialize(s);
        */
    }

    pub fn has_witness(&self) -> bool {
        
        todo!();
        /*
            for (size_t i = 0; i < vin.size(); i++) {
                if (!vin[i].scriptWitness.IsNull()) {
                    return true;
                }
            }
            return false;
        */
    }
    
    /**
      | Compute the hash of this CMutableTransaction.
      | This is computed on the fly, as opposed
      | to GetHash() in CTransaction, which
      | uses a cached result.
      |
      */
    pub fn get_hash(&self) -> u256 {
        
        todo!();
        /*
            return SerializeHash(*this, SER_GETHASH, SERIALIZE_TRANSACTION_NO_WITNESS);
        */
    }
}

pub type TransactionRef = Amo<Transaction>;

#[inline] pub fn make_transaction_ref<Tx>(tx_in: Tx) -> TransactionRef {

    todo!();
        /*
            return std::make_shared<const CTransaction>(std::forward<Tx>(txIn));
        */
}

