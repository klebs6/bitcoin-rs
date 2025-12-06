crate::ix!();

impl Version {

    /**
      | Reference count management (so Versions
      | do not disappear out from under live
      | iterators)
      |
      */
    pub fn ref_(&mut self)  {
        
        todo!();
        /*
            ++refs_;
        */
    }
    
    pub fn unref(&mut self)  {
        
        todo!();
        /*
            assert(this != &vset_->dummy_versions_);
      assert(refs_ >= 1);
      --refs_;
      if (refs_ == 0) {
        delete this;
      }
        */
    }
}
