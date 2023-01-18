crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/policy/settings.h]

/**
  | Policy settings which are configurable
  | at runtime.
  |
  */
lazy_static!{
    /*
    extern CFeeRate incrementalRelayFee;
    extern CFeeRate dustRelayFee;
    extern unsigned int nBytesPerSigOp;
    extern bool fIsBareMultisigStd;
    */
}

#[inline] pub fn is_standard_tx(
        tx:     &Transaction,
        reason: &mut String) -> bool {
    
    todo!();
        /*
            return IsStandardTx(tx, ::fIsBareMultisigStd, ::dustRelayFee, reason);
        */
}

#[inline] pub fn get_virtual_transaction_size_for_tx(
        tx:         &Transaction,
        sigop_cost: i64) -> i64 {
    
    todo!();
        /*
            return GetVirtualTransactionSize(tx, sigop_cost, ::nBytesPerSigOp);
        */
}

//-------------------------------------------[.cpp/bitcoin/src/policy/settings.cpp]

lazy_static!{
    /*
    bool fIsBareMultisigStd = DEFAULT_PERMIT_BAREMULTISIG;
    CFeeRate incrementalRelayFee = CFeeRate(DEFAULT_INCREMENTAL_RELAY_FEE);
    CFeeRate dustRelayFee = CFeeRate(DUST_RELAY_TX_FEE);
    unsigned int nBytesPerSigOp = DEFAULT_BYTES_PER_SIGOP;
    */
}
