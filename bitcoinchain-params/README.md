# bitcoinchain-params

Rust crate for Bitcoin chain parameters as part of
the direct translation of the Bitcoin C++ codebase
to Rust.

## Overview

`bitcoinchain-params` is a Rust crate that handles
the various chain parameters in a Bitcoin
system. As the translation is still in progress,
some function bodies may still be in the process
of being translated.

This crate mainly deals with the configuration and
management of the different parameters associated
with various Bitcoin network chains, such as
MainNet, TestNet, SigNet, and RegTest. It provides
structures and methods for defining, accessing,
and modifying these parameters, which are
essential for the proper functioning of a Bitcoin
node.

### Key Components

Some of the primary structures and functions
provided by the `bitcoinchain-params` crate
include:

- `ChainParams`: Represents the various parameters
  of a specific Bitcoin network chain, such as
  block version, transaction data, and network ID.

- `ChainTxData`: Contains the transaction data for
  a specific Bitcoin network chain.

- `create_chain_params`: Function to create
  a `ChainParams` object based on the given
  network ID.

- `select_params`: Function to select the
  appropriate `ChainParams` object for the desired
  network chain.

Additionally, the crate contains definitions for
`MainParams`, `TestNetParams`, `SigNetParams`, and
`RegTestParams`, which represent the specific
parameters for the MainNet, TestNet, SigNet, and
RegTest network chains, respectively.

2> what distinguishes the parameters for the
MainNet, SigNet, TestNet, and RegTest network
chains?

The parameters for the MainNet, SigNet, TestNet,
and RegTest network chains are distinguished by
several key attributes that define the specific
behavior and characteristics of each
network. These parameters are essential for the
proper functioning of a Bitcoin node, as they help
nodes communicate and interact with the correct
network.

Here's an overview of some distinguishing
parameters for each network chain:

1. **MainNet**: The main Bitcoin network, where
   real transactions take place and actual value
   is exchanged. MainNet parameters include:

    - Unique network ID (known as "magic bytes" or
      "message start")

    - Consensus rules for block validation and
      transaction processing

    - Default ports for peer-to-peer communication

    - Seed nodes and DNS seeds for bootstrapping
      new nodes

    - Address prefixes for Base58 and Bech32
      addresses

2. **TestNet**: A separate network used for
   testing and development purposes. TestNet
   allows developers to experiment with new
   features and perform tests without risking real
   value. TestNet parameters differ from MainNet
   in the following ways:

    - Different network ID

    - Different default ports

    - Separate seed nodes and DNS seeds

    - Unique address prefixes for Base58 and
      Bech32 addresses

    - Less strict consensus rules, making it
      easier to mine blocks

3. **SigNet**: A more customizable testing
   network, which allows developers to set up
   private test networks with specific consensus
   rules. SigNet parameters are similar to
   TestNet, but with added flexibility in defining
   consensus rules and network topology. Some
   distinguishing parameters include:

    - Custom network ID

    - Custom default ports

    - Custom seed nodes and DNS seeds

    - Unique address prefixes for Base58 and
      Bech32 addresses

    - Configurable consensus rules

4. **RegTest**: A local testing network that
   operates in a controlled environment. RegTest
   allows developers to generate blocks on-demand,
   providing a more predictable and manageable
   testing environment. RegTest parameters are
   similar to TestNet, with some key differences:

    - Unique network ID

    - No seed nodes or DNS seeds, as it is
      intended for local use

    - Unique address prefixes for Base58 and
      Bech32 addresses

    - Ability to generate blocks on-demand and
      modify consensus rules as needed

Each network chain has its purpose, and the
parameters help ensure that nodes can interact
with the correct network without interfering with
other networks.
