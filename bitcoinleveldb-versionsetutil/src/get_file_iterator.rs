// ---------------- [ File: bitcoinleveldb-versionsetutil/src/get_file_iterator.rs ]
crate::ix!();

pub fn get_file_iterator(
        arg:        *mut c_void,
        options:    &ReadOptions,
        file_value: &Slice) -> *mut LevelDBIterator {
    
    todo!();
        /*
            TableCache* cache = reinterpret_cast<TableCache*>(arg);
      if (file_value.size() != 16) {
        return NewErrorIterator(
            Status::Corruption("FileReader invoked with unexpected value"));
      } else {
        return cache->NewIterator(options, DecodeFixed64(file_value.data()),
                                  DecodeFixed64(file_value.data() + 8));
      }
        */
}
