crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/util/error.h]

/**
  | util/error.h is a common place for definitions
  | of simple error types and string functions.
  | Types and functions defined here should
  | not require any outside dependencies.
  | 
  | Error types defined here can be used
  | in different parts of the codebase,
  | to avoid the need to write boilerplate
  | code catching and translating errors
  | passed across wallet/node/rpc/gui
  | code boundaries.
  |
  */
pub enum TransactionError {

    /**
      | No error
      |
      */
    OK, 

    MISSING_INPUTS,
    ALREADY_IN_CHAIN,
    P2P_DISABLED,
    MEMPOOL_REJECTED,
    MEMPOOL_ERROR,
    INVALID_PSBT,
    PSBT_MISMATCH,
    SIGHASH_MISMATCH,
    MAX_FEE_EXCEEDED,
    EXTERNAL_SIGNER_NOT_FOUND,
    EXTERNAL_SIGNER_FAILED,
}

//-------------------------------------------[.cpp/bitcoin/src/util/error.cpp]

pub fn transaction_error_string(err: TransactionError) -> BilingualStr {
    
    todo!();
        /*
            switch (err) {
            case TransactionError::OK:
                return Untranslated("No error");
            case TransactionError::MISSING_INPUTS:
                return Untranslated("Inputs missing or spent");
            case TransactionError::ALREADY_IN_CHAIN:
                return Untranslated("Transaction already in block chain");
            case TransactionError::P2P_DISABLED:
                return Untranslated("Peer-to-peer functionality missing or disabled");
            case TransactionError::MEMPOOL_REJECTED:
                return Untranslated("Transaction rejected by AcceptToMemoryPool");
            case TransactionError::MEMPOOL_ERROR:
                return Untranslated("AcceptToMemoryPool failed");
            case TransactionError::INVALID_PSBT:
                return Untranslated("PSBT is not well-formed");
            case TransactionError::PSBT_MISMATCH:
                return Untranslated("PSBTs not compatible (different transactions)");
            case TransactionError::SIGHASH_MISMATCH:
                return Untranslated("Specified sighash value does not match value stored in PSBT");
            case TransactionError::MAX_FEE_EXCEEDED:
                return Untranslated("Fee exceeds maximum configured by user (e.g. -maxtxfee, maxfeerate)");
            case TransactionError::EXTERNAL_SIGNER_NOT_FOUND:
                return Untranslated("External signer not found");
            case TransactionError::EXTERNAL_SIGNER_FAILED:
                return Untranslated("External signer failed to sign");
            // no default case, so the compiler can warn about missing cases
        }
        assert(false);
        */
}

pub fn resolve_err_msg(
        optname:  &String,
        str_bind: &String) -> BilingualStr {
    
    todo!();
        /*
            return strprintf(_("Cannot resolve -%s address: '%s'"), optname, strBind);
        */
}

pub fn amount_high_warn(optname: &String) -> BilingualStr {
    
    todo!();
        /*
            return strprintf(_("%s is set very high!"), optname);
        */
}

pub fn amount_err_msg(
        optname:   &String,
        str_value: &String) -> BilingualStr {
    
    todo!();
        /*
            return strprintf(_("Invalid amount for -%s=<amount>: '%s'"), optname, strValue);
        */
}
