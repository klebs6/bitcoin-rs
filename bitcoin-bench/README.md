# bitcoin-bench

This Rust crate is a port of the Bitcoin Core
benchmarking framework, which allows for
benchmarking various components of the Bitcoin
system. The crate is still in the process of being
translated from C++ to Rust, and some function
bodies may still be in the process of translation.

The benchmarks in this crate cover a wide range of
Bitcoin system components, including but not
limited to: transaction addition to the mempool,
block assembly, bech32 encoding and decoding,
available coins selection, GCS filters, hash
functions, and chacha20-poly1305 AEAD encryption.

The benchmarks in this crate serve an important
role in measuring and improving the performance of
the Bitcoin system. By identifying performance
bottlenecks and areas for improvement, developers
can work to optimize and improve the efficiency of
the system.

Some of the notable benchmarks in this crate include:

- `add_tx`: measures the time it takes to add
  a transaction to the mempool

- `assemble_block`: measures the time it takes to
  assemble a block from a set of transactions

- `bech_32encode` and `bech_32decode`: measures
  the time it takes to encode and decode Bech32
  addresses

- `available::coin_selection`: measures the
  performance of the available coins selection
  algorithm used in coin selection for
  transactions

- `poly1305_1mb`: measures the time it takes to
  perform a Poly1305 hash on a 1MB message

Overall, the bitcoin-bench crate is an important
tool for developers working on the Bitcoin system,
providing a means to measure and optimize system
performance.

22> do you think in the long run it is better for
these benchmarks to exist alongside the
implementations of the functionalities which they
are benchmarking?

Yes, it is generally considered best practice to
have benchmarks alongside the implementation of
the functionalities they are measuring. This is
because benchmarks can help identify performance
bottlenecks and provide insights into how to
optimize code. By having benchmarks as part of the
implementation, developers can easily track the
performance of their code and make any necessary
optimizations as the code evolves. Additionally,
having benchmarks available to users can help them
make informed decisions about which implementation
to use for their specific use case, based on
performance metrics.

23> ok. one day it this will be done.  do you
think it is okay to keep it as-is for now, while
the rest of the codebase is being translated?

Yes, it's perfectly fine to keep the benchmark
code separate for now and focus on translating the
rest of the codebase. Once the translation is
complete, it can be integrated with the main
codebase and used for performance testing and
optimization. In the meantime, it's important to
make sure that the benchmark code is kept up to
date and accurate so that it can be used
effectively in the future.
