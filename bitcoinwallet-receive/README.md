# bitcoinwallet-receive

This Rust crate is a direct translation from C++
to Rust of the Bitcoin Core receive wallet
implementation. It provides functions for managing
Bitcoin addresses, calculating balances, and
receiving Bitcoin payments.

Notice: This crate is part of a direct translation
from C++ to Rust of the Bitcoin Core. As such,
some of the function bodies may still be in the
process of translation. Please note that this
system will become testable once the translation
process is complete.

The following mathematical concepts are associated
with some of the tokens used in this crate:

- Balance: The balance of a Bitcoin address is the
  total amount of Bitcoin that has been received
  to that address minus the total amount that has
  been spent from that address. The balance can be
  calculated by summing the outputs of all the
  transactions that have sent Bitcoin to the
  address and subtracting the inputs of all the
  transactions that have spent Bitcoin from the
  address.

- OutputEntry: An output entry in a Bitcoin
  transaction represents an amount of Bitcoin that
  is being sent to a specific Bitcoin address. The
  amount is specified in satoshis, which are the
  smallest unit of Bitcoin (1 satoshi = 0.00000001
  BTC).

- cached_tx_get_amounts: This function returns the
  total input and output amounts of a transaction.

- cached_tx_get_available_credit: This function
  returns the total amount of unspent Bitcoin
  outputs that can be used as inputs in new
  transactions.

- cached_tx_get_change: This function returns the
  amount of Bitcoin that was sent back to the
  sender as change in a transaction.

- cached_tx_get_credit: This function returns the
  total amount of Bitcoin that was received in
  a transaction.

- cached_tx_get_debit: This function returns the
  total amount of Bitcoin that was spent in
  a transaction.

- cached_tx_get_immature_credit: This function
  returns the total amount of Bitcoin that was
  received in a coinbase transaction that has not
  yet reached maturity (i.e., has not yet been
  confirmed by enough blocks).

- cached_tx_get_immature_watch_only_credit: This
  function returns the total amount of Bitcoin
  that was received in a coinbase transaction that
  is associated with a watch-only address.

- cached_tx_is_from_me: This function returns true
  if a transaction was sent from one of the
  Bitcoin addresses managed by the wallet.

- cached_tx_is_trusted: This function returns true
  if a transaction has been confirmed by enough
  blocks to be considered safe.

- cached_tx_is_trusted_with_trusted_parents: This
  function returns true if a transaction has been
  confirmed by enough blocks and all of its parent
  transactions are also confirmed.

- found: This function returns true if a specific
  Bitcoin address is found in the wallet's list of
  managed addresses.

- get_address_balances: This function returns the
  balance of a list of Bitcoin addresses.

- get_address_groupings: This function groups the
  wallet's managed addresses into lists based on
  their ownership.

- get_balance: This function returns the total
  balance of all the Bitcoin addresses managed by
  the wallet.

- get_cachable_amount: This function returns the
  amount of Bitcoin that can be spent from
  a specific address without requiring a new
  signature from the wallet's private key.

- script_is_change: This function returns true if
  a Bitcoin transaction output is a change output
  (i.e., sends Bitcoin back to the sender as
  change).

- tx_get_change: This function returns the amount
  of Bitcoin that was sent back to the sender as
  change in a transaction.

- tx_get_credit: This function returns the total
  amount of Bitcoin that was received in
  a transaction.
