// ---------------- [ File: bitcoinleveldb-skiplist/src/lib.rs ]
/*!
  | Thread safety -------------
  |
  | Writes require external synchronization, most
  | likely a mutex.  Reads require a guarantee that
  | the SkipList will not be destroyed while the
  | read is in progress.  Apart from that, reads
  | progress without any internal locking or
  | synchronization.
  |
  | Invariants:
  |
  | (1) Allocated nodes are never deleted until the
  | SkipList is destroyed.  This is trivially
  | guaranteed by the code since we never delete
  | any skip list nodes.
  |
  | (2) The contents of a SkipListNode except for the
  | next/prev pointers are immutable after the SkipListNode
  | has been linked into the SkipList.  Only
  | Insert() modifies the list, and it is careful
  | to initialize a node and use release-stores to
  | publish the nodes in one or more lists.
  |
  | ... prev vs. next pointer ordering ...
  */
//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/skiplist.h]
#[macro_use] mod imports; use imports::*;

x!{skip_list}
x!{skip_list_comparator}
x!{skip_list_iterator}
x!{skip_list_node}
