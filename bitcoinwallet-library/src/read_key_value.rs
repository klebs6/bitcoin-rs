crate::ix!();

/**
  | Unserialize a given Key-Value pair
  | and load it into the wallet
  |
  */
pub fn read_key_value(
        pwallet:   *mut Wallet,
        ss_key:    &mut DataStream,
        ss_value:  &mut DataStream,
        str_type:  &mut String,
        str_err:   &mut String,
        filter_fn: Option<&KeyFilterFn>) -> bool {
    
    todo!();
        /*
        CWalletScanState dummy_wss;
        LOCK(pwallet->cs_wallet);
        return ReadKeyValue(pwallet, ssKey, ssValue, dummy_wss, strType, strErr, filter_fn);
        */
}
