// ---------------- [ File: bitcoin-argsman/src/get_blocks_dir_path.rs ]
crate::ix!();

pub fn strip_redundant_last_elements_of_path(path: &mut PathBuf) {
    
    let mut result = path.clone();

    while result.file_name() == Some(OsStr::new(".")) {
        result.pop();
    }

    assert!(is_same_file(&result,&path).unwrap());

    *path = result;
}

impl ArgsManagerInner {

    /**
      | Get blocks directory path
      | 
      | -----------
      | @return
      | 
      | Blocks path which is network specific
      |
      */
    pub fn get_blocks_dir_path(&self) -> Box<Path> {

        let maybe_cached = self.cached_blocks_path.as_ref();

        // Cache the path to avoid calling
        // fs::create_directories on every call of
        // this function
        if let Some(path) = maybe_cached {

            //in c++ we just returned a reference
            //to path.
            //
            //depending on how frequently this is
            //called, we may not want to alway
            //clone
            //
            //however, this function either
            //generates a new boxed path or
            //returns one which already exists
            //
            //currently, we clone the one which
            //already exists in this branch
            //
            //if we want to avoid this clone, we
            //could consider using an Rc
            return path.clone();
        }

        let mut buf: PathBuf = PathBuf::new();

        if self.is_arg_set("-blocksdir") {

            let arg = self.get_arg("-blocksdir","");

            let arg_path = std::fs::canonicalize(Path::new(&arg)).unwrap();

            buf.push(arg_path);

            if !buf.as_path().is_dir() {
                buf.clear();
                return buf.into_boxed_path();
            }

        } else {
            buf.push(self.get_data_dir_base());
        }

        buf.push(base_params().data_dir());

        buf.push("blocks");

        std::fs::create_dir_all(buf.as_path());

        strip_redundant_last_elements_of_path(&mut buf);

        buf.into_boxed_path()
    }
}
