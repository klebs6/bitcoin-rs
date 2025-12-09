// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_get_range.rs ]
crate::ix!();

impl VersionSetGetRange for VersionSet {

    /**
      | Stores the minimal range that covers all
      | entries in inputs in *smallest, *largest.
      |
      | REQUIRES: inputs is not empty
      */
    fn get_range(&mut self, 
        inputs:   &Vec<*mut FileMetaData>,
        smallest: *mut InternalKey,
        largest:  *mut InternalKey)  {
        
        todo!();
        /*
            assert(!inputs.empty());
      smallest->Clear();
      largest->Clear();
      for (size_t i = 0; i < inputs.size(); i++) {
        FileMetaData* f = inputs[i];
        if (i == 0) {
          *smallest = f->smallest;
          *largest = f->largest;
        } else {
          if (icmp_.Compare(f->smallest, *smallest) < 0) {
            *smallest = f->smallest;
          }
          if (icmp_.Compare(f->largest, *largest) > 0) {
            *largest = f->largest;
          }
        }
      }
        */
    }

    /**
      | Stores the minimal range that covers all
      | entries in inputs1 and inputs2 in *smallest,
      | *largest.
      |
      | REQUIRES: inputs is not empty
      */
    fn get_range2(&mut self, 
        inputs1:  &Vec<*mut FileMetaData>,
        inputs2:  &Vec<*mut FileMetaData>,
        smallest: *mut InternalKey,
        largest:  *mut InternalKey)  {
        
        todo!();
        /*
            std::vector<FileMetaData*> all = inputs1;
      all.insert(all.end(), inputs2.begin(), inputs2.end());
      GetRange(all, smallest, largest);
        */
    }
}
