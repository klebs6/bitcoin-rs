// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_builder_maybe_add_file.rs ]
crate::ix!();

impl VersionSetBuilder {
    
    pub fn maybe_add_file(&mut self, 
        v:     *mut Version,
        level: i32,
        f:     *mut FileMetaData)  {
        
        todo!();
        /*
            if (levels_[level].deleted_files.count(f->number) > 0) {
          // File is deleted: do nothing
        } else {
          std::vector<FileMetaData*>* files = &v->files_[level];
          if (level > 0 && !files->empty()) {
            // Must not overlap
            assert(vset_->icmp_.Compare((*files)[files->size() - 1]->largest,
                                        f->smallest) < 0);
          }
          f->refs++;
          files->push_back(f);
        }
        */
    }
}
