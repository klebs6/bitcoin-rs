// ---------------- [ File: bitcoinleveldb-skiplist/src/skiplist.rs ]
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

crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/skiplist.h]

pub struct SkipList<Comparator> {

    /**
      | Immutable after construction
      |
      */
    compare:    Comparator,

    /**
      | Arena used for allocations of nodes
      |
      */
    arena:      *const Arena,

    head:       *const SkipListNode,

    /**
      | Modified only by Insert(). Read racily
      | by readers, but stale values are ok.
      | 
      | Height of the entire list
      |
      */
    max_height: Atomic<i32>,

    /**
      | Read/written only by Insert().
      |
      */
    rnd:        Random,
}

pub const SkipListMaxHeight: usize = 12;

/**
  | Iteration over the contents of a skip
  | list
  |
  | Intentionally copyable
  */
pub struct SkipListIterator {
    list: *mut SkipList<Box<dyn SliceComparator>>,
    node: *mut SkipListNode,
}

impl Clone for SkipListIterator {
    fn clone(&self) -> Self {
        todo!();
    }
}

impl SkipListIterator {

    /**
      | Initialize an iterator over the specified
      | list.
      |
      | The returned iterator is not valid.
      */
    pub fn new(list: *const SkipList<Box<dyn SliceComparator>>) -> Self {
    
        todo!();
        /*
           list_ = list;
           node_ = nullptr;
           */
    }
    
    /**
      | Returns true iff the iterator is positioned
      | at a valid node.
      |
      */
    #[inline] pub fn valid(&self) -> bool {
        
        todo!();
        /*
            return node_ != nullptr;
        */
    }
    
    /**
      | Returns the key at the current position.
      | 
      | REQUIRES: Valid()
      |
      */
    #[inline] pub fn key(&self) -> &dyn Key {
        
        todo!();
        /*
            assert(Valid());
      return node_->key;
        */
    }
    
    /**
      | Advances to the next position.
      | 
      | REQUIRES: Valid()
      |
      */
    #[inline] pub fn next(&mut self)  {
        
        todo!();
        /*
            assert(Valid());
      node_ = node_->Next(0);
        */
    }
    
    /**
      | Advances to the previous position.
      | 
      | REQUIRES: Valid()
      |
      */
    #[inline] pub fn prev(&mut self)  {
        
        todo!();
        /*
            // Instead of using explicit "prev" links, we just search for the
      // last node that falls before key.
      assert(Valid());
      node_ = list_->FindLessThan(node_->key);
      if (node_ == list_->head_) {
        node_ = nullptr;
      }
        */
    }
    
    /**
      | Advance to the first entry with a key
      | >= target
      |
      */
    #[inline] pub fn seek(&mut self, target: &dyn Key)  {
        
        todo!();
        /*
            node_ = list_->FindGreaterOrEqual(target, nullptr);
        */
    }
    
    /**
      | Position at the first entry in list.
      |
      | Final state of iterator is Valid() iff list
      | is not empty.
      */
    #[inline] pub fn seek_to_first(&mut self)  {
        
        todo!();
        /*
            node_ = list_->head_->Next(0);
        */
    }
    
    /**
      | Position at the last entry in list.
      |
      | Final state of iterator is Valid() iff list
      | is not empty.
      */
    #[inline] pub fn seek_to_last(&mut self)  {
        
        todo!();
        /*
            node_ = list_->FindLast();
      if (node_ == list_->head_) {
        node_ = nullptr;
      }
        */
    }
}

/**
  | Implementation details follow
  |
  */
pub struct SkipListNode {
    key_:  Box<dyn Key>,

    /**
      | Array of length equal to the node height.
      | next_[0] is lowest level link.
      |
      */
    next: [Atomic<*mut SkipListNode>; 1],
}

impl SkipListNode {

    pub fn new(k: &Box<dyn Key>) -> Self {
    
        todo!();
        /*
        : key(k),
        */
    }

    /**
      | Accessors/mutators for links. Wrapped
      | in methods so we can add the appropriate
      | barriers as necessary.
      |
      */
    pub fn next(&mut self, n: i32) -> *mut SkipListNode {
        
        todo!();
        /*
            assert(n >= 0);
        // Use an 'acquire load' so that we observe a fully initialized
        // version of the returned SkipListNode.
        return next_[n].load(std::memory_order_acquire);
        */
    }
    
    pub fn set_next(&mut self, 
        n: i32,
        x: *mut SkipListNode)  {
        
        todo!();
        /*
            assert(n >= 0);
        // Use a 'release store' so that anybody who reads through this
        // pointer observes a fully initialized version of the inserted node.
        next_[n].store(x, std::memory_order_release);
        */
    }

    /**
      | No-barrier variants that can be safely
      | used in a few locations.
      |
      */
    pub fn no_barrier_next(&mut self, n: i32) -> *mut SkipListNode {
        
        todo!();
        /*
            assert(n >= 0);
        return next_[n].load(std::memory_order_relaxed);
        */
    }
    
    pub fn no_barrier_set_next(&mut self, 
        n: i32,
        x: *mut SkipListNode)  {
        
        todo!();
        /*
            assert(n >= 0);
        next_[n].store(x, std::memory_order_relaxed);
        */
    }
}

impl<C: SliceComparator> SkipList<C> {

    #[inline] pub fn get_max_height(&self) -> i32 {
        
