crate::ix!();

/**
  | Utility function to add all of a transaction's
  | outputs to a cache.
  |
  | When check is false, this assumes that
  | overwrites are only possible for coinbase
  | transactions.
  |
  | When check is true, the underlying view may be
  | queried to determine whether an addition is an
  | overwrite.
  |
  | TODO: pass in a boolean to limit these possible
  | overwrites to known (pre-BIP34) cases.
  */
pub fn add_coins(
        cache:               &mut CoinsViewCache,
        tx:                  &Transaction,
        n_height:            i32,
        check_for_overwrite: Option<bool>)  {

    let check_for_overwrite: bool = check_for_overwrite.unwrap_or(false);

    let coinbase = tx.is_coinbase();
    let txid     = tx.get_hash();

    for i in 0..tx.vout.len() {

        let outpoint = OutPoint::new(txid,i as u32);

        let overwrite: bool = match check_for_overwrite { 
            true  => cache.have_coin(&outpoint),
            false => coinbase,
        };

        /*
           | Coinbase transactions can always be
           | overwritten, in order to correctly deal
           | with the pre-BIP30 occurrences of
           | duplicate coinbase transactions.
           */
        cache.add_coin(
            &outpoint, 
            Coin {
                out: tx.vout[i].clone(), 
                bits: CoinBitfield::from_fields(
                    n_height, 
                    coinbase
                )
            }, 
            overwrite
        );
    }
}

