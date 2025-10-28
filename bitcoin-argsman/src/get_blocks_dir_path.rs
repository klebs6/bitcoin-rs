// ---------------- [ File: bitcoin-argsman/src/get_blocks_dir_path.rs ]
crate::ix!();

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
            let arg_path = Path::new(&arg);
            // Canonicalize if it exists; otherwise use it as provided.
            let abs = std::fs::canonicalize(arg_path).unwrap_or_else(|_| arg_path.to_path_buf());

            buf.push(abs);

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    static M: OnceLock<Mutex<()>> = OnceLock::new();
    fn lock() -> std::sync::MutexGuard<'static,()> { M.get_or_init(|| Mutex::new(())).lock().unwrap() }

    #[test]
    fn blocks_dir_derived_from_datadir() {
        let _g = lock();

        // temp datadir
        let tmp = tempfile::tempdir().unwrap();
        let datadir = tmp.path().join("data");
        fs::create_dir_all(&datadir).unwrap();

        // Prepare inner
        let mut inner = ArgsManagerInner::default();
        inner.force_set_arg("-datadir", datadir.to_str().unwrap());
        // Ensure base params are set (MAIN)
        select_base_params(base_chain_params::MAIN);

        let blocks = inner.get_blocks_dir_path();
        // MAIN has empty subdir; expect <datadir>/blocks
        assert!(blocks.ends_with("blocks"), "{:?}", blocks);
        assert!(blocks.as_ref().exists());
    }
}
