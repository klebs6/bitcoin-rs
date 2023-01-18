crate::ix!();

/**
  | Helper class to limit resource usage to avoid
  | exhaustion.
  |
  | Currently used to limit read-only file
  | descriptors and mmap file usage so that we do
  | not run out of file descriptors or virtual
  | memory, or run into kernel performance problems
  | for very large databases.
  */
pub struct Limiter {

    /**
      | The number of available resources.
      |
      | This is a counter and is not tied to the
      | invariants of any other class, so it can be
      | operated on safely using
      | std::memory_order_relaxed.
      */
    acquires_allowed: Atomic<i32>,
}

impl Limiter {

    /**
      | Limit maximum number of resources to
      | max_acquires|.
      |
      */
    pub fn new(max_acquires: i32) -> Self {
    
        todo!();
        /*
        : acquires_allowed(max_acquires),

        
        */
    }

    /**
      | If another resource is available, acquire it
      | and return true.
      |
      | Else return false.
      */
    pub fn acquire(&mut self) -> bool {
        
        todo!();
        /*
            int old_acquires_allowed =
            acquires_allowed_.fetch_sub(1, std::memory_order_relaxed);

        if (old_acquires_allowed > 0) return true;

        acquires_allowed_.fetch_add(1, std::memory_order_relaxed);
        return false;
        */
    }

    /**
      | Release a resource acquired by a previous
      | call to Acquire() that returned true.
      |
      */
    pub fn release(&mut self)  {
        
        todo!();
        /*
            acquires_allowed_.fetch_add(1, std::memory_order_relaxed);
        */
    }
}

