// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_drop.rs ]
crate::ix!();

impl Drop for VersionSet {

    fn drop(&mut self) {
        todo!();
        /*
            current_->Unref();
      assert(dummy_versions_.next_ == &dummy_versions_);  // List must be empty
      delete descriptor_log_;
      delete descriptor_file_;
        */
    }
}
