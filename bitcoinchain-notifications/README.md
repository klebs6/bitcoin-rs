# bitcoinchain-notifications crate

The `BitcoinChain-Notifications` crate provides
chain notifications for Bitcoin clients. This
crate contains trait definitions for various
events that clients can subscribe to in order to
be notified about various changes in the Bitcoin
chain.

The `ChainNotifications` trait is the main trait
for listening to chain notifications. This trait
is composed of other traits, including
`TransactionAddedToMempool`,
`TransactionRemovedFromMempool`, `BlockConnected`,
`BlockDisconnected`, `UpdatedBlockTip`, and
`ChainStateFlushed`. Each of these traits defines
a method for listening to a specific type of
event.

The `TransactionAddedToMempool` trait defines
a method that is called when a transaction is
added to the mempool. 

The `TransactionRemovedFromMempool` trait defines
a method that is called when a transaction is
removed from the mempool. 

This notification is fired for transactions that
are removed from the mempool for various reasons
including expiration, size limit, reorg, conflict,
and replaced. 

However, it does not fire for transactions that
are removed from the mempool because they have
been included in a block. Any client interested in
transactions removed from the mempool for
inclusion in a block can learn about those
transactions from the `BlockConnected`
notification.

The `BlockConnected` trait defines a method that
is called when a new block is connected to the
chain. 

The `BlockDisconnected` trait defines a method
that is called when a block is disconnected from
the chain. 

The `UpdatedBlockTip` trait defines a method that
is called when the block tip is updated. 

Finally, the `ChainStateFlushed` trait defines
a method that is called when the chain state is
flushed.

All of these traits are designed to be used on
a background thread.

To use this crate, you can implement any of the
traits in your client code and register your
implementation with the client's notification
system. When a chain event occurs that matches
your implementation, the corresponding method will
be called.