        todo!();
        /*
            return max_height_.load(std::memory_order_relaxed);
        */
    }
    
    pub fn equal(&self, a: &Box<dyn Key>, b: &Box<dyn Key>) -> bool {
        
        todo!();
        /*
            return (compare_(a, b) == 0);
        */
    }

    pub fn new_node(&mut self, 
        key_:    &Box<dyn Key>,
        height: i32) -> *mut SkipListNode {
        
        todo!();
        /*
            char* const node_memory = arena_->AllocateAligned(
          sizeof(SkipListNode) + sizeof(std::atomic<SkipListNode*>) * (height - 1));
      return new (node_memory) SkipListNode(key);
        */
    }
    
    pub fn random_height(&mut self) -> i32 {
        
        todo!();
        /*
            // Increase height with probability 1 in kBranching
      static const unsigned int kBranching = 4;
      int height = 1;
      while (height < SkipListMaxHeight && ((rnd_.Next() % kBranching) == 0)) {
        height++;
      }
      assert(height > 0);
      assert(height <= SkipListMaxHeight);
      return height;
        */
    }
    
    /**
      | Return true if key is greater than the
      | data stored in "n"
      |
      */
    pub fn key_is_after_node(&self, 
        key_: &Box<dyn Key>,
        n:   *mut SkipListNode) -> bool {
        
        todo!();
        /*
            // null n is considered infinite
      return (n != nullptr) && (compare_(n->key, key) < 0);
        */
    }
    
    /**
      | Return the earliest node that comes at or
      | after key.
      |
      | Return nullptr if there is no such node.
      |
      | If prev is non-null, fills prev[level] with
      | pointer to previous node at "level" for every
      | level in [0..max_height_-1].
      */
    pub fn find_greater_or_equal(&self, 
        key_:  &Box<dyn Key>,
        prev: *mut *mut SkipListNode) -> *mut SkipListNode {
        
        todo!();
        /*
            SkipListNode* x = head_;
      int level = GetMaxHeight() - 1;
      while (true) {
        SkipListNode* next = x->Next(level);
        if (KeyIsAfterNode(key, next)) {
          // Keep searching in this list
          x = next;
        } else {
          if (prev != nullptr) prev[level] = x;
          if (level == 0) {
            return next;
          } else {
            // Switch to next list
            level--;
          }
        }
      }
        */
    }
    
    /**
      | Return the latest node with a key < key.
      | 
      | Return head_ if there is no such node.
      |
      */
    pub fn find_less_than(&self, key_: &Box<dyn Key>) -> *mut SkipListNode {
        
        todo!();
        /*
            SkipListNode* x = head_;
      int level = GetMaxHeight() - 1;
      while (true) {
        assert(x == head_ || compare_(x->key, key) < 0);
        SkipListNode* next = x->Next(level);
        if (next == nullptr || compare_(next->key, key) >= 0) {
          if (level == 0) {
            return x;
          } else {
            // Switch to next list
            level--;
          }
        } else {
          x = next;
        }
      }
        */
    }
    
    /**
      | Return the last node in the list.
      | 
      | Return head_ if list is empty.
      |
      */
    pub fn find_last(&self) -> *mut SkipListNode {
        
        todo!();
        /*
            SkipListNode* x = head_;
      int level = GetMaxHeight() - 1;
      while (true) {
        SkipListNode* next = x->Next(level);
        if (next == nullptr) {
          if (level == 0) {
            return x;
          } else {
            // Switch to next list
            level--;
          }
        } else {
          x = next;
        }
      }
        */
    }
    
    /**
      | Create a new SkipList object that will use
      | "cmp" for comparing keys, and will allocate
      | memory using "*arena".  Objects allocated in
      | the arena must remain allocated for the
      | lifetime of the skiplist object.
      */
    pub fn new(
        cmp:   Box<dyn SliceComparator>,
        arena: *mut Arena) -> Self {
    
        todo!();
        /*
          : compare_(cmp),
          arena_(arena),
          head_(NewNode(0 /* any key will do */, kMaxHeight)),
          max_height_(1),
          rnd_(0xdeadbeef) 

          for (int i = 0; i < kMaxHeight; i++) {
            head_->SetNext(i, nullptr);
          }
        */
    }
    
    /**
      | Insert key into the list.
      |
      | REQUIRES: nothing that compares equal to key
      | is currently in the list.
      */
    pub fn insert(&mut self, key_: &Box<dyn Key>)  {
        
        todo!();
        /*
          // TODO(opt): We can use a barrier-free variant of FindGreaterOrEqual()
          // here since Insert() is externally synchronized.
          SkipListNode* prev[kMaxHeight];
          SkipListNode* x = FindGreaterOrEqual(key, prev);

          // Our data structure does not allow duplicate insertion
          assert(x == nullptr || !Equal(key, x->key));

          int height = RandomHeight();
          if (height > GetMaxHeight()) {
            for (int i = GetMaxHeight(); i < height; i++) {
              prev[i] = head_;
            }
            // It is ok to mutate max_height_ without any synchronization
            // with concurrent readers.  A concurrent reader that observes
            // the new value of max_height_ will see either the old value of
            // new level pointers from head_ (nullptr), or a new value set in
            // the loop below.  In the former case the reader will
            // immediately drop to the next level since nullptr sorts after all
            // keys.  In the latter case the reader will use the new node.
            max_height_.store(height, std::memory_order_relaxed);
          }

          x = NewNode(key, height);
          for (int i = 0; i < height; i++) {
            // NoBarrier_SetNext() suffices since we will add a barrier when
            // we publish a pointer to "x" in prev[i].
            x->NoBarrier_SetNext(i, prev[i]->NoBarrier_Next(i));
            prev[i]->SetNext(i, x);
          }
        */
    }
    
    /**
      | Returns true iff an entry that compares
      | equal to key is in the list.
      |
      */
    pub fn contains(&self, key_: &Box<dyn Key>) -> bool {
        
        todo!();
        /*
          SkipListNode* x = FindGreaterOrEqual(key, nullptr);

          if (x != nullptr && Equal(key, x->key)) {
            return true;
          } else {
            return false;
          }
        */
    }
}
