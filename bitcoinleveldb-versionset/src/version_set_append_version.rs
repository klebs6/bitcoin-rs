// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_append_version.rs ]
crate::ix!();

impl AppendVersion for VersionSet {
    
    fn append_version(&mut self, v: *mut Version)  {
        
        todo!();
        /*
            // Make "v" current
      assert(v->refs_ == 0);
      assert(v != current_);
      if (current_ != nullptr) {
        current_->Unref();
      }
      current_ = v;
      v->Ref();

      // Append to linked list
      v->prev_ = dummy_versions_.prev_;
      v->next_ = &dummy_versions_;
      v->prev_->next_ = v;
      v->next_->prev_ = v;
        */
    }
}
