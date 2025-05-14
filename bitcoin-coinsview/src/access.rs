// ---------------- [ File: bitcoin-coinsview/src/access.rs ]
crate::ix!();

/**
  | Utility function to find any unspent output
  | with a given txid.
  |
  | This function can be quite expensive because in
  | the event of a transaction which is not found
  | in the cache, it can cause up to
  | MAX_OUTPUTS_PER_BLOCK lookups to database, so
  | it should be used with care.
  */
pub fn access_by_txid<'a>(
        view: &'a CoinsViewCache,
        txid: &u256) -> &'a Coin {

    let mut iter = OutPoint::new(txid, 0);

    while (iter.n as usize) < *MAX_OUTPUTS_PER_BLOCK {

        let alternate = view.access_coin(&iter);

        if !alternate.is_spent() {
            return alternate;
        }

        iter.n += 1;
    }

    &COIN_EMPTY
}
