## bitcoin-epoch

This crate is part of a direct translation of the
Bitcoin Core C++ codebase to Rust. 

It provides types and functions for managing
epoch-based graph traversal algorithms, which are
used extensively in the Bitcoin Core codebase to
traverse transaction dependency graphs.

### Epoch

`Epoch` is the main type of the crate. It is
a RAII-style guard for using epoch-based graph
traversal algorithms. It is used to avoid visiting
the same transactions twice when walking ancestors
or descendants.

The type contains a `raw_epoch` field that
represents the current epoch, and a `guarded`
field that indicates whether the epoch is
currently being guarded by an `EpochGuard`. The
`default` function creates an `Epoch` instance
with a `raw_epoch` of 0 and `guarded` set to
`false`.

The type provides a `visited` function, which
takes a mutable reference to an `EpochMarker` and
returns a boolean indicating whether the marker
has already been visited during the current
epoch. The `guarded` function returns a boolean
indicating whether the epoch is currently being
guarded by an `EpochGuard`.

### EpochGuard

`EpochGuard` is a scoped lock guard that is used
to guard an epoch for the duration of a block. It
is responsible for incrementing the epoch when it
goes out of scope, to ensure clear separation
between epochs.

The type contains an `epoch` field that is an
`Rc<RefCell<Epoch>>`, representing the epoch being
guarded. The `new` function creates a new
`EpochGuard` instance and guards the epoch,
incrementing its `raw_epoch` field and setting its
`guarded` field to `true`. The guard is released
and the epoch's `guarded` field is set to `false`
when the guard goes out of scope and the guard's
`drop` function is called.

### EpochMarker

`EpochMarker` is a simple struct that contains
a `marker` field, which is used to keep track of
whether a transaction has already been visited
during the current epoch. The `marker` field is
a `u64` representing the epoch time.

The `default` function creates an `EpochMarker`
instance with a `marker` field of 0.

## Disclaimer

Note that some of the function bodies in this
crate may still be in the process of translation
from C++ to Rust. 
