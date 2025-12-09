// ---------------- [ File: bitcoinleveldb-modeldb/src/iter.rs ]
crate::ix!();

pub struct ModelIter<'a> {
    map:   *const KVMap,

    /**
      | Do we own map_
      |
      */
    owned: bool,

    iter:  Box<KVMapConstIterator<'a>>,
}

impl<'a> Drop for ModelIter<'a> {
    fn drop(&mut self) {
        todo!();
        /*
            if (owned_) delete map_;
        */
    }
}

impl<'a> ModelIter<'a> {
    
    pub fn new(
        map:   *const KVMap,
        owned: bool) -> Self {
    
        todo!();
        /*


            : map_(map), owned_(owned), iter_(map_->end())
        */
    }
    
    pub fn valid(&self) -> bool {
        
        todo!();
        /*
            return iter_ != map_->end();
        */
    }
    
    pub fn seek_to_first(&mut self)  {
        
        todo!();
        /*
            iter_ = map_->begin();
        */
    }
    
    pub fn seek_to_last(&mut self)  {
        
        todo!();
        /*
            if (map_->empty()) {
            iter_ = map_->end();
          } else {
            iter_ = map_->find(map_->rbegin()->first);
          }
        */
    }
    
    pub fn seek(&mut self, k: &Slice)  {
        
        todo!();
        /*
            iter_ = map_->lower_bound(k.ToString());
        */
    }
    
    pub fn next(&mut self)  {
        
        todo!();
        /*
            ++iter_;
        */
    }
    
    pub fn prev(&mut self)  {
        
        todo!();
        /*
            --iter_;
        */
    }
    
    pub fn key(&self) -> Slice {
        
        todo!();
        /*
            return iter_->first;
        */
    }
    
    pub fn value(&self) -> Slice {
        
        todo!();
        /*
            return iter_->second;
        */
    }
    
    pub fn status(&self) -> crate::Status {
        
        todo!();
        /*
            return Status::OK();
        */
    }
}
