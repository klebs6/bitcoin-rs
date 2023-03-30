## bitcoinnode-stats

This Rust crate is a subcomponent of the Bitcoin
system, providing functionality for gathering and
reporting statistics about nodes in the network.

Notice: This crate is part of a direct translation
from C++ to Rust of the Bitcoin core. As such,
some of the function bodies may still be in the
process of translation. Please note that this
system will become testable once the translation
process is complete.

The `NodeStats` struct represents the statistics
for a single node, including information on the
node's version, services, and the time it was last
seen. The `NodesStats` struct holds a vector of
`NodeStats` objects, allowing for reporting on
multiple nodes. The `NodeStateStats` struct
provides additional statistics for a node's state,
including the number of connections and the number
of active threads.

There are no specific mathematical concepts
associated with the tokens in this crate.

95> which stats do we want to track for a bitcoin
node?

There are several stats that can be useful to
track for a Bitcoin node, depending on what
information you're interested in. Here are a few
examples:

1. Block and transaction throughput: This tracks
   the number of blocks and transactions processed
   by the node over time. It can give you an idea
   of the network's overall usage and how much
   load your node is handling.

2. Uptime and downtime: This tracks how long your
   node has been running without any issues
   (uptime) and how long it has been down or
   experiencing issues (downtime). It can help you
   identify any reliability issues with your node
   or with the network as a whole.

3. Peer connections: This tracks the number of
   peers your node is connected to over time. It
   can give you an idea of how well-connected your
   node is to the network and whether you need to
   adjust your settings to establish more
   connections.

4. Memory and disk usage: This tracks how much
   memory and disk space your node is using over
   time. It can help you identify any resource
   constraints on your system and whether you need
   to optimize your node's configuration.

5. Mempool size: This tracks the size of your
   node's mempool over time. It can give you an
   idea of how much unconfirmed transaction
   traffic is flowing through the network and how
   much of that traffic your node is handling.

6. Fee estimation accuracy: This tracks the
   accuracy of your node's fee estimation
   algorithm over time. It can help you identify
   any issues with your algorithm and whether you
   need to make adjustments to ensure your node is
   providing accurate fee estimates.

These are just a few examples, and there are many
other stats that could be tracked depending on
your specific needs and interests.

96> what are some others?

Here are some additional stats that could be
useful to track for a Bitcoin node:

- Uptime: This would track how long the node has
  been running continuously without any
  downtime. It can help identify stability issues
  and maintenance requirements.

- Block validation time: This would track the time
  it takes for the node to validate a new block
  after receiving it. It can help identify
  performance bottlenecks and resource utilization
  issues.

- Memory usage: This would track how much memory
  the node is using over time. It can help
  identify memory leaks and other memory-related
  issues.

- Disk usage: This would track how much disk space
  the node is using over time. It can help
  identify storage-related issues and help plan
  for storage requirements.

- Transaction processing time: This would track
  the time it takes for the node to process new
  transactions after receiving them. It can help
  identify performance bottlenecks and resource
  utilization issues.

- Network latency: This would track the time it
  takes for the node to receive data from other
  nodes in the network. It can help identify
  connectivity and networking issues.

- Connection count: This would track how many
  connections the node has to other nodes in the
  network. It can help identify network topology
  issues and resource utilization issues.

- Bandwidth usage: This would track how much data
  the node is sending and receiving over the
  network. It can help identify bandwidth-related
  issues and help plan for bandwidth requirements.

- Fee estimation accuracy: This would track how
  accurate the node's fee estimation algorithm is
  compared to actual transaction fees. It can help
  identify issues with the fee estimation
  algorithm and help optimize transaction
  processing.

- Mempool size: This would track how many
  unconfirmed transactions are currently in the
  node's mempool. It can help identify
  mempool-related issues and help optimize
  transaction processing.

97> what are the acceptable values associated with
these metrics?

The acceptable values for the metrics associated
with a Bitcoin node will depend on the specific
metric. Here are some examples:

- Block height: This should be a non-negative
  integer representing the height of the current
  best block known to the node.

- Difficulty: This should be a positive
  floating-point number representing the current
  difficulty of the Bitcoin network.

- Hashrate: This should be a positive
  floating-point number representing the estimated
  hashrate of the Bitcoin network in hashes per
  second.

- Memory usage: This should be a non-negative
  integer representing the amount of memory (in
  bytes) currently used by the Bitcoin node.

- Uptime: This should be a non-negative integer
  representing the number of seconds the node has
  been running without interruption.

- Peers: This should be a non-negative integer
  representing the number of connected peers the
  node has.

- Transactions: This should be a non-negative
  integer representing the total number of
  transactions the node has processed.

- Transactions per second (TPS): This should be
  a positive floating-point number representing
  the average number of transactions processed per
  second by the node over a specific time period.

- Mempool size: This should be a non-negative
  integer representing the current size of the
  node's transaction memory pool (mempool).

- Fee estimates: These should be positive
  floating-point numbers representing the
  estimated fees (in satoshis per byte) required
  to get a transaction confirmed within a certain
  number of blocks.

98> what are their ranges in practice?

The acceptable ranges for various Bitcoin node
metrics can vary depending on the specific
implementation, configuration, and network
conditions. Here are some approximate ranges for
a few common metrics:

- Block size: up to 4 MB for Bitcoin and up to 32
  MB for Bitcoin Cash

- Block interval: 10 minutes for Bitcoin and 10
  minutes to 20 seconds for Bitcoin Cash depending
  on difficulty adjustments

- Hash rate: varies widely but can be measured in
  exahashes per second (EH/s) for Bitcoin

- Number of transactions per second: up to a few
  hundred transactions per second for Bitcoin and
  several thousand transactions per second for
  Bitcoin Cash

- Transaction fee: usually measured in satoshis
  per byte (sat/B), with typical fees ranging from
  1-10 sat/B for low priority transactions and up
  to several hundred sat/B for priority
  transactions

- Memory usage: can vary widely depending on the
  node implementation, but typically ranges from
  several hundred megabytes to several gigabytes

- Disk space usage: depends on the size of the
  blockchain, which is currently over 400 GB for
  Bitcoin and over 50 GB for Bitcoin Cash as of
  March 2023.
