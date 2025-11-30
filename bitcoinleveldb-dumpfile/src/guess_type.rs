// ---------------- [ File: bitcoinleveldb-dumpfile/src/guess_type.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/db/dumpfile.cc]

pub fn guess_type(
        fname: &String,
        ty:    *mut FileType) -> bool {
    
    todo!();
        /*
            size_t pos = fname.rfind('/');
      std::string basename;
      if (pos == std::string::npos) {
        basename = fname;
      } else {
        basename = std::string(fname.data() + pos + 1, fname.size() - pos - 1);
      }
      uint64_t ignored;
      return ParseFileName(basename, &ignored, type);
        */
}
