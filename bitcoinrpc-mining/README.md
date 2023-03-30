## bitcoinrpc-mining

This Rust crate provides an interface for
mining-related RPC functions in the Bitcoin
system. These functions can be used to generate
blocks, estimate transaction fees, and prioritize
transactions.

Note that this crate is part of a direct
translation from C++ to Rust of the Bitcoin
Core. As such, some of the function bodies may
still be in the process of translation. Please
note that this system will become testable once
the translation process is complete.

Here is a brief description of the tokens in this
crate:

- `SubmitBlockStateCatcher`: a struct that handles
  the state of block submission

- `VBDeploymentInfo`: a struct that stores
  information about version-bit deployments

- `bip22validation_result`: a struct that stores
  the result of BIP 22 validation

- `block_checked`: a function that checks whether
  a block is valid

- `estimaterawfee`: a function that estimates the
  fee needed for a raw transaction to be included
  in the next block

- `estimatesmartfee`: a function that estimates
  the fee needed for a transaction to be confirmed
  within a certain number of blocks

- `gbt_vb_name`: a function that returns the name
  of a version-bit deployment for the
  getblocktemplate RPC

- `generate`: a function that generates blocks
  locally (without communicating with the network)

- `generate_block`: a function that generates
  a block using the given transactions

- `generate_blocks`: a function that generates
  a specified number of blocks (optionally with
  a specified address to send the block rewards
  to)

- `generateblock`: a function that generates
  a block with a specified coinbase and optionally
  adds transactions to it

- `generatetoaddress`: a function that generates
  a specified number of blocks and sends the block
  rewards to the specified address

- `generatetodescriptor`: a function that
  generates a specified number of blocks and sends
  the block rewards to the specified output
  descriptor

- `get_network_hashps`: a function that returns
  the estimated network hash rate in hashes per
  second

- `get_script_from_descriptor`: a function that
  converts a descriptor string to a scriptPubKey

- `getblocktemplate`: a function that returns
  a template for constructing a new block to mine

- `getmininginfo`: a function that returns
  information about the mining process (such as
  the number of threads being used)

- `getnetworkhashps`: a function that returns the
  estimated network hash rate in hashes per second

- `prioritisetransaction`: a function that sets
  the priority of a transaction

- `register_mining_rpc_commands`: a function that
  registers mining-related RPC commands

- `submitblock`: a function that submits a block
  to the network

- `submitheader`: a function that submits a block
  header to the network for mining.

110> which RPCs are associated with mining?

The following RPCs are associated with mining:

- `generate`: Generates blocks in the local node's
  blockchain. This RPC call is used for testing
  purposes only and should not be used on the main
  network.

- `generatetoaddress`: Generates a specified
  number of blocks and sends the block rewards to
  a specified address.

- `getblocktemplate`: Returns data needed to
  construct a block to mine.

- `prioritisetransaction`: Sets the priority of
  a transaction to be included in the next block.

- `submitblock`: Submits a new block to the
  network.

- `submitheader`: Submits a block header to the
  network for mining.

In addition to these RPCs, the `estimaterawfee`
and `estimatesmartfee` calls can also be useful
for miners to estimate transaction fees for
including in their blocks.
