# bitcoinrpc-blockchain

The `bitcoinrpc-blockchain` Rust crate is
a subcomponent of the Bitcoin system that provides
a Rust interface to blockchain-related
functionality. It is a direct translation from C++
to Rust of the Bitcoin Core, and as such, some
function bodies may still be in the process of
translation. This system will become testable once
the translation process is complete.

The crate provides functionality to interact with
the blockchain, including fetching block data,
calculating difficulty and finding the best block
hash, and managing the mempool. It also provides
functionality for scanning and reserving coins,
computing block statistics, and managing UTXO
snapshots.

The crate includes several mathematical ideas,
such as calculating percentiles by weight,
truncated median calculation, and difficulty
calculation.

Overall, `bitcoinrpc-blockchain` provides
a powerful and flexible set of tools for
interacting with the Bitcoin blockchain and
managing its associated data.

104> which interactions do we want to perform?

The `bitcoinrpc-blockchain` crate provides
a collection of functions that interact with the
blockchain and its associated data structures in
various ways. Some of the interactions that can be
performed using this crate include:

- Retrieving information about the blockchain,
  such as the best block hash, the current block
  height, and the difficulty target.

- Retrieving information about specific blocks,
  such as block headers, block contents, and block
  statistics.

- Retrieving information about transactions in the
  mempool, such as transaction data and mempool
  statistics.

- Scanning the UTXO set for unspent outputs
  matching specific criteria.

- Performing various validation and verification
  tasks, such as verifying the blockchain's
  consistency, calculating difficulty targets, and
  validating transaction scripts.

- Modifying the blockchain state by invalidating
  or revalidating blocks, pruning old blocks, and
  syncing with the validation interface queue.

Overall, the `bitcoinrpc-blockchain` crate
provides a wide range of functionality for
interacting with the blockchain and its associated
data structures, enabling developers to build
powerful blockchain applications with ease.
