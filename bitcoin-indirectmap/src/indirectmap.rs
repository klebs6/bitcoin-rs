crate::ix!();

pub trait HasIndirectMapTypes<'a> {
    type Iterator;
    type ConstIterator;
    type SizeType;
    type ValueType;
}

//-------------------------------------------[.cpp/bitcoin/src/indirectmap.h]

pub struct DereferencingComparator<T> {
    phantom: std::marker::PhantomData<T>,
}

impl<T> DereferencingComparator<T> {

    pub fn invoke(&self, a: T, b: T) -> bool {
        
        todo!();
        /*
            return *a < *b;
        */
    }
}

/**
  | Map whose keys are pointers, but are
  | compared by their dereferenced values.
  | 
  | Differs from a plain std::map<const
  | K*, T, DereferencingComparator<K*>
  | > in that methods that take a key for comparison
  | take a K rather than taking a K* (taking
  | a K* would be confusing, since it's the
  | value rather than the address of the
  | object for comparison that matters
  | due to the dereferencing comparator).
  | 
  | Objects pointed to by keys must not be
  | modified in any way that changes the
  | result of DereferencingComparator.
  |
  */
pub struct IndirectMap<K,T> {
    m: HashMap<Arc<K>,T,DereferencingComparator<Arc<K>>>,
}

impl<'a,K,T> HasIndirectMapTypes<'a> for IndirectMap<K,T> where T: 'a {
    type Iterator      = Box<dyn Iterator<Item = (Arc<K>, &'a T)>>;
    type ConstIterator = Box<dyn Iterator<Item = (Arc<K>, &'a T)>>;
    type SizeType      = usize;
    type ValueType     = T;
}

/**
  | indirectmap has underlying map with
  | pointer as key
  |
  */
impl<X, Y> DynamicUsage for IndirectMap<X,Y> {

    #[inline] fn dynamic_usage(&self) -> usize {

        todo!();
            /*
                return MallocUsage(sizeof(stl_tree_node<std::pair<const X*, Y> >)) * m.size();
            */
    }
}

impl<X, Y> IncrementalDynamicUsage for IndirectMap<X,Y> {

    #[inline] fn incremental_dynamic_usage(&self) -> usize {

        todo!();
            /*
                return MallocUsage(sizeof(stl_tree_node<std::pair<const X*, Y> >));
            */
    }
}

impl<'a,K,T> IndirectMap<K,T> where T: 'a {

    /**
      | passthrough (pointer interface)
      |
      */
    pub fn insert(&mut self, value: &<Self as HasIndirectMapTypes<'a>>::ValueType) -> (<Self as HasIndirectMapTypes<'a>>::Iterator,bool) {
        
        todo!();
        /*
            return m.insert(value);
        */
    }

    /**
      | pass address (value interface)
      |
      */
    pub fn find_mut(&mut self, key: &K) -> <Self as HasIndirectMapTypes<'a>>::Iterator {
        
        todo!();
        /*
            return m.find(&key);
        */
    }
    
    pub fn find(&self, key: &K) -> <Self as HasIndirectMapTypes<'a>>::ConstIterator {
        
        todo!();
        /*
            return m.find(&key);
        */
    }
    
    pub fn lower_bound_mut(&mut self, key: &K) -> <Self as HasIndirectMapTypes<'a>>::Iterator {
        
        todo!();
        /*
            return m.lower_bound(&key);
        */
    }
    
    pub fn lower_bound(&self, key: &K) -> <Self as HasIndirectMapTypes<'a>>::ConstIterator {
        
        todo!();
        /*
            return m.lower_bound(&key);
        */
    }
    
    pub fn erase(&mut self, key: &K) -> <Self as HasIndirectMapTypes<'a>>::SizeType {
        
        todo!();
        /*
            return m.erase(&key);
        */
    }
    
    pub fn count(&self, key: &K) -> <Self as HasIndirectMapTypes<'a>>::SizeType {
        
        todo!();
        /*
            return m.count(&key);
        */
    }

    /* ------------------ passthrough  ------------------ */
    
    pub fn empty(&self) -> bool {
        
        todo!();
        /*
            return m.empty();
        */
    }
    
    pub fn size(&self) -> <Self as HasIndirectMapTypes<'a>>::SizeType {
        
        todo!();
        /*
            return m.size();
        */
    }
    
    pub fn max_size(&self) -> <Self as HasIndirectMapTypes<'a>>::SizeType {
        
        todo!();
        /*
            return m.max_size();
        */
    }
    
    pub fn clear(&mut self)  {
        
        todo!();
        /*
            m.clear();
        */
    }
    
    pub fn begin_mut(&mut self) -> <Self as HasIndirectMapTypes<'a>>::Iterator {
        
        todo!();
        /*
            return m.begin();
        */
    }
    
    pub fn end_mut(&mut self) -> <Self as HasIndirectMapTypes<'a>>::Iterator {
        
        todo!();
        /*
            return m.end();
        */
    }
    
    pub fn begin(&self) -> <Self as HasIndirectMapTypes<'a>>::ConstIterator {
        
        todo!();
        /*
            return m.begin();
        */
    }
    
    pub fn end(&self) -> <Self as HasIndirectMapTypes<'a>>::ConstIterator {
        
        todo!();
        /*
            return m.end();
        */
    }
    
    pub fn cbegin(&self) -> <Self as HasIndirectMapTypes<'a>>::ConstIterator {
        
        todo!();
        /*
            return m.cbegin();
        */
    }
    
    pub fn cend(&self) -> <Self as HasIndirectMapTypes<'a>>::ConstIterator {
        
        todo!();
        /*
            return m.cend();
        */
    }
}
