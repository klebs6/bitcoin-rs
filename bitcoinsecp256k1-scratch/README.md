# bitcoinsecp256k1-scratch

This Rust crate provides a `Scratch` type and
associated functions for managing memory
allocation in the bitcoinsecp256k1 library. This
library is part of a direct translation from C++
to Rust of the bitcoin core, and as such, some of
the function bodies may still be in the process of
translation. Please note that this system will
become testable once the translation process is
complete.

The `Scratch` type is used to allocate temporary
memory for computations within the
bitcoinsecp256k1 library. This is important
because certain computations can be quite
memory-intensive, and by using a scratch space,
the library can minimize memory allocation and
deallocation overhead.

The `scratch_create` function is used to create
a new scratch space, and `scratch_destroy` is used
to free the memory allocated for the scratch space
once it is no longer needed. The `scratch_alloc`
function is used to allocate a block of memory
within the scratch space, and
`scratch_max_allocation` returns the maximum
amount of memory that can be allocated within
a scratch space.

The `scratch_checkpoint` and
`scratch_apply_checkpoint` functions are used to
manage the scratch space as a stack of memory
allocations. `scratch_checkpoint` creates a new
checkpoint within the scratch space, while
`scratch_apply_checkpoint` frees all memory
allocated since the last checkpoint.

There are no direct mathematical ideas associated
with these functions, as they are primarily
concerned with memory management. However,
efficient memory management is crucial for the
performance of cryptographic operations, and so
the use of scratch spaces can have important
implications for the efficiency of elliptic curve
cryptography on the secp256k1 curve.

26> what is the memory footprint of the
computations involved?

The memory footprint of computations involved in
elliptic curve cryptography on the secp256k1 curve
can vary widely depending on the specific
operation being performed. Some operations, such
as point addition and scalar multiplication, can
be relatively memory-efficient, requiring only
a few temporary variables for intermediate
calculations. Other operations, such as point
multiplication and signature verification, can
require much larger amounts of memory,
particularly when working with multiple points or
signatures simultaneously.

The use of scratch spaces, such as those provided
by the `Scratch` type in the
`bitcoinsecp256k1-scratch` crate, can help to
minimize the memory footprint of these
computations by reusing memory allocated for
intermediate calculations. This can lead to
significant performance improvements, particularly
when working with large numbers of points or
signatures.

27> let's go through the operations one by one to
analyze their memory performance

Sure, I'd be happy to help with that! Here's
a brief overview of the memory requirements for
some common operations in elliptic curve
cryptography on the secp256k1 curve:

- Point addition: This operation involves adding
  two points on the curve to obtain a third
  point. The memory requirements for this
  operation are relatively low, as it only
  involves a few temporary variables to hold
  intermediate calculations. Typically, only
  a single scratch space is required for this
  operation.

- Scalar multiplication: This operation involves
  multiplying a scalar value by a point on the
  curve to obtain another point on the curve. The
  memory requirements for this operation are also
  relatively low, as it only involves a few
  temporary variables to hold intermediate
  calculations. Typically, only a single scratch
  space is required for this operation.

- Point multiplication: This operation involves
  multiplying a point on the curve by a scalar
  value to obtain another point on the curve. The
  memory requirements for this operation can be
  higher than for scalar multiplication, as it
  typically involves working with multiple points
  and requires more intermediate
  calculations. Multiple scratch spaces may be
  required to perform this operation efficiently.

- Signature verification: This operation involves
  verifying the validity of a signature on
  a message using a public key on the curve. The
  memory requirements for this operation can be
  quite high, particularly when working with
  multiple signatures or public keys
  simultaneously. Multiple scratch spaces may be
  required to perform this operation efficiently.

- Signature generation: This operation involves
  generating a signature on a message using
  a private key on the curve. The memory
  requirements for this operation are typically
  low, as it only involves a few temporary
  variables to hold intermediate
  calculations. Typically, only a single scratch
  space is required for this operation.

Overall, the memory requirements for elliptic
curve cryptography on the secp256k1 curve can vary
widely depending on the specific operation being
performed, the size of the data being processed,
and the number of points or signatures being
worked with simultaneously. Effective use of
scratch spaces can help to minimize the memory
footprint of these operations and improve overall
performance.

28> when you say "multiple scratch spaces", how
many? how much memory total per scratch space?

The number of scratch spaces required and the size
of each scratch space depend on the specific
operation being performed and the memory
constraints of the system. In general, a single
scratch space can be used for simple operations
like point addition or scalar multiplication,
while more complex operations like point
multiplication or signature verification may
require multiple scratch spaces.

The size of each scratch space also depends on the
specific operation being performed and the
available memory. For example, the
`bitcoinsecp256k1-scratch` crate provides
a `scratch_max_allocation` function that returns
the maximum amount of memory that can be allocated
for a single scratch space. According to the
crate's documentation, this value is currently set
to 262,144 bytes (256 KiB), although this may
change depending on the specific system and
configuration.

In practice, the number of scratch spaces and
their size will depend on the specific application
and the available resources. For systems with
limited memory, it may be necessary to use smaller
scratch spaces and/or reuse scratch spaces for
multiple operations to conserve memory. In
general, it is important to carefully manage
memory usage when performing cryptographic
operations to ensure that sensitive data is not
leaked or compromised.
