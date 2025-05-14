// ---------------- [ File: bitcoin-mem/src/dynamic_usage.rs ]
crate::ix!();

pub trait DynamicUsage {
    fn dynamic_usage(&self) -> usize;
}

pub trait IncrementalDynamicUsage {
    fn incremental_dynamic_usage(&self) -> usize;
}

/**
 | Dynamic memory usage for built-in types
 | is zero.
 |
 */
impl DynamicUsage for i8 {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

impl DynamicUsage for u8 {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

impl DynamicUsage for i16 {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

impl DynamicUsage for u16 {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

impl DynamicUsage for i32 {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

impl DynamicUsage for u32 {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

impl DynamicUsage for i64 {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

impl DynamicUsage for u64 {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

impl DynamicUsage for f32 {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

impl DynamicUsage for f64 {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

impl<X> DynamicUsage for *mut X {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

impl<X> DynamicUsage for *const X {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
        /*
        return 0;
        */
    }
}

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

impl<X> DynamicUsage for Vec<X> {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
            /*
                return MallocUsage(v.capacity() * sizeof(X));
            */
    }
}

impl<T: Default,const N: usize> DynamicUsage for PreVector<T,N> {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
            /*
                return MallocUsage(v.allocated_memory());
            */
    }
}

impl<X,Y> DynamicUsage for HashSet<X,Y> {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
            /*
                return MallocUsage(sizeof(stl_tree_node<X>)) * s.size();
            */
    }
}

impl<X, Y> IncrementalDynamicUsage for HashSet<X,Y> {

    #[inline] fn incremental_dynamic_usage(&self) -> usize {

        todo!();
            /*
                return MallocUsage(sizeof(stl_tree_node<X>));
            */
    }
}

impl<X, Y, Z> DynamicUsage for HashMap<X,Y,Z> {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
            /*
                return MallocUsage(sizeof(stl_tree_node<std::pair<const X, Y> >)) * m.size();
            */
    }
}

impl<X,Y,Z> IncrementalDynamicUsage for HashMap<X,Y,Z> {

    #[inline] fn incremental_dynamic_usage(&self) -> usize {

        todo!();
            /*
                return MallocUsage(sizeof(stl_tree_node<std::pair<const X, Y> >));
            */
    }
}

impl<X> DynamicUsage for Box<X> {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
            /*
                return p ? MallocUsage(sizeof(X)) : 0;
            */
    }
}

impl<X> DynamicUsage for Arc<X> {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
            /*
                // A shared_ptr can either use a single continuous memory block for both
            // the counter and the storage (when using std::make_shared), or separate.
            // We can't observe the difference, however, so assume the worst.
            return p ? MallocUsage(sizeof(X)) + MallocUsage(sizeof(stl_shared_counter)) : 0;
            */
    }
}

pub struct UnorderedNode<X> {
    base: X,
    ptr:  *mut c_void,
}

lazy_static!{
    /*
        impl<X, Y> DynamicUsage for HashSet<X,Y> {

            #[inline] fn dynamic_usage(&self) -> usize {

                todo!();
                    /*
                        return MallocUsage(sizeof(unordered_node<X>)) * s.size() + MallocUsage(sizeof(c_void*) * s.bucket_count());
                    */
            }
        }

        impl<X, Y, Z> DynamicUsage for HashMap<X,Y,Z> {

            #[inline] fn dynamic_usage(&self) -> usize {

                todo!();
                    /*
                        return MallocUsage(sizeof(unordered_node<std::pair<const X, Y> >)) * m.size() + MallocUsage(sizeof(c_void*) * m.bucket_count());
                    */
            }
        }
    */
}
