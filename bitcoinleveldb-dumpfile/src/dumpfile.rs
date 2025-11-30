// ---------------- [ File: bitcoinleveldb-dumpfile/src/dumpfile.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/leveldb/include/leveldb/dumpfile.h]

/**
  | Dump the contents of the file named by fname in
  | text format to *dst.  Makes a sequence of
  | dst->Append() calls; each call is passed the
  | newline-terminated text corresponding to
  | a single item found in the file.
  |
  | Returns a non-OK result if fname does not name
  | a leveldb storage file, or if the file cannot
  | be read.
  */
pub fn dump_file(
        env:   Rc<RefCell<dyn Env>>,
        fname: &String,
        dst:   *mut dyn WritableFile) -> crate::Status {
    
    todo!();
        /*
            FileType ftype;
      if (!GuessType(fname, &ftype)) {
        return Status::InvalidArgument(fname + ": unknown file type");
      }
      switch (ftype) {
        case kLogFile:
          return DumpLog(env, fname, dst);
        case kDescriptorFile:
          return DumpDescriptor(env, fname, dst);
        case kTableFile:
          return DumpTable(env, fname, dst);
        default:
          break;
      }
      return Status::InvalidArgument(fname + ": not a dump-able file type");
        */
}
