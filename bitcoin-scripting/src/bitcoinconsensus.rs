// ---------------- [ File: bitcoin-scripting/src/bitcoinconsensus.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/script/bitcoinconsensus.h]

pub const BITCOINCONSENSUS_API_VER: usize = 1;

pub enum BitcoinConsensusError
{
    Ok = 0,
    Txindex,
    TxSizeMismatch,
    TxDeserialize,
    AmountRequired,
    InvalidFlags,
}

bitflags! {
    pub struct BitcoinConsensusScriptVerificationFlags: u32 {
        const None                = 0b0000000000000000;

        /*
           | evaluate P2SH (BIP16) subscripts
           |
           */
        const P2sh                = 0b0000000000000001;

        /*
           | enforce strict DER (BIP66) compliance
           |
           */
        const Dersig              = 0b0000000000000100;

        /*
          | enforce NULLDUMMY (BIP147)
          |
          */
        const Nulldummy           = 0b0000000000010000;

        /*
          | enable CHECKLOCKTIMEVERIFY (BIP65)
          |
          */
        const Checklocktimeverify = 0b0000001000000000;

        /*
          | enable CHECKSEQUENCEVERIFY (BIP112)
          |
          */
        const Checksequenceverify = 0b0000010000000000;

        /*
          | enable WITNESS (BIP141)
          |
          */
        const Witness             = 0b0000100000000000;

        const ALL = 
            Self::P2sh.bits 
            | Self::Dersig.bits 
            | Self::Nulldummy.bits
            | Self::Checklocktimeverify.bits
            | Self::Checksequenceverify.bits
            | Self::Witness.bits;
    }
}

//-------------------------------------------[.cpp/bitcoin/src/script/bitcoinconsensus.cpp]

/**
  | A class that deserializes a single CTransaction
  | one time.
  |
  */
pub struct TxInputStream {
    version:   i32,
    data:      *const u8,
    remaining: usize,
}

impl<T> Shr<T> for TxInputStream {
    type Output = TxInputStream;

    #[inline] fn shr(self, rhs: T) -> Self::Output {
        todo!();
        /*
            ::Unserialize(*this, obj);
            return *this;
        */
    }
}

impl TxInputStream {

    pub fn new(
        n_version_in: i32,
        tx_to:        *const u8,
        tx_to_len:    usize) -> Self {
    
        todo!();
        /*
        : version(nVersionIn),
        : data(txTo),
        : remaining(txToLen),

        
        */
    }
    
    pub fn read(&mut self, 
        pch:    *mut u8,
        n_size: usize)  {
        
        todo!();
        /*
            if (nSize > m_remaining)
                throw std::ios_base::failure(std::string(__func__) + ": end of data");

            if (pch == nullptr)
                throw std::ios_base::failure(std::string(__func__) + ": bad destination buffer");

            if (m_data == nullptr)
                throw std::ios_base::failure(std::string(__func__) + ": bad source buffer");

            memcpy(pch, m_data, nSize);
            m_remaining -= nSize;
            m_data += nSize;
        */
    }
    
    pub fn get_version(&self) -> i32 {
        
        todo!();
        /*
            return m_version;
        */
    }
}

#[inline] pub fn set_error(
        ret:    *mut BitcoinConsensusError,
        serror: BitcoinConsensusError) -> i32 {
    
    todo!();
        /*
            if (ret)
            *ret = serror;
        return 0;
        */
}

pub struct ECCryptoClosure
{
    handle: ECCVerifyHandle,
}

lazy_static!{
    /*
    ECCryptoClosure instance_of_eccryptoclosure;
    */
}

/**
  | Check that all specified flags are part
  | of the libconsensus interface.
  |
  */
pub fn verify_flags(flags: u32) -> bool {
    
    todo!();
        /*
            return (flags & ~(bitcoinconsensus_SCRIPT_FLAGS_VERIFY_ALL)) == 0;
        */
}

pub fn verify_script(
        script_pub_key:     *const u8,
        script_pub_key_len: u32,
        amount:             Amount,
        tx_to:              *const u8,
        tx_to_len:          u32,
        n_in:               u32,
        flags:              u32,
        err:                *mut BitcoinConsensusError) -> i32 {
    
    todo!();
        /*
            if (!verify_flags(flags)) {
            return set_error(err, bitcoinconsensus_ERR_INVALID_FLAGS);
        }
        try {
            TxInputStream stream(PROTOCOL_VERSION, txTo, txToLen);
            CTransaction tx(deserialize, stream);
            if (nIn >= tx.vin.size())
                return set_error(err, bitcoinconsensus_ERR_TX_INDEX);
            if (GetSerializeSize(tx, PROTOCOL_VERSION) != txToLen)
                return set_error(err, bitcoinconsensus_ERR_TX_SIZE_MISMATCH);

            // Regardless of the verification result, the tx did not error.
            set_error(err, bitcoinconsensus_ERR_OK);

            PrecomputedTransactionData txdata(tx);
            return VerifyScript(tx.vin[nIn].scriptSig, CScript(scriptPubKey, scriptPubKey + scriptPubKeyLen), &tx.vin[nIn].scriptWitness, flags, TransactionSignatureChecker(&tx, nIn, amount, txdata, MissingDataBehavior::FAIL), nullptr);
        } catch (const std::exception&) {
            return set_error(err, bitcoinconsensus_ERR_TX_DESERIALIZE); // Error deserializing
        }
        */
}

pub fn bitcoinconsensus_verify_script_with_amount(
        script_pub_key:     *const u8,
        script_pub_key_len: u32,
        amount:             i64,
        tx_to:              *const u8,
        tx_to_len:          u32,
        n_in:               u32,
        flags:              u32,
        err:                *mut BitcoinConsensusError) -> i32 {
    
    todo!();
        /*
            CAmount am(amount);
        return ::verify_script(scriptPubKey, scriptPubKeyLen, am, txTo, txToLen, nIn, flags, err);
        */
}

/**
  | Returns 1 if the input nIn of the serialized
  | transaction pointed to by txTo correctly
  | spends the scriptPubKey pointed to by
  | scriptPubKey under the additional constraints
  | specified by flags.
  |
  | If not nullptr, err will contain an
  | error/success code for the operation
  */
pub fn bitcoinconsensus_verify_script(
        script_pub_key:     *const u8,
        script_pub_key_len: u32,
        tx_to:              *const u8,
        tx_to_len:          u32,
        n_in:               u32,
        flags:              u32,
        err:                *mut BitcoinConsensusError) -> i32 {
    
    todo!();
        /*
            if (flags & bitcoinconsensus_SCRIPT_FLAGS_VERIFY_WITNESS) {
            return set_error(err, bitcoinconsensus_ERR_AMOUNT_REQUIRED);
        }

        CAmount am(0);
        return ::verify_script(scriptPubKey, scriptPubKeyLen, am, txTo, txToLen, nIn, flags, err);
        */
}

pub fn bitcoinconsensus_version() -> u32 {
    
    todo!();
        /*
            // Just use the API version for now
        return BITCOINCONSENSUS_API_VER;
        */
}
