# bitcoinwallet-spend

This Rust crate is a direct translation from C++
to Rust of the Bitcoin Core spend wallet
implementation. It provides functions for creating
and signing Bitcoin transactions, selecting input
coins, and estimating transaction fees.

Notice: This crate is part of a direct translation
from C++ to Rust of the Bitcoin Core. As such,
some of the function bodies may still be in the
process of translation. Please note that this
system will become testable once the translation
process is complete.

The following mathematical concepts are associated
with some of the tokens used in this crate:

- Output: An output in a Bitcoin transaction
  represents an amount of Bitcoin that is being
  sent to a specific Bitcoin address. The output
  is specified in satoshis, which are the smallest
  unit of Bitcoin (1 satoshi = 0.00000001 BTC).

- TxSize: The size of a Bitcoin transaction is the
  total number of bytes required to encode the
  transaction. The size is an important factor in
  calculating the transaction fee, which is a fee
  paid to the Bitcoin network to incentivize
  miners to include the transaction in the
  blockchain.

- calculate_maximum_signed_tx_size: This function
  calculates the maximum size of a signed Bitcoin
  transaction given a set of input coins and
  output amounts. The calculation takes into
  account the size of the inputs, outputs, and
  transaction metadata, as well as the signature
  size.

- calculate_maximum_signed_tx_size_with_txouts:
  This function is similar to
  calculate_maximum_signed_tx_size, but also takes
  into account the size of the output scripts.

- create_transaction: This function creates a new
  Bitcoin transaction by selecting input coins,
  specifying output amounts, and calculating the
  transaction fee. The function returns the signed
  transaction as a hexadecimal string.

- fund_transaction: This function is used to fund
  a Bitcoin transaction by selecting input coins
  and specifying output amounts. The function
  returns the unsigned transaction as a Bitcoin
  transaction object.

- select_coins: This function selects input coins
  from a list of available coins to create a new
  Bitcoin transaction. The function uses a coin
  selection algorithm to minimize the transaction
  fee while still including enough input coins to
  cover the output amounts.

- get_available_balance: This function calculates
  the total available balance of a Bitcoin wallet
  by summing the values of all unspent output
  coins.

- get_tx_spend_size: This function calculates the
  size of a Bitcoin transaction given a set of
  input coins and output amounts. The calculation
  takes into account the size of the inputs,
  outputs, and transaction metadata, as well as
  the signature size.

Overall, the bitcoinwallet-spend crate provides
a set of tools for managing Bitcoin transactions,
calculating transaction fees, and selecting input
coins. The functions in this crate are important
for building Bitcoin applications that require the
ability to create, sign, and broadcast Bitcoin
transactions on the Bitcoin network.
