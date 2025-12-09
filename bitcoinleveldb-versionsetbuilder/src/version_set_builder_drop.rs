// ---------------- [ File: bitcoinleveldb-versionsetbuilder/src/version_set_builder_drop.rs ]
crate::ix!();

impl Drop for VersionSetBuilder {

    fn drop(&mut self) {
        todo!();
        /*
            for (int level = 0; level < config::NUM_LEVELS; level++) {
          const VersionSetBuilderFileSet* added = levels_[level].added_files;
          std::vector<FileMetaData*> to_unref;
          to_unref.reserve(added->size());
          for (VersionSetBuilderFileSet::const_iterator it = added->begin(); it != added->end();
               ++it) {
            to_unref.push_back(*it);
          }
          delete added;
          for (uint32_t i = 0; i < to_unref.size(); i++) {
            FileMetaData* f = to_unref[i];
            f->refs--;
            if (f->refs <= 0) {
              delete f;
            }
          }
        }
        base_->Unref();
        */
    }
}
