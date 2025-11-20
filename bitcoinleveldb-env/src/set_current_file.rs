// ---------------- [ File: bitcoinleveldb-env/src/set_current_file.rs ]
crate::ix!();

/**
  | Make the CURRENT file point to the descriptor
  | file with the specified number.
  |
  */
pub fn set_current_file(
        env:               Rc<RefCell<dyn Env>>,
        dbname:            &String,
        descriptor_number: u64) -> crate::Status {
    
    todo!();
        /*
      // Remove leading "dbname/" and add newline to manifest file name
      std::string manifest = DescriptorFileName(dbname, descriptor_number);
      Slice contents = manifest;
      assert(contents.starts_with(dbname + "/"));
      contents.remove_prefix(dbname.size() + 1);
      std::string tmp = TempFileName(dbname, descriptor_number);
      Status s = WriteStringToFileSync(env, contents.ToString() + "\n", tmp);
      if (s.ok()) {
        s = env->RenameFile(tmp, CurrentFileName(dbname));
      }
      if (!s.ok()) {
        env->DeleteFile(tmp);
      }
      return s;
        */
}
