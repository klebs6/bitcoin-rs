# Bitcoin-foundblock

Bitcoin-foundblock is a Rust crate that is part of
a direct translation of the Bitcoin codebase from
C++ to Rust. This crate provides functions and
traits to help locate and work with Bitcoin
blocks.

Please note that while the majority of the code
has been translated from C++, some function bodies
are still being worked on.

## Found Block

The `FoundBlock` struct is provided as a helper
for selectively returning pieces of block data
when blocks are found. 

This crate is useful for developers who want to
interact with the Bitcoin network in Rust and need
to search for and retrieve block data.

## Traits

The following traits are included in the crate:

- `FindBlock`: Returns whether a node has the
  specified block and optionally returns block
  metadata or contents.

- `FindFirstBlockWithTimeAndHeight`: Finds the
  first block in the chain with a timestamp
  greater than or equal to the specified time and
  a height greater than or equal to the specified
  height. Returns false if there is no block with
  a high enough timestamp and height.

- `FindAncestorByHeight`: Finds the ancestor of
  the block at the specified height and optionally
  returns ancestor information.

- `FindAncestorByHash`: Returns whether a block
  descends from a specified ancestor, and
  optionally returns ancestor information.

- `FindCommonAncestor`: Finds the most recent
  common ancestor between two blocks and
  optionally returns block information.

## FoundBlock

The `FoundBlock` struct is a helper for the
`findBlock` function and selectively returns
pieces of block data. If the block is found, the
data will be returned by setting specified output
variables. If the block is not found, the output
variables will keep their previous values.

## Usage

To use Bitcoin-foundblock in your Rust project,
simply add it as a dependency in your `Cargo.toml`
file:

```toml
[dependencies]
bitcoin-foundblock = "0.1"
```

Then, import the relevant functions and traits in
your code as needed:

```rust
use bitcoin_foundblock::{FindBlock, FoundBlock};
```

## License

Bitcoin-foundblock is licensed under the MIT
License. See the `LICENSE` file for details.

## Contributing

Contributions are welcome! Please see the
`CONTRIBUTING` file for details on how to
contribute.

## Acknowledgments

This crate is part of a direct translation of the
Bitcoin codebase from C++ to Rust. Thank you to
the Bitcoin developers for their work on this
important project.

## Traits

These traits define the functionality of the
`FoundBlock` type within the `bitcoin-foundblock`
crate. The `FoundBlock` type represents a block in
the Bitcoin blockchain and contains various
metadata about the block, such as its hash,
height, time, and maximum time.

The `FindBlock` trait provides a method to check
whether a node has a specific block and optionally
return the block's metadata or contents.

The `FindFirstBlockWithTimeAndHeight` trait
provides a method to find the first block in the
chain with a timestamp greater than or equal to
a given time and a height greater than or equal to
a given height. It returns false if there is no
block with a high enough timestamp and height.

The `FindAncestorByHeight` trait provides a method
to find the ancestor of a block at a specified
height and optionally return ancestor information.

The `FindAncestorByHash` trait provides a method
to check whether a block descends from a specified
ancestor and optionally return ancestor
information.

The `FindCommonAncestor` trait provides a method
to find the most recent common ancestor between
two blocks and optionally return block
information.

183> thanks -- could we talk more about the
FoundBlock functionality in more depth? why is it
called "FoundBlock"?

Sure, I'd be happy to explain the FoundBlock
functionality in more depth.

The name "FoundBlock" likely comes from its use in
the Bitcoin Core codebase. The FoundBlock struct
is used to help locate and retrieve information
about a specific block within the blockchain. When
a block is "found" in this context, it means that
the node has successfully located the block and
has access to its metadata and/or contents.

The FoundBlock struct contains several fields that
can be used to selectively retrieve pieces of
block data, including the block's hash, height,
time, max time, mean time past, and whether or not
the block is in the active chain.

The struct is also designed to work with the
FindBlock trait, which defines a method for
finding a block by its hash and returning
a boolean indicating whether or not the block was
found. The FindFirstBlockWithTimeAndHeight trait,
on the other hand, allows you to find the first
block in the chain with a timestamp greater than
or equal to a specified time and a height greater
than or equal to a specified height.

Overall, the FoundBlock functionality provides
a convenient way to locate and retrieve specific
blocks within the blockchain, allowing developers
to build more advanced functionality on top of the
Bitcoin Core codebase.

184> who wants to find the blocks?

The `FoundBlock` struct is mainly used by the
Bitcoin node software to keep track of block
metadata and contents while traversing the
blockchain. The software needs to find and
retrieve specific blocks in order to validate new
transactions and blocks, and to synchronize with
the rest of the network. The `FoundBlock` struct
provides a convenient way to selectively return
pieces of block data based on what is needed by
the software. The name "FoundBlock" is likely
derived from the fact that it represents a block
that has been found in the blockchain by the
software.
