// ---------------- [ File: bitcoinleveldb-dumpfile/src/version_edit_printer.rs ]
crate::ix!();

/**
  | Called on every log record (each one
  | of which is a WriteBatch) found in a kDescriptorFile.
  |
  */
pub fn version_edit_printer(
        pos:    u64,
        record: Slice,
        dst:    *mut dyn WritableFile)  {
    
    todo!();
        /*
            std::string r = "--- offset ";
      AppendNumberTo(&r, pos);
      r += "; ";
      VersionEdit edit;
      Status s = edit.DecodeFrom(record);
      if (!s.ok()) {
        r += s.ToString();
        r.push_back('\n');
      } else {
        r += edit.DebugString();
      }
      dst->Append(r);
        */
}
