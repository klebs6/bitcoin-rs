**Bitcoin CoinSelect: A Rust crate for Bitcoin UTXO selection**

A Rust implementation of Bitcoin's UTXO selection
algorithms, part of a direct translation of the
Bitcoin codebase from C++ to Rust. Some function
bodies may still be in the process of translation.

---

This Rust crate, `bitcoin-coinselect`, provides
a collection of algorithms and utilities for
selecting unspent transaction outputs (UTXOs) in
the Bitcoin network. The crate is part of a direct
translation of the Bitcoin codebase from C++ to
Rust, and some function bodies may still be in the
process of translation.

The crate includes several core components:

- `OutputGroup`: A data structure representing
  a group of UTXOs eligible for spending in
  a transaction.

- `eligible_for_spending`: A function that
  determines if a given UTXO is eligible for
  spending based on various criteria, such as age
  and amount.

- `knapsack_solver`: An implementation of the
  knapsack algorithm for UTXO selection, which
  aims to find the most efficient combination of
  UTXOs to cover a target transaction amount while
  minimizing change.

- `get_selection_waste`: A function that
  calculates the waste associated with
  a particular UTXO selection, considering factors
  such as transaction fees and privacy.

- `DescendingOrderComparator`: A utility for
  sorting UTXOs in descending order based on their
  value, allowing for efficient selection of the
  highest-value UTXOs.

- `CoinSelectionParams`: A struct that holds
  parameters for the UTXO selection process, such
  as the target transaction amount, the desired
  number of inputs, and other constraints.

`find_coins`: A function that searches for
eligible UTXOs in a wallet or set of UTXOs,
filtering them based on the provided
`CoinEligibilityFilter` criteria. This allows
users to search for UTXOs that meet specific
requirements, such as minimum confirmations or
inclusion in a particular address range.

- `CoinEligibilityFilter`: A struct that defines
  the eligibility criteria for UTXOs, including
  minimum and maximum number of confirmations,
  whether to include watch-only transactions, and
  other filters.

- `select_coins_bnb`: An implementation of the
  Branch and Bound algorithm for UTXO selection,
  which aims to find an exact match for the target
  transaction amount without producing change, if
  possible. The algorithm searches through the
  UTXO set using a depth-first search with
  pruning, considering each UTXO's value and the
  total transaction fee.

- `select_coinssrd`: A function that implements
  the Single Random Draw (SRD) UTXO selection
  algorithm. This method randomly selects UTXOs
  from the available set until the target
  transaction amount is reached or exceeded. The
  SRD algorithm is typically used as a fallback
  when more advanced selection algorithms, such as
  the knapsack solver or Branch and Bound, fail to
  find a suitable solution.

- `approximate_best_subset`: A function that finds
  an approximate solution to the UTXO selection
  problem, using a greedy algorithm to select the
  highest-value UTXOs that cover the target
  transaction amount. This approach is less
  computationally intensive than the knapsack
  solver and Branch and Bound algorithm but may
  result in suboptimal selections.

- `InputCoin`: A struct representing a single UTXO
  in the context of the selection algorithms,
  containing information about the UTXO's value,
  associated transaction, and other relevant data.

- `cmp`, `eq`, `partial_cmp`: Comparison functions
  for sorting and comparing UTXOs based on their
  value or other criteria, allowing for efficient
  UTXO selection and management.

The `bitcoin-coinselect` crate provides
a comprehensive toolkit for UTXO selection in
Bitcoin transactions, offering a variety of
algorithms and utilities to suit different use
cases and requirements. By implementing these
selection algorithms in Rust, the crate aims to
provide a fast, efficient, and secure foundation
for Bitcoin wallet software and other applications
that require UTXO management.
