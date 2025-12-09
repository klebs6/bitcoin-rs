// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_approximate_offset_of.rs ]
crate::ix!();

impl ApproximateOffsetOf for VersionSet {
    
    /**
      | Return the approximate offset in the
      | database of the data for "key" as of version
      | "v".
      |
      */
    fn approximate_offset_of(
        &mut self, 
        v:    *mut Version,
        ikey_: &InternalKey) -> u64 {
        
        todo!();
        /*
            uint64_t result = 0;
      for (int level = 0; level < config::NUM_LEVELS; level++) {
        const std::vector<FileMetaData*>& files = v->files_[level];
        for (size_t i = 0; i < files.size(); i++) {
          if (icmp_.Compare(files[i]->largest, ikey) <= 0) {
            // Entire file is before "ikey", so just add the file size
            result += files[i]->file_size;
          } else if (icmp_.Compare(files[i]->smallest, ikey) > 0) {
            // Entire file is after "ikey", so ignore
            if (level > 0) {
              // Files other than level 0 are sorted by meta->smallest, so
              // no further files in this level will contain data for
              // "ikey".
              break;
            }
          } else {
            // "ikey" falls in the range for this table.  Add the
            // approximate offset of "ikey" within the table.
            Table* tableptr;
            Iterator* iter = table_cache_->NewIterator(
                ReadOptions(), files[i]->number, files[i]->file_size, &tableptr);
            if (tableptr != nullptr) {
              result += tableptr->ApproximateOffsetOf(ikey.Encode());
            }
            delete iter;
          }
        }
      }
      return result;
        */
    }
}
