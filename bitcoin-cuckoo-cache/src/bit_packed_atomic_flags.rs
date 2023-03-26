crate::ix!();

/**
  | @ref bit_packed_atomic_flags implements
  | a container for garbage collection
  | flags that is only thread unsafe on calls
  | to setup. This class bit-packs collection
  | flags for memory efficiency.
  | 
  | All operations are `std::memory_order_relaxed`
  | so external mechanisms must ensure
  | that writes and reads are properly synchronized.
  | 
  | On setup(n), all bits up to `n` are marked
  | as collected.
  | 
  | Under the hood, because it is an 8-bit
  | type, it makes sense to use a multiple
  | of 8 for setup, but it will be safe if that
  | is not the case as well.
  |
  | No default constructor, as there must
  | be some size.
  |
  */
pub struct BitPackedAtomicFlags {
    mem: Box<Rc<[AtomicU8]>>,
}

impl BitPackedAtomicFlags {

    /**
      | bit_packed_atomic_flags constructor
      | creates memory to sufficiently keep
      | track of garbage collection information
      | for `size` entries.
      | 
      | -----------
      | @param size
      | 
      | the number of elements to allocate space
      | for @post bit_set, bit_unset, and bit_is_set
      | function properly forall x. x < size
      | @post All calls to bit_is_set (without
      | subsequent bit_unset) will return
      | true.
      |
      */
    pub fn new(size: u32) -> Self {
    
        todo!();
        /*


            // pad out the size if needed
                size = (size + 7) / 8;
                mem.reset(new std::atomic<uint8_t>[size]);
                for (uint32_t i = 0; i < size; ++i)
                    mem[i].store(0xFF);
        */
    }

    /**
      | setup marks all entries and ensures
      | that bit_packed_atomic_flags can
      | store at least `b` entries.
      | 
      | -----------
      | @param b
      | 
      | the number of elements to allocate space
      | for @post bit_set, bit_unset, and bit_is_set
      | function properly forall x. x < b @post
      | All calls to bit_is_set (without subsequent
      | bit_unset) will return true.
      |
      */
    #[inline] pub fn setup(&mut self, b: u32)  {
        
        todo!();
        /*
            bit_packed_atomic_flags d(b);
                std::swap(mem, d.mem);
        */
    }

    /**
      | bit_set sets an entry as discardable.
      | 
      | -----------
      | @param s
      | 
      | the index of the entry to bit_set @post
      | immediately subsequent call (assuming
      | proper external memory ordering) to
      | bit_is_set(s) == true.
      |
      */
    #[inline] pub fn bit_set(&mut self, s: u32)  {
        
        todo!();
        /*
            mem[s >> 3].fetch_or(1 << (s & 7), std::memory_order_relaxed);
        */
    }

    /**
      | bit_unset marks an entry as something
      | that should not be overwritten.
      | 
      | -----------
      | @param s
      | 
      | the index of the entry to bit_unset @post
      | immediately subsequent call (assuming
      | proper external memory ordering) to
      | bit_is_set(s) == false.
      |
      */
    #[inline] pub fn bit_unset(&mut self, s: u32)  {
        
        todo!();
        /*
            mem[s >> 3].fetch_and(~(1 << (s & 7)), std::memory_order_relaxed);
        */
    }

    /**
      | bit_is_set queries the table for discardability
      | at `s`.
      | 
      | -----------
      | @param s
      | 
      | the index of the entry to read
      | 
      | -----------
      | @return
      | 
      | true if the bit at index `s` was set, false
      | otherwise
      |
      */
    #[inline] pub fn bit_is_set(&self, s: u32) -> bool {
        
        todo!();
        /*
            return (1 << (s & 7)) & mem[s >> 3].load(std::memory_order_relaxed);
        */
    }
}
