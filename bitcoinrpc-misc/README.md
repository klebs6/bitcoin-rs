## bitcoinrpc-misc

This Rust crate is a subcomponent of the Bitcoin
system, providing various miscellaneous functions
for interacting with a Bitcoin node via its remote
procedure call (RPC) interface.

**Notice:** This crate is part of a direct
translation from C++ to Rust of the Bitcoin
core. As such, some of the function bodies may
still be in the process of translation. Please
note that this system will become testable once
the translation process is complete.

Below are the RPC functions provided by this
crate:

- `createmultisig`: creates a P2SH multi-signature
  address.

- `deriveaddresses`: derives a set of addresses
  from an extended public key or descriptor.

- `echo`: returns the same input as output.

- `echo_default`: returns a hardcoded string.

- `echoipc`: sends a message to the Bitcoin IPC
  interface and returns the same message as
  output.

- `echojson`: returns the same input as output in
  JSON format.

- `enable_or_disable_log_categories`: enables or
  disables logging for specified log categories.

- `getdescriptorinfo`: returns information about
  a descriptor, such as the number of public keys
  it contains.

- `getindexinfo`: returns information about the
  transaction index database.

- `getmemoryinfo`: returns information about the
  node's memory usage.

- `invokedisallowedsyscall`: invokes a disallowed
  system call for testing purposes.

- `logging`: returns information about the node's
  logging configuration.

- `mockscheduler`: simulates a block scheduler for
  testing purposes.

- `register_misc_rpc_commands`: registers
  miscellaneous RPC commands with the Bitcoin
  node.

- `rpc_locked_memory_info`: returns information
  about the node's locked memory usage.

- `rpc_malloc_info`: returns information about the
  node's memory allocation usage.

- `setmocktime`: sets the node's system time for
  testing purposes.

- `signmessagewithprivkey`: signs a message with
  a private key.

- `summary_tojson`: returns a summary of the
  node's information in JSON format.

- `to`: prints a string to the node's standard
  output.

- `validateaddress`: validates a Bitcoin address.

- `verifymessage`: verifies a message signed with
  a Bitcoin address and its corresponding
  signature.

There are no mathematical ideas associated with
these RPC functions.
