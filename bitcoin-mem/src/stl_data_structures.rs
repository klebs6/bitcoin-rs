crate::ix!();

// STL data structures

pub struct StlTreeNode<X> {
    color:  i32,
    parent: *mut c_void,
    left:   *mut c_void,
    right:  *mut c_void,
    x:      X,
}

pub struct StlSharedCounter
{
    /**
      | Various platforms use different sized
      | counters here.
      | 
      | Conservatively assume that they won't
      | be larger than size_t.
      |
      */
    class_type: *mut c_void,

    use_count:  usize,
    weak_count: usize,
}
