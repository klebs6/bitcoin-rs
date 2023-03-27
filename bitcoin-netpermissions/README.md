# `bitcoin-netpermissions`

This Rust crate is a direct translation of the
Bitcoin codebase from C++ to Rust. It is currently
in the process of translation, and some function
bodies may still be in the process of being
translated.

The `NetPermissionFlags` enum and `NetPermissions`
struct define the various network permissions and
their associated flags. These permissions are used
to determine what network activities a node can
perform.

The `NetWhitelistPermissions` and
`NetWhitebindPermissions` structs are used to
define a list of IP addresses that are allowed to
connect to and bind to a node's network.

This crate provides functions to add, clear, and
check permissions flags, as well as to parse
permission flags and IP address lists.

While there are no specific mathematical equations
or concepts involved in the
`bitcoin-netpermissions` crate, understanding the
role of network permissions and IP address
whitelists is crucial to ensuring the security and
integrity of the Bitcoin network.

36> thanks -- could you teach me about the
interface provided by this crate?

Certainly! The `NetPermissions` type is the
primary interface provided by this crate. It
represents a set of permission flags that
determine which network services a node is allowed
to provide or consume. The permission flags are
represented as bit flags that can be set or
cleared individually using the `add_flag` and
`clear_flag` methods.

In addition to the permission flags,
`NetPermissions` also provides a few utility
methods. The `has_flag` method allows you to check
whether a particular permission flag is set, while
the `to_strings` method returns a vector of
strings representing the permission flags that are
currently set.

Finally, `NetPermissions` also provides
a `try_parse` method that can be used to construct
a new instance from a string representation of
permission flags. This method returns an
`Option<NetPermissions>` type, which will be
`Some(permissions)` if the string was successfully
parsed, or `None` otherwise.

Overall, the `NetPermissions` type provides
a simple but flexible interface for managing
network permission flags in a Bitcoin node.
