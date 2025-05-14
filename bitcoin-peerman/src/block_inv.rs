// ---------------- [ File: bitcoin-peerman/src/block_inv.rs ]
crate::ix!();

pub struct PeerBlockInv {

    /**
      | List of blocks that we'll announce via
      | an `inv` message.
      | 
      | There is no final sorting before sending,
      | as they are always sent immediately
      | and in the order requested.
      | 
      |
      */
    pub blocks_for_inv_relay:     Vec<u256>,

    /**
      | Unfiltered list of blocks that we'd
      | like to announce via a `headers` message.
      | If we can't announce via a `headers`
      | message, we'll fall back to announcing
      | via `inv`.
      | 
      |
      */
    pub blocks_for_headers_relay: Vec<u256>,


    /**
      | The final block hash that we sent in an
      | `inv` message to this peer.
      | 
      | When the peer requests this block, we
      | send an `inv` message to trigger the
      | peer to request the next sequence of
      | block hashes.
      | 
      | Most peers use headers-first syncing,
      | which doesn't use this mechanism
      | 
      |
      */
    pub continuation_block:       u256,
}
